use std::{env, fs, io::{Write, Seek, SeekFrom}, path::PathBuf};

fn create_bound(input_brew: &str, output_qrew: &str) -> anyhow::Result<()> {
    let stub_bin = include_bytes!("../rew-qrew-stub/target/release/rew-qrew-stub"); // Or use Path
    let script = fs::read(input_brew)?;
    let script_len = script.len() as u64;

    let mut output = fs::File::create(output_qrew)?;
    output.write_all(stub_bin)?;          // Copy the runtime
    output.write_all(&script)?;           // Append the script
    output.write_all(&script_len.to_le_bytes())?; // 8-byte length

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = output.metadata()?.permissions();
        perms.set_mode(0o755); // Make executable
        fs::set_permissions(output_qrew, perms)?;
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: rew create-bound <input.brew> -o <output.qrew>");
        std::process::exit(1);
    }

    let input = &args[1];
    let output = &args[2];

    create_bound(input, output)
}

