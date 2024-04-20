/*
 * @Author: yaoxingpu yaoxpu@163.com
 * @Date: 2024-04-20 23:38:55
 * @LastEditors: yaoxingpu yaoxpu@163.com
 * @LastEditTime: 2024-04-20 23:46:47
 * @FilePath: /rcli/src/opts.rs
 * @Description:
 *
 */
use std::path::Path;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(Csvopts),
}

#[derive(Debug, Parser)]
pub struct Csvopts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}
pub fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File doces not exist")
    }
}
