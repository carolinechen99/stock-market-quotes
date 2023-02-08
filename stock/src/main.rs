use chrono::prelude::*;
use chrono::{TimeZone, Utc};
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
        duration: String,
    },
    // Get history of quotes for given time period
    #[clap(about = "Get the history of a ticker")]
    History {
        #[clap(short, long)]
        ticker: String,
        #[clap(short, long)]
        // start time has format ymd, e.g. 20210101
        starttime: String,
        #[clap(short, long)]
        // end time has format ymd, e.g. 20210101
        endtime: String,
    },
}

fn main() {
    let source = yahoo::YahooConnector::new();
    let args = Cli::parse();
    match args.command {
        // if the first argument entered is "search" then search for the ticker given keyword
        Some(Commands::Search { name }) => {
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
            let response =
                tokio_test::block_on(source.get_latest_quotes(&ticker, &duration)).unwrap();
            let result = response.last_quote().unwrap();
            let time: DateTime<Utc> = UNIX_EPOCH
                .checked_add(Duration::from_secs(result.timestamp))
                .unwrap()
                .into();
            println!(
                "The latest price of {} is ${} at {}",
                ticker, result.close, time
            );
        }
        // if the first argument entered is "history" then get the history of the ticker
        Some(Commands::History {
            ticker,
            starttime,
            endtime,
        }) => {
            println!("Getting history of {ticker}...");
            // use `with_ymd_and_hms()` to create a `DateTime` object
            let start = Utc.with_ymd_and_hms(
                starttime[0..4].parse().unwrap(),
                starttime[4..6].parse().unwrap(),
                starttime[6..8].parse().unwrap(),
                0,
                0,
                0,
            );
            let end = Utc.with_ymd_and_hms(
                endtime[0..4].parse().unwrap(),
                endtime[4..6].parse().unwrap(),
                endtime[6..8].parse().unwrap(),
                23,
                59,
                59,
            );

            // get the response, start and end time type is chrono::DateTime
            let response = tokio_test::block_on(source.get_quote_history(&ticker, start.unwrap(), end.unwrap())).unwrap();

            let result = response.quotes().unwrap();
            // print out the result
            println!("{result:?}");
            
        }
        None => {
            println!("No command entered");
        }
    }
}
