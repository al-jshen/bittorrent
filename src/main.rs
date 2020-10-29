pub mod bencoding;
use bencoding::{decode, decode_file};

fn main() {
    let string = "4:spam".as_bytes();
    let a = decode(string);
    println!("result={:?}", a);
    let integer = "i123e".as_bytes();
    let b = decode(integer);
    println!("result={:?}", b);
    let intstring = "i123e4:spam".as_bytes();
    let c = decode(intstring);
    println!("result={:?}", c);
    let stringint = "4:spami123e".as_bytes();
    let d = decode(stringint);
    println!("result={:?}", d);
}
