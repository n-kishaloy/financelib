/*!
Implement Derivatives modules for the financelib library

Module      : financelib::derivatives::forwards <br>
Copyright   : (c) 2022 Kishaloy Neogi <br>
License     : MIT <br>
Maintainer  : Kishaloy Neogi <br>
Email       : <nkishaloy@yahoo.com>

The module describes the base modules of forward derivatives.
You may see the github repository at <https://github.com/n-kishaloy/financelib>
*/

use chrono::naive::NaiveDate as NDt;

/** Struct for reprenting forward contract using periods
- rf          = risk-free rate of return per period
- t_expiry    = Forward expiry date in period
- fwd_expiry  = Forward rate @ t_exp = S0*(1+r)^t_exp
- benefit     = Dividends and other benefits - Cost of holding the asset
 */
#[derive(Debug, Copy, Clone)]
pub struct Forward {
    pub rf: f64,
    pub t_expiry: f64,
    pub fwd_expiry: f64,
    pub benefit: f64,
}

/** Struct for reprenting forward contract using dates
- rf          = risk-free rate of return per period
- dt_begin    = Forward begin date in NaiveDate
- dt_expiry   = Forward expiry date in NaiveDate
- fwd_expiry  = Forward rate @ t_exp = S0*(1+r)^t_exp
- benefit     = Dividends and other benefits - Cost of holding the asset
 */
#[derive(Debug, Copy, Clone)]
pub struct XForward {
    pub rf: f64,
    pub dt_begin: NDt,
    pub dt_expiry: NDt,
    pub fwd_expiry: f64,
    pub benefit: f64,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
