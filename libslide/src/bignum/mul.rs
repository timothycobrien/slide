#![allow(clippy::suspicious_arithmetic_impl)]
use crate::bignum::utils::convert_poly;
use crate::bignum::utils::fft;
use crate::bignum::utils::ifft;
use crate::bignum::utils::ispowerof2;
use crate::bignum::utils::normalize_vecs;
use crate::bignum::utils::recast_user_vec;
use crate::bignum::utils::truncate_zeros;
use crate::bignum::Bignum;
use std::ops;

impl ops::Mul for Bignum {
    type Output = Bignum;

    fn mul(self, rhs: Bignum) -> Bignum {
        let is_neg: bool = self.is_neg ^ rhs.is_neg;
        // the length of the final size is the sum of all the initial sizes
        let total_length = self.dec.len() + self.int.len() + rhs.dec.len() + rhs.int.len();
        // need self and rhs dec lengths for split later
        // we need to make the vectors the same length at both the decimal and integer level.
        let (mut lhs_dec, mut rhs_dec) = normalize_vecs(self.dec, rhs.dec);
        let dec_len = lhs_dec.len() * 2;

        let (mut lhs_int, mut rhs_int) = normalize_vecs(self.int, rhs.int);

        // vector lengths must be a power of 2 to make use of recursive fft
        while rhs_int.len() + rhs_dec.len() < total_length {
            rhs_int.push(0);
            lhs_int.push(0);
        }
        while !ispowerof2(rhs_int.len() + rhs_dec.len()) || rhs_int.len() <=2 {
            rhs_int.push(0);
            lhs_int.push(0);
        }
        rhs_dec = rhs_dec.into_iter().rev().collect();
        lhs_dec = lhs_dec.into_iter().rev().collect();
        println!("rhs:{} -- lhs:{}", rhs_dec.len(), lhs_dec.len());
        rhs_dec.append(&mut rhs_int);
        lhs_dec.append(&mut lhs_int);
        println!("rhs:{} -- lhs:{}", rhs_dec.len(), lhs_dec.len());
        let rhs_len = rhs_dec.len();
        let rhs_cplx = recast_user_vec(rhs_dec).unwrap();
        let lhs_cplx = recast_user_vec(lhs_dec).unwrap();
        let mut rhs_fft = fft(rhs_cplx, rhs_len, false);
        let lhs_fft = fft(lhs_cplx, rhs_len, false);

        for i in 0..rhs_len {
            rhs_fft[i] = rhs_fft[i] * lhs_fft[i];
        }
        println!("rhs_fft: {:?} --- rhs_len{}", rhs_fft, rhs_len);

        let res: Vec<u16> = recast_user_vec(
            ifft(rhs_fft, rhs_len)
                .into_iter()
                .map(|e| e.round())
                .collect(),
        )
        .unwrap();
        println!("{}", res.len());
        let mut dec_vec: Vec<u8> = convert_poly(res);
        println!("dec_vec: {:?} --- dec_len: {} --- dec_vec.len(): {}", dec_vec, dec_len, dec_vec.len());
        let int_vec: Vec<u8> = dec_vec.split_off(dec_len);

        // remove preceeding zeros
        dec_vec = dec_vec.into_iter().rev().collect();
        Bignum {
            is_neg,
            int: truncate_zeros(int_vec),
            dec: truncate_zeros(dec_vec),
        }
    }
}

#[cfg(test)]
mod tests {
    macro_rules! bignum_test_mul {
        ($($name: ident: $lhs:expr, $rhs:expr, $program:expr)*) => {
        $(
            #[test]
            fn $name() {
                use crate::bignum::Bignum;
                let lhs = Bignum::new($lhs.to_string()).unwrap();
                let rhs = Bignum::new($rhs.to_string()).unwrap();
                let result = $program.to_string();
                assert_eq!((lhs*rhs).to_string(), result);
            }
        )*
        }
    }

    mod mul {
        bignum_test_mul! {
            int1: "99", "99", "9801"
            int2: "5", "5", "25"
            int3: "10", "10", "100"
            int4: "999", "999", "998001"
            int5: "9", "1", "9"
            int6: "9", "10", "90"
            int7: "25", "25", "625"
            int8: "12345678", "12345678", "152415765279684"
            float1: "0.001", "0.01", "0.00001"
            float2: "0.003", "0.212", "0.000636"
            float3: "0.192781230589", "0.12182387511", "0.02348535655882644773979"
            mixed1: "1.0", "1.0", "1"
            mixed2: "12912572835.19235098273", "19325812.193812388322389", "249545957551850939.50568665382187490021134197"
            mixed3: ".5", "5", "2.5"
            mixed4: ".5", "5.2", "2.6"
            mixed5: "100", ".25", "25"
            mixed6: "200", ".5", "100"
            mixed7: "0.01", "4.12564388199914384891937474984", ".0412564388199914384891937474984"
            zero1: "0000.00", "0", "0"
            zero2: "0.0000", "0", "0"
            zero3: "0", "0.000", "0"
            zero4: "0.0", "00.00", "0"
            zero5: "-0", "00.00", "0"
            zero6: "0.0000", "-00000.000", "0"
            zero7: "-0.00", "-0000.00", "0"
            zero8: "0.00", "2.23131442121312311", "0"
            trailing_zero1: "1.00000", "000001.0000", "1"
        }
    }
}
