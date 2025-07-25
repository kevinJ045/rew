use anyhow::Result;
use deno_core::{Extension, OpState, op2, v8};
use std::sync::{Arc, Mutex};
use tokio::time::{Duration, sleep};

struct LoopState {
  running: bool,
  main_cb: Option<v8::Global<v8::Function>>,
  live_cb: Option<v8::Global<v8::Function>>,
}

impl Default for LoopState {
  fn default() -> Self {
    Self {
      running: false,
      main_cb: None,
      live_cb: None,
    }
  }
}

#[op2]
#[serde]
fn op_loop_start(state: &mut OpState) -> Result<()> {
  let arc = state.borrow::<Arc<Mutex<LoopState>>>().clone();
  let handle = state.thread_safe_handle();

  {
    let mut guard = arc.lock().unwrap();
    guard.running = true;
  }

  tokio::spawn(async move {
    while arc.lock().unwrap().running {
      // Call the live callback
      handle
        .post_task(|scope, state| {
          let loop_state = state.borrow::<Arc<Mutex<LoopState>>>();
          let guard = loop_state.lock().unwrap();
          if let Some(ref cb) = guard.live_cb {
            let func = v8::Local::new(scope, cb);
            let this = v8::undefined(scope).into();
            let _ = func.call(scope, this, &[]);
          }
        })
        .unwrap();

      // Sleep to simulate frame delay (60 FPS)
      sleep(Duration::from_millis(16)).await;
    }
  });

  Ok(())
}

#[op2(fast)]
fn op_loop_stop(state: &mut OpState) {
  let arc = state.borrow::<Arc<Mutex<LoopState>>>().clone();
  arc.lock().unwrap().running = false;
}

#[op2(fast)]
fn op_loop_set_live_cb(
  scope: &mut v8::HandleScope,
  cb: v8::Local<v8::Function>,
  state: &mut OpState,
) {
  let arc = state.borrow::<Arc<Mutex<LoopState>>>().clone();
  arc.lock().unwrap().live_cb = Some(v8::Global::new(scope, cb));
}

pub fn init_loop_ext() -> Extension {
  Extension::builder("loop_ext")
    .state(|state| {
      state.put(Arc::new(Mutex::new(LoopState::default())));
      Ok(())
    })
    .ops(vec![
      op_loop_start::decl(),
      op_loop_stop::decl(),
      op_loop_set_live_cb::decl(),
    ])
    .build()
}
