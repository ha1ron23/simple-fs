use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Debug)]
enum FsError {
    NotFound,
    AlreadyExists,
    IsDirectory,
    IoError(String),
}

type Result<T> = std::result::Result<T, FsError>;

trait FileSystem {
    fn create(&mut self, path: &str, content: Vec<u8>) -> Result<()>;
    fn read(&self, path: &str) -> Result<Vec<u8>>;
    fn write(&mut self, path: &str, content: Vec<u8>) -> Result<()>;
    fn delete(&mut self, path: &str) -> Result<()>;
    fn list(&self, dir: &str) -> Result<Vec<String>>;
}

struct RealFs {
    root: PathBuf,
}

impl RealFs {
    fn new(root: PathBuf) -> Self {
        fs::create_dir_all(&root).unwrap();
        RealFs { root }
    }

    fn full_path(&self, virt_path: &str) -> PathBuf {
        let cleaned = virt_path.trim_start_matches('/');
        if cleaned.is_empty() {
            self.root.clone()
        } else {
            self.root.join(cleaned)
        }
    }
}

impl FileSystem for RealFs {
    fn create(&mut self, path: &str, content: Vec<u8>) -> Result<()> {
        let full = self.full_path(path);
        if full.exists() {
            return Err(FsError::AlreadyExists);
        }
        if let Some(parent) = full.parent() {
            fs::create_dir_all(parent).map_err(|e| FsError::IoError(e.to_string()))?;
        }
        fs::write(&full, content).map_err(|e| FsError::IoError(e.to_string()))?;
        Ok(())
    }

    fn read(&self, path: &str) -> Result<Vec<u8>> {
        let full = self.full_path(path);
        if !full.exists() {
            return Err(FsError::NotFound);
        }
        if full.is_dir() {
            return Err(FsError::IsDirectory);
        }
        fs::read(&full).map_err(|e| FsError::IoError(e.to_string()))
    }

    fn write(&mut self, path: &str, content: Vec<u8>) -> Result<()> {
        let full = self.full_path(path);
        if !full.exists() {
            return Err(FsError::NotFound);
        }
        if full.is_dir() {
            return Err(FsError::IsDirectory);
        }
        fs::write(&full, content).map_err(|e| FsError::IoError(e.to_string()))?;
        Ok(())
    }

    fn delete(&mut self, path: &str) -> Result<()> {
        let full = self.full_path(path);
        if !full.exists() {
            return Err(FsError::NotFound);
        }
        if full.is_dir() {
            fs::remove_dir_all(&full).map_err(|e| FsError::IoError(e.to_string()))?;
        } else {
            fs::remove_file(&full).map_err(|e| FsError::IoError(e.to_string()))?;
        }
        Ok(())
    }

    fn list(&self, dir: &str) -> Result<Vec<String>> {
        let full = self.full_path(dir);
        if !full.exists() {
            return Err(FsError::NotFound);
        }
        if full.is_file() {
            return Err(FsError::IsDirectory);
        }
        let entries = fs::read_dir(&full).map_err(|e| FsError::IoError(e.to_string()))?;
        let mut names = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|e| FsError::IoError(e.to_string()))?;
            if let Some(name) = entry.file_name().to_str() {
                names.push(name.to_string());
            }
        }
        names.sort();
        Ok(names)
    }
}

fn main() {
    let root = std::env::current_dir().expect("Can't get current dir");
    let mut fs = RealFs::new(root);
    let mut input = String::new();

    println!("SimpleFS (real filesystem) interactive shell");
    println!("Working directory: {}", fs.root.display());
    println!("Commands: create <path> [content], read <path>, write <path> <content>, delete <path>, list [dir], exit");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "create" => {
                if parts.len() < 2 {
                    println!("Usage: create <path> [text content]");
                    continue;
                }
                let path = parts[1];
                let content = if parts.len() > 2 {
                    parts[2..].join(" ").into_bytes()
                } else {
                    b"".to_vec()
                };
                match fs.create(path, content) {
                    Ok(()) => println!("File '{}' created", path),
                    Err(FsError::AlreadyExists) => println!("File already exists"),
                    Err(FsError::NotFound) => println!("Path not found"),
                    Err(FsError::IoError(e)) => println!("IO error: {}", e),
                    _ => println!("Error"),
                }
            }
            "read" => {
                if parts.len() < 2 {
                    println!("Usage: read <path>");
                    continue;
                }
                let path = parts[1];
                match fs.read(path) {
                    Ok(data) => {
                        let text = String::from_utf8_lossy(&data);
                        println!("Content: {}", text);
                    }
                    Err(FsError::NotFound) => println!("File not found"),
                    Err(FsError::IsDirectory) => println!("Path is a directory"),
                    Err(FsError::IoError(e)) => println!("IO error: {}", e),
                    _ => println!("Error"),
                }
            }
            "write" => {
                if parts.len() < 3 {
                    println!("Usage: write <path> <new content>");
                    continue;
                }
                let path = parts[1];
                let content = parts[2..].join(" ").into_bytes();
                match fs.write(path, content) {
                    Ok(()) => println!("File '{}' written", path),
                    Err(FsError::NotFound) => println!("File not found"),
                    Err(FsError::IsDirectory) => println!("Path is a directory"),
                    Err(FsError::IoError(e)) => println!("IO error: {}", e),
                    _ => println!("Error"),
                }
            }
            "delete" => {
                if parts.len() < 2 {
                    println!("Usage: delete <path>");
                    continue;
                }
                let path = parts[1];
                match fs.delete(path) {
                    Ok(()) => println!("Deleted '{}'", path),
                    Err(FsError::NotFound) => println!("File/dir not found"),
                    Err(FsError::IoError(e)) => println!("IO error: {}", e),
                    _ => println!("Error"),
                }
            }
            "list" => {
                let dir = if parts.len() > 1 { parts[1] } else { "" };
                match fs.list(dir) {
                    Ok(entries) => {
                        if entries.is_empty() {
                            println!("(empty)");
                        } else {
                            for entry in entries {
                                println!("  {}", entry);
                            }
                        }
                    }
                    Err(FsError::NotFound) => println!("Directory not found"),
                    Err(FsError::IsDirectory) => println!("Path is a file"),
                    Err(FsError::IoError(e)) => println!("IO error: {}", e),
                    _ => println!("Error"),
                }
            }
            "exit" | "quit" => break,
            _ => println!("Unknown command"),
        }
    }
}