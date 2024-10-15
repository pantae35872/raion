use std::{env::args, fs::File, io::Read};

use common::sin::Sin;

fn main() {
    let mut args = args();
    args.next();
    let mut sin = File::open(&args.next().unwrap()).unwrap();
    let mut buf = Vec::new();
    sin.read_to_end(&mut buf).unwrap();
    let sin = Sin::from_bytes(&buf).map_err(|e| eprintln!("{e}"));
    println!("{:?}", sin);
}
