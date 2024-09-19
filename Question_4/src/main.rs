use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::time::{Duration, Instant};
use reqwest::blocking::Client;
use serde_json::Value;

const API_URL: &str = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";
const OUTPUT_FILE: &str = "btc_prices.json";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: ./simple --mode=<cache|read>");
        std::process::exit(1);
    }

    let mode = &args[1];
    match mode.as_str() {
        "--mode=cache" => cache_mode(),
        "--mode=read" => read_mode(),
        _ => {
            eprintln!("Invalid mode. Use --mode=cache or --mode=read");
            std::process::exit(1);
        }
    }
}

fn cache_mode() {
    let client = Client::new();
    let start = Instant::now();
    let mut prices = Vec::new();

    while start.elapsed() < Duration::from_secs(10) {
        match client.get(API_URL).send() {
            Ok(response) => {
                if let Ok(json) = response.json::<Value>() {
                    if let Some(price) = json["bitcoin"]["usd"].as_f64() {
                        prices.push(price);
                        println!("Fetched price: ${}", price); // Added for visibility
                    }
                }
            }
            Err(e) => eprintln!("Error fetching data: {}", e),
        }
        std::thread::sleep(Duration::from_secs(1));
    }

    if prices.is_empty() {
        eprintln!("No price data collected. Exiting.");
        std::process::exit(1);
    }

    let aggregate = prices.iter().sum::<f64>() / prices.len() as f64;

    let output = serde_json::json!({
        "aggregate": aggregate,
        "data_points": prices
    });

    let file = File::create(OUTPUT_FILE).expect("Unable to create file");
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, &output).expect("Unable to write data");

    println!("Cache complete. Aggregate price: ${:.2}", aggregate);
}

fn read_mode() {
    let file = File::open(OUTPUT_FILE).expect("Unable to open file");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("Unable to read file");
    println!("{}", contents);
}