use std::path::PathBuf;
mod file;
mod fruits;

use std::env;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{self, ErrorKind, Read, Write};
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

use structopt::{self, StructOpt};

#[derive(Clone, Debug, StructOpt)]
struct Opt {
    #[structopt(
        name = "INPUT_FILE",
        short = "i",
        long = "input_file",
        help = "Input a path to simple text file."
    )]
    input: PathBuf,
}

fn parse_input_file(input_path: PathBuf) -> Result<PathBuf, io::Error> {
    if input_path.is_relative() {
        Ok(env::current_dir()?.join(input_path))
    } else {
        Ok(input_path)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let input_file_path = parse_input_file(opt.clone().input)?;

    let file = File::options().read(true).open(input_file_path)?;

    for line in BufReader::new(file).lines().into_iter() {
        match fruits::fruits::Fruit::from_str(line?.as_str()) {
            Ok(fruit) => {
                println!("{:?}", fruit)
            }
            Err(err) => {
                eprintln!("{:?}", err)
            }
        }
    }

    Ok(())
}
