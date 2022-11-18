use serde::Deserialize;
use std::io::{self, BufRead};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    currency: String,
}

#[derive(Deserialize, Debug)]
struct Data {
    amount: String,
    // token: String,
    //TODO Rafał gas, reserved & so on
}

fn main() {
    let opt = Opt::from_args();

    let stdin = io::stdin();

    let mut lines = String::new();

    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");

        lines.push_str(&line);
    }

    let parsed: Data = serde_json::from_str(&lines).unwrap();

    let out = reqwest::blocking::get(format!(
        "https://api.coingecko.com/api/v3/simple/price?ids=golem&vs_currencies={}",
        opt.currency
    ))
    .unwrap()
    .text()
    .unwrap();

    let out_parsed: serde_json::Value = serde_json::from_str(&out).unwrap();

    let price = &out_parsed
        .get("golem")
        .unwrap()
        .get(&opt.currency)
        .unwrap()
        .as_f64()
        .unwrap();

    let output: f64 = price * parsed.amount.parse::<f64>().unwrap();

    println!("You have {} {}", output, opt.currency);
}
