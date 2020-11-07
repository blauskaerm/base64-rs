fn encode_base64_chunk(input_vec: Vec<u8>) {
    let base64_map = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijlmnopqrstuvwxyz0123456789+/";
    let an = ((input_vec[0] & 0xFC) >> 2) as usize;
    let bn = (((input_vec[0] & 0x03) << 4) | ((input_vec[1] & 0xF0) >> 4)) as usize;
    let cn = (((input_vec[1] & 0x0F) << 2) | ((input_vec[1] & 0xC0) >> 6)) as usize;
    let dn = (input_vec[2] & 0x3F) as usize;

    let a = base64_map.chars().nth(an).unwrap();
    let b = base64_map.chars().nth(bn).unwrap();
    let c = base64_map.chars().nth(cn).unwrap();
    let d = base64_map.chars().nth(dn).unwrap();

    print!("{}{}{}{}", a, b, c, d);
}

fn main() {
    let mut input_vec: Vec<u8> = Vec::new();
    input_vec.push('A' as u8);
    input_vec.push('B' as u8);
    input_vec.push('C' as u8);

    println!(
	"Input: {}{}{}",
	input_vec[0] as char, input_vec[1] as char, input_vec[2] as char
    );

    print!("Base64: ");
    encode_base64_chunk(input_vec);
    println!();
}
