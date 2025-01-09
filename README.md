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

### Examples

#### Show the original data from ECB
![eur-to-all](screenshots/eur-to-all.png)

#### ...with only select currencies

![eur-to-all](screenshots/eur-to-all-select.png)

#### Put the exchange rate in the perspective of any currency

![usd-to-all](screenshots/usd-to-all.png)

#### Flip it

![all-to-usd](screenshots/all-to-usd.png)

## Acknowledgment

The data is (obviously) provided by the [European Central Bank](https://www.ecb.europa.eu/stats/policy_and_exchange_rates/euro_reference_exchange_rates/html/index.en.html)