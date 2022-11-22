use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::io::{self, BufRead};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    currency: String,
}

#[derive(Deserialize, Debug)]
struct Input {
    amount: String,
    //TODO gas, reserved, token & so on
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let input = get_input()?;

    let amount = get_amount(input)?;
    let price = get_price(&opt.currency)?;

    let output = amount * price;

    println!("You have {:.2} {}", output, opt.currency);

    Ok(())
}

fn get_input() -> Result<Input> {
    let lines = read_lines_from_stdin()?;

    serde_json::from_str(&lines).map_err(|_| anyhow!("Input format not known"))
}

fn get_amount(input: Input) -> Result<f64> {
    input
        .amount
        .parse::<f64>()
        .map_err(|_| anyhow!("Failed to read amount from input"))
}

fn read_lines_from_stdin() -> Result<String> {
    let stdin = io::stdin();
    let mut lines = String::new();

    for line in stdin.lock().lines() {
        let line = line.map_err(|_| anyhow!("Could not read line from standard in"))?;

        lines.push_str(&line);
    }

    Ok(lines)
}

fn get_price(currency: &str) -> Result<f64> {
    let out = reqwest::blocking::get(format!(
        "https://api.coingecko.com/api/v3/simple/price?ids=golem&vs_currencies={}",
        currency
    ))
    .and_then(|r| r.text())
    .map_err(|_| anyhow!("Request to API failed"))?;

    serde_json::from_str::<serde_json::Value>(&out)
        .ok()
        .as_ref()
        .and_then(|v| v.get("golem"))
        .and_then(|v| v.get(currency))
        .and_then(|v| v.as_f64())
        .ok_or_else(|| anyhow!("Bad response from API"))
}
