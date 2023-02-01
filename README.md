# Stock Market Quote

## Overview
This is a simple tool that fetches stock market quotes from the Yahoo! Finance API. It allows you to fetch the current price of a stock, as well as the price at a specific time in the past. It also includes a simple search feature that allows you to search for a stock by keyword.

## Usage
- To search for a ticker by keyword, 
    type in `stock search --name <NAME>` in command line.
    Here is an example for searching for keyword "rust":
    ```
    cargo run -- search --name "rust" 
    ```
    And the output will be:
    ```
    Searching for ticker containing keyword rust...
    Keystone Realtors Limited: RUSTOMJEE.NS
    Keystone Realtors Limited: RUSTOMJEE.BO
    U.S. Strategic Equity Fund Clas: RUSTX
    ```

- To fetch the latest price of a stock, 
    type in `cargo run -- latest --ticker <TICKER> <DURATION>` in command line.
    Here is an example for fetching the latest price of Tesla stock:
    ```
    cargo run -- latest --ticker "TSLA" "1d"
    ```
    And the output will be:
    ```
    Getting latest price of TSLA...
    The latest price of TSLA is $173.22000122070313 at 2023-01-31 21:00:04 UTC
    ```


### References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* [yahoo! finance API](https://crates.io/crates/yahoo_finance_api)
* [tokio-test](https://crates.io/crates/tokio-test)
