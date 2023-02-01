use chrono::prelude::*;
use clap::Parser;
use std::time::{Duration, UNIX_EPOCH};
use yahoo_finance_api as yahoo;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Caroline Chen",
    about = "Tool to get stock information"
)]

struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(about = "Search for ticker by entering a company name")]
    Search {
        #[clap(short, long)]
        name: String,
    },
    #[clap(about = "Get the latest price of a ticker")]
    Latest {
        #[clap(short, long)]
        ticker: String,
        duration: String
    },
}

fn main() {
    let source = yahoo::YahooConnector::new();
    let args = Cli::parse();
    match args.command {
        // if the first argument entered is "search" then search for the ticker given keyword
        Some(Commands::Search { name,  }) => {
            println!("Searching for ticker containing keyword {name}...");
            let response = tokio_test::block_on(source.search_ticker(&name)).unwrap();
            let mut _ticker_found = false;
            for i in response.quotes {
                println!("{}: {}", i.short_name, i.symbol);
            }
        }
        // if the first argument entered is "latest" then get the latest price of the ticker
        Some(Commands::Latest { ticker, duration }) => {
            println!("Getting latest price of {ticker}...");
            let response = tokio_test::block_on(source.get_latest_quotes(&ticker, &duration)).unwrap();
            let result = response.last_quote().unwrap();
            let time: DateTime<Utc> = UNIX_EPOCH
                .checked_add(Duration::from_secs(result.timestamp))
                .unwrap()
                .into();
            println!("The latest price of {} is ${} at {}", ticker, result.close, time);
        }
        None => {
            println!("No command entered");
        }
    }
}
