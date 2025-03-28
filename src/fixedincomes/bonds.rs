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

pub mod durations;
pub mod rates;

/**
CouponBond : struct defining a Coupon bond..

- par   = Par value
- c     = Coupon rate per period
- freq  = Frequency of coupon payment per period
- T     = Life of the Bond
 */
#[derive(Debug, Copy, Clone)]
pub struct CouponBond {
    pub par: f64,
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
        crate::pv_annuity(
            rate,
            self.t_life,
            self.freq,
            -self.c / self.freq * self.par,
            0.0,
        ) + crate::pvm(rate, self.t_life, self.freq, self.par)
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

    - rc = Discount rate given as Nominal RateCurve
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
        let c = self.par * self.c / self.freq;
        let sz = (self.freq * self.t_life) as usize;
        let mut cb: Vec<f64> = vec![c; sz];
        cb[sz - 1] += self.par;
        cb
    }

    /**
    Calculates the accrued interest when the purchase is t periods into the next cycle.

    Note that t is in period, so for 26 days, in a full period of 1 year, t = 26/360
    */
    pub fn accrued_interest(&self, t: f64) -> f64 {
        t * self.c * self.par
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

/**
CouponBond : struct defining a Coupon bond..

- par               = Par value
- quoted_margin     = Quoted margin payment over the Index rate per period
- freq              = Frequency of payment per period
- discount_margin
- T             = Life of the Bond
 */
#[derive(Debug, Copy, Clone)]
pub struct FloatingRateNotes {
    pub par: f64,
    pub quoted_margin: f64,
    pub freq: f64,
    pub t_life: f64,
}

impl FloatingRateNotes {
    /**
    Price of Floating Rate Note given a discount rate

    - rate  = Discount rate given as Nominal rate
     */
    pub fn price(&self, index_rate: f64, discount_margin: f64) -> f64 {
        crate::pv_annuity(
            index_rate + discount_margin,
            self.t_life,
            self.freq,
            -(self.quoted_margin + index_rate) * self.par / self.freq,
            0.0,
        ) + crate::pvm(
            index_rate + discount_margin,
            self.t_life,
            self.freq,
            self.par,
        )
    }

    /**
    Discount margin of a Floating Rate Note given the price and index rate

    - price         = Price of the Floating Rate Note
    - index_rate    = Index rate
     */
    pub fn discount_margin(&self, price: f64, index_rate: f64) -> f64 {
        crate::newt_raph(|x| self.price(index_rate, x) - price, 0.005, 1e-6).unwrap()
    }

    /**
    Price of Floating Rate Note given a Index RateCurve

    - rc  = Discount rate given as Nominal RateCurve
     */
    pub fn price_ratecurve(&self, _idx_rc: &rates::RateCurve, _discount_margin: f64) -> f64 {
        // TODO: Add imolementation
        unimplemented!()
    }

    /**
    Discount margin of a Floating Rate Note given a Index RateCurve and a Price
     */
    pub fn discount_margin_ratecurve(&self, price: f64, idx_rc: &rates::RateCurve) -> f64 {
        // TODO: Check implementation
        crate::newt_raph(|x| self.price_ratecurve(idx_rc, x) - price, 0.005, 1e-6).unwrap()
    }
}

#[cfg(test)]
mod bonds_fn {
    use super::*;
    #[test]
    fn coupon_bonds() {
        let cb = CouponBond {
            par: 100.0,
            c: 0.05,
            freq: 2.0,
            t_life: 3.0,
        };
        assert_eq!(cb.price(0.03), 105.6971871654752);
        // println!("{:?}", cb.generate_cashflow());
        assert_eq!(
            cb.price_ratecurve(&rates::RateCurve::NominalRateCurve {
                rate: vec![0.0016, 0.0021, 0.0027, 0.0033, 0.0037, 0.0041],
                freq: 2.0
            }),
            113.69147941993403
        );
        assert_eq!(cb.ytm(113.69147941993403), 0.004038639185261329);
        assert!(crate::approx(cb.ytm(105.6971871654752), 0.03));

        let cb = CouponBond {
            par: 100.0,
            c: 0.05,
            freq: 2.0,
            t_life: 9.0,
        };

        assert_eq!(cb.accrued_interest(88.0 / 362.0), 1.2154696132596685);
        assert_eq!(cb.pv_full(0.048, 88.0 / 362.0), 102.62432259347733);
        assert_eq!(cb.pv_flat(0.048, 88.0 / 362.0), 101.40885298021766);

        assert_eq!(
            FloatingRateNotes {
                par: 100.0,
                quoted_margin: 0.005,
                freq: 2.0,
                t_life: 2.0,
            }
            .price(0.0125, 0.004),
            100.19594209266003
        );

        assert_eq!(
            FloatingRateNotes {
                par: 100.0,
                quoted_margin: 0.0075,
                freq: 4.0,
                t_life: 5.0
            }
            .discount_margin(95.50, 0.011),
            0.01718056179887085
        );
    }
}
