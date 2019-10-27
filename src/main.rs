use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::time::SystemTime;

use glob::glob;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "isfresh", about = "Checks if the output file is fresh.")]
#[structopt(rename_all = "kebab-case")]
struct Opt {
    /// Output file
    #[structopt(parse(from_os_str))]
    output: PathBuf,

    /// Input files, globs accepted
    inputs: Vec<String>,

    /// Inverts the exit code, i.e. returns 1 for fresh and 0 for dirty
    #[structopt(short, long)]
    not: bool,
}

fn time_modified(p: &Path) -> SystemTime {
    let mt = p.metadata().unwrap().modified().unwrap();
    if p.is_file() {
        mt
    } else {
        fs::read_dir(p)
            .unwrap()
            .map(|entry| time_modified(&entry.unwrap().path()))
            .max()
            .unwrap_or(mt)
    }
}

fn main() {
    let opt = Opt::from_args();

    let out_mod = time_modified(&opt.output);
    let ins_mod = opt
        .inputs
        .iter()
        .filter_map(|input| {
            glob(input)
                .expect("Invalid glob")
                .map(|entry| time_modified(&entry.unwrap()))
                .max()
        })
        .max()
        .expect("No input files found");

    exit(if (out_mod >= ins_mod) != opt.not {
        0
    } else {
        1
    });
}
