use std::collections::HashMap;
use std::fs::read;
use std::io;
use std::io::prelude::*;

/// Decodes a file that is in Bencoded
pub fn decode_file(filepath: &str) {
    // read file as Result<Vec<u8>>
    let f = read(filepath).unwrap();
    decode(&f);
}

/// Decodes a Bencoded array of bytes
pub fn decode(bytes: &[u8]) -> Vec<u8> {
    let mut position = 0;
    let mut decoded: Vec<u8> = Vec::new();
    while position < bytes.len() {
        // TODO: dont pass the entire array of bytes each time
        let (partial, advanced) = decode_next(bytes, position);
        decoded.extend(partial);
        position += advanced;
    }
    decoded
}

fn decode_next(bytes: &[u8], position: usize) -> (Vec<u8>, usize) {
    // TODO: convert to proper type (eg. see test_decode_integer)
    match bytes[position] {
        100 => decode_dictionary(bytes, position), // "d" (100) marks beginning of dictionary
        105 => decode_integer(bytes),              // "i" (105) marks beginning of integer
        108 => decode_list(bytes, position),       // "l" (108) marks beginning of list
        _ => decode_string(bytes, position),
    }
}

fn decode_integer(bytes: &[u8]) -> (Vec<u8>, usize) {
    let mut int: Vec<u8> = Vec::new();
    let mut bytes_iter = bytes.iter();
    let mut advanced = 1;
    bytes_iter.next();
    while let Some(b) = bytes_iter.next() {
        advanced += 1;
        // "e" (101) marks end of integer
        if *b == 101 {
            break;
        }
        int.push(*b);
    }
    (int, advanced)
}

fn decode_string(bytes: &[u8], position: usize) -> (Vec<u8>, usize) {
    unimplemented!("decode string");
}

fn decode_list(bytes: &[u8], position: usize) -> (Vec<u8>, usize) {
    unimplemented!("decode list");
}

fn decode_dictionary(bytes: &[u8], position: usize) -> (Vec<u8>, usize) {
    unimplemented!("decode dict");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_integer() {
        let int1 = "i12362e".as_bytes();
        println!("{:?}", int1);
        let decoded = decode(int1);
        let decoded_int = std::str::from_utf8(&decoded)
            .unwrap()
            .parse::<i64>()
            .unwrap();
        assert_eq!(decoded_int, 12362);
    }
}
