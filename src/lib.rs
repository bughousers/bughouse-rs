mod logic;

#[cfg(test)]
mod tests {
    use crate::logic::board::Piece;
    use crate::logic::ChessLogic;

    #[test]
    fn test_trivial() {
        assert_eq!(1,1);
    }
    fn test_rook(){
        let mut cl = ChessLogic::new();
        cl.all_empty(true);
        cl.set_piece(true,Piece::r,4,4);
        cl.print_w_legal(true,&cl.get_legal_moves(true,4,4));
    }
}