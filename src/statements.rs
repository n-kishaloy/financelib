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

use chrono::{naive::NaiveDate as NDt, Datelike, Duration};
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
    AccumulatedAmortizationLease,
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
    AccumulatedOCI,
    MinorityInterests,
    Equity,
    BalanceSheetCheck,
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
    EBITDA,
    Depreciation,
    TaxDepreciation,
    AssetImpairment,
    LossDivestitures,
    Amortization,
    EBITX,
    InterestRevenue,
    InterestExpense,
    CostDebt,
    OtherFinancialRevenue,
    EBTX,
    ExtraordinaryItems,
    PriorYears,
    EBT,
    TaxesCurrent,
    TaxesDeferred,
    EAT,
    NetIncomeDiscontinuedOps,
    NetIncome,
    Dividends,
    ContributionRetainedEarnings,
    GainsLossesForex,
    GainsLossesActurial,
    GrossSalesPPE,
    GrossSalesLeaseRentalAssets,
    GrossSalesIntangibleAssets,
    AccAmortSalesPPE,
    AccAmortSalesLeaseRental,
    AccAmortSalesIntangible,
    SalesAmountPPE,
    SalesAmountLeaseRentalAssets,
    SalesAmountIntangibleAssets,
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
    ChangeReserves,
    AdjustmentsSalesAssets,
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
    FreeCashFlowFirm,
    CashFlowTaxShield,
    FreeCashFlowEquity,
    CashFlowDebt,
}

/**
FinOthersTyp - Enum for all Other types used in Financial statements.

This is primarily used in creating Hashmap for keeping Cash Flow items.
 */
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub enum FinOthersTyp {
    CorporateTaxRate,
    GrossProfitTaxRate,
    RevenueTaxRate,
    CurrentRatio,
    AcidRatio,
    DaysOfInventory,
    InventoryTurnoverRatio,
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

use core::{fmt, panic};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    hash::Hash,
    vec,
};

use BsType::*;
use CfType::*;
use FinOthersTyp::*;
use PlType::*;

use crate::Currency;

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
                vec![AccumulatedAmortizationLease],
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
                    AccumulatedOCI,
                    MinorityInterests,
                ],
                vec![],
            )
        ),
        (BalanceSheetCheck, (vec![Assets], vec![Liabilities, Equity]))
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
            EBITDA,
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
            EBITX,
            (
                vec![EBITDA],
                vec![
                    Depreciation,
                    AssetImpairment,
                    LossDivestitures,
                    Amortization
                ],
            )
        ),
        (
            EBTX,
            (
                vec![EBITX, InterestRevenue, OtherFinancialRevenue],
                vec![InterestExpense, CostDebt],
            )
        ),
        (EBT, (vec![EBTX], vec![ExtraordinaryItems, PriorYears],)),
        (EAT, (vec![EBT], vec![TaxesCurrent, TaxesDeferred],)),
        (NetIncome, (vec![EAT, NetIncomeDiscontinuedOps], vec![],)),
        (
            ContributionRetainedEarnings,
            (vec![NetIncome], vec![Dividends])
        ),
        (
            GainsLossesSales,
            (
                vec![
                    SalesAmountPPE,
                    SalesAmountLeaseRentalAssets,
                    SalesAmountIntangibleAssets,
                    AccAmortSalesPPE,
                    AccAmortSalesLeaseRental,
                    AccAmortSalesIntangible,
                ],
                vec![
                    GrossSalesPPE,
                    GrossSalesLeaseRentalAssets,
                    GrossSalesIntangibleAssets,
                ]
            )
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
                vec![
                    OtherCashFlowInvestments,
                    AdjustmentsSalesAssets,
                    ChangeReserves
                ],
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
        ),
        (
            FreeCashFlowEquity,
            (
                vec![
                    CashFlowOperations,
                    ChangeDebt,
                    ChangeReserves,
                    AdjustmentsSalesAssets
                ],
                vec![CashFlowInterests, ChangePPE]
            )
        ),
        (CashFlowDebt, (vec![CashFlowInterests], vec![ChangeDebt])),
        (
            FreeCashFlowFirm,
            (
                vec![FreeCashFlowEquity, CashFlowDebt],
                vec![CashFlowTaxShield]
            )
        )
    ];
    static ref CASH_FLOW_BALANCE_SHEET: Vec<(CfType, (Vec<BsType>, Vec<BsType>))> = vec![
        (
            ChangeCurrentAssets,
            (
                vec![
                    CurrentReceivables,
                    CurrentLoans,
                    CurrentAdvances,
                    OtherCurrentAssets,
                    CurrentInvestments,
                    RawMaterials,
                    WorkInProgress,
                    FinishedGoods,
                ],
                vec![],
            )
        ),
        (
            ChangeLongTermAssets,
            (
                vec![AccountReceivables, LongTermAdvances, CapitalWip,],
                vec![
                    AccumulatedDepreciation,
                    AccumulatedAmortizationLease,
                    AccumulatedAmortization,
                ],
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
        (ChangeAccumulatedOci, (vec![AccumulatedOCI,], vec![])),
        (
            ChangePPE,
            (vec![PlantPropertyEquipment, LeasingRentalAssets,], vec![])
        ),
        (
            ChangeReserves,
            (vec![RevaluationReserves, Reserves,], vec![])
        ),
        (
            InvestmentsCapDevp,
            (vec![IntangibleAssetsDevelopment], vec![],)
        ),
        (InvestmentsLoans, (vec![LongTermLoanAssets,], vec![],)),
        (ChangeEquityAssets, (vec![IntangibleAssets,], vec![],)),
        (
            ChangeInvestments,
            (vec![LongTermInvestments, Goodwill,], vec![],)
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
            (
                vec![
                    InterestExpense,
                    CostDebt,
                    Dividends,
                    AccAmortSalesPPE,
                    AccAmortSalesLeaseRental,
                    AccAmortSalesIntangible
                ],
                vec![GainsLossesSales]
            )
        ),
        (
            AdjustmentsSalesAssets,
            (
                vec![GainsLossesSales],
                vec![
                    AccAmortSalesPPE,
                    AccAmortSalesLeaseRental,
                    AccAmortSalesIntangible
                ]
            )
        ),
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
    for x in a {
        if BALANCE_SHEET_CALC.contains(x) {
            debit_mapping(debit_map, *x, calc_pos, calc_neg)
        } else {
            debit_map.insert(*x, calc_pos);
        }
    }
    for x in b {
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
    fn remove_calc_clean(&mut self) -> &mut Self;

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
        for (k, v) in x {
            self.add(*k, *v);
        }
        self
    }

    /** upsert items from a vector of tuples */
    fn upsert_vec(&mut self, x: &Vec<(Self::Key, f64)>) -> &mut Self {
        for (k, v) in x {
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
            let p = calc_elem(self, d, b);
            if p.abs() > 1e-5 {
                self.insert(*k, p);
            } else {
                self.remove(k);
            }
        }
        self
    }

    fn remove_calc_clean(&mut self) -> &mut Self {
        self.retain(|k, v| !k.is_calc() && v.abs() > 1e-5);
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
            let p = calc_elem(self, d, b);
            if p.abs() > 1e-5 {
                self.insert(*k, p);
            } else {
                self.remove(k);
            }
        }
        self
    }

    fn remove_calc_clean(&mut self) -> &mut Self {
        self.retain(|k, v| !k.is_calc() && v.abs() > 1e-5);
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

pub fn depreciation_tax_adjust(pl: &PlMap) -> f64 {
    if let Some(x) = pl.get(&TaxDepreciation) {
        pl.get(&Depreciation).unwrap_or(&0.0) - x
    } else {
        0.0
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
pub fn calc_cash_flow(
    b0: &BsMap,
    b1: &BsMap,
    pl: &PlMap,
    corp_tax: f64,
    gp_tax: f64,
    revenue_tax: f64,
) -> CfMap {
    let mut cf = CfMap::new();
    for (k, (d, b)) in CASH_FLOW_PROFIT_LOSS.iter() {
        let elem = calc_elem(pl, d, b);
        if elem.abs() > 1e-5 {
            cf.insert(*k, elem);
        }
    }
    for (k, (d, b)) in CASH_FLOW_BALANCE_SHEET.iter() {
        let elem = calc_elem(b1, d, b) - calc_elem(b0, d, b);
        if elem.abs() > 1e-5 {
            cf.insert(*k, elem);
        }
    }

    let intr = cf.get(&CashFlowInterests).unwrap_or(&0.0);
    let ebit_tx = corp_tax * (pl.get(&EBT).unwrap_or(&0.0) + intr + depreciation_tax_adjust(&pl));
    let gr_tx = gp_tax * pl.get(&GrossProfit).unwrap_or(&0.0);
    let rev_tx = revenue_tax * pl.get(&Revenue).unwrap_or(&0.0);

    cf.insert(
        CashFlowTaxShield,
        f64::min(corp_tax * intr, f64::max(0.0, ebit_tx + gr_tx - rev_tx)),
    );
    cf
}

impl FinMaps for CfMap {
    type Key = CfType;

    fn calc_elements(&mut self) -> &mut Self {
        for (k, (d, b)) in CASH_FLOW_MAP.iter() {
            let p = calc_elem(self, d, b);
            if p.abs() > 1e-5 {
                self.insert(*k, p);
            } else {
                self.remove(k);
            }
        }
        self
    }

    fn remove_calc_clean(&mut self) -> &mut Self {
        self.retain(|k, v| !k.is_calc() && v.abs() > 1e-5);
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
pub struct FinancialReport {
    pub date_beg: NDt,
    pub date_end: NDt,

    pub balance_sheet_beg: Option<BsMap>,
    pub balance_sheet_end: Option<BsMap>,
    pub profit_loss: Option<PlMap>,
    pub cash_flow: Option<CfMap>,

    pub others: Option<FinOthersMap>,
}

impl FinancialReport {
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
                Some(FinancialReport {
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
        fn get_hm<T: FinMaps + Clone>(h: &Option<T>) -> Option<T> {
            match h.clone() {
                None => None,
                Some(mut x) => Some(x.calc_elements().clone()),
            }
        }

        let pl = get_hm(&self.profit_loss).unwrap();
        let b_beg = get_hm(&self.balance_sheet_beg);
        let b_end = get_hm(&self.balance_sheet_end);
        let oth = self.others.clone().unwrap();
        let cf = match (&b_beg, &b_end) {
            (Some(bs_b), Some(bs_e)) => Some(
                calc_cash_flow(
                    &bs_b,
                    &bs_e,
                    &pl,
                    oth[&CorporateTaxRate],
                    oth[&GrossProfitTaxRate],
                    oth[&RevenueTaxRate],
                )
                .calc_elements()
                .clone(),
            ),
            (_, _) => None,
        };

        self.balance_sheet_beg = b_beg;
        self.balance_sheet_end = b_end;
        self.profit_loss = Some(pl);
        self.cash_flow = cf;

        self
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
pub struct Accounts {
    pub currency: Currency,
    pub consolidated: bool,
    pub dates: BTreeSet<NDt>,
    pub balance_sheet: BTreeMap<NDt, BsMap>,
    pub profit_loss: BTreeMap<(NDt, NDt), PlMap>,
    pub cash_flow: BTreeMap<(NDt, NDt), CfMap>,
    pub others: BTreeMap<(NDt, NDt), FinOthersMap>,
}

impl Accounts {
    pub fn valid_date(&self, dt: NDt) -> bool {
        self.dates.contains(&dt)
    }

    pub fn set_dates_from_profit_loss(&mut self) -> &mut Self {
        for (d0, d1) in self.profit_loss.keys() {
            if d0 > d1 {
                panic!("Dates are not in order ({},{})", d0, d1)
            } else {
                self.dates.insert(*d0);
                self.dates.insert(*d1);
            }
        }
        self
    }

    pub fn split_periods(&self) -> (BTreeSet<(NDt, NDt)>, BTreeSet<(NDt, NDt)>) {
        let dts = self.profit_loss.keys().into_iter();
        (
            dts.clone()
                .filter_map(|&(d0, d1)| {
                    if d0 + Duration::days(120) < d1 {
                        Some((d0, d1))
                    } else {
                        None
                    }
                })
                .collect(),
            dts.filter_map(|&(d0, d1)| {
                if d0 + Duration::days(120) > d1 {
                    Some((d0, d1))
                } else {
                    None
                }
            })
            .collect(),
        )
    }

    pub fn remove_calc_clean(&mut self) -> &mut Self {
        for v in self.balance_sheet.values_mut() {
            v.remove_calc_clean();
        }
        for v in self.profit_loss.values_mut() {
            v.remove_calc_clean();
        }
        for v in self.cash_flow.values_mut() {
            v.remove_calc_clean();
        }
        self
    }

    pub fn calc_elements(&mut self) -> &mut Self {
        for v in self.balance_sheet.values_mut() {
            v.calc_elements();
        }
        for v in self.profit_loss.values_mut() {
            v.calc_elements();
        }
        for v in self.cash_flow.values_mut() {
            v.calc_elements();
        }
        self
    }

    pub fn get_account(&self, d0: NDt, d1: NDt) -> Option<FinancialReport> {
        if let Some(pl) = self.profit_loss.get(&(d0, d1)) {
            fn get_hashmap<K: Hash + Eq + Ord, T: Hash + Clone>(
                k: K,
                h: &BTreeMap<K, HashMap<T, f64>>,
            ) -> Option<HashMap<T, f64>> {
                Some(h.get(&k)?.clone())
            }

            Some(FinancialReport {
                date_beg: d0,
                date_end: d1,
                balance_sheet_beg: get_hashmap(d0, &self.balance_sheet),
                balance_sheet_end: get_hashmap(d1, &self.balance_sheet),
                profit_loss: Some(pl.clone()),
                cash_flow: get_hashmap((d0, d1), &self.cash_flow),
                others: get_hashmap((d0, d1), &self.others),
            })
        } else {
            None
        }
    }

    pub fn calc_cash_flow(&mut self) -> &mut Self {
        for &(d0, d1) in self.profit_loss.keys() {
            match (self.balance_sheet.get(&d0), self.balance_sheet.get(&d1)) {
                (Some(b0), Some(b1)) => {
                    let oth = self.others.get(&(d0, d1)).unwrap();
                    let mut cf = calc_cash_flow(
                        b0,
                        b1,
                        self.profit_loss.get(&(d0, d1)).unwrap(),
                        oth[&CorporateTaxRate],
                        oth[&GrossProfitTaxRate],
                        oth[&RevenueTaxRate],
                    );
                    self.cash_flow.insert((d0, d1), cf.calc_elements().clone());
                }
                _ => (),
            }
        }
        self
    }

    pub fn to_account_vec(&self) -> Vec<FinancialReport> {
        self.profit_loss
            .keys()
            .map(|&(d0, d1)| self.get_account(d0, d1).unwrap())
            .collect()
    }

    pub fn from_account_vec(&mut self, _ac_vec: &Vec<FinancialReport>) -> &mut Self {
        // TODO: Add implementation
        todo!()
    }

    pub fn format_table(&self, separator: &str, mult: f64) -> String {
        let mut x = String::from("");

        fn print_statements<K: Hash + Eq + Ord + Copy, T: Hash + Copy + Eq + std::fmt::Debug>(
            s: T,
            d: &Vec<T>,
            c: &Vec<T>,
            h: &BTreeMap<K, HashMap<T, f64>>,
            lt: &BTreeSet<K>,
            mut x: String,
            mult: f64,
            separator: &str,
        ) -> String {
            fn item_exist<K: Hash + Eq + Copy, T: Hash + Copy + Eq>(
                k: T,
                h: &BTreeMap<K, HashMap<T, f64>>,
            ) -> bool {
                h.iter()
                    .map(|(_, b)| b.get(&k).unwrap_or(&0.0).abs())
                    .sum::<f64>()
                    > 0.0001
            }

            fn print_items<K: Hash + Eq + Ord + Copy, T: Hash + Copy + Eq + std::fmt::Debug>(
                k: T,
                h: &BTreeMap<K, HashMap<T, f64>>,
                lt: &BTreeSet<K>,
                mut x: String,
                mult: f64,
                separator: &str,
            ) -> String {
                if item_exist(k, h) {
                    let kx = format!("{:?}", k);
                    x = x + &format!("{:<30}", kx);
                    for &z in lt {
                        x = x + &(if let Some(w) = h.get(&z) {
                            let itm = *w.get(&k).unwrap_or(&0.0);
                            if itm < 1.0 && itm > 0.0001 {
                                format!("{separator}{:>8.1}%", itm * 100.0)
                            } else {
                                format!("{separator}{:>9.0}", itm / mult)
                            }
                        } else {
                            String::from(format!("{separator}         "))
                        });
                    }
                    x = x + &format!("\n");
                }
                x
            }

            for &k in d {
                x = print_items(k, h, lt, x, mult, separator);
            }
            for &k in c {
                x = print_items(k, h, lt, x, mult, separator);
            }
            x = print_items(s, h, lt, x, mult, separator);
            if item_exist(s, h) {
                x = x + &format!("\n");
            }
            x
        }

        x = x + &format!(
            "\nAll values in multiples of {separator}{:?}{separator}{mult}\n\n",
            self.currency
        );

        let bs_ky = self.balance_sheet.iter().map(|(&d0, _)| d0).collect();

        x = x + &format!("BALANCE SHEETS\n\n{:<30}", "Date");
        for (d0, _) in self.balance_sheet.iter() {
            x = x + &format!("{separator}  {}-{:02}", d0.year(), d0.month(),);
        }
        x = x + &format!("\n\n");
        for (s, (d, c)) in BALANCE_SHEET_MAP.iter() {
            x = print_statements(*s, d, c, &self.balance_sheet, &bs_ky, x, mult, separator);
        }

        let (pl_ann, pl_qtr) = self.split_periods();
        let d0 = pl_ann.iter().map(|(x0, _)| x0);
        let d1 = pl_ann.iter().map(|(_, x1)| x1);
        let q0 = pl_qtr.iter().map(|(x0, _)| x0);
        let q1 = pl_qtr.iter().map(|(_, x1)| x1);

        x = x + &format!("\n\nPROFIT LOSS - ANNUAL\n\n{:<30}", "Begin Date");
        for v in d0.clone() {
            x = x + &format!("{separator}  {}-{:02}", v.year(), v.month());
        }
        x = x + &format!("\n{:<30}", "End Date");
        for v in d1.clone() {
            x = x + &format!("{separator}  {}-{:02}", v.year(), v.month());
        }
        x = x + &format!("\n\n");
        for (s, (d, c)) in PROFIT_LOSS_MAP.iter() {
            x = print_statements(*s, d, c, &self.profit_loss, &pl_ann, x, mult, separator);
        }

        x = x + &format!("\n\n");
        x = print_statements(
            RevenueTaxRate,
            &vec![CorporateTaxRate, GrossProfitTaxRate],
            &vec![],
            &self.others,
            &pl_ann,
            x,
            1.0,
            separator,
        );

        x = x + &format!("\n\nCASH FLOW - ANNUAL\n\n{:<30}", "Begin Date");
        for v in d0 {
            x = x + &format!("{separator}  {}-{:02}", v.year(), v.month());
        }
        x = x + &format!("\n{:<30}", "End Date");
        for v in d1 {
            x = x + &format!("{separator}  {}-{:02}", v.year(), v.month());
        }
        x = x + &format!("\n\n");
        for (s, (d, c)) in CASH_FLOW_MAP.iter() {
            x = print_statements(*s, d, c, &self.cash_flow, &pl_ann, x, mult, separator);
        }

        if !(pl_qtr.is_empty()) {
            x = x + &format!("\n\nPROFIT LOSS - QUARTERLY\n\n{:<30}", "Begin Date");
            for v in q0.clone() {
                x = x + &format!("{separator}  {}-{:02}", v.year(), v.month());
            }
            x = x + &format!("\n{:<30}", "End Date");
            for v in q1.clone() {
                x = x + &format!("{separator}  {}-{:02}", v.year(), v.month());
            }
            x = x + &format!("\n\n");
            for (s, (d, c)) in PROFIT_LOSS_MAP.iter() {
                x = print_statements(*s, d, c, &self.profit_loss, &pl_qtr, x, mult, separator);
            }
            x = x + &format!("\n\n");
            x = print_statements(
                RevenueTaxRate,
                &vec![CorporateTaxRate, GrossProfitTaxRate],
                &vec![],
                &self.others,
                &pl_qtr,
                x,
                1.0,
                separator,
            );

            x = x + &format!("\n\nCASH FLOW - QUARTERLY\n\n{:<30}", "Begin Date");
            for v in q0 {
                x = x + &format!("{separator}  {}-{:02}", v.year(), v.month());
            }
            x = x + &format!("\n{:<30}", "End Date");
            for v in q1 {
                x = x + &format!("{separator}  {}-{:02}", v.year(), v.month());
            }
            x = x + &format!("\n\n");
            for (s, (d, c)) in CASH_FLOW_MAP.iter() {
                x = print_statements(*s, d, c, &self.cash_flow, &pl_qtr, x, mult, separator);
            }
        }
        x
    }

    pub fn calc_tax(&mut self) -> &mut Self {
        self.calc_elements();
        for y in self.profit_loss.clone().keys() {
            let pl = self.profit_loss.get(y).unwrap();
            self.put_profit_loss(*y, TaxesCurrent, {
                let ebt = pl.get(&EBT).unwrap_or(&0.0) + depreciation_tax_adjust(&pl);
                let oth = self.others.get(y).unwrap();
                let (&ct, &gt, &mt) = (
                    oth.get(&CorporateTaxRate).unwrap_or(&0.0),
                    oth.get(&GrossProfitTaxRate).unwrap_or(&0.0),
                    oth.get(&RevenueTaxRate).unwrap_or(&0.0),
                );
                f64::max(
                    ebt * ct + pl.get(&EBITDA).unwrap_or(&0.0) * gt,
                    pl.get(&Revenue).unwrap_or(&0.0) * mt,
                )
            });
        }
        self
    }

    pub fn to_csv(&self, file: &str) {
        std::fs::write(file, self.format_table(",", 1.0)).unwrap();
    }

    pub fn set_tax_rates(
        &mut self,
        corp_tax: f64,
        gross_profit_tax: f64,
        revenue_tax: f64,
    ) -> &mut Self {
        for y in self.profit_loss.keys() {
            if let Some(z) = self.others.get_mut(y) {
                z.insert(CorporateTaxRate, corp_tax);
                z.insert(GrossProfitTaxRate, gross_profit_tax);
                z.insert(RevenueTaxRate, revenue_tax);
            } else {
                self.others.insert(
                    *y,
                    HashMap::from([
                        (CorporateTaxRate, corp_tax),
                        (GrossProfitTaxRate, gross_profit_tax),
                        (RevenueTaxRate, revenue_tax),
                    ]),
                );
            }
        }
        self
    }

    pub fn calc_other(&mut self) -> &mut Self {
        // TODO: Calculate others
        todo!()
    }

    pub fn get_balance_sheet(&self, d: NDt, ty: BsType) -> f64 {
        *self.balance_sheet.get(&d).unwrap().get(&ty).unwrap_or(&0.0)
    }

    pub fn get_profit_loss(&self, d: (NDt, NDt), ty: PlType) -> f64 {
        *self.profit_loss.get(&d).unwrap().get(&ty).unwrap_or(&0.0)
    }

    pub fn get_cash_flow(&self, d: (NDt, NDt), ty: CfType) -> f64 {
        *self.cash_flow.get(&d).unwrap().get(&ty).unwrap_or(&0.0)
    }

    pub fn put_balance_sheet(&mut self, d: NDt, ty: BsType, val: f64) -> &mut Self {
        if ty.is_calc() {
            panic!("{:?} is a calculated item", ty)
        } else {
            self.balance_sheet.get_mut(&d).unwrap().insert(ty, val);
        }
        self
    }

    pub fn put_profit_loss(&mut self, d: (NDt, NDt), ty: PlType, val: f64) -> &mut Self {
        if ty.is_calc() {
            panic!("{:?} is a calculated item", ty)
        } else {
            self.profit_loss.get_mut(&d).unwrap().insert(ty, val);
        }
        self
    }

    pub fn put_cash_flow(&mut self, d: (NDt, NDt), ty: CfType, val: f64) -> &mut Self {
        if ty.is_calc() {
            panic!("{:?} is a calculated item", ty)
        } else {
            self.cash_flow.get_mut(&d).unwrap().insert(ty, val);
        }
        self
    }
}

impl fmt::Display for Accounts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mult = 1000f64.powi(
            ((std::cmp::max(
                self.balance_sheet
                    .iter()
                    .map(|(_, x)| x.value(Assets).floor() as i64)
                    .max()
                    .unwrap(),
                self.profit_loss
                    .iter()
                    .map(|(_, x)| x.value(Revenue).floor() as i64)
                    .max()
                    .unwrap(),
            ) as f64)
                .log10()
                .ceil() as i32)
                / 3
                - 2,
        );

        write!(f, "{}", self.format_table("|", mult))
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
        assert!(EBITDA.is_calc());
        assert!(!Salaries.is_calc());
        assert_eq!(DEBIT_TYPE.get(&Inventories), None);
        assert_eq!(DEBIT_TYPE.get(&RawMaterials), Some(&AssetEntry));
        assert_eq!(DEBIT_TYPE.get(&CurrentAdvances), Some(&AssetEntry));
        assert_eq!(DEBIT_TYPE.get(&NetPlantPropertyEquipment), None);
        assert_eq!(DEBIT_TYPE.get(&AccumulatedDepreciation), Some(&AssetContra));
        assert_eq!(
            DEBIT_TYPE.get(&AccumulatedAmortizationLease),
            Some(&AssetContra)
        );
        assert_eq!(DEBIT_TYPE.get(&LongTermLiabilities), None);
        assert_eq!(DEBIT_TYPE.get(&BondsPayable), Some(&LiabilityEntry));
        assert_eq!(DEBIT_TYPE.get(&Equity), None);
        assert_eq!(DEBIT_TYPE.get(&MinorityInterests), Some(&EquityEntry));
    }

    #[test]
    fn account_check() {
        let ac1 = FinancialReport {
            date_beg: NDt::from_ymd(2009, 05, 22),
            date_end: NDt::from_ymd(2010, 09, 27),

            balance_sheet_beg: Some(HashMap::from([(Cash, 23.5), (Equity, 12.5)])),
            balance_sheet_end: None,

            profit_loss: Some(HashMap::from([
                (Revenue, -2.58),
                (EAT, 24.8),
                (EBITX, 11.3),
            ])),
            cash_flow: None,

            others: Some(HashMap::new()),
        };

        let ac_js = serde_json::to_string(&ac1).unwrap();
        let acx: FinancialReport = serde_json::from_str(&ac_js).unwrap();
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
        assert!(approx(pl.items.value(EAT), 0.0));

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

        let mut tx: Accounts =
            ron::from_str(&std::fs::read_to_string("./testdocs/tatamotors.ron").unwrap()).unwrap();

        tx.set_dates_from_profit_loss().remove_calc_clean();

        assert_eq!(
            tx.get_account(NDt::from_ymd(2018, 6, 1), NDt::from_ymd(2018, 9, 1))
                .unwrap()
                .balance_sheet_beg,
            None
        );

        tx.calc_elements().set_tax_rates(0.1, 0.02, 0.005);
        assert_eq!(
            (tx.get_account(NDt::from_ymd(2018, 3, 1), NDt::from_ymd(2019, 3, 1))
                .unwrap()
                .balance_sheet_beg
                .unwrap())[&Inventories],
            82172000000.0
        );
        // println!("{:?}\n\n\n", tx.split_periods());
        // println!("{}", tx);
        tx.calc_cash_flow();
        // println!("{}", tx);

        assert!(approx(
            tx.get_cash_flow(
                (NDt::from_ymd(2013, 3, 1), NDt::from_ymd(2014, 03, 1)),
                NetCashFlow
            ),
            8599e+6
        ));

        // tx.put_balance_sheet(NDt::from_ymd(2013, 3, 1), CurrentAssets, 50000.0);
        // This fails as CurrentAssets is a calculated item

        assert!(approx(
            tx.get_balance_sheet(NDt::from_ymd(2012, 3, 1), CurrentReceivables),
            8237000000.0
        ));

        tx.put_balance_sheet(NDt::from_ymd(2012, 3, 1), CurrentReceivables, 512_000_000.0)
            .put_balance_sheet(NDt::from_ymd(2013, 3, 1), FinishedGoods, 256_000_000.0)
            .put_balance_sheet(NDt::from_ymd(2013, 3, 1), RawMaterials, 600_000_000.0);

        assert!(approx(
            tx.get_balance_sheet(NDt::from_ymd(2012, 3, 1), CurrentReceivables),
            512000000.0
        ));

        assert!(approx(
            tx.get_balance_sheet(NDt::from_ymd(2013, 3, 1), RawMaterials),
            600000000.0
        ));

        tx.put_balance_sheet(NDt::from_ymd(2013, 3, 1), RawMaterials, 0.0);

        tx.put_balance_sheet(
            NDt::from_ymd(2012, 3, 1),
            CurrentReceivables,
            8237_000_000.0,
        )
        .put_balance_sheet(NDt::from_ymd(2013, 3, 1), FinishedGoods, 21037_000_000.0);

        assert!(approx(
            tx.get_balance_sheet(NDt::from_ymd(2012, 3, 1), CurrentReceivables),
            8237000000.0
        ));

        assert!(approx(
            tx.get_balance_sheet(NDt::from_ymd(2013, 3, 1), FinishedGoods),
            21037000000.0
        ));

        assert!(approx(
            tx.get_balance_sheet(NDt::from_ymd(2012, 3, 1), Assets),
            142767000000.0
        ));

        assert!(approx(
            tx.get_profit_loss((NDt::from_ymd(2019, 3, 1), NDt::from_ymd(2019, 6, 1)), EAT),
            -3698000000.0
        ));

        assert!(approx(
            tx.get_cash_flow(
                (NDt::from_ymd(2019, 3, 1), NDt::from_ymd(2020, 3, 1)),
                CashFlowOperations
            ),
            26276000000.0
        ));

        tx.put_profit_loss(
            (NDt::from_ymd(2012, 3, 1), NDt::from_ymd(2013, 3, 1)),
            InterestExpense,
            13560e+6,
        )
        .put_cash_flow(
            (NDt::from_ymd(2011, 3, 1), NDt::from_ymd(2012, 3, 1)),
            AdjustmentsRetainedEarnings,
            22982e+6,
        );

        assert!(approx(
            tx.get_profit_loss(
                (NDt::from_ymd(2012, 3, 1), NDt::from_ymd(2013, 3, 1)),
                InterestExpense
            ),
            13560e+6
        ));

        assert!(approx(
            tx.get_cash_flow(
                (NDt::from_ymd(2011, 3, 1), NDt::from_ymd(2012, 3, 1)),
                AdjustmentsRetainedEarnings
            ),
            22982e+6
        ));

        tx.put_profit_loss(
            (NDt::from_ymd(2012, 3, 1), NDt::from_ymd(2013, 3, 1)),
            InterestExpense,
            3560e+6,
        )
        .put_cash_flow(
            (NDt::from_ymd(2011, 3, 1), NDt::from_ymd(2012, 3, 1)),
            AdjustmentsRetainedEarnings,
            2982e+6,
        );

        assert!(approx(
            tx.get_cash_flow(
                (NDt::from_ymd(2014, 3, 1), NDt::from_ymd(2015, 3, 1)),
                NetCashFlow
            ),
            2404e+6
        ));

        // println!("{}", tx);

        // std::fs::write("./testdocs/tms.ron", ron::to_string(&tx).unwrap()).unwrap();
        // tx.to_csv("./testdocs/tata.csv");
    }
}
