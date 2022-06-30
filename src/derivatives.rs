/*!
Implement Derivatives modules for the financelib library

Module      : financelib::derivatives <br>
Copyright   : (c) 2022 Kishaloy Neogi <br>
License     : MIT <br>
Maintainer  : Kishaloy Neogi <br>
Email       : <nkishaloy@yahoo.com>

The module describes the base modules of Derivatives like .
You may see the github repository at <https://github.com/n-kishaloy/financelib>
*/

pub mod forwards;
pub mod options;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
