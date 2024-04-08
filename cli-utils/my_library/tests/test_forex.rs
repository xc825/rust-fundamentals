extern crate my_library;

use my_library::forex::{Money, Currency};

#[test]
fn test_money_display() {
    let money = Money::new(100.0, Currency::USD);
    assert_eq!(money.display(), "$100.00");
}

#[test]
fn test_money_convert() {
    let mut exchange_rates = vec![
        (Currency::USD, Currency::EUR, 0.92),
        (Currency::EUR, Currency::USD, 1.09),
        (Currency::GBP, Currency::EUR, 0.75),
        (Currency::EUR, Currency::GBP, 1.33),
    ];
    let money = Money::new(100.0, Currency::USD);
    let converted = money.convert(Currency::EUR, &mut exchange_rates,  &mut "0.95\n".as_bytes());
    assert_eq!(converted.unwrap().display(), "€92.00");
}

#[test]
fn test_money_convert_missing_rate() {
    let mut exchange_rates = vec![
        (Currency::USD, Currency::EUR, 0.92),
        (Currency::EUR, Currency::USD, 1.09),
        (Currency::GBP, Currency::EUR, 0.75),
        (Currency::EUR, Currency::GBP, 1.33),
    ];
    let money = Money::new(100.0, Currency::USD);

    let converted = money.convert(Currency::JPY, &mut exchange_rates, &mut "108\n".as_bytes());
    assert_eq!(converted.unwrap().display(), "¥10800.00");
}