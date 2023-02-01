# Stock Market Quote

## Overview
This is a simple tool that fetches stock market quotes from the Yahoo! Finance API. It allows you to fetch the current price of a stock, as well as the price at a specific time in the past. It also includes a simple search feature that allows you to search for a stock by keyword.

## Usage
- To search for a ticker by keyword, type in
`stock search --name <NAME>` in command line.
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


### References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* [yahoo! finance API](https://crates.io/crates/yahoo_finance_api)
* [tokio-test](https://crates.io/crates/tokio-test)
