use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::time::{Duration, Instant};
use tungstenite::{connect, Message};
use url::Url;
use serde_json::Value;

const WEBSOCKET_URL: &str = "wss://ws.coincap.io/prices?assets=bitcoin";
const FILE_NAME: &str = "btc_prices.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: ./simple --mode=<cache|read>");
        std::process::exit(1);
    }

    let mode = &args[1];
    match mode.as_str() {
        "--mode=cache" => cache_mode()?,
        "--mode=read" => read_mode()?,
        _ => {
            eprintln!("Invalid mode. Use --mode=cache or --mode=read");
            std::process::exit(1);
        }
    }

    Ok(())
}

fn cache_mode() -> Result<(), Box<dyn std::error::Error>> {
    let (mut socket, _) = connect(Url::parse(WEBSOCKET_URL)?)?;
    let mut prices = Vec::new();
    let start = Instant::now();

    while start.elapsed() < Duration::from_secs(10) {
        let msg = socket.read_message()?;
        if let Message::Text(text) = msg {
            let v: Value = serde_json::from_str(&text)?;
            if let Some(price_str) = v["bitcoin"].as_str() { // Change to access "bitcoin" directly
                prices.push(price_str.parse::<f64>()?);
            }
        }
    }

    if !prices.is_empty() {
        let aggregate = prices.iter().sum::<f64>() / prices.len() as f64;
        let mut file = File::create(FILE_NAME)?;
        writeln!(file, "Aggregate: {}", aggregate)?;
        writeln!(file, "Data points:")?;
        for price in prices {
            writeln!(file, "{}", price)?;
        }

        println!("Cache complete");
    } else {
        println!("No prices received in the time limit.");
    }

    Ok(())
}

fn read_mode() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open(FILE_NAME)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", contents);
    Ok(())
}
