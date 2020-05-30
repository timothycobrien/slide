use crate::bignum::Bignum;
use std::cmp::Ordering;

impl Ord for Bignum {
    fn cmp(&self, other: &Bignum) -> Ordering {
        let (lhs, rhs) = (strip_zeros(self), strip_zeros(other));
        // lhs and rhs have no leading or trailing (in decimal portion) zeros
        let greater: Ordering;
        let less: Ordering;
        if lhs.is_neg ^ rhs.is_neg { // need to check for negative 0
            // use the fact that a pure zero will have an empty int and empty dec vector
            if lhs.dec.is_empty() && rhs.dec.is_empty() && lhs.int.is_empty() && rhs.int.is_empty() {
                return Ordering::Equal;
            }
            return rhs.is_neg.cmp(&lhs.is_neg);
        } else if rhs.is_neg && lhs.is_neg {
            greater = Ordering::Less;
            less = Ordering::Greater;
        } else {
            greater = Ordering::Greater;
            less = Ordering::Less;
        }
        match lhs.int.len().cmp(&rhs.int.len()) {
            Ordering::Greater => return greater,
            Ordering::Less => return less,
            _ => (),
        }

        for i in (0..rhs.int.len()).rev() {
            match lhs.int[i].cmp(&rhs.int[i]) {
                Ordering::Greater => return greater,
                Ordering::Less => return less,
                _ => (),
            }
        }

        let smaller_dec_size: usize = if lhs.dec.len() > rhs.dec.len() {
            rhs.dec.len()
        } else {
            lhs.dec.len()
        };

        for i in 0..smaller_dec_size {
            match lhs.dec[i].cmp(&rhs.dec[i]) {
                Ordering::Greater => return greater,
                Ordering::Less => return less,
                _ => (),
            }
        }

        println!("smaller: {} ; lhs: {} ; rhs: {}", smaller_dec_size, lhs.dec.len(), rhs.dec.len());

        match lhs.dec.len().cmp(&rhs.dec.len()) {
            Ordering::Equal => Ordering::Equal,
            Ordering::Less => less,
            Ordering::Greater => greater,
        }
    }
}

fn strip_zeros(x: &Bignum) -> Bignum {
    let t_neg: bool = x.is_neg;
    let mut t_int: Vec<u8> = Vec::new();
    let mut t_dec: Vec<u8> = Vec::new();
    for i in 0..x.int.len() {
        t_int.push(x.int[i]);
    }
    for i in 0..x.dec.len() {
        t_dec.push(x.dec[i]);
    }
    // @todo clean up this code area
    let mut return_val = Bignum::new(String::from("0")).unwrap();
    return_val.is_neg = t_neg;
    return_val.int = t_int;
    return_val.dec = t_dec;

    println!("{:?}: len{} ;; {:?}: len{}", return_val.int, return_val.int.len(), return_val.dec, return_val.dec.len());

    for i in (0..x.dec.len()).rev() {
        if x.dec[i] == 0 {
            return_val.dec.remove(i);
            /* since the length of return_val decreases by one every time a zero is removed,
            the index does not have to be adjusted when iterating over x
             */
        }
        else {
            break;
        }
    }
    for i in (0..x.int.len()).rev() {
        if x.int[i] == 0 {
            return_val.int.remove(i);
            /* because int is stored backwards,
            implemented so that leading zeros are removed first
             */
        }
        else {
            break;
        }
    }
    return return_val;
}

impl Eq for Bignum {}

impl PartialEq for Bignum {
    fn eq(&self, other: &Bignum) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

// returns true if the rhs is smaller then lhs
impl PartialOrd for Bignum {
    fn partial_cmp(&self, other: &Bignum) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    macro_rules! bignum_test_cmp {
        ($($name: ident: $stmt:expr)*) => {
        $(
            #[test]
            fn $name() {
                use crate::bignum::Bignum;
                let v: Vec<&str> = $stmt.split(' ').collect();
                let lhs = Bignum::new(v[0].to_string()).unwrap();
                let rhs = Bignum::new(v[2].to_string()).unwrap();
                match v[1] {
                    "=" => assert!(lhs == rhs),
                    "!=" => assert!(lhs != rhs),
                    "<=" => assert!(lhs <= rhs),
                    ">=" => assert!(lhs >= rhs),
                    ">" => assert!(lhs > rhs),
                    "<" => assert!(lhs < rhs),
                    _ => panic!("Test input invalid"),
                }
            }
        )*
        }
    }

    mod cmp {
        bignum_test_cmp! {
            equal1: "1 = 1"
            equal2: "1.1 = 1.1"
            equal3: "-25.25 = -25.25"
            equal4: "1.0 = 1"
            equal5: "0.0 = 0"
            equal6: "0 = -0"
            equal7: "000 = 0"
            equal8: "000 = 0.0"
            equal9: "0.000 = 0.0"
            not_equal1: "1 != 2"
            not_equal2: "1.1 != 1.2"
            not_equal3: "2.1 != 1.1"
            not_equal4: "-1.1 != 1.1"
            not_equal5: "1.1 != 1"
            less_than1: "1 < 2"
            less_than2: "1.1 < 2.1"
            less_than3: "2.1 < 2.2"
            less_than4: "-1 < 1"
            less_than5: "-2 < -1"
            less_than6: "1 < 1.8"
            less_than_equal1: "1 < 2"
            less_than_equal2: "1.1 <= 2.1"
            less_than_equal3: "2.1 <= 2.2"
            less_than_equal4: "-1 <= 1"
            less_than_equal5: "-2 <= -1"
            less_than_equal6: "1 <= 1"
            less_than_equal7: "1.1 <= 1.1"
            less_than_equal8: "-25.25 <= -25.25"
            less_than_equal9: "1 <= 1.1"
            less_than_equal10: "1 <= 1.0"
            greater_than1: "2 > 1"
            greater_than2: "2.1 > 1.1"
            greater_than3: "2.2 > 2.1"
            greater_than4: "1 > -1"
            greater_than5: "-1 > -2"
            greater_than6: "1.2 > 1"
            greater_than7: "1 > .5"
            greater_than8: ".5 > .2"
            greater_than9: ".5 > -1"
            greater_than_equal1: "2 >= 1"
            greater_than_equal2: "2.1 >= 1.1"
            greater_than_equal3: "2.2 >= 2.1"
            greater_than_equal4: "1 >= -1"
            greater_than_equal5: "-1 >= -2"
            greater_than_equal6: "1 >= 1"
            greater_than_equal7: "1.1 >= 1.1"
            greater_than_equal8: "-25.25 >= -25.25"
            greater_than_equal9: "1.1 >= 1"
            greater_than_equal10: "1 >= 1.0"
        }
    }
}
