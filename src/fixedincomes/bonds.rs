/*!
Module      : financelib::fixedIncomes::bonds <b>
Description : Implement Fixed Incomes modules for the financelib library <br>
Copyright   : (c) 2022 Kishaloy Neogi <br>
License     : MIT <br>
Maintainer  : Kishaloy Neogi <br>
Email       : nkishaloy@yahoo.com <br>

The module describes the base modules of Bonds.

You may see the github repository at <https://github.com/n-kishaloy/financelib>
 */
pub mod rates;

/**
CouponBond : struct defining a Coupon bond..

- c     = Coupon per period
- freq  = Frequency of coupon payment per period
- T     = Life of the Bond
 */
#[derive(Debug, Copy, Clone)]
pub struct CouponBond {
    pub c: f64,
    pub freq: f64,
    pub t_life: f64,
}

impl CouponBond {
    /**
    Price of Coupon bond given a discount rate

    - rate  = Discount rate given as Nominal rate
     */
    pub fn price(&self, rate: f64) -> f64 {
        crate::pv_annuity(self.c / self.freq, rate, self.t_life, self.freq)
            + crate::pvm(1.0, rate, self.t_life, self.freq)
    }

    /**
    YTM of a Coupon bond given its price

    - price = Price of Coupon bond
     */
    pub fn ytm(&self, price: f64) -> f64 {
        crate::newt_raph(|r| self.price(r) - price, 0.05, 1e-6).unwrap()
    }

    pub fn price_ratecurve(&self, rc: &rates::RateCurve) -> f64 {
        self.generate_cashflow()
            .iter()
            .enumerate()
            .map(|(i, &c)| rc.pv(c, ((i + 1) as f64) / self.freq))
            .sum()
    }

    pub fn generate_cashflow(&self) -> Vec<f64> {
        let mut cb: Vec<f64> = (0..(self.freq * self.t_life) as i64)
            .into_iter()
            .map(|_| self.c / self.freq)
            .collect();
        cb[(self.freq * self.t_life) as usize - 1] += 1.0;
        cb
    }
}

#[cfg(test)]
mod bonds_fn {
    use super::*;
    #[test]
    fn coupon_bonds() {
        let cb = CouponBond {
            c: 0.05,
            freq: 2.0,
            t_life: 3.0,
        };
        assert_eq!(cb.price(0.03), 1.056971871654752);
        println!("{:?}", cb.generate_cashflow());
        assert_eq!(
            cb.price_ratecurve(&rates::RateCurve::NominalRateCurve {
                rate: vec![0.0016, 0.0021, 0.0027, 0.0033, 0.0037, 0.0041],
                freq: 2.0
            }),
            1.1369147941993403
        );
        assert_eq!(cb.ytm(1.1369147941993403), 0.004038639185260506);
        assert!(crate::approx(cb.ytm(1.056971871654752), 0.03));
    }
}
