use std::env;
use std::fs::File;
use std::io::{Write,Read,stdin,stdout};

const PRINT_LINES: u32 = 11;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 5 {
    println!["========================="];
    println!["rrrw v1.0"];
    println!["Syntax: rrrw <src> <dest> <Unit> <Chunksize>"];
    println!["Example: rrrw srcfile.img /dev/sdc M 4"];
    println![];
    println!["Note:"];
    println!["The <Chunksize> * <Unit> will be the allocated memory"];
    println!["The result must not be larger than the physical RAM you have available"];
    println!["========================="];
    return
  }

  let src: &str = &args[1];
  let dest: &str = &args[2];
  let unit: &str = &args[3];
  let mut chunksize: usize = match &args[4].parse() {
    Ok(chunksize) => *chunksize,
    Err(error) => {
      eprintln!["Couldn't parse string to unsigned int {} [Error: {}]",args[4],error];
      return
    }
  };

  match unit {
    "b" => {},
    "B" => {},
    "k" => chunksize *= 1000_usize,
    "K" => chunksize *= 1024_usize,
    "m" => chunksize *= 1000_usize.pow(2),
    "M" => chunksize *= 1024_usize.pow(2),
    "g" => chunksize *= 1000_usize.pow(3),
    "G" => chunksize *= 1024_usize.pow(3),
    "t" => chunksize *= 1000_usize.pow(4),
    "T" => chunksize *= 1024_usize.pow(4),
    _ => {
      eprintln!["Invalid unit {}",unit];
      return
    }
  };

  let mut srcfile: File = match File::open(src) {
    Ok(file) => file,
    Err(error) => {
      eprintln!["Couldn't open {} [Error: {}]",src,error];
      return
    }
  };

  let mut destfile: File = match File::create(dest) {
    Ok(file) => file,
    Err(error) => {
      eprintln!["Couldn't open {} [Error: {}]",dest,error];
      return
    }
  };

  if !user_confirmation(src,dest) {
    return
  }

  let mut read_bytes_total: usize = 0;
  let mut memory: Vec<u8> = vec![0;chunksize];

  print!["\n"];

  loop {
    let read_bytes: usize;

    read_bytes = match srcfile.read(&mut memory) {
      Ok(0) => break,
      Ok(read_bytes) => read_bytes,
      Err(error) => {
        if error.kind() != std::io::ErrorKind::Interrupted {
          break
        }
        0
      }
    };

    read_bytes_total += read_bytes;

    match destfile.write_all(&memory[..read_bytes]) {
      Ok(()) => (),
      Err(error) => {
        for _ in 0..PRINT_LINES + 10 {
          print!["\n"];
        }
        panic!["Couldn't write all read bytes into the destination [Error: {}]",error];
      }
    };

    destfile.sync_data().unwrap();
    print_status(read_bytes_total);
  }

  for _ in 0..PRINT_LINES {
    print!["\n"];
  }

	print!["\n"];
}

fn print_status(bytes: usize) {
  let kib: f64 = bytes as f64 / 1024_f64;
  let mib: f64 = bytes as f64 / 1024_f64.powf(2_f64);
  let gib: f64 = bytes as f64 / 1024_f64.powf(3_f64);
  let tib: f64 = bytes as f64 / 1024_f64.powf(4_f64);
  let kb: f64 = bytes as f64 / 1000_f64;
  let mb: f64 = bytes as f64 / 1000_f64.powf(2_f64);
  let gb: f64 = bytes as f64 / 1000_f64.powf(3_f64);
  let tb: f64 = bytes as f64 / 1000_f64.powf(4_f64);

  // Clear the lines before the print
  for _ in 0..PRINT_LINES {
	  println!["\x1B[2K"];
	}
  print!["\r\x1B[{}A",PRINT_LINES]; // Go lines up

  println!["B: {}",bytes];
  println![];
  println!["KiB: {}",kib];
  println!["MiB: {}",mib];
  println!["GiB: {}",gib];
  println!["TiB: {}",tib];
  println![];
  println!["KB: {}",kb];
  println!["MB: {}",mb];
  println!["GB: {}",gb];
  println!["TB: {}",tb];
  print!["\r\x1B[{}A",PRINT_LINES]; // Go lines up
}

fn user_confirmation(src: &str,dest: &str) -> bool {
  print!["Copy {} to {}? [y,n] ",src,dest];
  stdout().flush().expect("Couldn't flush stdout");

  let mut userinput: String = String::new();
  stdin().read_line(&mut userinput).expect("Coulnd't read from stdin");

  userinput = userinput.replace("\n","");
  userinput = userinput.replace("\r","");

  if userinput == "y" || userinput == "Y" {
    return true
  }

  return false
}
