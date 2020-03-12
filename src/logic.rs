mod board;
use crate::logic::board::ChessBoard;
use crate::logic::board::Piece;
use std::cmp;

pub struct ChessLogic {
    pub chess_board1: ChessBoard,
    pub chess_board2: ChessBoard,
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
        let mut vec = Vec::new();
        for &(i,j) in locs.iter() {
            if self.chess_board1.board[i][j] == Piece::E {
                self.chess_board1.board[i][j] = Piece::L;
            }else{
                vec.push((i,j,self.chess_board1.board[i][j]));
                self.chess_board1.board[i][j] = Piece::L;
            }
        }
        self.chess_board1.print_board();
        for &(i,j) in locs.iter() {
            self.chess_board1.board[i][j] = Piece::E;
        }
        for &(i,j,t) in vec.iter() {
            self.chess_board1.board[i][j] = t;
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
            Piece::R => self.cross_mov(board1,old_i,old_j),
            Piece::r => self.cross_mov(board1,old_i,old_j),
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
        if white {
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
        }else {
            match self.chess_board1.board[i][j] {
                Piece::E => false,
                Piece::L => false,
                Piece::p => false,
                Piece::r => false,
                Piece::n => false,
                Piece::b => false,
                Piece::q => false,
                Piece::k => false,
                _ => true,

            }
        }
    }

    fn is_white(&self, board1:bool, i:usize, j:usize) -> bool {
        match self.chess_board1.board[i][j] {
            Piece::E => false,
            Piece::L => false,
            Piece::p => false,
            Piece::r => false,
            Piece::n => false,
            Piece::b => false,
            Piece::q => false,
            Piece::k => false,
            _ => true,
        }
    }

    fn horizontal_mov(&self,board1:bool, i:usize, j:usize) -> Vec<(usize,usize)> {
        let mut vec = Vec::new();
        let mut jc = j;
        if jc < 7 {
            jc+= 1;
            while jc <= 7 && (self.is_empty(board1,i,jc) ||
             self.is_enemy(board1,self.is_white(board1,i,j),i,jc)){
                vec.push((i,jc));
                jc+= 1;

                if self.is_enemy(board1,self.is_white(board1,i,j),i,jc-1) {
                    jc = 8;
                }
            }
        }
        if jc > 0 {
            jc-= 1;
            while jc > 0 && (self.is_empty(board1,i,jc) || 
            self.is_enemy(board1,self.is_white(board1,i,j),i,jc)){
                vec.push((i,jc));
                jc-= 1;

                if self.is_enemy(board1,self.is_white(board1,i,j),i,jc+1) {
                    jc = 0;
                }
            }
            if self.is_empty(board1,i,0) {
                vec.push((i,0));
            }
        }
        vec
    }

    fn vertical_mov(&self,board1:bool, i:usize, j:usize)  -> Vec<(usize,usize)>{
        let mut vec = Vec::new();
        let mut ic = i;
        if ic < 7 {
            ic+= 1;
            while ic <= 7 && (self.is_empty(board1,ic,j) || 
            self.is_enemy(board1,self.is_white(board1,i,j),ic,j)) {
                vec.push((ic,j));
                ic+= 1;
                if self.is_enemy(board1,self.is_white(board1,i,j),ic-1,j) {
                    ic = 8;
                }
            }
        }
        if ic > 0 {
            ic-= 1;
            while ic > 0 && (self.is_empty(board1,ic,j) || 
            self.is_enemy(board1,self.is_white(board1,i,j),ic,j)) {
                vec.push((ic,j));
                ic-= 1;

                if self.is_enemy(board1,self.is_white(board1,i,j),ic+1,j) {
                    ic = 0;
                }
            }
            if self.is_empty(board1,0,j) {
                vec.push((0,j));
            }
        }
        vec
    }

    fn cross_mov(&self,board1:bool, i:usize, j:usize) -> Vec<(usize,usize)> {
        let mut vec = self.vertical_mov(board1,i,j);
        vec.append(&mut self.horizontal_mov(board1,i,j));
        vec
    }

    fn x_mov(&self, i:usize, j:usize)  -> Vec<(usize,usize)>{
        Vec::new()
    }
}