mod base64;

fn main() {
    // input_vec.push('A' as u8);
    // input_vec.push('B' as u8);
    // input_vec.push('C' as u8);
    let mut input_vec: Vec<u8> = Vec::new();

    let test_string = String::from("any carnal pleasure.");
    let test_encode_byte_vec = test_encode_string.into_bytes();
    base64::base64_encode(&test_encode_byte_vec);
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
