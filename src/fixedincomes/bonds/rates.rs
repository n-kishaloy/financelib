/*!
Implement Fixed Incomes Rates modules for the FinanceLib library

Module      : financeLib::fixedincomes::rates <br>
Copyright   : (c) 2022 Kishaloy Neogi <br>
License     : MIT <br>
Maintainer  : Kishaloy Neogi <br>
Email       : <nkishaloy@yahoo.com>

The module describes the base modules of Fixed Incomes Rates.

You may see the github repository at <https://github.com/n-kishaloy/financelib>
*/

/**
RateCurve defines Enum for different type of Rates (Nominal, Effective, Exponential)
given as curve.

RateCurve tracked rates which varies over a period and is given as a periodic in terms
of Rates given in regular intervals. It can be of 3 types:
- NominalRateCurve
- EffectiveRateCurve
- ExponentialRateCurve

Each type has 2 fields
- rate  = Vector of rates
- freq  = freq at which the rates are being given per period.
 */
#[derive(Debug, Clone)]
pub enum RateCurve {
    NominalRateCurve { rate: Vec<f64>, freq: f64 },
    EffectiveRateCurve { rate: Vec<f64>, freq: f64 },
    ExponentialRateCurve { rate: Vec<f64>, freq: f64 },
}

impl RateCurve {
    /**
    Estimate the rate at a particular time by interpolating between the rate curves points

    - y = the time given as period whose rate is being sought.
     */
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

    /**
    The Present Value of a cash flow at a particular time.

    - c     = cash flow
    - tim   = time in period at which the cash flow occurs.
     */
    pub fn pv(&self, c: f64, tim: f64) -> f64 {
        let rate = self.rate_estim(tim);
        match self {
            Self::NominalRateCurve { rate: _, freq: f } => crate::pvm(c, rate, tim, *f),
            Self::EffectiveRateCurve { rate: _, freq: _ } => crate::pv(c, rate, tim),
            Self::ExponentialRateCurve { rate: _, freq: _ } => crate::pvc(c, rate, tim),
        }
    }

    /**
    Convert the RateCurve to a curve with Nominal rates
     */
    pub fn to_nominal(&self) -> RateCurve {
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

    /**
    Convert the RateCurve to a curve with Effective rates
     */
    pub fn to_effective(&self) -> RateCurve {
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

    /**
    Convert the RateCurve to a curve with Exponential rates
     */
    pub fn to_exponential(&self) -> RateCurve {
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

/**
The Rates enum defines different types of Rates represemted in RateCurves

- SpotRates
- ParRates
- ForwardRates
 */
#[derive(Debug, Clone)]
pub enum Rates {
    SpotRates { rate: RateCurve },
    ParRates { rate: RateCurve },
    ForwardRates { rate: RateCurve },
}

impl Rates {
    /** Change ParRates to SpotRates */
    pub fn to_spot(&self) -> Rates {
        match &self {
            &Self::ParRates { rate } => {
                let (rt, fq) = match &rate {
                    &RateCurve::NominalRateCurve { rate, freq } => (rate, freq),
                    _ => unimplemented!(),
                };
                let n = rt.len();
                let mut y = vec![0.0; n];

                y[0] = rt[0];
                for i in 1..n {
                    let xm = rt[i] / fq;
                    let sm = (0..i)
                        .map(|k| xm / (1.0 + y[k] / fq).powf((k + 1) as f64))
                        .sum::<f64>();
                    y[i] = (((1.0 + xm) / (1.0 - sm)).powf(1.0 / ((i + 1) as f64)) - 1.0) * fq
                }
                Rates::SpotRates {
                    rate: RateCurve::NominalRateCurve { rate: y, freq: *fq },
                }
            }
            _ => unimplemented!(),
        }
    }

    /** Change SpotRates to ParRates */
    pub fn to_par(&self) -> Rates {
        match &self {
            &Self::SpotRates { rate } => {
                let (rt, fq) = match &rate {
                    &RateCurve::NominalRateCurve { rate, freq } => (rate, freq),
                    _ => unimplemented!(),
                };

                Rates::ParRates {
                    rate: RateCurve::NominalRateCurve {
                        rate: {
                            (0..rt.len())
                                .map(|i| {
                                    fq * (1.0 - 1.0 / (1.0 + rt[i] / fq).powf((i + 1) as f64))
                                        / (0..=i)
                                            .map(|k| 1.0 / (1.0 + rt[k] / fq).powf((k + 1) as f64))
                                            .sum::<f64>()
                                })
                                .collect()
                        },
                        freq: *fq,
                    },
                }
            }
            _ => unimplemented!(),
        }
    }

    /** Estimate Rate at a  */
    pub fn rate_estim(&self, y: f64) -> f64 {
        match &self {
            &Self::SpotRates { rate } => rate.rate_estim(y),
            &Self::ParRates { rate } => rate.rate_estim(y),
            &Self::ForwardRates { rate } => rate.rate_estim(y),
        }
    }

    /**
    Estimate the forward rate for a given forward period of a given tenor

    - forward_period    = forward period start point
    - tenor             = tenor of the forward period
     */
    pub fn forward_rate(&self, forward_period: f64, tenor: f64) -> f64 {
        match &self {
            &Self::SpotRates { rate } => {
                let f = match rate {
                    RateCurve::NominalRateCurve { freq, .. } => freq,
                    _ => unimplemented!(),
                };
                (1.0 + rate.rate_estim(forward_period + tenor) / f).powf(tenor * f)
                    / (1.0 + rate.rate_estim(forward_period) / f).powf(forward_period * f)
            }
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod rate_fn {
    use core::panic;

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
        let er = et.to_effective();
        let en = er.to_nominal();

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

        let ez = et.to_exponential().to_effective().to_nominal();

        let ey = et.to_effective().to_exponential().to_nominal();

        let ew = et.to_exponential().to_nominal();

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

        let er = Rates::ParRates {
            rate: RateCurve::NominalRateCurve {
                rate: vec![0.020000, 0.024000, 0.027600, 0.030840, 0.033756, 0.036380],
                freq: 2.0,
            },
        }
        .to_spot();

        let rx = match er {
            Rates::SpotRates { ref rate } => match rate {
                RateCurve::NominalRateCurve { rate, .. } => rate,
                _ => panic!("hi"),
            },
            _ => panic!("hiya"),
        };

        assert!(approx(rx[0], 0.02));
        assert_eq!(rx[3], 0.030973763781325214);
        assert_eq!(rx[4], 0.03397441792873934);
        assert_eq!(rx[5], 0.036700426487687565);

        let xt = er.to_par();
        let et = match xt {
            Rates::ParRates { ref rate } => match rate {
                RateCurve::NominalRateCurve { rate, .. } => rate,
                _ => panic!("hi"),
            },
            _ => panic!("hiya"),
        };

        assert!(approx(et[2], 0.027600));
        assert!(approx(et[3], 0.030840));
        assert!(approx(et[5], 0.036380));
    }
}
