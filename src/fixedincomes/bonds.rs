/*!
Implement Fixed Incomes modules for the financelib library

Module      : financelib::fixedIncomes::bonds <br>
Copyright   : (c) 2022 Kishaloy Neogi <br>
License     : MIT <br>
Maintainer  : Kishaloy Neogi <br>
Email       : <nkishaloy@yahoo.com>

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

    /**
    Price of Coupon bond given a discount RateCurve

    - rates  = Discount rate given as Nominal RateCurve
     */
    pub fn price_ratecurve(&self, rc: &rates::RateCurve) -> f64 {
        self.generate_cashflow()
            .iter()
            .enumerate()
            .map(|(i, &c)| rc.pv(c, ((i + 1) as f64) / self.freq))
            .sum()
    }

    /**
    Generate cash flow of the CouponBond
     */
    pub fn generate_cashflow(&self) -> Vec<f64> {
        let mut cb: Vec<f64> = (0..(self.freq * self.t_life) as i64)
            .into_iter()
            .map(|_| self.c / self.freq)
            .collect();
        cb[(self.freq * self.t_life) as usize - 1] += 1.0;
        cb
    }

    /**
    Calculates the accrued interest when the purchase is t periods into the next cycle.

    Note that t is in period, so for 26 days, in a full period of 1 year, t = 26/360
    */
    pub fn accrued_interest(&self, t: f64) -> f64 {
        t * self.c
    }

    /**
    PV Full when purchase is t periods into next cycle.
     */
    pub fn pv_full(&self, rate: f64, t: f64) -> f64 {
        self.price(rate) * (1.0 + rate / self.freq).powf(t * self.freq)
    }

    /**
    PV Flat when purchase is t periods into next cycle.
     */
    pub fn pv_flat(&self, rate: f64, t: f64) -> f64 {
        self.pv_full(rate, t) - self.accrued_interest(t)
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

        let cb = CouponBond {
            c: 0.05,
            freq: 2.0,
            t_life: 9.0,
        };

        assert_eq!(cb.accrued_interest(88.0 / 362.0), 0.012154696132596685);
        assert_eq!(cb.pv_full(0.048, 88.0 / 362.0), 1.0262432259347734);
        assert_eq!(cb.pv_flat(0.048, 88.0 / 362.0), 1.0140885298021767);
    }
}
