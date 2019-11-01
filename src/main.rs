extern crate structopt;

use std::fs::File;
use std::io::Read;

use structopt::StructOpt;

mod errors;
mod dos;
mod coff;
mod formats;
mod utils;

#[derive(Debug, StructOpt)]
pub struct Cli {
    path: std::path::PathBuf,
    
    #[structopt(short="f", long="format")]
    format: Option<String>,

    #[structopt(short, long)]
    interactive: bool,
    
    #[structopt(short="s", long="show")]
    field: Option<String>
}

fn main() {
    let args = Cli::from_args();

    let mut file: File = File::open(args.path).unwrap();
    let mut file_contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut file_contents).unwrap();

    if args.format.is_none() {
        let file_format: formats::Format = utils::infer_format(&mut file_contents);
    } else {
        let file_format: formats::Format = formats::Format::from_string(
            args.format.unwrap());
    }

    if args.field.is_some() {
        unimplemented!();
    }
}
