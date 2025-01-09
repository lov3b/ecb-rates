# ECB Rates

A cli utility to fetch the currency rates against the Euro from the ECB.

## Install

First, make sure that you have the rust toolchain installed. If not, then go to [rustup](https://rustup.rs) to install it.

Now, Git clone the project, then cd into the projects root-dir. Thereafter run:

```sh
cargo install --path .
```

Congratulations! Now the cli binary `ecb-rates` will be in your cargo bin folder.

## Features

#### Fetch in different views

- Last available day.
- Last 90 days
- Since the dawn of the *EUR*

#### Display select currencies

- as an ASCII table
- in JSON prettified
- in JSON minified

#### Cache

It features an extensive cache, which will [calculate hollidays](src/holiday.rs) in order to know whether to invalidate it or not.

### Example

```sh
ecb-rates -c sek -c nok -c usd
```

```plain
     2025-01-07
Currency         Rate
---------------------
USD            1.0393
SEK            11.475
NOK           11.7385
```

## Acknowledgment

The data is (obviously) provided by the [European Central Bank](https://www.ecb.europa.eu/stats/policy_and_exchange_rates/euro_reference_exchange_rates/html/index.en.html)