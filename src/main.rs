/*
 * @Author: yaoxingpu yaoxpu@163.com
 * @Date: 2024-04-18 23:19:37
 * @LastEditors: yaoxingpu yaoxpu@163.com
 * @LastEditTime: 2024-04-21 00:30:15
 * @FilePath: /rcli/src/main.rs
 * @Description:
 *
 */
use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

//  cargo run -- csv -i ./assets/juventus.csv
fn main() -> anyhow::Result<()> {
    println!("Hello, rust");
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            process_csv(&opts.input, &opts.output)?;
        }
    }
    Ok(())
}
