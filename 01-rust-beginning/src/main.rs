use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let output_stream = stdout();
    let message = String::from("こんにちはラストシアンの皆さん!");
    let width = message.chars().count();
    let mut output_writer = BufWriter::new(output_stream.lock());
    say(message.as_bytes(), width, &mut output_writer).unwrap();
}
