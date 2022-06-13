/*!
Module      : financelib <br>
Description : Implement Base modules for the FinanceLib library <br>
Copyright   : (c) 2022 Kishaloy Neogi <br>
License     : MIT <br>
Maintainer  : Kishaloy Neogi <br>
Email       : <nkishaloy@yahoo.com> <br>

The module describes the base modules of Finance like npv,xnpv,irr,xirr,time value of money etc.

PV is mentioned as PV, Future value as FV and Terminal value as TV

You may see the github repository at <https://github.com/n-kishaloy/financelib>
*/

pub mod derivatives;
pub mod fixedincomes;
pub mod statements;

use chrono::{naive::NaiveDate, Duration};

/** Day difference (d1 - d0) in fraction of a year
- d0 = start date
- d1 = end date

This function is similar to its counterpart in MS Excel except it divides the day
by 365.25 instead of more complicated rules followed in Excel.
*/
pub fn yearfrac(d0: NaiveDate, d1: NaiveDate) -> f64 {
    (d1.signed_duration_since(d0).num_days() as f64) / 365.25
}

/** Reverse of yearfrac with anchor Date and yearfrac to calculate final Date.
- d0 = Date from which to calculate.
- yf = Year Fraction as f64.
*/
pub fn inv_yearfrac(d0: NaiveDate, yf: f64) -> NaiveDate {
    d0 + Duration::days((yf * 365.25).round() as i64)
}

/** Discount factor for 1 period = 1/(1+r)
- r = rate for 1 period
*/
pub fn dis_fact_annual(r: f64) -> f64 {
    1.0 / (1.0 + r)
}

/** Discount factor = 1/(1+r)^n
- r = rate for 1 period
- n = period given as f64
*/
pub fn dis_fact(r: f64, n: f64) -> f64 {
    1.0 / (1.0 + r).powf(n)
}

/** Discount Factor between period = 1/(1+r)^year_frac(d0, d1)
- r  = rate for 1 year during period (d0,d1)
- d0 = begin date
- d1 = end date
*/
pub fn xdis_fact(r: f64, d0: NaiveDate, d1: NaiveDate) -> f64 {
    1.0 / (1.0 + r).powf(yearfrac(d0, d1))
}

/** fwd_dis_fact((r0,t0), (r1,t1)) <br>
    = Forward rate between t0 and t1 given as f64 <br>
    = dis_fact(r1,t1) / dis_fact(r0,t0)
- (r0, t0) = Tuple of Rate and Time in (0,t0)
- (r1, t1) = Tuple of Rate and Time in (0,t1)
*/
pub fn fwd_dis_fact((r0, t0): (f64, f64), (r1, t1): (f64, f64)) -> f64 {
    dis_fact(r1, t1) / dis_fact(r0, t0)
}

/** PV of a Future cash flow
- fv = Future cash flow
- r  = rate of return
- n  = number of periods
*/
pub fn pv(fv: f64, r: f64, n: f64) -> f64 {
    fv / (1.0 + r).powf(n)
}

/** PV of a Future cash flow with multiple compounding per period
- fv = Future cash flow
- r  = rate of return
- n  = number of periods
- m  = number of compounding per period
*/
pub fn pvm(fv: f64, r: f64, n: f64, m: f64) -> f64 {
    pv(fv, r / m, n * m)
}

/** PV of continuous growth in year or year ratio
- fv = FV
- r  = rate of return in year on year term
- n  = number of periods
*/
pub fn pvr(fv: f64, r: f64, n: f64) -> f64 {
    fv / r.powf(n)
}

/** PV of continuous expontial growth
- fv = FV
- r  = rate of return in exponential term
- n  = number of periods
*/
pub fn pvc(fv: f64, r: f64, n: f64) -> f64 {
    fv / (r * n).exp()
}

/** FV of a Present cash flow
- pv = Present cash flow
- r  = rate of return
- n  = number of periods
*/
pub fn fv(pv: f64, r: f64, n: f64) -> f64 {
    pv * (1.0 + r).powf(n)
}

/** FV of a Future cash flow with multiple compounding per period`
- pv = Present cash flow
- r  = rate of return
- n  = number of periods
- m  = number of compounding per period
*/
pub fn fvm(pv: f64, r: f64, n: f64, m: f64) -> f64 {
    fv(pv, r / m, n * m)
}

/** FV of continuous growth in year or year ratio
- pv = PV
- r  = rate of return in year on year term
- n  = number of periods */
pub fn fvr(pv: f64, r: f64, n: f64) -> f64 {
    pv * r.powf(n)
}

/** FV of continuous expontial growth
- pv = PV
- r  = rate of return in exponential term
- n  = number of periods
*/
pub fn fvc(pv: f64, r: f64, n: f64) -> f64 {
    pv * (r * n).exp()
}

/** PV of an annuity with multiple payments per period
- pmt = payment made in each transaction
- r   = rate of return
- n   = number of periods (say, years)
- m   = number of payments per period (say, monthly where `m = 12`)
*/
pub fn pv_annuity(pmt: f64, r: f64, n: f64, m: f64) -> f64 {
    pmt / (r / m) * (1.0 - 1.0 / (1.0 + r / m).powf(n * m))
}

/** Payment to cover the PV of an Annuity`
- pv = PV of Annuity
- r  = rate of return
- n  = number of periods (say, years)
- m  = number of payments per period (say, monthly where m = 12)
*/
pub fn pmt(pv: f64, r: f64, n: f64, m: f64) -> f64 {
    pv * (r / m) / (1.0 - 1.0 / (1.0 + r / m).powf(n * m))
}

/** Effective rate of return for multiple compounding per period
- r = nominal rate of return in a period
- m = number of compounding per period
*/
pub fn nom_eff_rate(r: f64, m: f64) -> f64 {
    (1.0 + r / m).powf(m) - 1.0
}

/** Nominal rate of return for multiple compounding per period
- r = effective rate of return in a period
- m = number of compounding per period
*/
pub fn eff_nom_rate(r: f64, m: f64) -> f64 {
    ((1.0 + r).powf(1.0 / m) - 1.0) * m
}

/** Exp rate to effective rate
- r = exponential rate
*/
pub fn exp_eff_rate(r: f64) -> f64 {
    r.exp() - 1.0
}

/** Effective rate to exp rate
- r = effective rate
*/
pub fn eff_exp_rate(r: f64) -> f64 {
    (1.0 + r).ln()
}

/** Easy-to-use reasonable way of emulating approx
- x = first variable
- y = second variable
*/
pub fn approx(x: f64, y: f64) -> bool {
    let mx = f64::max(x.abs(), y.abs());
    mx < 1e-8 || (x - y).abs() / mx < 1e-6
}

/** NPV of cash flows against time given in periods`
- r   = rate of return across the periods
- tim = vector of time of cash flows given as Float64
- cf  = vector of corresponding cash flows
- t0  = time period at which the NPV is sought. Essentially, NPV(ti - t0)

```
let r   = 0.08;
let tim = vec![0.25, 6.25, 3.5, 4.5, 1.25];
let cf  = vec![-6.25, 1.2, 1.25, 3.6, 2.5];
let t0  = -0.45;
assert_eq!(financelib::npv(r, &tim, &cf, t0), 0.36962283798505946);
```
*/
pub fn npv(mut r: f64, tim: &Vec<f64>, cf: &Vec<f64>, t0: f64) -> f64 {
    r += 1.0;
    (tim.iter().zip(cf).map(|(&t, c)| c / r.powf(t)).sum::<f64>()) * r.powf(t0)
}

#[allow(dead_code)]
fn newt_raph(f: impl Fn(f64) -> f64, mut x: f64, xtol: f64) -> Option<f64> {
    let mut dx: f64;
    let del_x = xtol / 10.0;
    for _ in 0..100 {
        dx = f(x);
        dx = del_x * dx / (f(x + del_x) - dx);
        x -= dx;
        if dx.abs() < xtol {
            return Some(x);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_value() {
        let d0 = NaiveDate::from_ymd(2027, 2, 12);
        let d1 = NaiveDate::from_ymd(2018, 2, 12);
        let yf = -8.999315537303216;
        assert_eq!(yearfrac(d0, d1), yf);
        assert_eq!(inv_yearfrac(d1, -yf), d0);
        assert_eq!(dis_fact_annual(0.07), 0.9345794392523364);
        assert_eq!(dis_fact(0.09, 3.0), 0.7721834800610642);
        assert_eq!(
            xdis_fact(
                0.09,
                NaiveDate::from_ymd(2015, 3, 15),
                NaiveDate::from_ymd(2018, 10, 8)
            ),
            0.7353328680759499
        );
        assert_eq!(fwd_dis_fact((0.07, 1.0), (0.09, 3.0)), 0.8262363236653387);
    }

    #[test]
    fn present_future_value() {
        assert_eq!(pv(10_000_000.0, 0.09, 5.0), 6_499_313.862983453);
        assert_eq!(pvm(12_704_891.610953823, 0.06, 4.0, 12.0), 10_000_000.0);
        assert_eq!(pvr(10_000_000., 1.09, 5.0), 6_499_313.862983453);
        assert_eq!(pvc(11_735.108709918102, 0.08, 2.0), 10_000.0);
        assert_eq!(fv(6_499_313.862983453, 0.09, 5.0), 10_000_000.0);
        assert_eq!(fvm(10_000_000.0, 0.06, 4.0, 12.0), 12_704_891.610953823);
        assert_eq!(fvr(10_000_000., 1.09, -5.0), 6_499_313.862983452);
        assert_eq!(fvc(10_000., 0.08, 2.0), 11_735.108709918102);
        assert_eq!(pv_annuity(7.33764573879378, 0.08, 30.0, 12.0), 1000.0);
        assert_eq!(pmt(1000.0, 0.08, 30.0, 12.0), 7.33764573879378);
    }

    #[test]
    fn rates_calc() {
        assert!(approx(nom_eff_rate(0.08, 2.0), 0.0816));
        assert!(approx(nom_eff_rate(eff_nom_rate(0.08, 4.0), 4.0), 0.08));
    }

    #[test]
    fn npv_irr_calc() {
        assert_eq!(newt_raph(|x| (x - 3.0) * (x - 4.0), 2.0, 1e-6), Some(3.0));
        assert_eq!(
            newt_raph(|x| (x - 4.0).powf(2.0), 2.0, 1e-6),
            Some(3.9999990972409805)
        );
        assert_eq!(newt_raph(|x| (x - 4.0).powf(2.0) + 5.0, 2.0, 1e-6), None);
        assert!(approx(1.0e+7, 10_000_000.05));
        assert_eq!(
            npv(
                0.08,
                &vec![0.25, 6.25, 3.5, 4.5, 1.25],
                &vec![-6.25, 1.2, 1.25, 3.6, 2.5],
                -0.45
            ),
            0.36962283798505946
        );
    }
}
