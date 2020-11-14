mod base64;

use std::fs::File;
use std::io::Read;
use std::process;

extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("base64-rs")
        .version("0.1")
        .author("BlauskaerM <blauskaerm@protonmail.ch>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("FILE")
                .help("File to encode/decode, or - to read from stdin")
                .index(1),
        )
        .arg(Arg::with_name("decode").short("d").help("Decode data"))
        .get_matches();

    let local_file = matches.value_of("FILE").unwrap();
    let operation = matches.is_present("decode");
    println!("Decode: {}", operation);

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
    let buffer_size: usize = 3 * 1024;
    let mut buffer_vec = Vec::with_capacity(buffer_size);
    loop {
        // Read 3kB from file into buffer_vec and iterate over file
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

		// Encode buffer
                base64::base64_encode(&buffer_vec);

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
