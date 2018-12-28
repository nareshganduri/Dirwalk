# Dirwalk

A little (useless) Rust utility for creating an iterator that recursively 
walks through a directory tree and returns the pathnames of all 
subdirectories and files in the starting directory.

The directory tree is traversed via depth-first search.

## Usage
See [main.rs](src/main.rs) for example usage.

```rust
extern crate dirwalk;

let tree = dirwalk::dir_walk("/my/example/directory");

for entry in tree {
    println!("{}", entry);
}
```
