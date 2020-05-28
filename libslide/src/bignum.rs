use std::fmt;
mod add;
mod compare;
mod complex;
mod modulo;
mod mul;
mod negate;
mod sub;
mod utils;

pub use modulo::*;
// this probably can go in utils but I put it in here for now
fn to_u8(c: char) -> u8 {
    c.to_digit(10).unwrap() as u8
}

fn to_char(n: u8) -> char {
    std::char::from_digit(n as u32, 10).unwrap()
}

#[derive(Clone)]
pub struct Bignum {
    is_neg: bool,
    // we use vector since it will allow faster computation time for smaller numbers.
    int: Vec<u8>, // integer part with LSB first, e.g. int of 123.456 is 321.
    dec: Vec<u8>, // decimal part with MSB first, e.g. dec of 123.456 is 456.
}

static INPUT_ERR_MSG: &str = "Input is not valid";

impl Bignum {
    pub fn new(input: String) -> Result<Bignum, &'static str> {
        let mut chars = input.chars();
        let mut is_neg: bool = false;
        let mut has_decimal = false;
        let mut int = Vec::new();
        let mut dec = Vec::new();

        if input.is_empty() {
            return Err(INPUT_ERR_MSG);
        }

        match chars.next() {
            Some('-') => is_neg = true,
            Some('.') => has_decimal = true,
            Some(x) => int.push(to_u8(x)),
            _ => (),
        };
        for c in chars {
            if c.is_digit(10) {
                if has_decimal {
                    dec.push(to_u8(c));
                } else {
                    int.push(to_u8(c));
                }
            } else if c == '.' {
                has_decimal = !has_decimal;
                if !has_decimal {
                    return Err(INPUT_ERR_MSG);
                }
            } else {
                return Err(INPUT_ERR_MSG);
            }
        }
        if !dec.is_empty() && dec.iter().all(|&i| i == 0) {
            dec.clear();
        }
        Ok(Bignum {
            is_neg,
            int: int.into_iter().rev().collect(),
            dec,
        })
    }
}

impl fmt::Display for Bignum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        if self.is_neg {
            result.push('-');
        }
        if self.int.is_empty() {
            result.push('0');
        } else {
            for n in self.int.iter().rev() {
                result.push(to_char(*n));
            }
        }
        if !self.dec.is_empty() {
            result.push('.');
            for n in self.dec.iter() {
                result.push(to_char(*n));
            }
        }
        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests {
    macro_rules! bignum_test {
        ($($name:ident: $program:expr)*) => {
        $(
            #[test]
            fn $name() {
                use crate::bignum::Bignum;
                let result = $program.to_string();
                let bg = Bignum::new(result.clone());
                assert_eq!(bg.unwrap().to_string(), result);
            }
        )*
        }
    }
    mod num {
        bignum_test! {
            int: "5"
            negative_int: "-5"
            float: "5.5"
            negative_float: "-5.5"
            bigger_int: "1002039444884993020"
            bigger_float: "102399959939999.393993"
            decimal: "0.3"
            negative_decimal: "-0.3"
        }
    }
}
