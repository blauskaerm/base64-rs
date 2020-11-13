use std::process;

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

pub fn base64_encode(input_vec: &Vec<u8>) {
    let input_vector_len = input_vec.len();

    let mut n = 0;
    loop {
        if (n + 3) > input_vector_len {
            break;
        }

        let encode = ((input_vec[n] as u32) << 16)
            | ((input_vec[n + 1] as u32) << 8)
            | (input_vec[n + 2] as u32);
        let encode_arr: [usize; 4] = [
            ((encode >> 18) & 0x3F) as usize,
            ((encode >> 12) & 0x3F) as usize,
            ((encode >> 6) & 0x3F) as usize,
            (encode & 0x3F) as usize,
        ];

        print!(
            "{}{}{}{}",
            BASE64_MAP[encode_arr[0]],
            BASE64_MAP[encode_arr[1]],
            BASE64_MAP[encode_arr[2]],
            BASE64_MAP[encode_arr[3]]
        );

        n = n + 3;
    }

    let check_padding = input_vector_len - n;
    if check_padding > 0 {
        let encode: u32;
        let encode_arr: [usize; 4];
        if check_padding == 1 {
            encode = (input_vec[n] as u32) << 16;
            encode_arr = [
                ((encode >> 18) & 0x3F) as usize,
                ((encode >> 12) & 0x3F) as usize,
                PADDING_SYMBOL as usize,
                PADDING_SYMBOL as usize,
            ];
        } else if check_padding == 2 {
            encode = ((input_vec[n] as u32) << 16) | ((input_vec[n + 1] as u32) << 8);
            encode_arr = [
                ((encode >> 18) & 0x3F) as usize,
                ((encode >> 12) & 0x3F) as usize,
                ((encode >> 6) & 0x3F) as usize,
                PADDING_SYMBOL as usize,
            ];
        } else {
            eprintln!("Invalid padding offset");
            process::exit(-1);
        }

        print!(
            "{}{}{}{}",
            BASE64_MAP[encode_arr[0]],
            BASE64_MAP[encode_arr[1]],
            BASE64_MAP[encode_arr[2]],
            BASE64_MAP[encode_arr[3]]
        );
    }
}

pub fn base64_decode(input_vec: &Vec<u8>) {
    let mut n = 0;

    let input_vec_len = input_vec.len();

    // Vector should have an even length of four
    if (input_vec_len % 4) != 0 {
        eprintln!("Decode vector has invalid length");
        process::exit(-1);
    }

    loop {
        if (n + 4) > input_vec_len {
            break;
        }

        let dec_an = (input_vec[n] - BASE64_INV_NORMALIZER) as usize;
        let dec_bn = (input_vec[n + 1] - BASE64_INV_NORMALIZER) as usize;
        let dec_cn = (input_vec[n + 2] - BASE64_INV_NORMALIZER) as usize;
        let dec_dn = (input_vec[n + 3] - BASE64_INV_NORMALIZER) as usize;

        if BASE64_INV_MAP[dec_an] == INVALID
            || BASE64_INV_MAP[dec_bn] == INVALID
            || (BASE64_INV_MAP[dec_cn] == INVALID && input_vec[n + 2] != ('=' as u8))
            || (BASE64_INV_MAP[dec_dn] == INVALID && input_vec[n + 3] != ('=' as u8))
        {
            eprintln!("Decode garbage in chunk {}", n);
            process::exit(-1);
        }

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
