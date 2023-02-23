// This is a very basic rust implementation that approximates the UNIX 'wc' program for text-files; for example, the file you are reading now
// has 50 lines, 303 words, and 2076 bytes, which you can verify by running cargo run -- src/main.rs
//
// However, reading a particular line in a file can fail (for example, if it is not a valid UTF-8 file); this will be very rare, and the code
// just uses unwrap() to convert a Result<String,Error> into a String. If this doesn't succeed, the program panics.
// And of course, opening a file can fail for obvious reasons.
//
// You can trigger this behaviour by running this program on something that is
// not a text file, e.g. cargo run -- /bin/bash or cargo run -- c:\windows\system32\cmd.exe
//
// Your tasks:
// 1) change the functions read_lines and count_bytes_and_lines so they return a Result<TYPE, io::Error>, and make them propagate errors.
//
// 2) handle these errors in main, reporting any error that occurred in main() using eprintln!

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};

//change this into:
//fn read_lines(filename: &str) -> Result<Lines<BufReader<File>>, io::Error> {
fn read_lines(filename: &str) -> Lines<BufReader<File>> {
    let file = File::open(filename).unwrap(); // this can easily fail
    BufReader::new(file).lines()
}

//change this into:
//fn count_bytes_and_lines(filename: &str) -> Result<(usize, usize, usize), io::Error> {
fn count_bytes_and_lines(filename: &str) -> (usize, usize, usize) {
    let lines = read_lines(filename);
    let mut line_count = 0;
    let mut word_count = 0;
    let mut byte_count = 0;
    for line in lines {
        let text = line.unwrap(); // this will usually not fail
        line_count += 1;
        word_count += text.split_whitespace().count();
        byte_count += text.len();
    }

    (line_count, word_count, byte_count)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let (lines, words, bytes) = count_bytes_and_lines(filename);
    println!("{filename}: {lines} lines, {words} words, {bytes} bytes");
}
