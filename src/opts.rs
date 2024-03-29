use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap()]
pub struct Opts {
    pub args: Vec<String>,

    #[clap(short = 'c', long = "config")]
    pub config: Option<PathBuf>,

    pub filename: Option<String>,
}
