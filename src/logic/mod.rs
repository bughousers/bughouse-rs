mod board;
use crate::logic::board::ChessBoard;
use crate::logic::board::Piece;
use std::cmp;

pub struct ChessLogic {
    pub chess_board1: ChessBoard,
    pub chess_board2: ChessBoard,
    //tuple of start point offset and if bool to indicate 
    //if it has moved
}

impl ChessLogic {
    pub fn print(&self){
        self.chess_board1.print_board();
    }

    pub fn testfoo(&mut self) {
        self.chess_board1.board[5][4] = Piece::P;
        self.chess_board1.board[5][5] = Piece::p;
        self.chess_board1.board[4][2] = Piece::R;
        self.chess_board1.board[4][3] = Piece::q;
        self.chess_board1.board[1][7] = Piece::E;
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
        }
    }
    pub fn get_legal_moves(&self,board1:bool, old_i:usize, old_j:usize)
    -> Vec<(usize,usize)>
    {
        match self.chess_board1.board[old_i][old_j] {
            Piece::P => {
                match (old_i,old_j) {
                    //unmoved (Double move)
                    (6,_) =>  {
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
                    //6th lane with en passant
                    (3,_) => {
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
                        //en passant
                        if old_j > 0 && self.is_enemy(board1,true,old_i,old_j-1) 
                        && self.get_piece(board1,old_i,old_j-1) == Piece::P {
                            vec.push((old_i-1,old_j-1));
                        }
                        if old_j < 7 && self.is_enemy(board1,true,old_i,old_j+1)
                        && self.get_piece(board1,old_i,old_j+1) == Piece::P {
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
                match (old_i,old_j) {
                    (1,_) =>  {
                        let mut vec = Vec::new();
                        if self.is_empty(board1,old_i+1,old_j) {
                            vec.push((old_i+1,old_j));
                            if self.is_empty(board1,old_i+2,old_j) {
                                vec.push((old_i+2,old_j));
                            }
                        }
                       
                        if old_j > 0 && self.is_enemy(board1,false,old_i+1,old_j-1) {
                            vec.push((old_i+1,old_j-1));
                        }
                        if old_j < 7 && self.is_enemy(board1,false,old_i+1,old_j+1) {
                            vec.push((old_i+1,old_j+1));
                        }
                        vec
                    },
                    //4th line with enpassant 
                    (5,_) => {
                            let mut vec = Vec::new();
                            if self.is_empty(board1,old_i+1,old_j) {
                                vec.push((old_i+1,old_j));
                            }
                            if old_j > 0 && self.is_enemy(board1,true,old_i+1,old_j-1) {
                                vec.push((old_i+1,old_j-1));
                            }
                            if old_j < 7 && self.is_enemy(board1,true,old_i+1,old_j+1) {
                                vec.push((old_i+1,old_j+1));
                            }
                            //en passant
                            if old_j > 0 && self.is_enemy(board1,false,old_i,old_j-1) 
                            && self.get_piece(board1,old_i,old_j-1) == Piece::P {
                                vec.push((old_i+1,old_j-1));
                            }
                            if old_j < 7 && self.is_enemy(board1,false,old_i,old_j+1)
                            && self.get_piece(board1,old_i,old_j+1) == Piece::P {
                                vec.push((old_i+1,old_j+1));
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
            Piece::R | Piece::r => self.cross_mov(board1,old_i,old_j),
            Piece::B | Piece::b => self.x_mov(board1,old_i,old_j),
            Piece::Q | Piece::q => 
            { let mut vec = self.cross_mov(board1,old_i,old_j);
             vec.append(&mut self.x_mov(board1,old_i,old_j));
             vec },
            Piece::N | Piece::n => self.horse_jump(board1,old_i,old_j),
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
        while jc > 0 {
            jc -= 1;
            if self.is_empty(board1,i,jc) {
                vec.push((i,jc));
            }else {
                if self.is_enemy(board1,self.is_white(board1,i,j),i,jc) {
                    vec.push((i,jc));
                }
                break;
            }
        }
        jc = j;
        while jc < 7 {
            jc += 1;
            if self.is_empty(board1,i,jc) {
                vec.push((i,jc));
            }else {
                if self.is_enemy(board1,self.is_white(board1,i,j),i,jc) {
                    vec.push((i,jc));
                }
                break;
            }
        }
        vec
    }

    fn vertical_mov(&self,board1:bool, i:usize, j:usize)  -> Vec<(usize,usize)>{
        let mut vec = Vec::new();
        let mut ic = i;
        while ic > 0 {
            ic -= 1;
            if self.is_empty(board1,ic,j) {
                vec.push((ic,j));
            }else {
                if self.is_enemy(board1,self.is_white(board1,i,j),ic,j) {
                    vec.push((ic,j));
                }
                break;
            }
        }
        ic = i;
        while ic < 7 {
            ic += 1;
            if self.is_empty(board1,ic,j) {
                vec.push((ic,j));
            }else {
                if self.is_enemy(board1,self.is_white(board1,i,j),i,j) {
                    vec.push((ic,j));
                }
                break;
            }
        }
        vec
    }

    fn cross_mov(&self,board1:bool, i:usize, j:usize) -> Vec<(usize,usize)> {
        let mut vec = self.vertical_mov(board1,i,j);
        vec.append(&mut self.horizontal_mov(board1,i,j));
        vec
    }

    fn x_mov(&self,board1:bool, i:usize, j:usize)  -> Vec<(usize,usize)>{
        let mut vec = Vec::new();
        let mut ic = i;
        let mut jc = j;

        //lower right
        while ic < 8 && jc < 8 {
            ic+=1;
            jc+=1;
            if self.is_empty(board1,ic,jc) {
                vec.push((ic,jc));
            }else{
                if self.is_enemy(board1,self.is_white(board1,i,j),ic,jc){
                    vec.push((ic,jc));
                }
                break;
            }
        }
        //upper right
        ic=i;
        jc=j;
        while ic > 0 && jc < 8 {
            ic-=1;
            jc+=1;
            if self.is_empty(board1,ic,jc) {
                vec.push((ic,jc));
            }else{
                if self.is_enemy(board1,self.is_white(board1,i,j),ic,jc){
                    vec.push((ic,jc));
                }
                break;
            }
        }
        //upper left 
        ic=i;
        jc=j;
        while ic > 0 && jc > 0 {
            ic-=1;
            jc-=1;
            if self.is_empty(board1,ic,jc) {
                vec.push((ic,jc));
            }else{
                if self.is_enemy(board1,self.is_white(board1,i,j),ic,jc){
                    vec.push((ic,jc));
                }
                break;
            }
        }
        //lower left
        ic=i;
        jc=j;
        while ic < 8 && jc > 0 {
            ic+=1;
            jc-=1;
            if self.is_empty(board1,ic,jc) {
                vec.push((ic,jc));
            }else{
                if self.is_enemy(board1,self.is_white(board1,i,j),ic,jc){
                    vec.push((ic,jc));
                }
                break;
            }
        }
        vec
    }

    fn horse_jump(&self, board1:bool, i:usize, j:usize) -> Vec<(usize,usize)> {
        //all possible combinations of +2, -2, +1, -1 for each square
        let twoway = [-2,2];
        let oneway = [1,-1];
        let mut vec = Vec::new();

        for i_off in twoway.iter() {
            for j_off in oneway.iter() {
                let mut i_signed = i as i32;
                let mut j_signed = j as i32;
                i_signed += i_off;
                j_signed += j_off;
                if i_signed >= 0 && i_signed <= 7 &&
                j_signed >= 0 && j_signed <= 7 {
                    let itmp = i_signed as usize;
                    let jtmp = j_signed as usize;
                    if self.is_empty(board1,itmp,jtmp) || 
                    self.is_enemy(board1,self.is_white(board1,i,j),itmp,jtmp) {
                        vec.push((itmp,jtmp));
                    }
                   
                }
            }
        }
        for i_off in oneway.iter() {
            for j_off in twoway.iter() {
                let mut i_signed = i as i32;
                let mut j_signed = j as i32;
                i_signed += i_off;
                j_signed += j_off;
                if i_signed >= 0 && i_signed <= 7 &&
                j_signed >= 0 && j_signed <= 7 {
                    let itmp = i_signed as usize;
                    let jtmp = j_signed as usize;
                    if self.is_empty(board1,itmp,jtmp) || 
                    self.is_enemy(board1,self.is_white(board1,i,j),itmp,jtmp) {
                        vec.push((itmp,jtmp));
                    }
                }
            }
        }
        vec
    }

    pub fn get_piece(&self, chessboard1:bool, i:usize, j:usize) -> Piece {
        match chessboard1 {
            true => self.chess_board1.board[i][j],
            false => self.chess_board2.board[i][j],
        }
    }

    //iswhite <=> the unit on the square is white
    pub fn is_attacked(&self, board1:bool, iswhite:bool, i:usize,j:usize) -> bool {
        //check for pawns

        //check for rook|queen on vertical

        //check for rook|queen on horizontal

        //check for bishop on x

        //check for horses

        //check for enemy king 

        let mut ic = (i as i32)-1;
        let mut jc = (j as i32)-1;
        if iswhite{
            //check for pawns
            if self.valid(ic,jc) {
                let a = ic as usize;
                let b = jc as usize;
                if board1 {
                    if self.chess_board1.board[a][b]==Piece::p
                    && self.is_enemy(true, true, a,b){
                        return true
                    }
                }else{
                    if self.chess_board2.board[a][b]==Piece::p
                    && self.is_enemy(false, true, a,b){
                        return true
                    }
                } 
            }
            jc += 2;
            if self.valid(ic,jc) {
                let a = ic as usize;
                let b = jc as usize;
                if board1 {
                    if self.chess_board1.board[a][b]==Piece::p
                    && self.is_enemy(true, true, a,b){
                        return true
                    }
                }else{
                    if self.chess_board2.board[a][b]==Piece::p
                    && self.is_enemy(false, true, a,b){
                        return true
                    }
                } 
            }
            //check for horizontal line
            jc = j as i32;
            ic = i as i32;
            let mut jx = jc+1;
            while jx < 8 {
                if self.check_for_piece(board1,Piece::r,ic,jx) || 
                self.check_for_piece(board1,Piece::q,ic,jx) {
                    return true
                }else if !self.check_for_piece(board1,Piece::E, ic,jx){
                    break;
                }
                jx+=1;
            }
            jx = jc-1;
            while jx >= 0{
                if self.check_for_piece(board1,Piece::r,ic,jx) || 
                self.check_for_piece(board1,Piece::q,ic,jx) {
                    return true
                }else if !self.check_for_piece(board1,Piece::E, ic,jx){
                    break;
                }
                jx -=1;
            }
            //check for vertical
            let mut ix = ic+1;
            while ix <8 {
                if self.check_for_piece(board1,Piece::r,ix,jc) || 
                self.check_for_piece(board1,Piece::q,ix,jc) {
                    return true
                }else if !self.check_for_piece(board1,Piece::E, ix,jc){
                    break;
                }
                ix+=1;
            }
            ix = ic-1;
            while ix >=0 {
                if self.check_for_piece(board1,Piece::r,ix,jc) || 
                self.check_for_piece(board1,Piece::q,ix,jc) {
                    return true
                }else if !self.check_for_piece(board1,Piece::E, ix,jc){
                    break;
                }
                ix-=1;
            }
            //check for capraz
            ic=i as i32;
            jc=j as i32;
            //lower right
            while ic < 7 && jc < 7 {
                ic+=1;
                jc+=1;
                if self.check_for_piece(board1,Piece::q,ic,jc) ||
                self.check_for_piece(board1,Piece::b,ic,jc) {
                    return true
                }else if !self.check_for_piece(board1,Piece::E, ic,jc){
                    break;
                }
                
            }
            //upper right
            ic=i as i32;
            jc=j as i32;
            while ic > 0 && jc < 7 {
                ic-=1;
                jc+=1;
                if self.check_for_piece(board1,Piece::q,ic,jc) ||
                self.check_for_piece(board1,Piece::b,ic,jc) {
                    return true
                }else if !self.check_for_piece(board1,Piece::E, ic,jc){
                    break;
                }
                
            }
            //upper left 
            ic=i as i32;
            jc=j as i32;
            while ic > 0 && jc > 0 {
                ic-=1;
                jc-=1;
                if self.check_for_piece(board1,Piece::q,ic,jc) ||
                self.check_for_piece(board1,Piece::b,ic,jc) {
                    return true
                }else if !self.check_for_piece(board1,Piece::E, ic,jc){
                    break;
                }
                
            }
            //lower left
            ic=i as i32;
            jc=j as i32;
            while ic < 7 && jc > 0 {
                ic+=1;
                jc-=1;
                if self.check_for_piece(board1,Piece::q,ic,jc) ||
                self.check_for_piece(board1,Piece::b,ic,jc) {
                    return true
                }else if !self.check_for_piece(board1,Piece::E, ic,jc){
                    break;
                }
            
            }
   
        }
        false
    }

    fn valid(&self, a: i32, b: i32) -> bool {
        if a >= 0 && a <= 7 && b >= 0 && b<= 7 {
            return true
        }else {
            return false
        }
    }

    fn check_for_piece(&self,board1:bool, piece:Piece, i : i32, j:i32) -> bool {
        if board1 {
            if self.chess_board1.board[i as usize][j as usize]==piece
            {
                return true
            }else {
                return false
            }
        }else{
            if self.chess_board2.board[i as usize][j as usize]==piece
            {
                return true 
            }else {
                return false
            }
        }

    }

    pub fn all_empty(&mut self,board1:bool) {
        for i in 0..8 {
            for j in 0..8 {
                if board1{
                    self.chess_board1.board[i][j] = Piece::E;
                }else{
                    self.chess_board2.board[i][j] = Piece::E;
                }
                
            }
        }
    }

    pub fn set_piece(&mut self,board1:bool,piece:Piece,i:usize,j:usize){
        if board1{
            self.chess_board1.board[i][j] = piece;
        }else{
            self.chess_board2.board[i][j] = piece;
        }
    }



}

