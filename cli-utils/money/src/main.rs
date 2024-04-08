extern crate my_library;

use my_library::forex::{Money, Currency};
use std::io::{BufReader};

fn main() {

    let mut logging = my_library::config::Logging::new();
    println!("Logging level: {:?}", logging.get_level());
    println!("Setting logging level to Debug");
    logging.set_level(my_library::config::LogLevel::Debug);
    println!("Logging level: {:?}", logging.get_level());

    #[allow(unused_mut)]
    let mut exchange_rates: Vec<(Currency, Currency, f64)> = vec![
        (Currency::USD, Currency::EUR, 0.92),
        (Currency::EUR, Currency::USD, 1.09),
        (Currency::GBP, Currency::EUR, 0.75),
        (Currency::EUR, Currency::GBP, 1.33),
        // Add more currencies and their exchange rates as needed
    ];

    let mut my_eurs = Money::new(100.0, my_library::forex::Currency::EUR);

    println!("Convert {} to {}", my_eurs.display(), my_library::forex::Currency::JPY.symbol());
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    #[allow(unused_mut)]
    let mut to_yens = my_eurs.convert(my_library::forex::Currency::JPY, &mut exchange_rates, &mut reader);
    match to_yens {
        Some(yens) => println!("{} is {}", my_eurs.display(), yens.display()),
        None => {
                    println!("Conversion failed");
                }
    }

    println!("\nConvert {} to {}", my_eurs.display(), my_library::forex::Currency::USD.symbol());
    let to_usd = my_eurs.convert(my_library::forex::Currency::USD, &mut exchange_rates, &mut reader);
    match to_usd {
        Some(usd) => println!("{} is {}", my_eurs.display(), usd.display()),
        None => println!("Conversion failed"),
    }

    my_eurs = Money::new(300.0, my_library::forex::Currency::EUR);
    println!("My EUR = {}", my_eurs.display());
    println!("\nConvert {} to {}", my_eurs.display(), my_library::forex::Currency::JPY.symbol());
    let to_yens = my_eurs.convert(my_library::forex::Currency::JPY, &mut exchange_rates, &mut reader);
    match to_yens {
        Some(usd) => println!("{} is {}", my_eurs.display(), usd.display()),
        None => println!("Conversion failed"),
    }
}
