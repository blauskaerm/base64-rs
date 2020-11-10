const BASE64_MAP: [char; 65] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/', '=',
];
const PADDING_SYMBOL: usize = 64;

fn encode_base64_chunk(input_vec: &Vec<u8>) {
    let chunk_len = input_vec.len();

    println!("Chunk-len: {}", chunk_len);

    let (mut an, mut bn, mut cn, mut dn): (usize, usize, usize, usize);
    let (mut a, mut b, mut c, mut d): (char, char, char, char);

    let mut n = 0;
    loop {
        //println!("n: {}", n);
        if (n + 3) >= chunk_len {
            break;
        }

        an = ((input_vec[n] & 0xFC) >> 2) as usize;
        bn = (((input_vec[n] & 0x03) << 4) | ((input_vec[n + 1] & 0xF0) >> 4)) as usize;
        cn = (((input_vec[n + 1] & 0x0F) << 2) | ((input_vec[n + 1] & 0xC0) >> 6)) as usize;
        dn = (input_vec[n + 2] & 0x3F) as usize;

        //println!("an: {}, bn: {}, cn: {}, dn: {}", an, bn, cn, dn);

        a = BASE64_MAP[an];
        b = BASE64_MAP[bn];
        c = BASE64_MAP[cn];
        d = BASE64_MAP[dn];

        print!("{}{}{}{}", a, b, c, d);

        n = n + 3;
    }
    let check_padding = chunk_len - n;
    if check_padding == 1 {
        //println!("Two padding");

        an = ((input_vec[n] & 0xFC) >> 2) as usize;
        bn = ((input_vec[n] & 0x03) << 4) as usize; //| ((input_vec[1] & 0xF0) >> 4)) as usize;
                                                    //cn = (((input_vec[1] & 0x0F) << 2) | ((input_vec[1] & 0xC0) >> 6)) as usize;
                                                    //dn = (input_vec[2] & 0x3F) as usize;

        a = BASE64_MAP[an];
        b = BASE64_MAP[bn];
        c = BASE64_MAP[PADDING_SYMBOL];
        d = BASE64_MAP[PADDING_SYMBOL];

        print!("{}{}{}{}", a, b, c, d);
    } else if check_padding == 2 {
        //println!("One padding");

        an = ((input_vec[n] & 0xFC) >> 2) as usize;
        bn = (((input_vec[n] & 0x03) << 4) | ((input_vec[n + 1] & 0xF0) >> 4)) as usize;
        cn = ((input_vec[n + 1] & 0x0F) << 2) as usize; //| ((input_vec[1] & 0xC0) >> 6)) as usize;
                                                        // dn = (input_vec[2] & 0x3F) as usize;

        a = BASE64_MAP[an];
        b = BASE64_MAP[bn];
        c = BASE64_MAP[cn];
        d = BASE64_MAP[PADDING_SYMBOL];

        print!("{}{}{}{}", a, b, c, d);
    }
    // match chunk_len {
    //     1 => {
    //         println!("Two padding");
    //         an = ((input_vec[0] & 0xFC) >> 2) as usize;
    //         bn = ((input_vec[0] & 0x03) << 4) as usize; //| ((input_vec[1] & 0xF0) >> 4)) as usize;
    //                                                     //cn = (((input_vec[1] & 0x0F) << 2) | ((input_vec[1] & 0xC0) >> 6)) as usize;
    //                                                     //dn = (input_vec[2] & 0x3F) as usize;

    //         a = BASE64_MAP.chars().nth(an).unwrap();
    //         b = BASE64_MAP.chars().nth(bn).unwrap();
    //         c = '=';
    //         d = '=';
    //     }
    //     2 => {
    //         println!("One padding");
    //         an = ((input_vec[0] & 0xFC) >> 2) as usize;
    //         bn = (((input_vec[0] & 0x03) << 4) | ((input_vec[1] & 0xF0) >> 4)) as usize;
    //         cn = ((input_vec[1] & 0x0F) << 2) as usize; //| ((input_vec[1] & 0xC0) >> 6)) as usize;
    //                                                     // dn = (input_vec[2] & 0x3F) as usize;

    //         a = BASE64_MAP.chars().nth(an).unwrap();
    //         b = BASE64_MAP.chars().nth(bn).unwrap();
    //         c = BASE64_MAP.chars().nth(cn).unwrap();
    //         d = '=';
    //     }
    //     _ => {
    //         println!("No padding");
    //         an = ((input_vec[0] & 0xFC) >> 2) as usize;
    //         bn = (((input_vec[0] & 0x03) << 4) | ((input_vec[1] & 0xF0) >> 4)) as usize;
    //         cn = (((input_vec[1] & 0x0F) << 2) | ((input_vec[1] & 0xC0) >> 6)) as usize;
    //         dn = (input_vec[2] & 0x3F) as usize;

    //         //println!("an: {}, bn: {}, cn: {}, dn: {}", an, bn, cn, dn);

    //         a = BASE64_MAP.chars().nth(an).unwrap();
    //         b = BASE64_MAP.chars().nth(bn).unwrap();
    //         c = BASE64_MAP.chars().nth(cn).unwrap();
    //         d = BASE64_MAP.chars().nth(dn).unwrap();
    //     }
    // }

    // print!("{}{}{}{}", a, b, c, d);
}

fn main() {
    //let mut input_vec: Vec<u8> = Vec::new();
    // input_vec.push('A' as u8);
    // input_vec.push('B' as u8);
    // input_vec.push('C' as u8);

    let test_string = String::from("any carnal pleasure.");
    let test_string_len = test_string.len();
    println!("Input string ({}): {}", test_string_len, test_string);
    let test_byte_vec = test_string.into_bytes();
    encode_base64_chunk(&test_byte_vec);
    println!();

    // // TWFu
    // input_vec.push('M' as u8);
    // input_vec.push('a' as u8);
    // input_vec.push('n' as u8);
    // print!("Base64: ");
    // encode_base64_chunk(&input_vec);
    // println!();
    // input_vec.clear();

    // // TWE=
    // input_vec.push('M' as u8);
    // input_vec.push('a' as u8);
    // print!("Base64: ");
    // encode_base64_chunk(&input_vec);
    // println!();
    // input_vec.clear();

    // // TQ==
    // input_vec.push('M' as u8);
    // print!("Base64: ");
    // encode_base64_chunk(&input_vec);
    // println!();
    // input_vec.clear();

    // println!(
    //     "Input: {}{}{}",
    //     input_vec[0] as char, input_vec[1] as char, input_vec[2] as char
    // );
}
