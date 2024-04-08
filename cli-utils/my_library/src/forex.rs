use std::io::{self, Write, BufRead};

#[derive(Copy, Clone)]
#[derive(PartialEq)]
pub enum Currency {
    USD,
    EUR,
    GBP,
    JPY,
}

impl Currency {
    pub fn symbol(&self) -> &'static str {
        match self {
            Currency::USD => "$",
            Currency::EUR => "€",
            Currency::GBP => "£",
            Currency::JPY => "¥",
        }
    }
}

/// Represents a certain amount of money in a specific currency.
pub struct Money {
    /// The amount of money.
    pub amount: f64,
    /// The currency of the money.
    pub currency: Currency,
}

impl Money {
    /// Creates a new `Money` instance.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount of money.
    /// * `currency` - The currency of the money.
    ///
    /// # Returns
    ///
    /// A new `Money` instance.
    pub fn new(amount: f64, currency: Currency) -> Self {
        Self { amount, currency }
    }

    /// Converts the money to a different currency.
    ///
    /// # Arguments
    ///
    /// * `to_currency` - The currency to convert to.
    /// * `exchange_rates` - A mutable reference to a vector of exchange rates.
    /// * `input` - A mutable reference to a `BufRead` instance.
    ///
    /// # Returns
    ///
    /// An `Option` that contains a `Money` instance in the new currency if the conversion was successful, or `None` if it wasn't.
    pub fn convert<R: BufRead>(&self, to_currency: Currency, exchange_rates: &mut Vec<(Currency, Currency, f64)>, input: &mut R) -> Option<Money> {
        let mut converted_amount = self.amount;
        let mut found = false;
        for (from, to, rate) in &mut *exchange_rates {
            if self.currency == *from && to_currency == *to {
                converted_amount *= *rate;
                found = true;
                break;
            }
        }
                match found {
                    true => Some(Money::new(converted_amount, to_currency)),
                    false => {
                        println!("Add: {} to {} exchange rate:", self.currency.symbol(), to_currency.symbol());
                        let mut input_text = String::new();
                        let reader = input;
                        io::stdout().flush().unwrap(); // Make sure the prompt is immediately displayed
                        reader.read_line(&mut input_text).expect("Failed to read line!");
                        let exchange_rate: f64 = input_text.trim().parse().expect("Please enter a number!");
                        exchange_rates.push((self.currency, to_currency, exchange_rate));
                        converted_amount *= exchange_rate;
                        Some(Money::new(converted_amount, to_currency))
                    },
                }
    }
    pub fn display(&self) -> String {
        format!("{}{:.2}", self.currency.symbol(), self.amount)
    }
}
