use std::env;
use std::io;
use std::time::UNIX_EPOCH;
use fs_set_times::{SystemTimeSpec, set_times};
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <dir_to_walk>", args[0]);
        std::process::exit(1);
    }

    let dir_to_walk = &args[1];

    const TIME: Option<SystemTimeSpec> = Some(SystemTimeSpec::Absolute(UNIX_EPOCH));
    for entry in WalkDir::new(dir_to_walk) {
        let entry = entry?;
        let entry_type = entry.file_type();
        if entry_type.is_file() || entry_type.is_symlink() || entry_type.is_dir() {
            set_times(entry.path(), TIME, TIME)?;
        }
    }

    Ok(())
}
