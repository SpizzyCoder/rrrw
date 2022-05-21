use std::fs::File;
use std::io::{Write,Read,stdin,stdout};
use std::path::Path;

use clap::Parser;
use clap::ArgEnum;

const PRINT_LINES: u32 = 11;

#[derive(ArgEnum,Clone,Copy,Debug)]
enum Unit {
  B,
  Kb,
  Kib,
  Mb,
  Mib,
  Gb,
  Gib,
  Tb,
  Tib,
}

/// Simple program to copy raw bytes
#[derive(Parser,Debug)]
#[clap(version,about)]
pub struct Args {
  /// Unit
  #[clap(short,long,arg_enum,default_value_t = Unit::B)]
  unit: Unit,

  /// Chunksize
  #[clap(short,long,default_value_t = 1)]
  chunksize: usize,

  /// Source
  #[clap()]
  src: String,

  /// Destination
  #[clap()]
  dest: String,
}

fn main() {
  let args: Args = Args::parse();

  let src_path: &Path = Path::new(&args.src);
  let dest_path: &Path = Path::new(&args.dest);
  if !src_path.exists() {
    eprintln!["{} doesn't exist",src_path.display()];
    return
  }

  if args.chunksize < 1 {
    eprintln!["Chunksize can't be 0 or negative"];
    return
  }

  let write_block = match args.unit {
    Unit::B => args.chunksize * 1,
    Unit::Kb => args.chunksize * 1000,
    Unit::Kib => args.chunksize * 1024,
    Unit::Mb => args.chunksize * 1000_usize.pow(2),
    Unit::Mib => args.chunksize * 1024_usize.pow(2),
    Unit::Gb => args.chunksize * 1000_usize.pow(3),
    Unit::Gib => args.chunksize * 1024_usize.pow(3),
    Unit::Tb => args.chunksize * 1000_usize.pow(4),
    Unit::Tib => args.chunksize * 1024_usize.pow(4),
  };

  let mut srcfile: File = match File::open(src_path) {
    Ok(file) => file,
    Err(error) => {
      eprintln!["Couldn't open {} [Error: {}]",src_path.display(),error];
      return
    }
  };

  let mut destfile: File = match File::create(dest_path) {
    Ok(file) => file,
    Err(error) => {
      eprintln!["Couldn't open {} [Error: {}]",dest_path.display(),error];
      return
    }
  };

  if !user_confirmation(&args.src,&args.dest) {
    return
  }

  let mut read_bytes_total: usize = 0;
  let mut memory: Vec<u8> = vec![0;write_block];

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

  if userinput.to_lowercase() == "y" {
    return true
  }

  return false
}
