use std::env;
use std::fs::File;
use std::io::{Read,Write};

const COPY_BYTES: usize = 1024 * 1024; // 1MiB

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!["=============================="];
        println!["rrrw [Raw Read Raw Write]"];
        println!["Syntax: rrrw <InputFile> <OutputFile>"];
        println!["Example: rrrw fedora.iso /dev/sde"];
        println!["=============================="];
        return
    }

    let mut input_file: File = File::open(&args[1]).expect("Couldn't open input file");
    let mut output_file: File = File::create(&args[2]).expect("Couldn't open output file");
    let mut copy_content: Vec<u8> = vec![0;COPY_BYTES];
    let mut written_bytes_total: u128 = 0;

    loop {
        let read_bytes: usize = match input_file.read(&mut copy_content) {
            Ok(read_bytes) => read_bytes,
            Err(error) => {
                // Print trace
                eprint!["\n\n\n\n\n\n"];
                eprintln!["==================== [Read error]"];
                eprintln!["input_file: {:?}",input_file];
                eprintln!["output_file: {:?}",output_file];
                eprintln!["written_bytes_total: {}",written_bytes_total];
                eprintln!["Error: {}",error];
                eprintln!["===================="];
                eprint!["\n\n\n\n\n\n"];
                panic![];
            }
        };

        let written_bytes: usize = match output_file.write(&copy_content[..read_bytes]) {
            Ok(written_bytes) => written_bytes,
            Err(error) => {
                // Print trace
                eprint!["\n\n\n\n\n\n"];
                eprintln!["==================== [Write error]"];
                eprintln!["input_file: {:?}",input_file];
                eprintln!["output_file: {:?}",output_file];
                eprintln!["written_bytes_total: {}",written_bytes_total];
                eprintln!["read_bytes: {}",read_bytes];
                eprintln!["Error: {}",error];
                eprintln!["===================="];
                eprint!["\n\n\n\n\n\n"];
                panic![];
            }
        };

        output_file.sync_data().expect("Couldn't sync the data to the destination");
        written_bytes_total += written_bytes as u128;
        print_status(written_bytes_total);

        if written_bytes != read_bytes {
            // Didn't write all read bytes
            panic!["Destination doesn't have enough space!"];
        }
        else if read_bytes != COPY_BYTES {
            // Probably read the last bytes
            break;
        }
    }

    print!["\n\n\n\n\n"]; // Print 5 newlines to get to the end

	print!["\n"];
}

fn print_status(bytes: u128) {
	let kib: f64 = bytes as f64 / 1024f64;
	let mib: f64 = bytes as f64 / 1048576f64; // 1024 * 1024 (1024^2)
	let gib: f64 = bytes as f64 / 1073741824f64; // 1024 * 1024 * 1024 (1024^3)
	let tib: f64 = bytes as f64 / 1099511627776f64; // 1024 * 1024 * 1024 * 1024 (1024^4)
    println!["  B: {}",bytes];
    println!["KiB: {}",kib];
    println!["MiB: {}",mib];
    println!["GiB: {}",gib];
    println!["TiB: {}",tib];
    print!["\r\x1b[5A"]; // Go five lines up
}
