const BASE64_MAP: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn encode_base64_chunk(input_vec: &Vec<u8>) {
    let chunk_len = input_vec.len();

    let (an, bn, cn, dn): (usize, usize, usize, usize);
    let (a, b, c, d): (char, char, char, char);

    match chunk_len {
        1 => {
            println!("Two padding");
            an = ((input_vec[0] & 0xFC) >> 2) as usize;
            bn = ((input_vec[0] & 0x03) << 4) as usize; //| ((input_vec[1] & 0xF0) >> 4)) as usize;
                                                        //cn = (((input_vec[1] & 0x0F) << 2) | ((input_vec[1] & 0xC0) >> 6)) as usize;
                                                        //dn = (input_vec[2] & 0x3F) as usize;

            a = BASE64_MAP.chars().nth(an).unwrap();
            b = BASE64_MAP.chars().nth(bn).unwrap();
            c = '=';
            d = '=';
        }
        2 => {
            println!("One padding");
            an = ((input_vec[0] & 0xFC) >> 2) as usize;
            bn = (((input_vec[0] & 0x03) << 4) | ((input_vec[1] & 0xF0) >> 4)) as usize;
            cn = ((input_vec[1] & 0x0F) << 2) as usize; //| ((input_vec[1] & 0xC0) >> 6)) as usize;
                                                        // dn = (input_vec[2] & 0x3F) as usize;

            a = BASE64_MAP.chars().nth(an).unwrap();
            b = BASE64_MAP.chars().nth(bn).unwrap();
            c = BASE64_MAP.chars().nth(cn).unwrap();
            d = '=';
        }
        _ => {
            println!("No padding");
            an = ((input_vec[0] & 0xFC) >> 2) as usize;
            bn = (((input_vec[0] & 0x03) << 4) | ((input_vec[1] & 0xF0) >> 4)) as usize;
            cn = (((input_vec[1] & 0x0F) << 2) | ((input_vec[1] & 0xC0) >> 6)) as usize;
            dn = (input_vec[2] & 0x3F) as usize;

            //println!("an: {}, bn: {}, cn: {}, dn: {}", an, bn, cn, dn);

            a = BASE64_MAP.chars().nth(an).unwrap();
            b = BASE64_MAP.chars().nth(bn).unwrap();
            c = BASE64_MAP.chars().nth(cn).unwrap();
            d = BASE64_MAP.chars().nth(dn).unwrap();
        }
    }

    print!("{}{}{}{}", a, b, c, d);
}

fn main() {
    let mut input_vec: Vec<u8> = Vec::new();
    // input_vec.push('A' as u8);
    // input_vec.push('B' as u8);
    // input_vec.push('C' as u8);

    // TWFu
    input_vec.push('M' as u8);
    input_vec.push('a' as u8);
    input_vec.push('n' as u8);
    print!("Base64: ");
    encode_base64_chunk(&input_vec);
    println!();
    input_vec.clear();

    // TWE=
    input_vec.push('M' as u8);
    input_vec.push('a' as u8);
    print!("Base64: ");
    encode_base64_chunk(&input_vec);
    println!();
    input_vec.clear();

    // TQ==
    input_vec.push('M' as u8);
    print!("Base64: ");
    encode_base64_chunk(&input_vec);
    println!();
    input_vec.clear();

    // println!(
    //     "Input: {}{}{}",
    //     input_vec[0] as char, input_vec[1] as char, input_vec[2] as char
    // );
}
