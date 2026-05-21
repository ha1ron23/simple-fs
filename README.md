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
| Command | Description | Example |
| :------ | :---------- | :------ |
| `create <path> [content]` | Create a new file | `create docs/foo.txt Hello` |
| `read <path>` | Print file content | `read docs/foo.txt` |
| `write <path> <content>` | Overwrite an existing file | `write docs/foo.txt HelloWorld` |
| `delete <path>` | Delete a file or directory | `delete docs/foo.txt` |
| `list [dir]` | List directory contents (default: current dir) | `list docs` |

## License
MIT License
