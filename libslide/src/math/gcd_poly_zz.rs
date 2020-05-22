use crate::math::gcd;

use core::cmp::{max, min};

type Poly = Vec<isize>;

fn gcd_poly_zz_heu(p: Poly, q: Poly) -> Option<(Poly, Poly, Poly)> {
    let p_norm = max_norm(&p);
    let q_norm = max_norm(&q);

    let B = 2 * min(p_norm, q_norm) + 29;
    let mut xi: usize = max(
        min(B, 10_000 * (B as f64).sqrt() as usize / 101),
        2 * min(
            p_norm / lc(&p).abs() as usize,
            q_norm / lc(&q).abs() as usize + 2,
        ),
    );

    const GCD_HEU_MAX_ITER: u8 = 6;
    let mut p_xi;
    let mut q_xi;
    let mut h;
    for _ in 0..GCD_HEU_MAX_ITER {
        p_xi = eval(&p, xi as isize);
        q_xi = eval(&q, xi as isize);

        eprintln!("p_xi={}, q_xi={}", p_xi, q_xi);

        if p_xi != 0 && q_xi != 0 {
            h = gcd(p_xi.abs() as usize, q_xi.abs() as usize) as isize;
            p_xi /= h;
            q_xi /= h;
        }

        xi = 73794 * xi * (xi as f64).sqrt().sqrt() as usize / 27011
    }

    None
}

fn max_norm(p: &Poly) -> usize {
    p.iter()
        .map(|&n| n.abs())
        .fold(0, |cur_max, n| max(cur_max, n as usize))
}

fn lc(p: &Poly) -> isize {
    *p.get(0).unwrap_or(&0)
}

fn eval(p: &Poly, x: isize) -> isize {
    p.iter().fold(0, |mut res, &n| {
        res *= x;
        res + n
    })
}

#[cfg(test)]
mod test {
    use super::gcd_poly_zz_heu;

    #[test]
    fn test_gcd_poly_zz_heu() {
        gcd_poly_zz_heu(vec![1, 0, -4], vec![1, -2]);
        assert!(false);
    }
}
