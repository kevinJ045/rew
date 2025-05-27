use std::{env, fs::File, io::{Read, Seek, SeekFrom}, path::PathBuf};
use rew::runtime::{add_virtual_file, RewRuntime};

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    let exe_path = env::current_exe()?;
    let mut file = File::open(&exe_path)?;
    let file_len = file.metadata()?.len();

    file.seek(SeekFrom::End(-8))?;
    let mut len_buf = [0u8; 8];
    file.read_exact(&mut len_buf)?;
    let script_len = u64::from_le_bytes(len_buf);

    file.seek(SeekFrom::End(-(8 + script_len as i64)))?;
    let mut script = vec![0u8; script_len as usize];
    file.read_exact(&mut script)?;
	
    let code = String::from_utf8(script)?;
    let mut runtime = RewRuntime::new(std::env::args.collect()).unwrap();

    let mut exe_vpath = exe_path.clone();
    exe_vpath.set_extension("brew");
    

    add_virtual_file(exe_path.to_str().unwrap(), &code.clone().as_str());

	runtime.run_file(exe_path).await?;
	
	Ok(())
}
