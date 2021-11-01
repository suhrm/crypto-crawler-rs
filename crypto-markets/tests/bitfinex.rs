use crypto_markets::{fetch_markets, fetch_symbols, get_market_types, MarketType};

#[macro_use]
mod utils;

const EXCHANGE_NAME: &str = "bitfinex";

#[test]
fn fetch_all_symbols() {
    gen_all_symbols!();
}

#[test]
fn fetch_spot_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::Spot).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.starts_with('t'));
    }
}

#[test]
fn fetch_linear_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::LinearSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.starts_with('t'));
        assert!(symbol.ends_with("F0:USTF0") || symbol.ends_with("F0:BTCF0"));
    }
}

#[test]
fn fetch_spot_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::Spot).unwrap();
    assert!(!markets.is_empty());

    let btc_usdt = markets.iter().find(|m| m.symbol == "tBTCUST").unwrap();
    assert_eq!(btc_usdt.precision.price, 5);
    assert_eq!(btc_usdt.precision.quantity, 8);
    assert_eq!(btc_usdt.quantity_limit.min, 0.00006);
    assert_eq!(btc_usdt.quantity_limit.max, 2000.0);
}

#[test]
fn fetch_linear_swap_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::LinearSwap).unwrap();
    assert!(!markets.is_empty());

    let btc_usdt = markets.iter().find(|m| m.symbol == "tBTCF0:USTF0").unwrap();
    assert_eq!(btc_usdt.precision.price, 5);
    assert_eq!(btc_usdt.precision.quantity, 8);
    assert_eq!(btc_usdt.quantity_limit.min, 0.00006);
    assert_eq!(btc_usdt.quantity_limit.max, 100.0);
}
