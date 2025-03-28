/*!
Implement Base modules for the FinanceLib library

Module      : financelib <br>
Copyright   : (c) 2022 Kishaloy Neogi <br>
License     : MIT <br>
Maintainer  : Kishaloy Neogi <br>
Email       : <nkishaloy@yahoo.com>

The module describes the base modules of Finance like npv,xnpv,irr,xirr,time value of money etc.

PV is mentioned as PV, Future value as FV and Terminal value as TV

You may see the github repository at <https://github.com/n-kishaloy/financelib>
*/

pub mod derivatives;
pub mod fixedincomes;
pub mod statements;
pub mod valuations;

use chrono::{Datelike, naive::NaiveDate as NDt};
use serde::{Deserialize, Serialize};
// use time::util::is_leap_year;
use DayCountConvention::*;

pub type Period = (NDt, NDt);

/**
 * Check if Leap year
 */
pub fn is_leap_year(y: i32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}

/**
 * Nos of days in a year
 */
pub fn days_in_year(yr: i32) -> i64 {
    if is_leap_year(yr) { 366 } else { 365 }
}

/**
Converts a date to a f64 float with 1(One) year represented by 1.0
*/
pub fn date_to_float(dt: NDt) -> f64 {
    let (yr, ds) = (dt.year(), dt.ordinal() as f64);
    yr as f64 + ds / days_in_year(yr) as f64
}

/**
Converts a float to a NaiveDate with 1(One) year represented by 1.0
*/
pub fn date_from_float(yr: f64) -> NDt {
    let y = (yr - 0.00274).floor();
    let yp = y as i32;
    NDt::from_yo_opt(yp, ((yr - y) * days_in_year(yp) as f64).round() as u32).unwrap()
}

/**
Enum defining different Days convention

- US30360 => US 30/360 or NASD 30/360
- EU30360 => EURO 30/360
- ACTACT => (Actual days in Leap year) / 366 + (Actual days in Normal year) / 365
- ACT360 => Actual nos of days / 360
- ACT365 => Actual nos of days / 365
 */
pub enum DayCountConvention {
    US30360,
    EU30360,
    ACTACT,
    ACT360,
    ACT365,
}

/** Day difference (dt1 - dt0) in fraction of a year
- dt0 = start date
- dt1 = end date

Following methods are supported
- US30360 => US 30/360 or NASD 30/360
- EU30360 => EURO 30/360
- ACTACT => (Days in Leap year) / 366 + (Days in Normal year) / 365
- ACT360 => Actual nos of days / 360
- ACT365 => Actual nos of days / 365

Note that the ACTACT formula is different from MS Excel and follows the Actual/Actual ISDA rule. For more details refer <https://en.wikipedia.org/wiki/Day_count_convention>.

The yearfrac function is also signed with the result coming as negative in case dt0 > dt1. This is different from MS Excel, where the yearfrac number return absolute difference between the dates. Use abs() at end to replicate the same.
*/
pub fn yearfrac(dt0: NDt, dt1: NDt, basis: DayCountConvention) -> f64 {
    let day_count_factor =
        |y0, m0, d0, y1, m1, d1| ((y1 - y0) * 360 + (m1 - m0) * 30 + (d1 - d0)) as f64 / 360.0;

    match basis {
        ACT360 => ((dt1 - dt0).num_days() as f64) / 360.0,
        ACT365 => ((dt1 - dt0).num_days() as f64) / 365.0,
        ACTACT => date_to_float(dt1) - date_to_float(dt0),
        EU30360 => {
            let (y0, m0, d0) = (dt0.year(), dt0.month() as i32, dt0.day() as i32);
            let (y1, m1, d1) = (dt1.year(), dt1.month() as i32, dt1.day() as i32);
            let lastday = |d| if d == 31 { 30 } else { d };
            day_count_factor(y0, m0, lastday(d0), y1, m1, lastday(d1))
        }
        US30360 => {
            let (y0, m0, mut d0) = (dt0.year(), dt0.month() as i32, dt0.day() as i32);
            let (y1, m1, mut d1) = (dt1.year(), dt1.month() as i32, dt1.day() as i32);
            let lsfeb = |d, m, y| m == 2 && if is_leap_year(y) { d == 29 } else { d == 28 };
            if lsfeb(d0, m0, y0) {
                if lsfeb(d1, m1, y1) {
                    d1 = 30;
                };
                d0 = 30;
            };
            if d1 == 31 && d0 >= 30 {
                d1 = 30;
            };
            if d0 == 31 {
                d0 = 30;
            };
            day_count_factor(y0, m0, d0, y1, m1, d1)
        }
    }
}

/** Calculated yearfrac between a Period
- (dt0, dt2) - Captures the period for the yearfrac

Uses the default US30360 option
 */
pub fn yrfrac((dt0, dt1): Period) -> f64 {
    yearfrac(dt0, dt1, US30360)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Currency {
    INR,
    USD,
    NGN,
    EUR,
    GBP,
    CNY,
    MZN,
    ZAR,
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
pub fn xdis_fact(r: f64, pr: Period) -> f64 {
    1.0 / (1.0 + r).powf(yrfrac(pr))
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
- r  = Effective rate of return
- n  = number of periods
*/
pub fn pv(r: f64, n: f64, fv: f64) -> f64 {
    fv / (1.0 + r).powf(n)
}

/** PV of a Future cash flow
- fv = Future cash flow
- r  = Effective rate of return
- pr = Period of discounting
*/
pub fn xpv(r: f64, pr: Period, fv: f64) -> f64 {
    fv / (1.0 + r).powf(yrfrac(pr))
}

/** PV of a Future cash flow with multiple compounding per period
- fv = Future cash flow
- r  = Nominal rate of return
- n  = number of periods
- m  = number of compounding per period
*/
pub fn pvm(r: f64, n: f64, m: f64, fv: f64) -> f64 {
    pv(r / m, n * m, fv)
}

/** PV of continuous expontial growth
- fv = FV
- r  = Exponential rate of return
- n  = number of periods
*/
pub fn pvc(r: f64, n: f64, fv: f64) -> f64 {
    fv / (r * n).exp()
}

/** FV of a Present cash flow
- pv = Present cash flow
- r  = Effective rate of return
- n  = number of periods
*/
pub fn fv(r: f64, n: f64, pv: f64) -> f64 {
    pv * (1.0 + r).powf(n)
}

/** FV of a Future cash flow
- pv = Present cash flow
- r  = Effective rate of return
- pr = Period of discounting
*/
pub fn xfv(r: f64, pr: Period, pv: f64) -> f64 {
    pv * (1.0 + r).powf(yrfrac(pr))
}

/** FV of a Future cash flow with multiple compounding per period
- pv = Present cash flow
- r  = Nominal rate of return
- n  = number of periods
- m  = number of compounding per period
*/
pub fn fvm(r: f64, n: f64, m: f64, pv: f64) -> f64 {
    fv(r / m, n * m, pv)
}

/** FV of continuous expontial growth
- pv = PV
- r  = Exponential rate of return in exponential term
- n  = number of periods
*/
pub fn fvc(r: f64, n: f64, pv: f64) -> f64 {
    pv * (r * n).exp()
}

/** PV of an annuity with multiple payments per period
- pmt = payment made in each transaction
- r   = rate of return
- n   = number of periods (say, years)
- m   = number of payments per period (say, monthly where `m = 12`)
*/
pub fn pv_annuity(r: f64, n: f64, m: f64, pmt: f64, fv: f64) -> f64 {
    let rn = (1.0 + r / m).powf(n * m);
    -pmt / (r / m) * (1.0 - 1.0 / rn) - fv / rn
}

/** Payment to cover the PV of an Annuity
- pv = PV of Annuity
- r  = rate of return
- n  = number of periods (say, years)
- m  = number of payments per period (say, monthly where m = 12)
*/
pub fn pmt(r: f64, n: f64, m: f64, pv: f64, fv: f64) -> f64 {
    let rn = (1.0 + r / m).powf(n * m);
    -(pv + fv / rn) * (r / m) / (1.0 - 1.0 / rn)
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

/** Nominal rate to exponential rate
- r = effective rate
- m = number of compounding per period
*/
pub fn nom_exp_rate(r: f64, m: f64) -> f64 {
    eff_exp_rate(nom_eff_rate(r, m))
}

/** Exponential rate to nominal rate
- r = effective rate
- m = number of compounding per period
*/
pub fn exp_nom_rate(r: f64, m: f64) -> f64 {
    eff_nom_rate(exp_eff_rate(r), m)
}

/** Easy-to-use reasonable way of emulating approx
- x = first variable
- y = second variable
*/
pub fn approx(x: f64, y: f64) -> bool {
    let mx = f64::max(x.abs(), y.abs());
    mx < 1e-8 || (x - y).abs() / mx < 1e-6
}

/** Check if a float is zero
- x = float to be checked
*/
pub fn is_zero(x: f64) -> bool {
    x.abs() < 1e-8
}

/** NPV of cash flows against time given in periods @ time = 0.0
- r   = rate of return across the periods
- tim = vector of time of cash flows given as Float64
- cf  = vector of corresponding cash flows
*/
pub fn npv_t0(mut r: f64, tim: &Vec<f64>, cf: &Vec<f64>) -> f64 {
    r += 1.0;
    tim.iter().zip(cf).map(|(&t, c)| c / r.powf(t)).sum::<f64>()
}

/** NPV of cash flows against time given in periods @ time = t0
- r   = rate of return across the periods
- tim = vector of time of cash flows given as Float64
- cf  = vector of corresponding cash flows
- t0  = time period at which the NPV is sought. Essentially, NPV(ti - t0)
*/
pub fn npv(r: f64, tim: &Vec<f64>, t0: f64, cf: &Vec<f64>) -> f64 {
    npv_t0(r, tim, cf) * (1.0 + r).powf(t0)
}

/** NPV of cash flows against time given by Date
- r   = rate of return across the years
- dt  = vector of time of cash flows given as Date
- cf  = vector of corresponding cash flows
- d0  = Date at which the NPV is sought.
*/
pub fn xnpv(r: f64, dt: &Vec<NDt>, d0: NDt, cf: &Vec<f64>) -> f64 {
    npv_t0(r, &dt.iter().map(|&d| yrfrac((d0, d))).collect(), cf)
}

/** IRR of cash flow against time given in periods
- tim = vector of time of cash flows given as Float64
- cf  = vector of corresponding cash flows
*/
pub fn irr(tim: &Vec<f64>, cf: &Vec<f64>) -> Option<f64> {
    newt_raph(|r| npv_t0(r, tim, cf), 0.1, 1e-6)
}

/** XIRR of cash flow against time given as NaiveDate
- dt  = vector of time of cash flows given as NaiveDate
- cf  = vector of corresponding cash flows
*/
pub fn xirr(dt: &Vec<NDt>, cf: &Vec<f64>) -> Option<f64> {
    irr(&dt.iter().map(|&d| yrfrac((dt[0], d))).collect(), cf)
}

pub fn newt_raph(f: impl Fn(f64) -> f64, mut x: f64, xtol: f64) -> Option<f64> {
    let dx = xtol / 10.0;
    for _ in 0..100 {
        let fx = f(x);
        let df = (f(x + dx) - fx) / dx;
        if is_zero(df) {
            return None;
        }
        let del_x = fx / df;
        x -= del_x;
        if is_zero(del_x) {
            return Some(x);
        }
    }
    None
}

#[cfg(test)]
mod base_fn {
    use super::*;

    #[test]
    fn time_value() {
        assert_eq!(dis_fact_annual(0.07), 0.9345794392523364);
        assert_eq!(dis_fact(0.09, 3.0), 0.7721834800610642);
        assert_eq!(
            xdis_fact(
                0.09,
                (
                    NDt::from_ymd_opt(2015, 3, 15).unwrap(),
                    NDt::from_ymd_opt(2018, 10, 8).unwrap()
                )
            ),
            0.7355566392384189
        );
        assert_eq!(fwd_dis_fact((0.07, 1.0), (0.09, 3.0)), 0.8262363236653387);
    }

    #[test]
    fn yearfrac_calc() {
        let dts = vec![
            (
                NDt::from_ymd_opt(2018, 2, 5).unwrap(),
                NDt::from_ymd_opt(2023, 5, 14).unwrap(),
            ),
            (
                NDt::from_ymd_opt(2020, 2, 29).unwrap(),
                NDt::from_ymd_opt(2024, 2, 28).unwrap(),
            ),
            (
                NDt::from_ymd_opt(2015, 8, 30).unwrap(),
                NDt::from_ymd_opt(2010, 3, 31).unwrap(),
            ),
            (
                NDt::from_ymd_opt(2016, 2, 28).unwrap(),
                NDt::from_ymd_opt(2016, 10, 30).unwrap(),
            ),
            (
                NDt::from_ymd_opt(2014, 1, 31).unwrap(),
                NDt::from_ymd_opt(2014, 8, 31).unwrap(),
            ),
            (
                NDt::from_ymd_opt(2014, 2, 28).unwrap(),
                NDt::from_ymd_opt(2014, 9, 30).unwrap(),
            ),
            (
                NDt::from_ymd_opt(2016, 2, 29).unwrap(),
                NDt::from_ymd_opt(2016, 6, 15).unwrap(),
            ),
        ]
        .iter()
        .map(|(dt0, dt1)| {
            (
                yearfrac(*dt0, *dt1, US30360),
                yearfrac(*dt0, *dt1, ACTACT),
                yearfrac(*dt0, *dt1, ACT360),
                yearfrac(*dt0, *dt1, ACT365),
                yearfrac(*dt0, *dt1, EU30360),
            )
        })
        .collect::<Vec<_>>();

        // println!("{:?}", dts[6]);
        assert_eq!(is_leap_year(2011), false);
        assert_eq!(is_leap_year(2016), true);
        assert_eq!(is_leap_year(1900), false);
        assert_eq!(is_leap_year(1600), true);
        assert!(
            dts[0]
                == (
                    5.27500000000000,
                    5.26849315068489,
                    5.34444444444444444,
                    5.271232876712329,
                    5.27500000000000
                )
        );
        assert!(
            dts[1]
                == (
                    3.9944444444444444444444,
                    3.9972677595626465,
                    4.0555555555555555555555,
                    4.00000000000000,
                    3.99722222222222222222222
                )
        );
        assert!(
            dts[2]
                == (
                    -5.4166666666666666666,
                    -5.4164383561642350000,
                    -5.4944444444444444444,
                    -5.4191780821917810000,
                    -5.4166666666666666666
                )
        );
        assert!(
            dts[3]
                == (
                    0.6722222222222222222,
                    0.6693989071038686000,
                    0.6805555555555555555,
                    0.6712328767123288000,
                    0.6722222222222222222
                )
        );
        assert!(
            dts[4]
                == (
                    0.5833333333333333333,
                    0.5808219178084073000,
                    0.5888888888888888888,
                    0.5808219178082191000,
                    0.5833333333333333333
                )
        );
        assert!(
            dts[5]
                == (
                    0.5833333333333333333,
                    0.5863013698631221000,
                    0.5944444444444444444,
                    0.5863013698630137000,
                    0.5888888888888888888
                )
        );
        assert!(
            dts[6]
                == (
                    0.2916666666666666666,
                    0.2923497267759103000,
                    0.2972222222222222222,
                    0.2931506849315068700,
                    0.2944444444444444444
                )
        );
    }

    #[test]
    fn present_future_value() {
        assert_eq!(pv(0.09, 5.0, 10_000_000.0), 6_499_313.862983453);
        assert_eq!(pvm(0.06, 4.0, 12.0, 12_704_891.610953823), 10_000_000.0);
        assert_eq!(pvc(0.08, 2.0, 11_735.108709918102), 10_000.0);
        assert_eq!(fv(0.09, 5.0, 6_499_313.862983453), 10_000_000.0);
        assert_eq!(fvm(0.06, 4.0, 12.0, 10_000_000.0), 12_704_891.610953823);
        assert_eq!(fvc(0.08, 2.0, 10_000.), 11_735.108709918102);
        assert_eq!(
            pv_annuity(0.08, 30.0, 12.0, 7.304096785187425, 50.0),
            -1000.0
        );
        assert_eq!(pmt(0.08, 30.0, 12.0, -1000.0, 50.0), 7.304096785187425);
        assert!(approx(
            xpv(
                0.08,
                (
                    NDt::from_ymd_opt(2020, 2, 29).unwrap(),
                    NDt::from_ymd_opt(2024, 2, 28).unwrap()
                ),
                5.638
            ),
            4.1458705451340800
        ));
        assert!(approx(
            xfv(
                0.08,
                (
                    NDt::from_ymd_opt(2020, 2, 29).unwrap(),
                    NDt::from_ymd_opt(2024, 2, 28).unwrap()
                ),
                5.638
            ),
            fv(0.08, 3.9944444444444400000, 5.638)
        ));
        assert!(approx(
            xfv(
                0.08,
                (
                    NDt::from_ymd_opt(2020, 2, 29).unwrap(),
                    NDt::from_ymd_opt(2024, 2, 28).unwrap()
                ),
                5.638
            ),
            7.6671578752761100
        ));
    }

    #[test]
    fn rates_calc() {
        assert!(approx(nom_eff_rate(0.08, 2.0), 0.0816));
        assert!(approx(nom_eff_rate(eff_nom_rate(0.08, 4.0), 4.0), 0.08));
        assert!(approx(nom_exp_rate(0.08, 2.0), 0.07844142630656266));
        assert!(approx(exp_nom_rate(nom_exp_rate(0.08, 2.0), 2.0), 0.08));
        assert!(approx(eff_nom_rate(nom_eff_rate(0.08, 2.0), 2.0), 0.08));
    }

    #[test]
    fn npv_irr_calc() {
        assert_eq!(newt_raph(|x| (x - 3.0) * (x - 4.0), 2.0, 1e-6), Some(3.0));
        assert_eq!(
            newt_raph(|x| (x - 4.0).powf(2.0), 2.0, 1e-6),
            Some(4.000000028157636)
        );
        assert_eq!(newt_raph(|x| (x - 4.0).powf(2.0) + 5.0, 2.0, 1e-6), None);
        assert!(approx(1.0e+7, 10_000_000.05));
        assert_eq!(
            npv(
                0.08,
                &vec![0.25, 6.25, 3.5, 4.5, 1.25],
                -0.45,
                &vec![-6.25, 1.2, 1.25, 3.6, 2.5],
            ),
            0.36962283798505946
        );
        assert_eq!(
            irr(
                &vec![0.125, 0.29760274, 0.49760274, 0.55239726, 0.812671233],
                &vec![-10.25, -2.5, 3.5, 9.5, 1.25]
            ),
            Some(0.3181338647519102)
        );
        assert_eq!(
            irr(
                &vec![0.125, 0.29760274, 0.49760274, 0.55239726, 0.812671233],
                &vec![10.25, 2.5, 3.5, 9.5, 1.25]
            ),
            None
        );
        assert_eq!(
            xnpv(
                0.08,
                &vec![
                    NDt::from_ymd_opt(2012, 2, 25).unwrap(),
                    NDt::from_ymd_opt(2012, 6, 28).unwrap(),
                    NDt::from_ymd_opt(2013, 2, 15).unwrap(),
                    NDt::from_ymd_opt(2014, 9, 18).unwrap(),
                    NDt::from_ymd_opt(2015, 2, 20).unwrap(),
                ],
                NDt::from_ymd_opt(2012, 1, 10).unwrap(),
                &vec![-15.0, 5.0, 25.0, -10.0, 50.0],
            ),
            44.165773653310936
        );
        assert_eq!(
            xirr(
                &vec![
                    NDt::from_ymd_opt(2012, 2, 25).unwrap(),
                    NDt::from_ymd_opt(2012, 6, 28).unwrap(),
                    NDt::from_ymd_opt(2013, 2, 15).unwrap(),
                    NDt::from_ymd_opt(2014, 9, 18).unwrap(),
                    NDt::from_ymd_opt(2015, 2, 20).unwrap(),
                ],
                &vec![-115.0, 5.0, 25.0, -10.0, 200.0]
            ),
            Some(0.27845538159261773)
        );
    }
}
