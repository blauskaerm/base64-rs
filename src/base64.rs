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

pub fn base64_encode(input_vec: &Vec<u8>, output_vec: &mut Vec<u8>) -> Result<(), ()> {
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

        output_vec.push(BASE64_MAP[encode_arr[0]] as u8);
        output_vec.push(BASE64_MAP[encode_arr[1]] as u8);
        output_vec.push(BASE64_MAP[encode_arr[2]] as u8);
        output_vec.push(BASE64_MAP[encode_arr[3]] as u8);

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
            return Err(());
        }

        output_vec.push(BASE64_MAP[encode_arr[0]] as u8);
        output_vec.push(BASE64_MAP[encode_arr[1]] as u8);
        output_vec.push(BASE64_MAP[encode_arr[2]] as u8);
        output_vec.push(BASE64_MAP[encode_arr[3]] as u8);
    }
    Ok(())
}

pub fn base64_decode(input_vec: &Vec<u8>, output_vec: &mut Vec<u8>) -> Result<(), ()> {
    let mut n = 0;

    let input_vec_len = input_vec.len();

    // Vector should have an even length of four
    if (input_vec_len % 4) != 0 {
        eprintln!("Decode vector has invalid length {}", input_vec_len);
        return Err(());
    }

    loop {
        if (n + 4) > input_vec_len {
            return Ok(());
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

	output_vec.push(a);
	output_vec.push(b);
	output_vec.push(c);

        n = n + 4;
    }
}

#[cfg(test)]
mod encode {

    use super::*;

    #[test]
    fn test_complete_chunk() {
        let input: Vec<u8> = vec!['M' as u8, 'a' as u8, 'n' as u8];
        let mut output: Vec<u8> = Vec::new();

        let res = base64_encode(&input, &mut output);

        assert_eq!(res, Ok(()));
        assert_eq!(output, ['T' as u8, 'W' as u8, 'F' as u8, 'u' as u8]);
    }

    #[test]
    fn test_single_padding() {
        let input: Vec<u8> = vec!['M' as u8, 'a' as u8];
        let mut output: Vec<u8> = Vec::new();

        let res = base64_encode(&input, &mut output);

        assert_eq!(res, Ok(()));
        assert_eq!(output, ['T' as u8, 'W' as u8, 'E' as u8, '=' as u8]);
    }

    #[test]
    fn test_double_padding() {
        let input: Vec<u8> = vec!['M' as u8];
        let mut output: Vec<u8> = Vec::new();

        let res = base64_encode(&input, &mut output);

        assert_eq!(res, Ok(()));
        assert_eq!(output, ['T' as u8, 'Q' as u8, '=' as u8, '=' as u8]);
    }
}

