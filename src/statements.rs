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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum BsTyp {
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum PlTyp {
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum CfTyp {
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

use std::collections::{HashMap, HashSet};

use BsTyp::*;
use PlTyp::*;

lazy_static! {
    static ref BALANCE_SHEET_MAP: Vec<(BsTyp, Vec<BsTyp>, Vec<BsTyp>)> = vec![
        (
            Inventories,
            vec![RawMaterials, WorkInProgress, FinishedGoods],
            vec![],
        ),
        (
            CurrentAssets,
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
        ),
        (
            NetPlantPropertyEquipment,
            vec![PlantPropertyEquipment],
            vec![AccumulatedDepreciation],
        ),
        (
            NetLeaseRentalAssets,
            vec![LeasingRentalAssets],
            vec![AccumulatedAmortizationLeaseRental],
        ),
        (
            NetIntangibleAssets,
            vec![IntangibleAssets, IntangibleAssetsDevelopment],
            vec![AccumulatedAmortization],
        ),
        (
            LongTermAssets,
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
        ),
        (Assets, vec![CurrentAssets, LongTermAssets], vec![]),
        (
            CurrentLiabilities,
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
        ),
        (
            LongTermLiabilities,
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
        ),
        (
            Liabilities,
            vec![CurrentLiabilities, LongTermLiabilities],
            vec![],
        ),
        (
            Equity,
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
        ),
    ];
    static ref PROFIT_LOSS_MAP: Vec<(PlTyp, Vec<PlTyp>, Vec<PlTyp>)> = vec![
        (
            Revenue,
            vec![OperatingRevenue, NonOperatingRevenue,],
            vec![ExciseStaxLevy],
        ),
        (COGS, vec![CostMaterial, DirectExpenses], vec![],),
        (GrossProfit, vec![Revenue], vec![COGS],),
        (
            Pbitda,
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
        ),
        (
            Pbitx,
            vec![Pbitda],
            vec![
                Depreciation,
                AssetImpairment,
                LossDivestitures,
                Amortization
            ],
        ),
        (
            Pbtx,
            vec![Pbitx, InterestRevenue, OtherFinancialRevenue],
            vec![InterestExpense, CostDebt],
        ),
        (Pbt, vec![Pbtx], vec![ExtraordinaryItems, PriorYears],),
        (Pat, vec![Pbt], vec![TaxesCurrent, TaxesDeferred],),
        (NetIncome, vec![Pat, NetIncomeDiscontinuedOps], vec![],),
        (
            OtherComprehensiveIncome,
            vec![
                GainsLossesForex,
                GainsLossesActurial,
                GainsLossesSales,
                FvChangeAvlSale
            ],
            vec![OtherDeferredTaxes],
        ),
        (
            TotalComprehensiveIncome,
            vec![NetIncome, OtherComprehensiveIncome],
            vec![],
        ),
    ];
    static ref BALANCE_SHEET_CALC: HashSet<BsTyp> =
        BALANCE_SHEET_MAP.iter().map(|&(x, _, _)| x).collect();
    static ref PROFIT_LOSS_CALC: HashSet<PlTyp> =
        PROFIT_LOSS_MAP.iter().map(|&(x, _, _)| x).collect();
}

pub struct Param {
    pub unlevered: f64,
    pub shield_tax: f64,
    pub equity: f64,
    pub debt: f64,
    pub valuation: f64,
}

pub type BsMap = HashMap<BsTyp, f64>;
pub type PlMap = HashMap<PlTyp, f64>;
pub type CfMap = HashMap<CfTyp, f64>;

pub struct BalanceSheet {
    pub date: NDt,
    pub items: BsMap,
}

impl BalanceSheet {
    pub fn to_json(&self) -> String {
        // TODO: Implement
        unimplemented!()
    }
}

pub fn balance_sheet_from_json(_js: &String) -> BalanceSheet {
    // TODO: Implement
    unimplemented!()
}

pub struct ProfitLoss {
    pub date_beg: NDt,
    pub date_end: NDt,
    pub items: PlMap,
}

impl ProfitLoss {
    pub fn to_json(&self) -> String {
        // TODO: Implement
        unimplemented!()
    }
}
pub fn profit_loss_from_json(_js: &String) -> BalanceSheet {
    // TODO: Implement
    unimplemented!()
}

pub struct CashFlow {
    pub date_beg: NDt,
    pub date_end: NDt,
    pub items: PlMap,
}

impl CashFlow {
    pub fn to_json(&self) -> String {
        // TODO: Implement
        unimplemented!()
    }
}
pub fn cash_flow_from_json(_js: &String) -> BalanceSheet {
    // TODO: Implement
    unimplemented!()
}

pub struct Account {
    pub date_beg: NDt,
    pub date_end: NDt,

    pub balance_sheet_beg: Option<BsMap>,
    pub balance_sheet_end: Option<BsMap>,
    pub profit_loss: Option<PlMap>,
    pub cash_flow: Option<CfMap>,

    pub others: Option<HashMap<String, f64>>,
}

impl Account {
    pub fn to_statements(
        &self,
    ) -> (
        Option<BalanceSheet>,
        Option<BalanceSheet>,
        Option<ProfitLoss>,
        Option<CashFlow>,
    ) {
        // TODO: Implement
        unimplemented!()
    }

    pub fn to_json(&self) -> String {
        // TODO: Implement
        unimplemented!()
    }
}

pub fn account_from_statements(
    _bs0: &Option<BalanceSheet>,
    _bs1: &Option<BalanceSheet>,
    _pl: &Option<ProfitLoss>,
    _cf: &Option<CashFlow>,
) -> Account {
    // TODO: Implement
    unimplemented!()
}

pub fn account_from_json(_js: &String) -> Account {
    // TODO: Implement
    unimplemented!()
}

pub struct Company {
    pub code: String,
    pub affiliation: HashMap<String, f64>,
    pub consolidated: bool,
    pub period: Vec<NDt>,
    pub balance_sheet: HashMap<NDt, Option<BsMap>>,
    pub profit_loss: HashMap<(NDt, NDt), Option<PlMap>>,
    pub cash_flow: HashMap<(NDt, NDt), Option<CfMap>>,
    pub others: HashMap<(NDt, NDt), HashMap<String, f64>>,
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
                balance_sheet_beg: (self.balance_sheet.get(&d0).unwrap()).clone(),
                balance_sheet_end: (self.balance_sheet.get(&d1).unwrap()).clone(),
                profit_loss: (self.profit_loss.get(&(d0, d1)).unwrap()).clone(),
                cash_flow: (self.cash_flow.get(&(d0, d1)).unwrap()).clone(),
                others: None,
            })
        } else {
            None
        }
    }

    pub fn to_account_vec(&self) -> Vec<Account> {
        // TODO: Implement
        unimplemented!()
    }

    pub fn to_json(&self) -> String {
        // TODO: Implement
        unimplemented!()
    }
}

pub fn company_from_account_vec(_ac_vec: &Vec<Account>) -> Company {
    // TODO: Implement
    unimplemented!()
}

pub fn company_from_json(_js: &String) -> Company {
    // TODO: Implement
    unimplemented!()
}

#[cfg(test)]
mod accounts {
    use super::*;
    #[test]
    fn threaded() {
        let (x, _, _) = BALANCE_SHEET_MAP[0];
        assert_eq!(Inventories, x);
        assert!(BALANCE_SHEET_CALC.contains(&NetPlantPropertyEquipment));
        assert!(!BALANCE_SHEET_CALC.contains(&RawMaterials));
        assert!(PROFIT_LOSS_CALC.contains(&Pbitda));
        assert!(!PROFIT_LOSS_CALC.contains(&Salaries));
    }
}
