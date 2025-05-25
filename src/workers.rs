use crate::ext::{console, ffi, url, web, webidl};
use crate::runtime::{get_rew_runtime, RewRuntime};
use anyhow::Result;
use deno_core::error::CoreError;
use deno_core::{op2, OpState};
use deno_core::{JsRuntime, RuntimeOptions};
use serde_json::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{
  mpsc::{channel, Receiver, Sender},
  Arc, Mutex,
};
use tokio::runtime::{Builder, Runtime};
use uuid::Uuid;

/// Represents a handle to a worker thread
struct WorkerHandle {
  to_worker: Sender<serde_json::Value>,
  from_worker: Arc<Mutex<Receiver<serde_json::Value>>>,
}

lazy_static::lazy_static! {
  static ref WORKER_REGISTRY: Arc<Mutex<HashMap<String, WorkerHandle>>> = Arc::new(Mutex::new(HashMap::new()));
}

#[op2]
#[string]
pub fn op_thread_spawn(
  #[string] source: String,
  state: Rc<RefCell<OpState>>,
) -> Result<String, CoreError> {
  let (to_worker_tx, to_worker_rx) = channel::<Value>();
  let (from_worker_tx, from_worker_rx) = channel::<Value>();

  let id = Uuid::new_v4().to_string();
  let worker_id = id.clone();

  // Spawn thread
  std::thread::spawn(move || {
    // Create a Tokio runtime for this thread
    let rt = match Builder::new_current_thread()
      .enable_all() // Enable all Tokio features including time
      .build()
    {
      Ok(rt) => rt,
      Err(e) => {
        eprintln!("Failed to create Tokio runtime: {}", e);
        return;
      }
    };

    // Enter the runtime context
    let _guard = rt.enter();

    // Create RewRuntime inside the Tokio runtime context
    let runtime_result = rt.block_on(async {
      // Create RewRuntime
      let mut runtime = match get_rew_runtime() {
        Ok(rt) => rt,
        Err(e) => {
          eprintln!("Failed to create RewRuntime: {}", e);
          return Err(e);
        }
      };

      // Store the worker ID and sender in the runtime state
      {
        let op_state = runtime.op_state();
        let mut op_state = op_state.borrow_mut();
        op_state.put(worker_id.clone());
        op_state.put(from_worker_tx);
      }

      // Initialize the worker with postMessage function
      if let Err(e) = runtime.execute_script(
        "<init>",
        r#"
              globalThis.onmessage = () => {};
              globalThis.postMessage = function (msg) {
                rew.ops.op_thread_post_message(JSON.stringify(msg));
              };
          "#,
      ) {
        eprintln!("Failed to initialize worker: {}", e);
        return Err(anyhow::anyhow!("Failed to initialize worker: {}", e));
      }

      // Load worker source
      if let Err(e) = runtime.execute_script(
        "<worker>",
        format!(
          r#"
            rew.prototype.mod.prototype.defineNew("<worker::{}>", function(ctx){{
              ctx.onmessage = (fn) => globalThis.onmessage = fn;
              ctx.postMessage = (msg) => globalThis.postMessage(msg);
              with({{...ctx, globalThis: {{}}}}){{  
                {}
              }}
            }}, ["::pvt"]);
          "#,
          worker_id.clone(),
          source.clone()
        ),
      ) {
        eprintln!("Failed to execute worker script: {}", e);
        return Err(anyhow::anyhow!("Failed to execute worker script: {}", e));
      }

      Ok(runtime)
    });

    let mut runtime = match runtime_result {
      Ok(rt) => rt,
      Err(e) => {
        eprintln!("Failed to initialize worker runtime: {}", e);
        return;
      }
    };

    // Process messages in the Tokio runtime
    rt.block_on(async {
      // Loop: receive and inject messages
      for msg in to_worker_rx {
        let msg_json = match serde_json::to_string(&msg) {
          Ok(json) => json,
          Err(e) => {
            eprintln!("Failed to serialize message: {}", e);
            continue;
          }
        };

        let js = format!("onmessage({});", msg_json);
        if let Err(e) = runtime.execute_script("<message>", js.clone()) {
          eprintln!("Failed to process message: {}", e);
          // Continue processing other messages even if one fails
        }

        // Give the event loop a chance to process events
        tokio::task::yield_now().await;
      }
    });
  });

  WORKER_REGISTRY.lock().unwrap().insert(
    id.clone(),
    WorkerHandle {
      to_worker: to_worker_tx,
      from_worker: Arc::new(Mutex::new(from_worker_rx)),
    },
  );

  Ok(id)
}

#[op2]
#[string]
pub fn op_thread_message(
  #[string] thread_id: String,
  #[string] msg: String,
  _: Rc<RefCell<OpState>>,
) -> Result<String, CoreError> {
  let registry = WORKER_REGISTRY.lock().unwrap();

  if let Some(handle) = registry.get(&thread_id) {
    handle
      .to_worker
      .send(serde_json::from_str(&msg).unwrap())
      .map_err(|e| {
        CoreError::Io(std::io::Error::new(
          std::io::ErrorKind::Other,
          e.to_string(),
        ))
      })?;
    Ok("".to_string())
  } else {
    Err(CoreError::Io(std::io::Error::new(
      std::io::ErrorKind::InvalidData,
      "Thread not found",
    )))
  }
}

#[op2]
#[string]
pub fn op_thread_terminate(
  #[string] thread_id: String,
  _: Rc<RefCell<OpState>>,
) -> Result<String, CoreError> {
  let mut registry = WORKER_REGISTRY.lock().unwrap();

  if registry.remove(&thread_id).is_some() {
    // Removing the handle will cause the channel to close,
    // which will terminate the message loop in the worker
    Ok("".to_string())
  } else {
    println!("Thread termination failed");
    Err(CoreError::Io(std::io::Error::new(
      std::io::ErrorKind::InvalidData,
      "Thread not found",
    )))
  }
}

#[op2]
#[string]
pub fn op_thread_post_message(
  #[string] message: String,
  state: Rc<RefCell<OpState>>,
) -> Result<String, CoreError> {
  // Parse the message from JSON string
  let msg: Value = serde_json::from_str(&message).map_err(|e| {
    CoreError::Io(std::io::Error::new(
      std::io::ErrorKind::InvalidData,
      e.to_string(),
    ))
  })?;

  // Get the worker ID and sender from the state
  let state = state.borrow();
  let worker_id = state.borrow::<String>();
  let sender = state.borrow::<Sender<Value>>();

  // Send the message back to the main thread
  sender.send(msg).map_err(|e| {
    CoreError::Io(std::io::Error::new(
      std::io::ErrorKind::BrokenPipe,
      e.to_string(),
    ))
  })?;

  Ok("".to_string())
}

#[op2]
#[serde]
pub fn op_thread_receive(
  #[string] thread_id: String,
  #[serde] timeout_ms: Option<u64>,
  _: Rc<RefCell<OpState>>,
) -> Result<Option<Value>, CoreError> {
  let registry = WORKER_REGISTRY.lock().unwrap();

  if let Some(handle) = registry.get(&thread_id) {
    let receiver = handle.from_worker.lock().unwrap();

    // If timeout is provided, wait for that duration
    if let Some(timeout) = timeout_ms {
      use std::time::Duration;
      match receiver.recv_timeout(Duration::from_millis(timeout)) {
        Ok(msg) => Ok(Some(msg)),
        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => Ok(None),
        Err(e) => Err(CoreError::Io(std::io::Error::new(
          std::io::ErrorKind::Other,
          e.to_string(),
        ))),
      }
    } else {
      // No timeout, try to receive immediately
      match receiver.try_recv() {
        Ok(msg) => Ok(Some(msg)),
        Err(std::sync::mpsc::TryRecvError::Empty) => Ok(None),
        Err(e) => Err(CoreError::Io(std::io::Error::new(
          std::io::ErrorKind::Other,
          e.to_string(),
        ))),
      }
    }
  } else {
    Err(CoreError::Io(std::io::Error::new(
      std::io::ErrorKind::InvalidData,
      "Thread not found",
    )))
  }
}
