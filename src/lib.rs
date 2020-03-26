#![allow(warnings)] 
pub mod logic;
pub mod parse;
pub mod infoCourier;
pub mod util;

#[cfg(test)]
mod tests_mod {
    use super::*;
    use crate::logic::board::Piece;
    use crate::logic::ChessLogic;
    use crate::parse::parser;
    use crate::infoCourier::infoCourier::*;
    use crate::util::contains;

    #[test]
    fn trivial() {
        assert_eq!(1,1);
    }

}