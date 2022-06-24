/*!
Module      : financeLib::fixedincomes::rates <br>
Description : Implement Fixed Incomes Rates modules for the FinanceLib library <br>
Copyright   : (c) 2022 Kishaloy Neogi <br>
License     : MIT <br>
Maintainer  : Kishaloy Neogi <br>
Email       : nkishaloy@yahoo.com <br>

The module describes the base modules of Fixed Incomes Rates.

You may see the github repository at <https://github.com/n-kishaloy/financelib>
*/

#[derive(Clone)]
pub enum RateCurve {
    NominalRateCurve { rate: Vec<f64>, freq: f64 },
    EffectiveRateCurve { rate: Vec<f64>, freq: f64 },
    ExponentialRateCurve { rate: Vec<f64>, freq: f64 },
}

impl RateCurve {
    pub fn rate_estim(&self, y: f64) -> f64 {
        fn estima(rx: &Vec<f64>, fq: f64, y: f64) -> f64 {
            let pt = y * fq;
            let fl = pt.floor();
            let f0 = fl as usize - 1;
            let pf = pt - fl;
            if pf < 1e-9 {
                rx[f0]
            } else {
                rx[f0] * (1.0 - pf) + rx[f0 + 1] * pf
            }
        }
        match self {
            Self::NominalRateCurve { rate, freq } => estima(rate, *freq, y),
            Self::EffectiveRateCurve { rate, freq } => estima(rate, *freq, y),
            Self::ExponentialRateCurve { rate, freq } => estima(rate, *freq, y),
        }
    }

    pub fn pv(&self, c: f64, tim: f64) -> f64 {
        let rate = self.rate_estim(tim);
        match self {
            Self::NominalRateCurve { rate: _, freq } => crate::pvm(c, rate, tim, *freq),
            Self::EffectiveRateCurve { rate: _, freq: _ } => crate::pv(c, rate, tim),
            Self::ExponentialRateCurve { rate: _, freq: _ } => crate::pvc(c, rate, tim),
        }
    }
}

#[cfg(test)]
mod rate_fn {
    use super::*;
    use RateCurve::*;

    #[test]
    fn rate_curves() {
        assert_eq!(
            NominalRateCurve {
                rate: vec![0.05, 0.06, 0.07, 0.08],
                freq: 2.0
            }
            .rate_estim(1.5),
            0.07
        );
        assert_eq!(
            NominalRateCurve {
                rate: vec![0.05, 0.06, 0.07, 0.08],
                freq: 2.0
            }
            .rate_estim(1.2),
            0.064
        );
    }
}
