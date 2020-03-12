mod logic; 
use crate::logic::ChessLogic;


fn main() {    
    //chess_board.move_piece(1,1,3,1);
    //chess_board.print_board();
    let mut chess_logic = ChessLogic::new();
    //let legals = chess_logic.get_legal_moves(true,6,7);
    //chess_logic.print_w_legal(true,&legals);
    //chess_logic.chess_board1.board[1][0] = Piece::E;
    chess_logic.print_w_legal(true,&chess_logic.get_legal_moves(true,1,0));
    chess_logic.print_w_legal(true,&chess_logic.get_legal_moves(true,7,7));
    chess_logic.print_w_legal(true,&chess_logic.get_legal_moves(true,7,0));
    chess_logic.print_w_legal(true,&chess_logic.get_legal_moves(true,0,0));
}