# SimpleFS

A minimal, real‑filesystem, interactive file system implemented in Rust.

## Features

- Create, read, write, delete files
- List directory contents
- Automatically creates parent directories
- Works with real files on disk (no in‑memory simulation)
- Simple interactive shell

## Usage

```bash
cargo run
```

## Commands
create <path> [content]   – create a new file
read   <path>             – print file content
write  <path> <content>   – overwrite an existing file
delete <path>             – remove a file or directory
list   [dir]              – show contents (default: current dir)
exit / quit               – exit the shell

## Example
> create docs/foo.txt Hello
> read docs/foo.txt
Content: Hello
> write docs/foo.txt HelloWorld
> list docs
  foo.txt
> delete docs/foo.txt

## License
MIT License