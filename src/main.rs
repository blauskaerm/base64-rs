const BASE64_MAP: [char; 65] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/', '=',
];
const PADDING_SYMBOL: usize = 64;

const INVALID: u8 = 255;
const BASE64_INV_NORMALIZER: u8 = '+' as u8;
const BASE64_INV_MAP: [u8; 80] = [
    62, INVALID, INVALID, INVALID, 63, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, INVALID, INVALID,
    INVALID, INVALID, INVALID, INVALID, INVALID, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14,
    15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, INVALID, INVALID, INVALID, INVALID, INVALID,
    INVALID, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
    47, 48, 49, 50, 51,
];

fn base64_encode(input_vec: &Vec<u8>) {
    let chunk_len = input_vec.len();

    let (mut an, mut bn, mut cn, mut dn): (usize, usize, usize, usize);
    let (mut a, mut b, mut c, mut d): (char, char, char, char);

    let mut n = 0;
    loop {
        if (n + 3) >= chunk_len {
            break;
        }

        an = ((input_vec[n] & 0xFC) >> 2) as usize;
        bn = (((input_vec[n] & 0x03) << 4) | ((input_vec[n + 1] & 0xF0) >> 4)) as usize;
        cn = (((input_vec[n + 1] & 0x0F) << 2) | ((input_vec[n + 2] & 0xC0) >> 6)) as usize;
        dn = (input_vec[n + 2] & 0x3F) as usize;

        a = BASE64_MAP[an];
        b = BASE64_MAP[bn];
        c = BASE64_MAP[cn];
        d = BASE64_MAP[dn];

        print!("{}{}{}{}", a, b, c, d);

        n = n + 3;
    }
    let check_padding = chunk_len - n;
    if check_padding == 1 {
        an = ((input_vec[n] & 0xFC) >> 2) as usize;
        bn = ((input_vec[n] & 0x03) << 4) as usize;

        a = BASE64_MAP[an];
        b = BASE64_MAP[bn];
        c = BASE64_MAP[PADDING_SYMBOL];
        d = BASE64_MAP[PADDING_SYMBOL];

        print!("{}{}{}{}", a, b, c, d);
    } else if check_padding == 2 {
        an = ((input_vec[n] & 0xFC) >> 2) as usize;
        bn = (((input_vec[n] & 0x03) << 4) | ((input_vec[n + 1] & 0xF0) >> 4)) as usize;
        cn = ((input_vec[n + 1] & 0x0F) << 2) as usize;

        a = BASE64_MAP[an];
        b = BASE64_MAP[bn];
        c = BASE64_MAP[cn];
        d = BASE64_MAP[PADDING_SYMBOL];

        print!("{}{}{}{}", a, b, c, d);
    }
}

fn base64_decode(input_vec: &Vec<u8>) {
    let mut n = 0;
    loop {
        if (n + 4) > input_vec.len() {
            break;
        }

        let dec_an = (input_vec[n] - BASE64_INV_NORMALIZER) as usize;
        let dec_bn = (input_vec[n + 1] - BASE64_INV_NORMALIZER) as usize;
        let dec_cn = (input_vec[n + 2] - BASE64_INV_NORMALIZER) as usize;
        let dec_dn = (input_vec[n + 3] - BASE64_INV_NORMALIZER) as usize;

        let dec_a = BASE64_INV_MAP[dec_an] as u32;
        let dec_b = BASE64_INV_MAP[dec_bn] as u32;
        let mut dec_c = BASE64_INV_MAP[dec_cn] as u32;
        let mut dec_d = BASE64_INV_MAP[dec_dn] as u32;

        // Check for padding
        // Could be single or double padding
        if input_vec[n + 3] == ('=' as u8) {
            if input_vec[n + 2] == ('=' as u8) {
                dec_c = 0;
                dec_d = 0;
            } else {
                dec_d = 0;
            }
        }

        let decode = (dec_a << 18) | (dec_b << 12) | (dec_c << 6) | dec_d;

        let a = ((decode >> 16) & 0xFF) as u8;
        let b = ((decode >> 8) & 0xFF) as u8;
        let c = (decode & 0xFF) as u8;

        print!("{}{}{}", a as char, b as char, c as char);

        n = n + 4;
    }
}

fn main() {
    // input_vec.push('A' as u8);
    // input_vec.push('B' as u8);
    // input_vec.push('C' as u8);
    let mut input_vec: Vec<u8> = Vec::new();

    let test_string = String::from("any carnal pleasure.");
    let test_string_len = test_string.len();
    println!("Input string ({}): {}", test_string_len, test_string);
    let test_byte_vec = test_string.into_bytes();
    base64_encode(&test_byte_vec);
    println!();

    input_vec.push('T' as u8);
    input_vec.push('W' as u8);
    input_vec.push('F' as u8);
    input_vec.push('u' as u8);
    base64_decode(&input_vec);
    println!();
    input_vec.clear();

    // // TWFu
    // input_vec.push('M' as u8);
    // input_vec.push('a' as u8);
    // input_vec.push('n' as u8);
    // print!("Base64: ");
    // base64_encode(&input_vec);
    // println!();
    // input_vec.clear();

    // // TWE=
    // input_vec.push('M' as u8);
    // input_vec.push('a' as u8);
    // print!("Base64: ");
    // base64_encode(&input_vec);
    // println!();
    // input_vec.clear();

    // // TQ==
    // input_vec.push('M' as u8);
    // print!("Base64: ");
    // base64_encode(&input_vec);
    // println!();
    // input_vec.clear();

    // println!(
    //     "Input: {}{}{}",
    //     input_vec[0] as char, input_vec[1] as char, input_vec[2] as char
    // );
}
