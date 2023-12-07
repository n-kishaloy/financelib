/*!
Implement Derivatives modules for the financelib library

Module      : financelib::valuations <br>
Copyright   : (c) 2022 Kishaloy Neogi <br>
License     : MIT <br>
Maintainer  : Kishaloy Neogi <br>
Email       : <nkishaloy@yahoo.com>

The module describes the base modules of Valuations like .
You may see the github repository at <https://github.com/n-kishaloy/financelib>
*/

use crate::statements::*;
use crate::*;
use chrono::naive::NaiveDate as NDt;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

/**
This is for Affiliation to different types of Industry of the economy
*/
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub enum Industry {
    General,
    Automotive,
    Aerospace,
    HeavyEngineering,
    InformationTech,
    Banking,
    Metals,
    Retail,
    Education,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompanyReports {
    param: Param,
    reports: FinancialReport,
    affiliation: HashMap<Industry, f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValuationModel {
    pub params: BTreeMap<Period, CompanyReports>,
    pub shareprice: BTreeMap<NDt, f64>,
}

#[cfg(test)]
mod valuations {
    use super::*;
    use crate::approx;
    use std::collections::{BTreeSet, HashMap};

    use chrono::Datelike;
    use BsType::*;
    use CfType::*;
    use PlType::*;

    #[test]
    fn trucking() {
        use crate::Currency::*;
        use std::fmt::Display;

        fn frmt<T: Display>(disp: &mut String, name: &str, dig: usize, val: T) -> T {
            disp.push_str(&format!("{name} = {val:.dig$}\n\n"));
            val
        }

        let mut disp = format!("# TRUCKING ANALYSIS\n\n");

        let start_dt = NDt::from_ymd_opt(2022, 3, 16).unwrap();
        let valuation_dt = NDt::from_ymd_opt(2022, 10, 9).unwrap();

        let (infl_fuel, infl_revenue, infl_salary, infl_other) = (0.18, 0.18, 0.15, 0.18);

        let (interest_cap, interest_wc) = (0.16, 0.18);

        let capacity = 45.0;
        let cap_util = 0.85;

        let (tax_rate, edu_tax, mat) = (0.1, 0.02, 0.005);
        let pioneer_stat = false;
        let max_alloc_rat = 0.9;

        let (si_head, tip_3xl, tip_4xl) = (26_247_189.0, 21_792_346.0, 27_279_460.0);
        let tip_type = 1; // 1 = 3 axle / 2 = 4 axle
        let veh_life = 4.0;
        let salvage_val = 15.0 * 350_000.0;
        let act_salv_val = 7_000_000.0;

        let plant_mc = 2_378_861.0;
        let plant_life = 10.0;

        let manage_fee = 250_000.0;
        let driver_sal = 172_000.0;
        let trip_allow = 14_000.0;

        let fuel_consp = 860.0 / 450.0;
        let diesel_pr = 770.0;

        let wc_ratio = 1.0 / 12.0; // 1 month of Diesel + spares

        let spares: [(f64, f64); 3] = [
            (120_000.0, 7_500.0),     // spares
            (2_400_000.0, 100_000.0), // tyres
            (40_000.0, 100_000.0),    // engine oil
        ];

        let cyc_time = 2.0;
        let ref_km = 860.0;
        let trip_km = 1_300.0;

        // WORKING

        disp.push_str("## Working of basic terms\n\n");

        let yf = crate::yearfrac(start_dt, valuation_dt, crate::DayCountConvention::US30360);
        frmt(&mut disp, "yearfrac", 3, yf);

        let working_days = frmt(&mut disp, "working days", 0, 360.0 * cap_util);

        let nos_trips = frmt(&mut disp, "nos of trips", 0, working_days / cyc_time);
        let asset_cost = si_head + if tip_type == 1 { tip_3xl } else { tip_4xl };
        frmt(&mut disp, "Asset cost", 0, asset_cost);

        let (yr, mt, dt) = (start_dt.year(), start_dt.month(), start_dt.day());

        let fst_yr_avg = |infl: f64| (1.0 + infl / 2.0) / (1.0 + infl).powf(yf);

        let (veh_depr, plant_depr) = ((asset_cost - salvage_val) / veh_life, plant_mc / plant_life);

        let opex01 = [
            (
                infl_other,
                spares
                    .iter()
                    .map(|(x, y)| x / y * ref_km * fst_yr_avg(infl_other))
                    .sum::<f64>(),
            ),
            (
                infl_fuel,
                diesel_pr / fuel_consp * ref_km * fst_yr_avg(infl_fuel),
            ),
            (
                infl_salary,
                manage_fee * 12.0 / nos_trips * fst_yr_avg(infl_salary),
            ),
            (
                infl_salary,
                driver_sal * 12.0 / nos_trips * fst_yr_avg(infl_salary),
            ),
            (infl_salary, trip_allow * fst_yr_avg(infl_salary)),
        ];

        let years = |yrs: i32| NDt::from_ymd_opt(yrs + yr, mt, dt).unwrap();

        disp.push_str(&format!("Opex expenses @ 1 year {:?}", opex01));

        let npv_trucking = |trip_price: f64| -> (f64, Accounts) {
            let mut trk: Accounts = Accounts {
                currency: NGN,
                consolidated: true,
                dates: BTreeSet::new(),
                balance_sheet: BTreeMap::new(),
                profit_loss: BTreeMap::new(),
                cash_flow: BTreeMap::new(),
                others: BTreeMap::new(),
            };

            let mut net_cap = asset_cost;

            let loan_payback = (plant_mc + asset_cost) / 4.0;

            trk.balance_sheet =
                BTreeMap::from([start_dt, years(1), years(2), years(3), years(4)].map(|x| {
                    (
                        x,
                        BsMap::from([
                            (PlantPropertyEquipment, plant_mc + asset_cost),
                            (LongTermBorrowings, plant_mc + asset_cost),
                        ]),
                    )
                }));

            trk.profit_loss = BTreeMap::from(
                [
                    (years(-1), start_dt),
                    (start_dt, years(1)),
                    (years(1), years(2)),
                    (years(2), years(3)),
                    (years(3), years(4)),
                ]
                .map(|x| (x, PlMap::from([(Depreciation, veh_depr + plant_depr)]))),
            );

            trk.cash_flow = BTreeMap::from(
                [
                    (years(-1), start_dt),
                    (start_dt, years(1)),
                    (years(1), years(2)),
                    (years(2), years(3)),
                    (years(3), years(4)),
                ]
                .map(|x| (x, CfMap::new())),
            );

            *(trk.profit_loss.get_mut(&(years(-1), start_dt)).unwrap()) = HashMap::new();
            trk.set_dates_from_profit_loss();
            if pioneer_stat {
                trk.set_tax_rates(0.0, 0.0, 0.0);
            } else {
                trk.set_tax_rates(tax_rate, edu_tax, mat);
            }

            *(trk.cash_flow.get_mut(&(years(-1), start_dt)).unwrap()) = HashMap::from([
                (ChangePPE, plant_mc + asset_cost),
                (ChangeDebt, plant_mc + asset_cost),
            ]);

            // Setup values as per Revenue model

            for y in 1..=4 {
                let (yr_prev, yr_curr, prd) = (years(y - 1), years(y), (years(y - 1), years(y)));
                trk.put_profit_loss(
                    prd,
                    CostMaterial,
                    opex01
                        .iter()
                        .map(|(infl, cst)| cst * (1.0 + infl).powi(y - 1))
                        .sum::<f64>()
                        * nos_trips,
                );

                trk.put_profit_loss(
                    prd,
                    OperatingRevenue,
                    trip_price * (1.0f64 + infl_revenue).powi(y - 1) * nos_trips,
                );

                trk.put_balance_sheet(
                    yr_curr,
                    AccumulatedDepreciation,
                    trk.get_balance_sheet(yr_prev, AccumulatedDepreciation) + veh_depr + plant_depr,
                );

                trk.put_balance_sheet(
                    yr_curr,
                    PlantPropertyEquipment,
                    trk.get_balance_sheet(yr_prev, PlantPropertyEquipment) + plant_depr,
                );

                trk.put_balance_sheet(
                    yr_curr,
                    LongTermBorrowings,
                    trk.get_balance_sheet(yr_prev, LongTermBorrowings) - loan_payback,
                );

                let wc = ((opex01[0].1) * (1.0f64 + infl_other).powi(y - 1)
                    + (opex01[1].1) * (1.0f64 + infl_fuel).powi(y - 1))
                    * nos_trips
                    * wc_ratio;

                trk.put_balance_sheet(yr_curr, CurrentBorrowings, wc);
                trk.put_balance_sheet(yr_curr, RawMaterials, wc);

                trk.put_profit_loss(
                    prd,
                    InterestExpense,
                    interest_cap * trk.get_balance_sheet(yr_prev, LongTermBorrowings)
                        + interest_wc * trk.get_balance_sheet(yr_prev, CurrentBorrowings),
                );

                trk.calc_elements();
                trk.put_profit_loss(prd, TaxDepreciation, {
                    if y == 4 {
                        net_cap
                    } else {
                        let alloc = f64::min(max_alloc_rat * net_cap, {
                            0.0f64.max({
                                trk.get_profit_loss(prd, EBT)
                                    + trk.get_profit_loss(prd, Depreciation)
                                    - (mat * trk.get_profit_loss(prd, Revenue)
                                        - edu_tax * trk.get_profit_loss(prd, EBITDA))
                                        / tax_rate
                            })
                        });
                        net_cap = net_cap - alloc;
                        alloc
                    }
                });
                trk.calc_tax().calc_elements();

                trk.put_balance_sheet(
                    yr_curr,
                    RetainedEarnings,
                    trk.get_balance_sheet(yr_prev, RetainedEarnings)
                        + trk.get_profit_loss((yr_prev, yr_curr), ContributionRetainedEarnings),
                );

                let cst_cs = trk.get_balance_sheet(yr_prev, CommonStock);
                trk.put_balance_sheet(yr_curr, CommonStock, cst_cs);

                trk.calc_cash_flow().calc_elements();

                let csh = trk.get_balance_sheet(yr_prev, Cash)
                    + trk.get_cash_flow((yr_prev, yr_curr), NetCashFlow);

                if csh < 0.0 {
                    trk.put_balance_sheet(yr_curr, Cash, 0.0)
                        .put_balance_sheet(yr_curr, CommonStock, cst_cs - csh)
                        .calc_elements();
                } else {
                    trk.put_balance_sheet(yr_curr, Cash, csh);
                }

                trk.calc_elements();
            }

            let (fin_dt, fin_per) = (years(4), (years(3), years(4)));

            trk.put_balance_sheet(fin_dt, RawMaterials, 0.0);
            trk.put_balance_sheet(fin_dt, CurrentBorrowings, 0.0);

            trk.calc_cash_flow().calc_elements();

            trk.put_profit_loss(fin_per, SalesAmountPPE, act_salv_val);

            trk.put_profit_loss(
                fin_per,
                GrossSalesPPE,
                trk.get_balance_sheet(years(3), PlantPropertyEquipment) + plant_depr,
            );

            trk.put_profit_loss(
                fin_per,
                AccAmortSalesPPE,
                trk.get_balance_sheet(years(3), AccumulatedDepreciation) + veh_depr + plant_depr,
            );

            trk.put_balance_sheet(fin_dt, PlantPropertyEquipment, 0.0);
            trk.put_balance_sheet(fin_dt, AccumulatedDepreciation, 0.0);

            trk.calc_elements();

            trk.put_balance_sheet(
                fin_dt,
                AccumulatedOCI,
                trk.get_profit_loss(fin_per, GainsLossesSales),
            );

            trk.calc_cash_flow().calc_elements();

            trk.put_balance_sheet(
                fin_dt,
                Cash,
                trk.get_balance_sheet(years(3), Cash) + trk.get_cash_flow(fin_per, NetCashFlow),
            );
            trk.calc_elements();
            // Calculate the rest of the statements and the NPV of the system

            (0.0, trk)
        };

        let solve = |tr_pr0: f64| -> (f64, Accounts) {
            let _calc_price = |x_tr| {
                let (xp, _) = npv_trucking(x_tr);
                xp
            };
            // let tr_price = newt_raph(calc_price, tr_pr0, 100.0).unwrap();
            let (_, tx) = npv_trucking(tr_pr0);
            (tr_pr0, tx)
        };

        let (tr_pr, tx) = solve(600_000.0);

        let tx = tx.consecutive_cash_flow();

        disp.push_str(&format!("\n\n``` \n{tx}\n```\n\n"));

        disp.push_str("## Final optimized solutions\n\n");
        frmt(&mut disp, "Trip cost start", 0, tr_pr);

        let tr_today = tr_pr * (1.0f64 + infl_revenue).powf(yf);
        frmt(&mut disp, "Trip cost today", 0, tr_today);
        frmt(&mut disp, "Trip cost / km", 2, tr_today / ref_km);
        frmt(
            &mut disp,
            "Trip cost for new trip",
            0,
            tr_today / ref_km * trip_km,
        );
        frmt(
            &mut disp,
            "Trip cost / km / MT",
            2,
            tr_today / ref_km / capacity,
        );

        println!("{disp}");

        assert!(approx(0.0, 0.0));
    }
}
