/*
 * @Author: yaoxingpu yaoxpu@163.com
 * @Date: 2024-04-20 23:51:07
 * @LastEditors: yaoxingpu yaoxpu@163.com
 * @LastEditTime: 2024-04-20 23:57:22
 * @FilePath: /rcli/src/process.rs
 * @Description:
 *
 */

use std::fs;

use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Player {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    /*             let records = reader
    .deserialize()
    .map(|record| record.unwrap())
    .collect::<Vec<Player>>(); */
    let mut ret = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let record: Player = result?;
        ret.push(record);
        // println!("{:?}", record);
    }
    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, json)?;
    Ok(())
}
