mod base64;

use std::fs::File;
use std::io::Read;
use std::process;

extern crate clap;
use clap::{App, Arg};

fn main() {
    let cmd_options = App::new("base64-rs")
        .version("0.1")
        .author("BlauskaerM <blauskaerm@protonmail.ch>")
        .about("Base64 encode/decode data and print to standard output")
        .arg(
            Arg::with_name("FILE")
                .help("File to encode/decode, or - to read from stdin")
                .index(1),
        )
        .arg(
            Arg::with_name("decode")
                .long("decode")
                .short("d")
                .help("Decode data"),
        )
        .get_matches();

    type OperationFnType = fn(&Vec<u8>);

    let local_file = cmd_options.value_of("FILE").unwrap();
    let decode_data = cmd_options.is_present("decode");

    let buffer_size: usize;
    let operation: OperationFnType;
    if decode_data {
        buffer_size = 4 * 1024;
        operation = base64::base64_decode;
    } else {
        buffer_size = 3 * 1024;
        operation = base64::base64_encode;
    }

    let mut file_fd = match File::open(&local_file) {
        Ok(file) => file,
        Err(error_description) => {
            eprintln!(
                "Unable to open file {} ({})",
                local_file,
                error_description.to_string()
            );
            process::exit(-1);
        }
    };

    let mut buffer_vec = Vec::with_capacity(buffer_size);
    loop {
        // Read buffer_size from file into buffer_vec and iterate over file
        // until the end of file.
        match file_fd
            .by_ref()
            .take(buffer_size as u64)
            .read_to_end(&mut buffer_vec)
        {
            Ok(chunk_size) => {
                // Break loop if the end of file has been reached.
                if chunk_size == 0 {
                    break;
                }

                // Run operation
                operation(&buffer_vec);

                // Break the loop if the amount of bytes indicates that the
                // end of file has been reached.
                if chunk_size < buffer_size {
                    break;
                }

                // Clear buffer vector
                buffer_vec.clear();
            }
            Err(error_description) => panic!("{}", error_description),
        }
    }
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
