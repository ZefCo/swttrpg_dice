// use std::path::Path;
use std::env;
use std::path::PathBuf;


pub fn cwd() -> std::io::Result<PathBuf> {
    let path = env::current_dir()?;
    // println!("The cwd is {}", path.display());
    Ok(path)
}

pub fn adjecent_file(file: &str, opt_path: Option<&str>, opt_root: Option<PathBuf>) -> PathBuf {
    let path = opt_path.unwrap_or("");
    let root = opt_root.unwrap_or(cwd().expect("Incorrect Permissions"));

    // println!("The inputs are: file = {0}, subpath = {1}, root = {2}", file, path, root.display());

    let rel_path: PathBuf = [path, file].iter().collect(); 
    let abs_path = root.join(rel_path);

    if abs_path.exists() {
        // println!("The path is {}", abs_path.display());
        return abs_path
    } else {
        panic!("{} does not exist, did the file get moved?", abs_path.display())
    }
}