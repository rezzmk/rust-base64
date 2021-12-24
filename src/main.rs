use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let chars: &[u8] = &args[1].as_bytes();

    let encoded_result = b64_encode(chars);

	println!("{:?}", encoded_result)
}

fn b64_encode(chars: &[u8]) -> String {
	let chars_len = chars.len();

    // In base64, to calculate the output size, we consider:
    // - each base64 character is mapped from 6 bits, i.e. log2(64) = 6
    // - base64 works in groups of 3 bytes, therefore, 3 bytes = 4 * 6 = 24 bits
    // - to represent n (chars.len()) bytes, we need 4 * (n / 3), rounded to multiples of 4
	let mut result = Vec::with_capacity(4 * (chars_len / 3 + 1));

    // Initialize counter, incremented by 3 until end of input
    let mut counter = 0;
    while counter + 3 <= chars_len {
        result.append(&mut vec! [
			B64_TABLE[get_bits_first(chars[counter]) as usize],
			B64_TABLE[get_bits_second(chars[counter], chars[counter + 1]) as usize],
			B64_TABLE[get_bits_third(chars[counter + 1], 0) as usize],
			B64_TABLE[(chars[counter + 2] & 0b00111111) as usize],
        ]);

        counter += 3;
    }

	// padding
    if counter + 1 == chars_len {
        result.append(&mut vec! [
			B64_TABLE[get_bits_first(chars[counter]) as usize],
			B64_TABLE[get_bits_second(chars[counter], 0) as usize],
            '=',
            '='
        ]);
    }
    else if counter + 2 == chars_len {
        result.append(&mut vec! [
			B64_TABLE[get_bits_first(chars[counter]) as usize],
			B64_TABLE[get_bits_second(chars[counter], chars[counter + 1]) as usize],
			B64_TABLE[get_bits_third(chars[counter + 1], 0) as usize],
            '='
        ]);
    }

    result.into_iter().collect::<String>()
}

fn get_bits_first(byte: u8) -> u8 {
    (byte & 0b11111100) >> 2
}

fn get_bits_second(first_byte: u8, second_byte: u8) -> u8 {
    (first_byte & 0b00000011) << 4 | ((second_byte & 0b11110000) >> 4)
}

fn get_bits_third(second_byte: u8, third_byte: u8) -> u8 {
    (second_byte & 0b00001111) << 2 | ((third_byte & 0b11000000) >> 6)
}

// base64 encoding table
// https://pt.wikipedia.org/wiki/Base64  
const B64_TABLE: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
    'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
    'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
    'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
    'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
    'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
    'w', 'x', 'y', 'z', '0', '1', '2', '3',
    '4', '5', '6', '7', '8', '9', '+', '/'
];

