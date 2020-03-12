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

    pub fn print_w_legal(&mut self,board1:bool,locs: &Vec<(usize,usize)>){
        for &(i,j) in locs.iter() {
            self.chess_board1.board[i][j] = Piece::L;
        }
        self.chess_board1.print_board();
        for &(i,j) in locs.iter() {
            self.chess_board1.board[i][j] = Piece::E;
        }
    }

    pub fn new() -> ChessLogic {
        ChessLogic{
            chess_board1: ChessBoard::new(),
            chess_board2: ChessBoard::new(),
            unmoved_black_pawns: [(0,true); 8],
            unmoved_white_pawns: [(0,true); 8],
        }
    }
    pub fn get_legal_moves(&self,board1:bool, old_i:usize, old_j:usize)
    -> Vec<(usize,usize)>
    {
        match self.chess_board1.board[old_i][old_j] {
            Piece::P => {
                match self.unmoved_white_pawns[old_j] {
                    (0,true) =>  {
                        let mut vec = Vec::new();
                        if self.is_empty(board1,old_i-1,old_j) {
                            vec.push((old_i-1,old_j));
                            if self.is_empty(board1,old_i-2,old_j) {
                                vec.push((old_i-2,old_j));
                            }
                        }
                        if old_j > 0 && self.is_enemy(board1,true,old_i-1,old_j-1) {
                            vec.push((old_i-1,old_j-1));
                        }
                        if old_j < 7 && self.is_enemy(board1,true,old_i-1,old_j+1) {
                            vec.push((old_i-1,old_j+1));
                        }
                        vec
                    },
                    (_,_) => {
                        let mut vec = Vec::new();
                        if self.is_empty(board1,old_i-1,old_j) {
                            vec.push((old_i-1,old_j));
                        }
                        if old_j > 0 && self.is_enemy(board1,true,old_i-1,old_j-1) {
                            vec.push((old_i-1,old_j-1));
                        }
                        if old_j < 7 && self.is_enemy(board1,true,old_i-1,old_j+1) {
                            vec.push((old_i-1,old_j+1));
                        }
                        vec
                    },
                }
            },
            Piece::p => {
                match self.unmoved_black_pawns[old_j] {
                    (0,true) =>  {
                        let mut vec = Vec::new();
                        if self.is_empty(board1,old_i+1,old_j) {
                            vec.push((old_i+1,old_j));
                            if self.is_empty(board1,old_i+2,old_j) {
                                vec.push((old_i+2,old_j));
                            }
                        }
                       
                        if old_j > 0 && self.is_enemy(board1,false,old_i+1,old_j-1) {
                            vec.push((old_i+1,old_j+1));
                        }
                        if old_j < 7 && self.is_enemy(board1,false,old_i+1,old_j+1) {
                            vec.push((old_i-1,old_j+1));
                        }
                        vec
                    },
                    (_,_) => {
                        let mut vec = Vec::new();
                        if self.is_empty(board1,old_i+1,old_j) {
                            vec.push((old_i+1,old_j));
                        }
                        if old_j > 0 && self.is_enemy(board1,false,old_i+1,old_j-1) {
                            vec.push((old_i+1,old_j-1));
                        }
                        if old_j < 7 && self.is_enemy(board1,false,old_i+1,old_j+1) {
                            vec.push((old_i+1,old_j+1));
                        }
                        vec
                    },
                }
            },
            _ => Vec::new(),
        }
    }

    fn is_empty(&self, board1:bool, i:usize, j:usize) -> bool {
        match self.chess_board1.board[i][j] {
            Piece::E => true, 
            _ => false,
        }
    }

    fn is_enemy(&self, board1:bool,white:bool, i:usize, j:usize) -> bool {
        match self.chess_board1.board[i][j] {
            Piece::E => false,
            Piece::L => false,
            Piece::P => false,
            Piece::R => false,
            Piece::N => false,
            Piece::B => false,
            Piece::Q => false,
            Piece::K => false,
            _ => true,

        }
    }

    fn vertical_mov(&self, i:usize, j:usize){
        
    }

    fn horizontal_mov(&self, i:usize, j:usize){

    }

    fn cross_mov(&self, i:usize, j:usize){

    }
}