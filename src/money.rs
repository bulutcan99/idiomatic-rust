use std::{error::Error, num::ParseIntError, str::FromStr};

// Amount can't use f32 because of floating point precision
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Money {
    pub amount: i32,
    pub currency: Currency,
}

impl Money {
    pub fn new(amount: i32, currency: Currency) -> Self {
        Money { amount, currency }
    }

    pub fn get_amount(&self) -> i32 {
        self.amount
    }

    pub fn get_currency(&self) -> &Currency {
        &self.currency
    }
}

impl FromStr for Money {
    type Err = MoneyError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let splits: Vec<&str> = input.split_whitespace().collect();

        match splits[..] {
            [amount, currency] => {
                let amount = amount.parse::<i32>()?;
                let currency: Currency = currency.parse()?;
                Ok(Money::new(amount, currency))
            }
            _ => Err(MoneyError {
                kind: MoneyErrorKind::FormatError,
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Currency {
    Lira,
    Dollar,
    Euro,
}

impl FromStr for Currency {
    type Err = MoneyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "lira" | "₺" | "tl" => Ok(Currency::Lira),
            "dollar" | "$" | "usd" => Ok(Currency::Dollar),
            "euro" | "€" | "eur" => Ok(Currency::Euro),
            _ => Err(MoneyError {
                kind: MoneyErrorKind::InvalidCurrency,
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoneyError {
    kind: MoneyErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoneyErrorKind {
    FormatError,
    ParseError,
    InvalidCurrency,
}

impl Error for MoneyError {}

impl std::fmt::Display for MoneyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.kind {
            MoneyErrorKind::FormatError => write!(
                f,
                "Format error: input should be in the form of <amount> <currency>"
            ),
            MoneyErrorKind::ParseError => {
                write!(f, "Parse error: could not convert input to an integer")
            }
            MoneyErrorKind::InvalidCurrency => {
                write!(
                    f,
                    "Invalid currency: the specified currency is not supported"
                )
            }
        }
    }
}

impl From<ParseIntError> for MoneyError {
    fn from(_: ParseIntError) -> Self {
        MoneyError {
            kind: MoneyErrorKind::ParseError,
        }
    }
}

// Test Modülü
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_integer() {
        let result = "4 $".parse::<Money>();
        assert!(result.is_ok());
        let money = result.unwrap();
        assert_eq!(money.get_amount(), 4);
        assert_eq!(money.get_currency(), &Currency::Dollar);
    }

    #[test]
    fn negative_integer() {
        let result = "-4 €".parse::<Money>();
        assert!(result.is_ok());
        let money = result.unwrap();
        assert_eq!(money.get_amount(), -4);
        assert_eq!(money.get_currency(), &Currency::Euro);
    }

    #[test]
    fn invalid_input() {
        let result = "not_valid $".parse::<Money>();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind, MoneyErrorKind::ParseError);
    }

    #[test]
    fn insufficient_input() {
        let result = "4".parse::<Money>();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind, MoneyErrorKind::FormatError);
    }

    #[test]
    fn invalid_currency() {
        let result = "4 yen".parse::<Money>();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind, MoneyErrorKind::InvalidCurrency);
    }

    #[test]
    fn invalid_amount() {
        let result = "4a $".parse::<Money>();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind, MoneyErrorKind::ParseError);
    }
}
