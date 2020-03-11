mod board;
use crate::logic::board::ChessBoard;
use crate::logic::board::Piece;
use std::cmp;

pub struct ChessLogic {
    chess_board1: ChessBoard,
    chess_board2: ChessBoard,
    //tuple of start point offset and if bool to indicate 
    //if it has moved
    unmoved_black_pawns: [(usize,bool); 8],
    unmoved_white_pawns: [(usize,bool); 8],
}

impl ChessLogic {
    pub fn print(&self){
        self.chess_board1.print_board();
    }

    pub fn print_w_legal(&self, x: Vec<(usize,usize)>){

    }

    pub fn new() -> ChessLogic {
        ChessLogic{
            chess_board1: ChessBoard::new(),
            chess_board2: ChessBoard::new(),
            unmoved_black_pawns: [(0,true); 8],
            unmoved_white_pawns: [(0,true); 8],
        }
    }
    pub fn get_legal_moves(&self,old_i:usize, old_j:usize)
    -> Vec<(usize,usize)>
    {
        match self.chess_board1.board[old_i][old_j] {
            Piece::P => {
                match self.unmoved_white_pawns[old_j] {
                    (0,true) =>  {
                        let mut vec = Vec::new();
                        vec.push((cmp::max(old_i-1,0),old_j));
                        println!("legal to go to {}, {}",old_i-1,old_j);
                        vec.push((cmp::max(old_i-2,0),old_j));
                        println!("legal to go to {}, {}",old_i-2,old_j);
                        vec
                    },
                    (_,_) => {
                        let mut vec = Vec::new();
                        vec.push((cmp::max(old)))
                    },
                }
            },
            _ => Vec::new(),
        }
    }
}