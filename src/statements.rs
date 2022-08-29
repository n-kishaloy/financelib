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
    /**
    is_calc - specifies if a item is calculated or entered

    e.g. Cash, Depreciation, Revenue etc are entered while items like Current
    Assets, Pat etc are calculated from items which are entered.
     */
    fn is_calc(self) -> bool;
}

/**
BsType - Enum for all Balance Sheet types.

This is primarily used in creating Hashmap for keeping Balance Sheet items.
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

This is primarily used in creating Hashmap for keeping Profit and Loss items.
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
    Dividends,
    ContributionRetainedEarnings,
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

This is primarily used in creating Hashmap for keeping Cash Flow items.
 */
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub enum CfType {
    ChangeCurrentAssets,
    ChangeLongTermAssets,
    ChangeCurrentLiabilities,
    ChangeLongTermLiabilities,
    ChangeProvisions,
    ChangeRetainedEarnings,
    AdjustmentsRetainedEarnings,
    ChangeAccumulatedOci,
    OtherCashFlowOperations,
    CashFlowOperations,
    ChangePPE,
    InvestmentsCapDevp,
    InvestmentsLoans,
    ChangeEquityAssets,
    ChangeInvestments,
    OtherCashFlowInvestments,
    CashFlowInvestments,
    StockSalesAndPurchase,
    ChangeDebt,
    CashFlowInterests,
    CashFlowDividends,
    DonorContribution,
    OtherCashFlowFinancing,
    CashFlowFinancing,
    NetCashFlow,
}

/**
FinOthersTyp - Enum for all Other types used in Financial statements.

This is primarily used in creating Hashmap for keeping Cash Flow items.
 */
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub enum FinOthersTyp {
    Taxrate,
    CurrentRatio,
    AcidRatio,
    DaysOfInventory,
    InventoryTurnoverRatio,
    FCFF,
    FCFS,
    FCFE,
    FCFD,
}

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

use core::panic;
use std::{
    collections::{btree_set::IntoIter, BTreeMap, BTreeSet, HashMap, HashSet},
    hash::Hash,
    vec,
};

use BsType::*;
use CfType::*;
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
            ContributionRetainedEarnings,
            (vec![NetIncome], vec![Dividends])
        ),
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
    static ref CASH_FLOW_MAP: Vec<(CfType, (Vec<CfType>, Vec<CfType>))> = vec![
        (
            CashFlowOperations,
            (
                vec![
                    ChangeCurrentLiabilities,
                    ChangeLongTermLiabilities,
                    ChangeProvisions,
                    ChangeRetainedEarnings,
                    AdjustmentsRetainedEarnings,
                    ChangeAccumulatedOci,
                    OtherCashFlowOperations
                ],
                vec![ChangeCurrentAssets, ChangeLongTermAssets]
            ),
        ),
        (
            CashFlowInvestments,
            (
                vec![OtherCashFlowInvestments],
                vec![
                    ChangePPE,
                    InvestmentsCapDevp,
                    InvestmentsLoans,
                    ChangeEquityAssets,
                    ChangeInvestments
                ]
            )
        ),
        (
            CashFlowFinancing,
            (
                vec![
                    StockSalesAndPurchase,
                    ChangeDebt,
                    DonorContribution,
                    OtherCashFlowFinancing
                ],
                vec![CashFlowInterests, CashFlowDividends]
            )
        ),
        (
            NetCashFlow,
            (
                vec![CashFlowOperations, CashFlowInvestments, CashFlowFinancing],
                vec![]
            )
        )
    ];
    static ref CASH_FLOW_BALANCE_SHEET: Vec<(CfType, (Vec<BsType>, Vec<BsType>))> = vec![
        (
            ChangeCurrentAssets,
            (
                vec![],
                vec![
                    CurrentReceivables,
                    CurrentLoans,
                    CurrentAdvances,
                    OtherCurrentAssets,
                    CurrentInvestments,
                    RawMaterials,
                    WorkInProgress,
                    FinishedGoods,
                ]
            )
        ),
        (
            ChangeLongTermAssets,
            (
                vec![
                    AccumulatedDepreciation,
                    AccumulatedAmortizationLeaseRental,
                    AccumulatedAmortization,
                ],
                vec![AccountReceivables, LongTermAdvances, CapitalWip,]
            )
        ),
        (
            ChangeCurrentLiabilities,
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
                vec![]
            )
        ),
        (
            ChangeLongTermLiabilities,
            (
                vec![
                    AccountPayables,
                    DeferredTaxLiabilities,
                    DeferredCompensation,
                    DeferredRevenues,
                    CustomerDeposits,
                    OtherLongTermLiabilities,
                ],
                vec![]
            )
        ),
        (
            ChangeProvisions,
            (
                vec![PensionProvision, TaxProvision, LongTermProvision,],
                vec![]
            )
        ),
        (ChangeRetainedEarnings, (vec![RetainedEarnings,], vec![])),
        (ChangeAccumulatedOci, (vec![AccumulatedOci,], vec![])),
        (
            ChangePPE,
            (
                vec![RevaluationReserves, Reserves,],
                vec![PlantPropertyEquipment, LeasingRentalAssets,]
            )
        ),
        (
            InvestmentsCapDevp,
            (vec![], vec![IntangibleAssetsDevelopment,])
        ),
        (InvestmentsLoans, (vec![], vec![LongTermLoanAssets,])),
        (ChangeEquityAssets, (vec![], vec![IntangibleAssets,])),
        (
            ChangeInvestments,
            (vec![], vec![LongTermInvestments, Goodwill,])
        ),
        (
            OtherCashFlowInvestments,
            (vec![], vec![OtherLongTermAssets, OtherTangibleAssets,])
        ),
        (
            StockSalesAndPurchase,
            (
                vec![
                    CommonStock,
                    PreferredStock,
                    PdInCapAbovePar,
                    PdInCapTreasuryStock,
                ],
                vec![]
            )
        ),
        (
            ChangeDebt,
            (
                vec![LongTermBorrowings, BondsPayable, LongTermLeasesLiability,],
                vec![]
            )
        ),
        (OtherCashFlowFinancing, (vec![MinorityInterests,], vec![])),
    ];
    static ref CASH_FLOW_PROFIT_LOSS: Vec<(CfType, (Vec<PlType>, Vec<PlType>))> = vec![
        (CashFlowInterests, (vec![InterestExpense, CostDebt], vec![])),
        (CashFlowDividends, (vec![Dividends], vec![])),
        (
            AdjustmentsRetainedEarnings,
            (vec![InterestExpense, CostDebt, Dividends], vec![])
        )
    ];
    static ref BALANCE_SHEET_CALC: HashSet<BsType> =
        BALANCE_SHEET_MAP.iter().map(|&(x, _)| x).collect();
    static ref PROFIT_LOSS_CALC: HashSet<PlType> =
        PROFIT_LOSS_MAP.iter().map(|&(x, _)| x).collect();
    static ref CASH_FLOW_CALC: HashSet<CfType> = CASH_FLOW_MAP.iter().map(|&(x, _)| x).collect();
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

impl FinType for CfType {
    fn is_calc(self) -> bool {
        CASH_FLOW_CALC.contains(&self)
    }
}

/**
FinMaps traits defines some of the common functions operating on HashMaps
representing BsMap, PlMaps, CfMap
 */
pub trait FinMaps {
    type Key: Copy + Eq + Hash;

    /**
    calc_elememts - which calculate the calculated items in a statement like
    Asset, Current Asset or Pat etc, which are calculated from other entries in
    the HashMaps
    */
    fn calc_elements(&mut self) -> &mut Self;

    /** remove_calc_elem - which removes all calculated elements from a HashMap */
    fn remove_calc_elem(&mut self) -> &mut Self;

    /** check - which checks if the particular Hashmap with the items are correct */
    fn check(&self) -> bool;

    /** common_size - use to create a common size statement */
    fn common_size(&self) -> Self;

    /** clean - removes all extraneous items in a HashMap. */
    fn clean(&mut self) -> &mut Self;

    /**
    Add value v to item k, if k does not exits then insert new item k with value v
    */
    fn add(&mut self, k: Self::Key, v: f64) -> &mut Self;

    /**
    upsert (update/insert), update value v to item k, if k does not exits
    then insert new item k with value v
    */
    fn upsert(&mut self, k: Self::Key, v: f64) -> &mut Self;

    /** add items from a vector of tuples */
    fn add_vec(&mut self, x: &Vec<(Self::Key, f64)>) -> &mut Self {
        for (k, v) in x.iter() {
            self.add(*k, *v);
        }
        self
    }

    /** upsert items from a vector of tuples */
    fn upsert_vec(&mut self, x: &Vec<(Self::Key, f64)>) -> &mut Self {
        for (k, v) in x.iter() {
            self.upsert(*k, *v);
        }
        self
    }

    /** return the value of key k */
    fn value(&self, k: Self::Key) -> f64;
}

fn calc_elem<T: Hash + Ord + Copy>(hm: &HashMap<T, f64>, x: &Vec<T>, y: &Vec<T>) -> f64 {
    x.iter().map(|k| *hm.get(k).unwrap_or(&0.0)).sum::<f64>()
        - y.iter().map(|k| *hm.get(k).unwrap_or(&0.0)).sum::<f64>()
}

impl FinMaps for BsMap {
    type Key = BsType;

    fn calc_elements(&mut self) -> &mut Self {
        for (k, (d, b)) in BALANCE_SHEET_MAP.iter() {
            self.insert(*k, calc_elem(self, d, b));
        }
        self
    }

    fn remove_calc_elem(&mut self) -> &mut Self {
        self.retain(|k, _| !k.is_calc());
        self
    }

    fn check(&self) -> bool {
        // TODO: Add implementation
        todo!()
    }

    fn common_size(&self) -> Self {
        let scale = self[&Assets];
        self.iter().map(|(k, v)| (*k, v / scale)).collect()
    }

    fn clean(&mut self) -> &mut Self {
        self.retain(|_, v| v.abs() > 1e-5);
        self
    }

    fn add(&mut self, k: Self::Key, v: f64) -> &mut Self {
        if k.is_calc() {
            panic!("Entering a calc item {:?}", k)
        } else {
            *self.entry(k).or_insert(0.0) += v;
        }
        self
    }

    fn upsert(&mut self, k: Self::Key, v: f64) -> &mut Self {
        if k.is_calc() {
            panic!("Entering a calc item {:?}", k)
        } else {
            self.insert(k, v);
        }
        self
    }

    fn value(&self, k: Self::Key) -> f64 {
        *self.get(&k).unwrap_or(&0.0)
    }
}

impl BsMapTrait for BsMap {
    fn debit(&mut self, k: BsType, v: f64) -> &mut Self {
        use BalanceSheetEntry::*;
        match DEBIT_TYPE[&k] {
            AssetEntry | LiabilityContra | EquityContra => self.add(k, v),
            _ => self.add(k, -v),
        }
    }
}

pub trait BsMapTrait {
    fn debit(&mut self, typ: BsType, val: f64) -> &mut Self;

    fn credit(&mut self, typ: BsType, val: f64) -> &mut Self {
        BsMapTrait::debit(self, typ, -val)
    }

    fn transact(&mut self, (deb, crd, val): (BsType, BsType, f64)) -> &mut Self {
        self.debit(deb, val).credit(crd, val)
    }

    fn transact_series(&mut self, trans: Vec<(BsType, BsType, f64)>) -> &mut Self {
        for x in trans {
            self.transact(x);
        }
        self
    }
}

impl FinMaps for PlMap {
    type Key = PlType;

    fn calc_elements(&mut self) -> &mut Self {
        for (k, (d, b)) in PROFIT_LOSS_MAP.iter() {
            self.insert(*k, calc_elem(self, d, b));
        }
        self
    }

    fn remove_calc_elem(&mut self) -> &mut Self {
        self.retain(|k, _| !k.is_calc());
        self
    }

    fn check(&self) -> bool {
        // TODO: Add implementation
        todo!()
    }

    fn common_size(&self) -> Self {
        let scale = self[&Revenue];
        self.iter().map(|(k, v)| (*k, v / scale)).collect()
    }

    fn clean(&mut self) -> &mut Self {
        self.retain(|_, v| v.abs() > 1e-5);
        self
    }

    fn add(&mut self, k: Self::Key, v: f64) -> &mut Self {
        if k.is_calc() {
            panic!("Entering a calc item {:?}", k)
        } else {
            *self.entry(k).or_insert(0.0) += v;
        }
        self
    }

    fn upsert(&mut self, k: Self::Key, v: f64) -> &mut Self {
        if k.is_calc() {
            panic!("Entering a calc item {:?}", k)
        } else {
            self.insert(k, v);
        }
        self
    }

    fn value(&self, k: Self::Key) -> f64 {
        *self.get(&k).unwrap_or(&0.0)
    }
}

/**
derive_cash_flow - Derives the non-calc items of the Cash Flow statement from
the beginning and ending Balance Sheets given as BsMap and Profit Loss statement
given as PlMap.

Note that you have to still run the calc_elements function to compute the full
Cash Flow statement. This function just provides an easy way to derive them of
a Balance Sheet.
 */
pub fn derive_cash_flow(bs_beg: &BsMap, bs_end: &BsMap, pl: &PlMap) -> CfMap {
    let mut cf = CfMap::new();
    for (k, (d, b)) in CASH_FLOW_PROFIT_LOSS.iter() {
        cf.insert(*k, calc_elem(pl, d, b));
    }
    for (k, (d, b)) in CASH_FLOW_BALANCE_SHEET.iter() {
        cf.insert(*k, calc_elem(bs_end, d, b) - calc_elem(bs_beg, d, b));
    }
    cf
}

impl FinMaps for CfMap {
    type Key = CfType;

    fn calc_elements(&mut self) -> &mut Self {
        for (k, (d, b)) in CASH_FLOW_MAP.iter() {
            self.insert(*k, calc_elem(self, d, b));
        }
        self
    }

    fn remove_calc_elem(&mut self) -> &mut Self {
        self.retain(|k, _| !k.is_calc());
        self
    }

    fn check(&self) -> bool {
        // TODO: Add implementation
        todo!()
    }

    fn common_size(&self) -> Self {
        let scale = self[&NetCashFlow];
        self.iter().map(|(k, v)| (*k, v / scale)).collect()
    }

    fn clean(&mut self) -> &mut Self {
        self.retain(|_, v| v.abs() > 1e-5);
        self
    }

    fn add(&mut self, k: Self::Key, v: f64) -> &mut Self {
        if k.is_calc() {
            panic!("Entering a calc item {:?}", k)
        } else {
            *self.entry(k).or_insert(0.0) += v;
        }
        self
    }

    fn upsert(&mut self, k: Self::Key, v: f64) -> &mut Self {
        if k.is_calc() {
            panic!("Entering a calc item {:?}", k)
        } else {
            self.insert(k, v);
        }
        self
    }

    fn value(&self, k: Self::Key) -> f64 {
        *self.get(&k).unwrap_or(&0.0)
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

    pub others: Option<FinOthersMap>,
}

impl Account {
    pub fn to_statements(
        &self,
    ) -> (
        Option<BalanceSheet>,
        Option<BalanceSheet>,
        Option<ProfitLoss>,
        Option<CashFlow>,
        Option<FinOthers>,
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
        bs0: &Option<BalanceSheet>,
        bs1: &Option<BalanceSheet>,
        plx: &Option<ProfitLoss>,
        cf: &Option<CashFlow>,
        ft: &Option<FinOthers>,
    ) -> Option<Self> {
        if let Some(pl) = plx {
            let bs_dt = |dt, bs: &Option<BalanceSheet>| match bs {
                Some(bsp) => (bsp.date, Some(bsp.items.clone())),
                _ => (dt, None),
            };

            let cf_dt = |dt0, dt1, cf: &Option<CashFlow>| match cf {
                Some(cp) => (cp.date_beg, cp.date_end, Some(cp.items.clone())),
                _ => (dt0, dt1, None),
            };

            let fo_dt = |dt0, dt1, fth: &Option<FinOthers>| match fth {
                Some(fo) => (fo.date_beg, fo.date_end, Some(fo.items.clone())),
                _ => (dt0, dt1, None),
            };

            let (date_beg, date_end, px) = (pl.date_beg, pl.date_end, pl.items.clone());
            let (dbt0, balance_sheet_beg) = bs_dt(date_beg, bs0);
            let (dbt1, balance_sheet_end) = bs_dt(date_end, bs1);
            let (dct0, dct1, cash_flow) = cf_dt(date_beg, date_end, cf);
            let (dft0, dft1, others) = fo_dt(date_beg, date_end, ft);
            if date_beg == dbt0
                && date_beg == dct0
                && date_beg == dft0
                && date_end == dbt1
                && date_end == dct1
                && date_end == dft1
            {
                Some(Account {
                    date_beg,
                    date_end,
                    balance_sheet_beg,
                    balance_sheet_end,
                    profit_loss: Some(px),
                    cash_flow,
                    others,
                })
            } else {
                None
            }
        } else {
            panic!("Profit Loss should not be None")
        }
    }

    pub fn calc_elements(&mut self) -> &mut Self {
        // TODO: Add implementation
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

    pub fn fin_others(&self) -> Option<FinOthers> {
        Some(FinOthers {
            date_beg: self.date_beg,
            date_end: self.date_end,
            items: (&self.others).clone()?,
        })
    }

    pub fn eps() -> f64 {
        // TODO: Add implementation of EPS
        todo!()
    }

    pub fn diluted_eps(
        _earn: f64,
        _pref_div: f64,
        _shares: f64,
        _share_price: f64,
        _options: f64,
    ) -> f64 {
        // TODO: Add implementation of Diluted EPS
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Company {
    pub code: String,
    pub link: String,
    pub affiliation: HashMap<Industry, f64>,
    pub consolidated: bool,
    pub dates: BTreeSet<NDt>,
    pub balance_sheet: HashMap<NDt, BsMap>,
    pub profit_loss: HashMap<(NDt, NDt), PlMap>,
    pub cash_flow: HashMap<(NDt, NDt), CfMap>,
    pub others: HashMap<(NDt, NDt), FinOthersMap>,
    pub share_price: BTreeMap<NDt, f64>,
    pub rate: BTreeMap<NDt, Param>,
    pub beta: BTreeMap<NDt, Param>,
}

impl Company {
    pub fn valid_date(&self, dt: NDt) -> bool {
        self.dates.contains(&dt)
    }

    pub fn date_iter(&self) -> IntoIter<NDt> {
        self.dates.clone().into_iter()
    }

    pub fn date_vec(&self) -> Vec<NDt> {
        self.date_iter().collect()
    }

    pub fn set_dates_from_profit_loss(&mut self) -> &mut Self {
        // TODO: Implement set_dates
        todo!()
    }

    pub fn get_account(&self, d0: NDt, d1: NDt) -> Option<Account> {
        if let Some(pl) = self.profit_loss.get(&(d0, d1)) {
            Some(Account {
                date_beg: d0,
                date_end: d1,
                balance_sheet_beg: Some(self.balance_sheet.get(&d0)?.clone()),
                balance_sheet_end: Some(self.balance_sheet.get(&d1)?.clone()),
                profit_loss: Some(pl.clone()),
                cash_flow: Some(self.cash_flow.get(&(d0, d1))?.clone()),
                others: Some(self.others.get(&(d0, d1))?.clone()),
            })
        } else {
            None
        }
    }

    pub fn calc_elements(&mut self) -> &mut Self {
        // TODO: Add implementation
        todo!()
    }

    pub fn transact(&mut self, _date: NDt, _deb: BsType, _crd: BsType, _x: f64) -> &mut Self {
        // TODO: Add implementation
        todo!()
    }

    pub fn to_account_vec(&self) -> Vec<Account> {
        // TODO: Add implementation
        todo!()
    }

    pub fn from_account_vec(&mut self, _ac_vec: &Vec<Account>) -> &mut Self {
        // TODO: Add implementation
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

            others: Some(HashMap::new()),
        };

        let ac_js = serde_json::to_string(&ac1).unwrap();
        let acx: Account = serde_json::from_str(&ac_js).unwrap();
        // println!("{:?} !true => {}", acx, !true);

        let bg = acx.balance_sheet_beg.clone().unwrap();

        assert!(approx(bg.value(Cash), 23.5));
        assert!(approx(bg.value(CommonStock), 0.0));

        assert!(approx(
            ac1.balance_sheet_beg.unwrap()[&Cash],
            acx.balance_sheet_beg.unwrap()[&Cash]
        ));

        let mut bs = BalanceSheet {
            date: NDt::from_ymd(2018, 3, 31),
            items: BsMap::from([(Cash, 30.45), (CurrentReceivables, 80.56)]),
        };

        bs.items
            .upsert_vec(&vec![(Cash, 24.45), (CurrentLoans, 34.56)])
            .add(WorkInProgress, 15.6)
            .add_vec(&vec![(Cash, 15.23), (RawMaterials, 87.5)])
            // .upsert(Assets, 58.5) // Calc item
            .upsert(CurrentPayables, 89.5)
            .add(RawMaterials, 12.13)
            .upsert(CurrentLoans, 22.86);

        bs.date = NDt::from_ymd(2018, 3, 30);

        assert!(approx(bs.items.value(Cash), 39.68));
        assert!(approx(bs.items.value(RawMaterials), 99.63));
        assert!(approx(bs.items.value(WorkInProgress), 15.6));
        assert!(approx(bs.items.value(CurrentPayables), 89.5));
        assert!(approx(bs.items.value(CurrentLoans), 22.86));
        assert!(approx(bs.items.value(Equity), 0.0));

        let mut pl = ProfitLoss {
            date_beg: NDt::from_ymd(2018, 3, 31),
            date_end: NDt::from_ymd(2018, 06, 30),
            items: PlMap::from([(OperatingRevenue, 58.35), (OtherExpenses, 41.58)]),
        };

        pl.items
            .upsert(OperatingRevenue, 15.76)
            .upsert_vec(&vec![(CostMaterial, 55.87), (OperatingRevenue, 88.65)])
            .add(OperatingRevenue, -22.6);

        assert!(approx(pl.items.value(OperatingRevenue), 66.05));
        assert!(approx(pl.items.value(Pat), 0.0));

        let cf = CashFlow {
            date_beg: NDt::from_ymd(2018, 3, 31),
            date_end: NDt::from_ymd(2018, 06, 30),
            items: CfMap::from([(CashFlowFinancing, 58.35), (NetCashFlow, 41.58)]),
        };

        // cf.items
        //     .upsert(CashFlowFinancing, 15.76)
        //     .upsert_vec(&vec![(NetCashFlow, 55.87), (CashFlowFinancing, 88.65)])
        //     .add(CashFlowFinancing, -22.6);

        assert!(approx(cf.items.value(CashFlowFinancing), 58.35));
        assert!(approx(cf.items.value(CashFlowInvestments), 0.0));

        // println!("{:?}", cf);

        let tx: Company =
            ron::from_str(&std::fs::read_to_string("./tatamotors.ron").unwrap()).unwrap();

        println!("{:?}", tx);
    }
}
