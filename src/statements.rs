/*!
Implement Account Statement modules for the FinanceLib library

Module      : financeLib::statements <br>
Copyright   : (c) 2022 Kishaloy Neogi <br>
License     : MIT <br>
Maintainer  : Kishaloy Neogi <br>
Email       : <nkishaloy@yahoo.com>

The module describes the base modules of Account Statements. These includes Balance Sheets,
Income Statements and Cash Flow Statements and their utiities.

You may see the github repository at <https://github.com/n-kishaloy/financelib>
*/

use chrono::naive::NaiveDate as NDt;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

/**
FinType - Defines Trait for all Financial types
 */
pub trait FinType {
    fn is_calc(self) -> bool;
}

/**
BsType - Enum for all Balance Sheet types.

This is primarily used in craeting Hashmap for keeping Balance Sheet items.
 */
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub enum BsType {
    Cash,
    CurrentReceivables,
    CurrentLoans,
    CurrentAdvances,
    OtherCurrentAssets,
    CurrentInvestments,
    Inventories,
    RawMaterials,
    WorkInProgress,
    FinishedGoods,
    CurrentAssets,
    AccountReceivables,
    LongTermLoanAssets,
    LongTermAdvances,
    LongTermInvestments,
    OtherLongTermAssets,
    PlantPropertyEquipment,
    AccumulatedDepreciation,
    NetPlantPropertyEquipment,
    LeasingRentalAssets,
    AccumulatedAmortizationLeaseRental,
    NetLeaseRentalAssets,
    Goodwill,
    CapitalWip,
    OtherTangibleAssets,
    IntangibleAssets,
    IntangibleAssetsDevelopment,
    AccumulatedAmortization,
    NetIntangibleAssets,
    LongTermAssets,
    Assets,
    CurrentPayables,
    CurrentBorrowings,
    CurrentNotesPayable,
    OtherCurrentLiabilities,
    InterestPayable,
    CurrentProvisions,
    CurrentTaxPayables,
    LiabilitiesSaleAssets,
    CurrentLeasesLiability,
    CurrentLiabilities,
    AccountPayables,
    LongTermBorrowings,
    BondsPayable,
    DeferredTaxLiabilities,
    LongTermLeasesLiability,
    DeferredCompensation,
    DeferredRevenues,
    CustomerDeposits,
    OtherLongTermLiabilities,
    PensionProvision,
    TaxProvision,
    LongTermProvision,
    LongTermLiabilities,
    Liabilities,
    CommonStock,
    PreferredStock,
    PdInCapAbovePar,
    PdInCapTreasuryStock,
    RevaluationReserves,
    Reserves,
    RetainedEarnings,
    AccumulatedOci,
    MinorityInterests,
    Equity,
}

/**
PlType - Enum for all Profit and Loss types.

This is primarily used in craeting Hashmap for keeping Profit and Loss items.
 */
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub enum PlType {
    OperatingRevenue,
    NonOperatingRevenue,
    ExciseStaxLevy,
    OtherIncome,
    Revenue,
    CostMaterial,
    DirectExpenses,
    COGS,
    Salaries,
    AdministrativeExpenses,
    ResearchNDevelopment,
    OtherOverheads,
    OtherOperativeExpenses,
    OtherExpenses,
    ExceptionalItems,
    GrossProfit,
    Pbitda,
    Depreciation,
    AssetImpairment,
    LossDivestitures,
    Amortization,
    Pbitx,
    InterestRevenue,
    InterestExpense,
    CostDebt,
    OtherFinancialRevenue,
    Pbtx,
    ExtraordinaryItems,
    PriorYears,
    Pbt,
    TaxesCurrent,
    TaxesDeferred,
    Pat,
    NetIncomeDiscontinuedOps,
    NetIncome,
    GainsLossesForex,
    GainsLossesActurial,
    GainsLossesSales,
    FvChangeAvlSale,
    OtherDeferredTaxes,
    OtherComprehensiveIncome,
    TotalComprehensiveIncome,
}

/**
CfType - Enum for all Cash Flow types.

This is primarily used in craeting Hashmap for keeping Cash Flow items.
 */
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub enum CfType {
    DeferredIncomeTaxes,
    ChangeInventories,
    ChangeReceivables,
    ChangePayables,
    ChangeLiabilities,
    ChangeProvisions,
    OtherCfOperations,
    StockCompensationExpense,
    StockCompensationTaxBenefit,
    AccretionDebtDiscount,
    CashFlowOperations,
    InvestmentsPpe,
    InvestmentsCapDevp,
    InvestmentsLoans,
    AcqEquityAssets,
    DisEquityAssets,
    DisPpe,
    ChangeInvestments,
    CfInvestmentInterest,
    CfInvestmentDividends,
    OtherCfInvestments,
    CashFlowInvestments,
    StockSales,
    StockRepurchase,
    DebtIssue,
    DebtRepay,
    InterestFin,
    Dividends,
    DonorContribution,
    OtherCfFinancing,
    CashFlowFinancing,
    NetCashFlow,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub enum FinOthersTyp {
    Taxrate,
    CurrentRatio,
    AcidRatio,
    DaysOfInventory,
    InventoryTurnoverRatio,
    Fcff,
    Fcfs,
    Fcfe,
    Fcfd,
}

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use BsType::*;
use PlType::*;

lazy_static! {
    static ref BALANCE_SHEET_MAP: Vec<(BsType, (Vec<BsType>, Vec<BsType>))> = vec![
        (
            Inventories,
            (vec![RawMaterials, WorkInProgress, FinishedGoods], vec![],)
        ),
        (
            CurrentAssets,
            (
                vec![
                    Cash,
                    CurrentReceivables,
                    CurrentLoans,
                    CurrentAdvances,
                    OtherCurrentAssets,
                    CurrentInvestments,
                    Inventories,
                ],
                vec![],
            )
        ),
        (
            NetPlantPropertyEquipment,
            (vec![PlantPropertyEquipment], vec![AccumulatedDepreciation],)
        ),
        (
            NetLeaseRentalAssets,
            (
                vec![LeasingRentalAssets],
                vec![AccumulatedAmortizationLeaseRental],
            )
        ),
        (
            NetIntangibleAssets,
            (
                vec![IntangibleAssets, IntangibleAssetsDevelopment],
                vec![AccumulatedAmortization],
            )
        ),
        (
            LongTermAssets,
            (
                vec![
                    AccountReceivables,
                    LongTermLoanAssets,
                    LongTermAdvances,
                    LongTermInvestments,
                    OtherLongTermAssets,
                    NetPlantPropertyEquipment,
                    NetLeaseRentalAssets,
                    Goodwill,
                    CapitalWip,
                    OtherTangibleAssets,
                    NetIntangibleAssets,
                ],
                vec![],
            )
        ),
        (Assets, (vec![CurrentAssets, LongTermAssets], vec![])),
        (
            CurrentLiabilities,
            (
                vec![
                    CurrentPayables,
                    CurrentBorrowings,
                    CurrentNotesPayable,
                    OtherCurrentLiabilities,
                    InterestPayable,
                    CurrentProvisions,
                    CurrentTaxPayables,
                    LiabilitiesSaleAssets,
                    CurrentLeasesLiability,
                ],
                vec![],
            )
        ),
        (
            LongTermLiabilities,
            (
                vec![
                    AccountPayables,
                    LongTermBorrowings,
                    BondsPayable,
                    DeferredTaxLiabilities,
                    LongTermLeasesLiability,
                    DeferredCompensation,
                    DeferredRevenues,
                    CustomerDeposits,
                    OtherLongTermLiabilities,
                    PensionProvision,
                    TaxProvision,
                    LongTermProvision,
                ],
                vec![],
            )
        ),
        (
            Liabilities,
            (vec![CurrentLiabilities, LongTermLiabilities], vec![],)
        ),
        (
            Equity,
            (
                vec![
                    CommonStock,
                    PreferredStock,
                    PdInCapAbovePar,
                    PdInCapTreasuryStock,
                    RevaluationReserves,
                    Reserves,
                    RetainedEarnings,
                    AccumulatedOci,
                    MinorityInterests,
                ],
                vec![],
            )
        ),
    ];
    static ref PROFIT_LOSS_MAP: Vec<(PlType, (Vec<PlType>, Vec<PlType>))> = vec![
        (
            Revenue,
            (
                vec![OperatingRevenue, NonOperatingRevenue,],
                vec![ExciseStaxLevy],
            )
        ),
        (COGS, (vec![CostMaterial, DirectExpenses], vec![],)),
        (GrossProfit, (vec![Revenue], vec![COGS],)),
        (
            Pbitda,
            (
                vec![GrossProfit, OtherIncome],
                vec![
                    Salaries,
                    AdministrativeExpenses,
                    ResearchNDevelopment,
                    OtherOverheads,
                    OtherOperativeExpenses,
                    OtherExpenses,
                    ExceptionalItems
                ],
            )
        ),
        (
            Pbitx,
            (
                vec![Pbitda],
                vec![
                    Depreciation,
                    AssetImpairment,
                    LossDivestitures,
                    Amortization
                ],
            )
        ),
        (
            Pbtx,
            (
                vec![Pbitx, InterestRevenue, OtherFinancialRevenue],
                vec![InterestExpense, CostDebt],
            )
        ),
        (Pbt, (vec![Pbtx], vec![ExtraordinaryItems, PriorYears],)),
        (Pat, (vec![Pbt], vec![TaxesCurrent, TaxesDeferred],)),
        (NetIncome, (vec![Pat, NetIncomeDiscontinuedOps], vec![],)),
        (
            OtherComprehensiveIncome,
            (
                vec![
                    GainsLossesForex,
                    GainsLossesActurial,
                    GainsLossesSales,
                    FvChangeAvlSale
                ],
                vec![OtherDeferredTaxes],
            )
        ),
        (
            TotalComprehensiveIncome,
            (vec![NetIncome, OtherComprehensiveIncome], vec![],)
        ),
    ];
    static ref BALANCE_SHEET_CALC: HashSet<BsType> =
        BALANCE_SHEET_MAP.iter().map(|&(x, _)| x).collect();
    static ref PROFIT_LOSS_CALC: HashSet<PlType> =
        PROFIT_LOSS_MAP.iter().map(|&(x, _)| x).collect();
    static ref BALANCE_SHEET_HASHMAP: HashMap<BsType, (&'static Vec<BsType>, &'static Vec<BsType>)> = {
        let mut yx = HashMap::new();
        for (k, (p, q)) in BALANCE_SHEET_MAP.iter() {
            yx.insert(*k, (p, q));
        }
        yx
    };
    static ref DEBIT_TYPE: HashMap<BsType, BalanceSheetEntry> = {
        let mut mz = HashMap::new();
        debit_mapping(
            &mut mz,
            BsType::Assets,
            BalanceSheetEntry::AssetEntry,
            BalanceSheetEntry::AssetContra,
        );
        debit_mapping(
            &mut mz,
            BsType::Liabilities,
            BalanceSheetEntry::LiabilityEntry,
            BalanceSheetEntry::LiabilityContra,
        );
        debit_mapping(
            &mut mz,
            BsType::Equity,
            BalanceSheetEntry::EquityEntry,
            BalanceSheetEntry::EquityContra,
        );
        mz
    };
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
enum BalanceSheetEntry {
    AssetEntry,
    AssetContra,
    LiabilityEntry,
    LiabilityContra,
    EquityEntry,
    EquityContra,
}

fn debit_mapping(
    debit_map: &mut HashMap<BsType, BalanceSheetEntry>,
    calc_type: BsType,
    calc_pos: BalanceSheetEntry,
    calc_neg: BalanceSheetEntry,
) {
    let (a, b) = BALANCE_SHEET_HASHMAP[&calc_type];
    for x in a.iter() {
        if BALANCE_SHEET_CALC.contains(x) {
            debit_mapping(debit_map, *x, calc_pos, calc_neg)
        } else {
            debit_map.insert(*x, calc_pos);
        }
    }
    for x in b.iter() {
        if BALANCE_SHEET_CALC.contains(x) {
            debit_mapping(debit_map, *x, calc_neg, calc_pos)
        } else {
            debit_map.insert(*x, calc_neg);
        }
    }
}

impl FinType for BsType {
    fn is_calc(self) -> bool {
        BALANCE_SHEET_CALC.contains(&self)
    }
}

impl FinType for PlType {
    fn is_calc(self) -> bool {
        PROFIT_LOSS_CALC.contains(&self)
    }
}

pub trait FinMaps {
    fn calc_elements(&mut self);
    fn remove_calc_elem(&mut self);
    fn check(&self) -> bool;
    fn common_size(&self) -> Self;

    fn clean(&mut self);
}

fn calc_indv_elem<T: Hash + Ord + Copy>(hm: &HashMap<T, f64>, x: &Vec<T>) -> f64 {
    x.iter().map(|z| hm.get(&z).unwrap_or(&0.0)).sum()
}

impl FinMaps for BsMap {
    fn calc_elements(&mut self) {
        for (k, (d, b)) in BALANCE_SHEET_MAP.iter() {
            self.insert(*k, calc_indv_elem(self, d) - calc_indv_elem(self, b));
        }
    }

    fn remove_calc_elem(&mut self) {
        self.retain(|k, _| !k.is_calc());
    }

    fn check(&self) -> bool {
        todo!()
    }

    fn common_size(&self) -> Self {
        let scale = self[&Assets];
        self.iter().map(|(k, v)| (*k, v / scale)).collect()
    }

    fn clean(&mut self) {
        self.retain(|_, v| v.abs() > 1e-5);
    }
}

impl BsMapTrait for BsMap {
    fn debit(&mut self, typ: BsType, val: f64) {
        let deb_type = DEBIT_TYPE[&typ];
        let mut adder = |x| *self.entry(typ).or_insert(0.0) += x;

        use BalanceSheetEntry::*;
        match deb_type {
            AssetEntry | LiabilityContra | EquityContra => adder(val),
            _ => adder(-val),
        }
    }
}

pub trait BsMapTrait {
    fn debit(&mut self, typ: BsType, val: f64);

    fn credit(&mut self, typ: BsType, val: f64) {
        BsMapTrait::debit(self, typ, -val)
    }

    fn transact(&mut self, tran: (BsType, BsType, f64)) {
        let (deb, crd, val) = tran;
        self.debit(deb, val);
        self.credit(crd, val);
    }

    fn transact_series(&mut self, trans: Vec<(BsType, BsType, f64)>) {
        for x in trans {
            self.transact(x)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Param {
    pub unlevered: f64,
    pub shield_tax: f64,
    pub equity: f64,
    pub debt: f64,
    pub valuation: f64,
}

pub type BsMap = HashMap<BsType, f64>;
pub type PlMap = HashMap<PlType, f64>;
pub type CfMap = HashMap<CfType, f64>;
pub type FinOthersMap = HashMap<FinOthersTyp, f64>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BalanceSheet {
    pub date: NDt,
    pub items: BsMap,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfitLoss {
    pub date_beg: NDt,
    pub date_end: NDt,
    pub items: PlMap,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CashFlow {
    pub date_beg: NDt,
    pub date_end: NDt,
    pub items: CfMap,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinOthers {
    pub date_beg: NDt,
    pub date_end: NDt,
    pub items: FinOthersMap,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Account {
    pub date_beg: NDt,
    pub date_end: NDt,

    pub balance_sheet_beg: Option<BsMap>,
    pub balance_sheet_end: Option<BsMap>,
    pub profit_loss: Option<PlMap>,
    pub cash_flow: Option<CfMap>,

    pub others: FinOthersMap,
}

impl Account {
    pub fn to_statements(
        &self,
    ) -> (
        Option<BalanceSheet>,
        Option<BalanceSheet>,
        Option<ProfitLoss>,
        Option<CashFlow>,
        FinOthers,
    ) {
        (
            self.balance_sheet_beg(),
            self.balance_sheet_end(),
            self.profit_loss(),
            self.cash_flow(),
            self.fin_others(),
        )
    }

    pub fn from_statements(
        _bs0: &Option<BalanceSheet>,
        _bs1: &Option<BalanceSheet>,
        _pl: &Option<ProfitLoss>,
        _cf: &Option<CashFlow>,
    ) -> Option<Self> {
        todo!()
    }

    pub fn calc_elements(&mut self) {
        todo!()
    }

    pub fn balance_sheet_beg(&self) -> Option<BalanceSheet> {
        Some(BalanceSheet {
            date: self.date_beg,
            items: (&self.balance_sheet_beg).clone()?,
        })
    }

    pub fn balance_sheet_end(&self) -> Option<BalanceSheet> {
        Some(BalanceSheet {
            date: self.date_end,
            items: (&self.balance_sheet_end).clone()?,
        })
    }

    pub fn profit_loss(&self) -> Option<ProfitLoss> {
        Some(ProfitLoss {
            date_beg: self.date_beg,
            date_end: self.date_end,
            items: (&self.profit_loss).clone()?,
        })
    }

    pub fn cash_flow(&self) -> Option<CashFlow> {
        Some(CashFlow {
            date_beg: self.date_beg,
            date_end: self.date_end,
            items: (&self.cash_flow).clone()?,
        })
    }

    pub fn fin_others(&self) -> FinOthers {
        FinOthers {
            date_beg: self.date_beg,
            date_end: self.date_end,
            items: (self.others).clone(),
        }
    }

    pub fn eps() -> f64 {
        todo!()
    }

    pub fn diluted_eps(
        _earn: f64,
        _pref_div: f64,
        _shares: f64,
        _share_price: f64,
        _options: f64,
    ) -> f64 {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Company {
    pub code: String,
    pub affiliation: HashMap<String, f64>,
    pub consolidated: bool,
    pub period: Vec<NDt>,
    pub balance_sheet: HashMap<NDt, Option<BsMap>>,
    pub profit_loss: HashMap<(NDt, NDt), Option<PlMap>>,
    pub cash_flow: HashMap<(NDt, NDt), Option<CfMap>>,
    pub others: HashMap<(NDt, NDt), FinOthersMap>,
    pub share_price: Option<Vec<(NDt, f64)>>,
    pub rate: Option<Vec<Param>>,
    pub beta: Option<Vec<Param>>,
}

impl Company {
    pub fn get_account(&self, d0: NDt, d1: NDt) -> Option<Account> {
        if self.period.contains(&d0) && self.period.contains(&d1) {
            Some(Account {
                date_beg: d0,
                date_end: d1,
                balance_sheet_beg: self.balance_sheet[&d0].clone(),
                balance_sheet_end: self.balance_sheet[&d1].clone(),
                profit_loss: self.profit_loss[&(d0, d1)].clone(),
                cash_flow: self.cash_flow[&(d0, d1)].clone(),
                others: self.others[&(d0, d1)].clone(),
            })
        } else {
            None
        }
    }

    pub fn calc_elements(&mut self) {
        todo!()
    }

    pub fn transact(&mut self, _date: NDt, _deb: BsType, _crd: BsType, _x: f64) {
        todo!()
    }

    pub fn sort_dates(&mut self) {
        todo!()
    }

    pub fn to_account_vec(&self) -> Vec<Account> {
        todo!()
    }

    pub fn from_account_vec(_ac_vec: &Vec<Account>) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod accounts {
    use crate::approx;

    use super::*;
    use BalanceSheetEntry::*;
    #[test]
    fn type_checks() {
        let y = Inventories;
        let (x, _) = BALANCE_SHEET_MAP[0];
        assert_eq!(Inventories, x);
        assert!(y.is_calc());
        assert!(y.is_calc());
        assert!(NetPlantPropertyEquipment.is_calc());
        assert!(!RawMaterials.is_calc());
        assert!(Pbitda.is_calc());
        assert!(!Salaries.is_calc());
        assert_eq!(DEBIT_TYPE.get(&Inventories), None);
        assert_eq!(DEBIT_TYPE.get(&RawMaterials), Some(&AssetEntry));
        assert_eq!(DEBIT_TYPE.get(&CurrentAdvances), Some(&AssetEntry));
        assert_eq!(DEBIT_TYPE.get(&NetPlantPropertyEquipment), None);
        assert_eq!(DEBIT_TYPE.get(&AccumulatedDepreciation), Some(&AssetContra));
        assert_eq!(
            DEBIT_TYPE.get(&AccumulatedAmortizationLeaseRental),
            Some(&AssetContra)
        );
        assert_eq!(DEBIT_TYPE.get(&LongTermLiabilities), None);
        assert_eq!(DEBIT_TYPE.get(&BondsPayable), Some(&LiabilityEntry));
        assert_eq!(DEBIT_TYPE.get(&Equity), None);
        assert_eq!(DEBIT_TYPE.get(&MinorityInterests), Some(&EquityEntry));
    }

    #[test]
    fn account_check() {
        // let b0 = BalanceSheet {
        //     date: NDt::from_ymd(2009, 05, 22),
        //     items: HashMap::from([(Cash, 23.5), (Equity, 12.5)]),
        // };
        // let b1 = BalanceSheet {
        //     date: NDt::from_ymd(2010, 09, 20),
        //     items: HashMap::from([(Cash, 14.5), (CurrentLoans, 10.5)]),
        // };

        let ac1 = Account {
            date_beg: NDt::from_ymd(2009, 05, 22),
            date_end: NDt::from_ymd(2010, 09, 27),

            balance_sheet_beg: Some(HashMap::from([(Cash, 23.5), (Equity, 12.5)])),
            balance_sheet_end: None,

            profit_loss: Some(HashMap::from([
                (Revenue, -2.58),
                (Pat, 24.8),
                (Pbitx, 11.3),
            ])),
            cash_flow: None,

            others: HashMap::new(),
        };

        let ac_js = serde_json::to_string(&ac1).unwrap();
        let acx: Account = serde_json::from_str(&ac_js).unwrap();
        println!("{:?} !false => {}", acx, !true);

        let bg = acx.balance_sheet_beg.clone().unwrap();

        println!("{}", bg.get(&Cash).unwrap_or(&0.0));
        println!("{}", bg.get(&CommonStock).unwrap_or(&0.0));

        assert!(approx(
            ac1.balance_sheet_beg.unwrap()[&Cash],
            acx.balance_sheet_beg.unwrap()[&Cash]
        ));
    }
}
