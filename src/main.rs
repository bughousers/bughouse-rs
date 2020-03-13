mod logic; 
use crate::logic::ChessLogic;


fn main() {    
    //chess_board.move_piece(1,1,3,1);
    //chess_board.print_board();
    let mut chess_logic = ChessLogic::new();
    //let legals = chess_logic.get_legal_moves(true,6,7);
    //chess_logic.print_w_legal(true,&legals);
    //chess_logic.chess_board1.board[1][0] = Piece::E;
    chess_logic.testfoo();
    chess_logic.print();
    chess_logic.print_w_legal(true,&chess_logic.get_legal_moves(true,1,0));
    chess_logic.print_w_legal(true,&chess_logic.get_legal_moves(true,7,7));
    chess_logic.print_w_legal(true,&chess_logic.get_legal_moves(true,7,0));
    println!("{}",chess_logic.get_piece(true,4,5));
    chess_logic.print_w_legal(true,&chess_logic.get_legal_moves(true,4,5));
    chess_logic.print_w_legal(true,&chess_logic.get_legal_moves(true,4,2));
    chess_logic.print_w_legal(true,&chess_logic.get_legal_moves(true,4,3));
    chess_logic.print_w_legal(true,&chess_logic.get_legal_moves(true,0,1));
    chess_logic.print_w_legal(true,&chess_logic.get_legal_moves(true,7,1));
    println!("{}",chess_logic.is_attacked(true,true,6,6));
    println!("{}",chess_logic.is_attacked(true,true,7,7));
    println!("{}",chess_logic.is_attacked(true,true,3,4));
}