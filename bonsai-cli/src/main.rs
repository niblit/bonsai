use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = String::with_capacity(256);

    loop {
        buffer.clear();

        let bytes_read = handle.read_line(&mut buffer).unwrap_or(0);
        
        if bytes_read == 0 {
            // EOF reached, GUI disconnected or pipe closed.
            break;
        }

        let input = buffer.trim();
        if input.is_empty() {
            continue;
        }

        println!("|->{input}<-|");
    }
}