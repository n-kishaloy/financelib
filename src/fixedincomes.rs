/*!
Module      : FinanceLib.FixedIncomes
Description : Implement Fixed Incomes modules for the FinanceLib library
Copyright   : (c) 2022 Kishaloy Neogi
License     : MIT
Maintainer  : Kishaloy Neogi
Email       : nkishaloy@yahoo.com

The module describes the base modules of FixedIncomes.

You may see the github repository at <https://github.com/n-kishaloy/financelib>
*/

pub mod bonds;
pub mod moneymarkets;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
