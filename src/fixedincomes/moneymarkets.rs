/*!
Implement money market formulas for the financeLib library

Module      : financelib::fixedincomes::moneymarkets <br>
Copyright   : (c) 2022 Kishaloy Neogi <br>
License     : MIT <br>
Maintainer  : Kishaloy Neogi <br>
Email       : <nkishaloy@yahoo.com>

You may see the github repository at <https://github.com/n-kishaloy/financelib>
*/

pub fn t_bill_r(t: f64, p0: f64, f: f64) -> f64 {
    (1.0 - p0 / f) * 360.0 / t
}

pub fn t_bill_d(r: f64, t: f64, f: f64) -> f64 {
    r * t * f / 360.0
}

pub fn holding_per_yield(p0: f64, p1: f64, d1: f64) -> f64 {
    (p1 + d1) / p0 - 1.0
}

pub fn eff_ann_yield(t: f64, p0: f64, p1: f64, d1: f64) -> f64 {
    ((p1 + d1) / p0).powf(365.0 / t) - 1.0
}

pub fn money_mkt_yield(t: f64, p0: f64, p1: f64, d1: f64) -> f64 {
    ((p1 + d1) / p0 - 1.0) * 360.0 / t
}

pub fn twrr_n(n: f64, bv: &Vec<f64>, b_inf: &Vec<f64>) -> f64 {
    let mut r = 1.0;
    for i in 0..(bv.len() - 1) {
        r *= bv[i + 1] / (bv[i] + b_inf[i]);
    }
    r.powf(1.0 / n) - 1.0
}

pub fn twrr(bv: &Vec<f64>, b_inf: &Vec<f64>) -> f64 {
    twrr_n((bv.len() - 1) as f64, &bv, &b_inf)
}

#[cfg(test)]
mod money_markets_fn {
    use super::*;
    use crate::approx;

    #[test]
    fn formulas() {
        assert!(approx(t_bill_r(150.0, 98_000.0, 100_000.0), 0.048));
        assert!(t_bill_d(0.048, 150.0, 100_000.0) == 2_000.0);

        assert!(holding_per_yield(98.0, 95.0, 5.0) == 0.020408163265306145);
        assert!(eff_ann_yield(150.0, 98.0, 95.0, 5.0) == 0.05038831660532006);
        assert!(money_mkt_yield(150.0, 98.0, 95.0, 5.0) == 0.04897959183673475);

        assert_eq!(
            twrr(
                &vec![4.0, 6.0, 5.775, 6.72, 5.508],
                &vec![1.0, -0.5, 0.225, -0.6]
            ),
            0.06159232319186159
        );
        assert_eq!(
            twrr_n(1.0, &vec![100.0, 112.0, 142.64], &vec![0.0, 20.0]),
            0.21027878787878795
        );
    }
}
