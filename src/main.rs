use std::error::Error;
use std::fs;
use std::path::Path;

use clap::Parser;

use patches::apply_patches;

mod patches;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long, default_value = r"C:\Program Files\AMD\RyzenMaster\bin\AMD Ryzen Master.exe")]
    path: String,
    #[arg(short, long, default_value = r"C:\Program Files\AMD\RyzenMaster\bin\Patched AMD Ryzen Master.exe")]
    output: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let exe_path = Path::new(&args.path);

    if !exe_path.exists() {
        eprintln!("File not found: {:?}", exe_path);
        return Ok(());
    }

    println!("Patching: {:?}", exe_path);

    let mut exe_data = fs::read(exe_path)?;

    let some_patches_applied =
        apply_patches(&mut exe_data);

    if !some_patches_applied {
        println!("Nothing to patch");
        return Ok(());
    }

    println!("Writing patched file: {:?}", args.output);
    fs::write(&args.output, &exe_data)?;

    Ok(())
}
