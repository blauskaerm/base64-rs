mod base64;

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
        .arg(
            Arg::with_name("ignore-garbage")
                .long("ignore-garbage")
                .short("i")
                .help("When decoding, ignore non-alphabet characters"),
        )
        .arg(
            Arg::with_name("WRAP")
                .short("w")
                .long("wrap")
                .value_name("COLS")
                .help("Wrap encoded lines after COLS characters (default 76)")
                .takes_value(true),
        )
        .get_matches();

    let data_src = cmd_options.value_of("FILE").unwrap_or("-");
    let decode_data = cmd_options.is_present("decode");
    let wrap_str = cmd_options.value_of("WRAP").unwrap_or("76");
    let wrap = wrap_str.parse::<i32>().unwrap();
    let ignore_garbage = cmd_options.is_present("ignore-garbage");

    let buffer_size: usize;
    if decode_data {
        buffer_size = 4 * 1024;
    } else {
        buffer_size = 3 * 1024;
    }

    // Select stdin as data source or local file
    let mut data_src_fd: Box<dyn std::io::Read + 'static> = if data_src.eq("-") {
        Box::new(std::io::stdin())
    } else {
        match std::fs::File::open(&data_src) {
            Ok(file) => Box::new(file),
            Err(err) => {
                eprintln!("Unable to open file {} ({})", data_src, err.to_string());
                process::exit(-1);
            }
        }
    };

    let mut wrap_guard = 0;

    let mut buffer_vec = Vec::with_capacity(buffer_size);
    let mut output_buffer_vec = Vec::new();
    loop {
        // Read buffer_size from file into buffer_vec and iterate over file
        // until the end of file.
        match data_src_fd
            .by_ref()
            .take(buffer_size as u64)
            .read_to_end(&mut buffer_vec)
        {
            Ok(chunk_size) => {
                // Break loop if the end of file has been reached.
                if chunk_size == 0 {
                    break;
                }

                // Run operation. Filter out new line characters if decoding data
                if decode_data {
                    buffer_vec.retain(|&c| c != ('\n' as u8));
                    if base64::base64_decode(&buffer_vec, &mut output_buffer_vec, ignore_garbage) == Err(()) {
			eprintln!("Decode garbage");
			break;
		    }

                    for x in &output_buffer_vec {
                        print!("{}", *x as char);
                    }

                    output_buffer_vec.clear();

                } else {
                    if base64::base64_encode(&buffer_vec, &mut output_buffer_vec) == Err(()) {
                        break;
                    }

                    for x in &output_buffer_vec {
                        if wrap_guard == wrap {
                            println!();
                            wrap_guard = 0;
                        } else {
                            wrap_guard += 1;
                        }
                        print!("{}", *x as char);
                    }

                    output_buffer_vec.clear();
                }

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
}
