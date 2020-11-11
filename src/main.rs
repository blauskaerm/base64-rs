const BASE64_MAP: [char; 65] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/', '=',
];
const PADDING_SYMBOL: usize = 64;

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
    let mut a: u8;
    let mut b: u8;
    let mut c: u8;
    let mut d: u8;

    // https://en.wikibooks.org/wiki/Algorithm_Implementation/Miscellaneous/Base64#Decode
    // https://stackoverflow.com/questions/11559203/decode-table-construction-for-base64

    let mut base64_inv: [u8; 80] = [
        62, 255, 255, 255, 63, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 255, 255, 255, 255, 255,
        255, 255, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
        23, 24, 25, 255, 255, 255, 255, 255, 255, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37,
        38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51,
    ];

    a = base64_inv[(input_vec[0] - ('+' as u8)) as usize];
    b = base64_inv[(input_vec[1] - ('+' as u8)) as usize];
    c = base64_inv[(input_vec[2] - ('+' as u8)) as usize];
    d = base64_inv[(input_vec[3] - ('+' as u8)) as usize];

    let mut f: u32 = 0;

    f = (((a as u32) << 18) | ((b as u32) << 12) | ((c as u32) << 6) | (d as u32)) as u32;

    let test1 = ((f >> 16) & 0xFF) as u8;
    let test2 = ((f >> 8) & 0xFF) as u8;
    let test3 = (f & 0xFF) as u8;

    println!("{}", test1 as char);
    println!("{}", test2 as char);
    println!("{} {}", test3 as char, test3);

    println!("{:#010b} {}", input_vec[0], input_vec[0] as char);
    println!("{:#010b} {}", input_vec[1], input_vec[1] as char);
    println!("{:#010b} {}", input_vec[2], input_vec[2] as char);
    println!("{:#010b} {}", input_vec[3], input_vec[3] as char);
    println!();

    // a = ((input_vec[0] & 0x3F) << 2) | ((input_vec[1] & 0x30) >> 4);
    //println!("{:#010b} {} {}", a, a, a as char);
    // println!("{:#010b} {} {}", 77, 77, 77 as char);

    // println!();

    // b = ((input_vec[1] & 0x0F) << 4) | ((input_vec[2] & 0x3C) >> 2);
    // println!("{:#010b} {} {}", b, b, b as char);
    // println!("{:#010b} {} {}", 97, 97, 97 as char);
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
