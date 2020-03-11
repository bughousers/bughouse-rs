mod logic; 
use crate::logic::ChessLogic;

fn main() {    
    //chess_board.move_piece(1,1,3,1);
    //chess_board.print_board();
    let mut chess_logic = ChessLogic::new();
    chess_logic.print();
    chess_logic.get_legal_moves(6,1);
}