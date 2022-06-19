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
pub struct CouponBond {
    pub c: f64,
    pub freq: f64,
    pub t_life: f64,
}

#[cfg(test)]
mod bonds_fn {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
