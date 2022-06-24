/*!
Module      : financeLib::fixedincomes::rates <br>
Description : Implement Fixed Incomes Rates modules for the FinanceLib library

Copyright   : (c) 2022 Kishaloy Neogi <br>
License     : MIT <br>
Maintainer  : Kishaloy Neogi <br>
Email       : nkishaloy@yahoo.com <br>

The module describes the base modules of Fixed Incomes Rates.

You may see the github repository at <https://github.com/n-kishaloy/financelib>
*/

#[derive(Debug, Clone)]
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
            Self::NominalRateCurve { rate: _, freq: f } => crate::pvm(c, rate, tim, *f),
            Self::EffectiveRateCurve { rate: _, freq: _ } => crate::pv(c, rate, tim),
            Self::ExponentialRateCurve { rate: _, freq: _ } => crate::pvc(c, rate, tim),
        }
    }

    pub fn convert_nominal(&self) -> RateCurve {
        match self {
            Self::EffectiveRateCurve { rate, freq } => RateCurve::NominalRateCurve {
                rate: rate
                    .iter()
                    .map(|x| crate::eff_nom_rate(*x, *freq))
                    .collect(),
                freq: *freq,
            },
            Self::ExponentialRateCurve { rate, freq } => RateCurve::NominalRateCurve {
                rate: rate
                    .iter()
                    .map(|x| crate::exp_nom_rate(*x, *freq))
                    .collect(),
                freq: *freq,
            },
            Self::NominalRateCurve { rate: _, freq: _ } => (*self).clone(),
        }
    }

    pub fn convert_effective(&self) -> RateCurve {
        match self {
            Self::NominalRateCurve { rate, freq } => RateCurve::EffectiveRateCurve {
                rate: rate
                    .iter()
                    .map(|x| crate::nom_eff_rate(*x, *freq))
                    .collect(),
                freq: *freq,
            },
            Self::ExponentialRateCurve { rate, freq } => RateCurve::EffectiveRateCurve {
                rate: rate.iter().map(|x| crate::exp_eff_rate(*x)).collect(),
                freq: *freq,
            },
            Self::EffectiveRateCurve { rate: _, freq: _ } => (*self).clone(),
        }
    }

    pub fn convert_exponential(&self) -> RateCurve {
        match self {
            Self::NominalRateCurve { rate, freq } => RateCurve::ExponentialRateCurve {
                rate: rate
                    .iter()
                    .map(|x| crate::nom_exp_rate(*x, *freq))
                    .collect(),
                freq: *freq,
            },
            Self::EffectiveRateCurve { rate, freq } => RateCurve::ExponentialRateCurve {
                rate: rate.iter().map(|x| crate::eff_exp_rate(*x)).collect(),
                freq: *freq,
            },
            Self::ExponentialRateCurve { rate: _, freq: _ } => (*self).clone(),
        }
    }
}

#[cfg(test)]
mod rate_fn {
    use super::*;
    use crate::approx;
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
        let et = NominalRateCurve {
            rate: vec![0.0016, 0.0021, 0.0027, 0.0033, 0.0037, 0.0041],
            freq: 2.0,
        };
        let er = et.convert_effective();
        let en = er.convert_nominal();

        if let RateCurve::EffectiveRateCurve { rate, .. } = er {
            assert!(approx(rate[0], 0.0016006400));
            assert!(approx(rate[1], 0.0021011025));
            assert!(approx(rate[2], 0.0027018225));
            assert!(approx(rate[3], 0.0033027225));
            assert!(approx(rate[4], 0.0037034225));
            assert!(approx(rate[5], 0.0041042025));
        }

        if let RateCurve::NominalRateCurve { rate, .. } = en {
            assert!(approx(rate[0], 0.0016));
            assert!(approx(rate[1], 0.0021));
            assert!(approx(rate[2], 0.0027));
            assert!(approx(rate[3], 0.0033));
            assert!(approx(rate[4], 0.0037));
            assert!(approx(rate[5], 0.0041));
        }

        let ez = et
            .convert_exponential()
            .convert_effective()
            .convert_nominal();

        let ey = et
            .convert_effective()
            .convert_exponential()
            .convert_nominal();

        let ew = et.convert_exponential().convert_nominal();

        let rz = if let RateCurve::NominalRateCurve { rate, .. } = ez {
            rate
        } else {
            panic!("Hiya")
        };

        let ry = if let RateCurve::NominalRateCurve { rate, .. } = ey {
            rate
        } else {
            panic!("Hiya")
        };

        let rw = if let RateCurve::NominalRateCurve { rate, .. } = ew {
            rate
        } else {
            panic!("Hiya")
        };

        assert!(rz[2] == rw[2]);
        assert!(ry[3] == rw[3]);
        assert!(rz[1] == ry[1]);
        assert!(rz[4] == ry[4]);
        assert!(rz[0] == rw[0]);
    }
}
