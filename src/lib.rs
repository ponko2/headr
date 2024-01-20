use anyhow::Result;
use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(value_name = "FILE", help = "Input file(s)", default_value = "-")]
    files: Vec<String>,

    #[arg(short = 'n', long, help = "Number of lines", default_value = "10")]
    lines: usize,

    #[arg(short = 'c', long, conflicts_with("lines"), help = "Number of bytes")]
    bytes: Option<usize>,
}

pub fn get_args() -> Result<Args> {
    Ok(Args::parse())
}

pub fn run(args: Args) -> Result<()> {
    for filename in args.files {
        match open(&filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(mut file) => {
                if let Some(bytes) = args.bytes {
                    let bytes = file.bytes().take(bytes).collect::<Result<Vec<_>, _>>();
                    print!("{}", String::from_utf8_lossy(&bytes?));
                } else {
                    let mut buf = String::new();
                    for _ in 0..args.lines {
                        let bytes = file.read_line(&mut buf)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{buf}");
                        buf.clear();
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    Ok(match filename {
        "-" => Box::new(BufReader::new(io::stdin())),
        _ => Box::new(BufReader::new(File::open(filename)?)),
    })
}
