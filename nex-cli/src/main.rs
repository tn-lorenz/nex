use std::fs;
use std::io;
use std::path::Path;

fn read_file(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}

fn main() -> io::Result<()> {
    let filename = "hello.nx";
    let content = read_file(Path::new(filename))?;
    println!("Dateiinhalt:\n{}", content);
    Ok(())
}
