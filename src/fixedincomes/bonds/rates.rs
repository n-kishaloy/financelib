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
    pub fn estimate_r(&self, y: f64) -> f64 {
        fn estima(rx: &Vec<f64>, fq: f64, y: f64) -> f64 {
            let pt = y * fq;
            let fl = pt.floor();
            let f0 = fl as usize;
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
