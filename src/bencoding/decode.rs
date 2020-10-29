use std::fs::read;

/// Decodes a file that is in Bencoded
pub fn decode_file(filepath: &str) {
    // read file as Result<Vec<u8>>
    let f = read(filepath).unwrap();
    decode(&f);
}

/// Decodes a Bencoded array of bytes
pub fn decode(bytes: &[u8]) -> Vec<String> {
    let mut position = 0;
    let mut decoded: Vec<String> = Vec::new();
    // println!("bytes.len={}", bytes.len());
    while position < bytes.len() {
        // println!(
        //     "position={} outerdecoded={:?}, bytes={:?}",
        //     position, decoded, bytes
        // );
        // TODO: dont pass the entire array of bytes each time
        let (partial, advanced) = decode_next(bytes, position);
        // println!("partial {} advanced {}", partial, advanced);
        // TODO: convert partial to proper type (eg. using bytes_to_integer)
        decoded.push(partial);
        // println!("pushed");
        position += advanced;
        // println!("advanced");
        // println!("position {}", position);
    }
    decoded
}

fn decode_next(bytes: &[u8], position: usize) -> (String, usize) {
    let (decoded_bytes, advanced) = match bytes[position] {
        // 100 => decode_dictionary(bytes, position), // "d" (100) marks beginning of dictionary
        105 => decode_integer(bytes, position), // "i" (105) marks beginning of integer
        // 108 => decode_list(bytes, position),       // "l" (108) marks beginning of list
        _ => decode_string(bytes, position),
    };
    // println!("decoded bytes={:?}", decoded_bytes);
    (bytes_to_string(&decoded_bytes), advanced)
}

fn decode_integer(bytes: &[u8], position: usize) -> (Vec<u8>, usize) {
    // TODO: make encodings with leading 0s fail (i03e is invalid)
    let mut int: Vec<u8> = Vec::new();
    let mut advanced = position;
    let mut bytes_iter = bytes[advanced..].iter();
    // println!("initial advanced={}", advanced);
    bytes_iter.next();
    while let Some(b) = bytes_iter.next() {
        advanced += 1;
        // "e" (101) marks end of integer
        if *b == 101 {
            break;
        }
        int.push(*b);
        // println!("advanced={} int={:?}", advanced, int);
    }
    (int, advanced + 1)
}

fn decode_string(bytes: &[u8], position: usize) -> (Vec<u8>, usize) {
    let string_length = bytes_to_integer(&[bytes[position]]) as usize;
    let decoded = bytes[position + 2..position + 2 + string_length].to_vec();
    // println!("decoded={:?}", decoded);
    // println!("string length={}", string_length);
    (decoded, string_length + 2)
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

fn bytes_to_string(bytes: &[u8]) -> String {
    std::str::from_utf8(bytes).unwrap().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_integer() {
        let int1 = "i12362e".as_bytes();
        let (decoded1, _) = decode_integer(int1, 0);
        let decoded_int1 = bytes_to_integer(&decoded1);
        assert_eq!(decoded_int1, 12362);

        let int2 = "i-3e".as_bytes();
        let (decoded2, _) = decode_integer(int2, 0);
        let decoded_int2 = bytes_to_integer(&decoded2);
        assert_eq!(decoded_int2, -3);

        // TODO: this should not be valid.
        let int3 = "i03e".as_bytes();
        let (decoded3, _) = decode_integer(int3, 0);
        let decoded_int3 = bytes_to_integer(&decoded3);
        assert_eq!(decoded_int3, 3);
    }

    #[test]
    fn test_decode_string() {
        let string1 = "4:spam".as_bytes();
        let (decoded1, _) = decode_string(string1, 0);
        let decoded_string1 = bytes_to_string(&decoded1);
        assert_eq!(decoded_string1, "spam".to_string());

        let string2 = "0:".as_bytes();
        let (decoded2, _) = decode_string(string2, 0);
        let decoded_string2 = bytes_to_string(&decoded2);
        assert_eq!(decoded_string2, "".to_string());

        let string3 = "8:announce".as_bytes();
        let (decoded3, _) = decode_string(string3, 0);
        let decoded_string3 = bytes_to_string(&decoded3);
        assert_eq!(decoded_string3, "announce".to_string());
    }
}
