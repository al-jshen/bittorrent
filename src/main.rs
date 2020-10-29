pub mod bencoding;
use bencoding::{decode, decode_file};

fn main() {
    let string = "4:spam".as_bytes();
    let a = decode(string);
    println!("result={:?}", a);
}
