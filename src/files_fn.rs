use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[allow(unused)]
pub fn move_file() {
    let mut src = String::new();
    let mut dst = String::new();

    print!("Enter source file path: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut src).unwrap();

    print!("Enter destination file path: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut dst).unwrap();

    let src = src.trim();
    let dst = dst.trim();

    match move_f(src, dst) {
        Ok(_) => println!("File moved successfully from '{}' to '{}'", src, dst),
        Err(e) => eprintln!("Failed to move file: {}", e),
    }

    fn move_f(src: &str, dst: &str) -> io::Result<()> {
        let src_path = Path::new(src);

        let mut dst_path = PathBuf::from(dst);
        if dst_path.is_dir() {
            if let Some(fname) = src_path.file_name() {
                dst_path.push(fname);
            }
        }

        match fs::rename(&src_path, &dst_path) {
            Ok(_) => Ok(()),
            Err(_) => {
                fs::copy(&src_path, &dst_path)?;
                fs::remove_file(&src_path)?;
                Ok(())
            }
        }
    }
}
