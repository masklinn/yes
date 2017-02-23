use std::env;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];

    let output = if args.is_empty() { "y".to_string() } else { args.join(" ") };
    let stdout = io::stdout();
    let mut out = stdout.lock();
    loop {
        let _ = out.write(output.as_bytes());
    }
}
