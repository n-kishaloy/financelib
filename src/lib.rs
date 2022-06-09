pub mod derivatives;
pub mod fixedincomes;
pub mod statements;

use chrono::{naive::NaiveDate, Duration};

/// Day difference (d1 - d0) in fraction of a year
/// * d0 = start date
/// * d1 = end date
/// This function is similar to its counterpart in MS Excel except it divides the day
/// by 365.25 instead of more complicated rules followed in Excel.
pub fn yearfrac(d0: NaiveDate, d1: NaiveDate) -> f64 {
    (d1.signed_duration_since(d0).num_days() as f64) / 365.25
}

/// Reverse of yearfrac with anchor Date and yearfrac to calculate final Date.
/// * d0 = Date from which to calculate.
/// * yf = Year Fraction as f64.
pub fn inv_yearfrac(d0: NaiveDate, yf: f64) -> NaiveDate {
    d0 + Duration::days((yf * 365.25).round() as i64)
}

/// Discount factor for 1 period = 1/(1+r)
/// * r = rate for 1 period
pub fn dis_fact_annual(r: f64) -> f64 {
    1.0 / (1.0 + r)
}

/// Discount factor = 1/(1+r)^n
/// * r = rate for 1 period
/// * n = period given as f64
pub fn dis_fact(r: f64, n: f64) -> f64 {
    1.0 / (1.0 + r).powf(n)
}

/// Discount Factor between period = 1/(1+r)^year_frac(d0, d1)`
/// * r  = rate for 1 year during period (d0,d1)
/// * d0 = begin date
/// * d1 = end date
pub fn xdis_fact(r: f64, d0: NaiveDate, d1: NaiveDate) -> f64 {
    1.0 / (1.0 + r).powf(yearfrac(d0, d1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tv() {
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
        )
    }
}
