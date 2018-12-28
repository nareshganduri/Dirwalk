use dirwalk;

use std::io;
use std::io::Write;

fn get_dir() -> String {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut s = String::new();

    print!("Enter a directory to traverse: ");
    stdout.flush().unwrap();

    stdin.read_line(&mut s).unwrap();
    let s = s.trim_end().into();

    s
}

fn main() {
    let path = get_dir();

    let files = dirwalk::dir_walk(&path);

    for file in files {
        println!("{}", file);
    }
}
