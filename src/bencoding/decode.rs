use std::fs::read;

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
    println!("bytes.len={}", bytes.len());
    while position < bytes.len() {
        println!("position={} outerdecoded={:?}", position, decoded);
        // TODO: dont pass the entire array of bytes each time
        let (partial, advanced) = decode_next(bytes, position);
        // TODO: convert partial to proper type (eg. using bytes_to_integer)
        decoded.extend(partial);
        position += advanced;
    }
    decoded
}

fn decode_next(bytes: &[u8], position: usize) -> (Vec<u8>, usize) {
    match bytes[position] {
        100 => decode_dictionary(bytes, position), // "d" (100) marks beginning of dictionary
        105 => decode_integer(bytes),              // "i" (105) marks beginning of integer
        108 => decode_list(bytes, position),       // "l" (108) marks beginning of list
        _ => decode_string(bytes, position),
    }
}

fn decode_integer(bytes: &[u8]) -> (Vec<u8>, usize) {
    // TODO: make encodings with leading 0s fail (i03e is invalid)
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
    let string_length = bytes_to_integer(&[bytes[position]]) as usize + 2;
    let decoded = bytes[position + 2..position + string_length as usize].to_vec();
    println!("decoded={:?}", decoded);
    (decoded, string_length)
}

fn decode_list(bytes: &[u8], position: usize) -> (Vec<u8>, usize) {
    unimplemented!("decode list");
}

fn decode_dictionary(bytes: &[u8], position: usize) -> (Vec<u8>, usize) {
    unimplemented!("decode dict");
}

fn bytes_to_integer(bytes: &[u8]) -> i64 {
    std::str::from_utf8(bytes).unwrap().parse::<i64>().unwrap()
}

fn bytes_to_string(bytes: &[u8]) -> &str {
    std::str::from_utf8(bytes).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_integer() {
        let int1 = "i12362e".as_bytes();
        let decoded1 = decode(int1);
        let decoded_int1 = bytes_to_integer(&decoded1);
        assert_eq!(decoded_int1, 12362);

        let int2 = "i-3e".as_bytes();
        let decoded2 = decode(int2);
        let decoded_int2 = bytes_to_integer(&decoded2);
        assert_eq!(decoded_int2, -3);

        // TODO: this should not be valid.
        let int3 = "i03e".as_bytes();
        let decoded3 = decode(int3);
        let decoded_int3 = bytes_to_integer(&decoded3);
        assert_eq!(decoded_int3, 3);
    }

    #[test]
    fn test_decode_string() {
        let string1 = "4:spam".as_bytes();
        let decoded1 = decode(string1);
        let decoded_string1 = bytes_to_string(&decoded1);
        assert_eq!(decoded_string1, "spam");

        let string2 = "0:".as_bytes();
        let decoded2 = decode(string2);
        let decoded_string2 = bytes_to_string(&decoded2);
        assert_eq!(decoded_string2, "");

        let string3 = "8:announce".as_bytes();
        let decoded3 = decode(string3);
        let decoded_string3 = bytes_to_string(&decoded3);
        assert_eq!(decoded_string3, "announce");
    }
}
