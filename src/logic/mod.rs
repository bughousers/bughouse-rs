pub mod board;
use crate::logic::board::ChessBoard;
use crate::logic::board::Piece;
use std::cmp;

pub struct ChessLogic {
    pub chess_board1: ChessBoard,
    pub chess_board2: ChessBoard,
    //tuple of start point offset and if bool to indicate 
    //if it has moved
    pawn_in_last_turn_b1: Option<(usize,usize)>,
    pawn_in_last_turn_b2: Option<(usize,usize)>,
    pub white_active_1: bool,
    pub white_active_2: bool,
    //boardX_color_capture ==
    //board1 or 2, the captured piece has color <color>
    //the order is P-R-N-B-Q
    pub upgrade_to1: Piece,
    pub upgrade_to2: Piece,
    half_moves_last_capture1: usize,
    half_moves_last_capture2: usize,
    movectr1: usize,
    movectr2: usize,
    board1_white_capture: [u8;5], //pieces that can be deployed on board1 (white pieces)
    board1_black_capture: [u8;5],
    board2_white_capture: [u8;5],
    board2_black_capture: [u8;5],
}

impl ChessLogic {
    pub fn print(&self,board1:bool){
        if board1{
            self.chess_board1.print_board();
            println!("------------------------");
        }else{
            self.chess_board2.print_board();
            println!("------------------------");
        }

    }

    pub fn get_white_active(&self, board1:bool) -> bool{
        match board1 {
            true => self.white_active_1,
            false => self.white_active_2,
        }
    }

    pub fn get_movectr(&self, board1:bool) -> usize {
        match board1 {
            true => self.movectr1,
            false => self.movectr2,
        }
    }

    pub fn get_half_moves(&self, board1:bool) -> usize {
        match board1 {
            true => self.half_moves_last_capture1,
            false => self.half_moves_last_capture2,
        }
    }

    pub fn get_captured_piece(&self, board1:bool,white:bool, i:usize) -> u8 {
        if board1 {
            if white { 
                self.board1_white_capture[i]
            }else{
                self.board1_black_capture[i]
            }
        }else{
            if white {
                self.board2_white_capture[i]
            }else{
                self.board2_black_capture[i]
            }
        }
    }

    pub fn get_pawn_in_last_turn(&self, board1:bool) -> Option<(usize,usize)> {
        match board1 {
            true => self.pawn_in_last_turn_b1,
            false => self.pawn_in_last_turn_b2,
        }
    }

    //only for test
    pub fn set_pawn_in_last_turn(&mut self, board1:bool, x:Option<(usize,usize)>) {
        match board1 {
            true => self.pawn_in_last_turn_b1 = x,
            false => self.pawn_in_last_turn_b2 = x,
        }
    }

    pub fn print_w_legal(&mut self,board1:bool,locs: &Vec<(usize,usize)>){
        let mut vec = Vec::new();
        for &(i,j) in locs.iter() {
            if self.get_piece(board1,i,j )== Piece::E {
                
                if board1 {
                    self.chess_board1.board[i][j] = Piece::L;
                }else{
                    self.chess_board2.board[i][j] = Piece::L;
                }
            }else{
                vec.push((i,j,self.get_piece(board1,i,j)));
                if board1{
                    self.chess_board1.board[i][j] = Piece::L;
                }else{
                    self.chess_board2.board[i][j] = Piece::L;
                }
               
            }
        }
        if board1 {
            self.chess_board1.print_board();
        }else{
            self.chess_board2.print_board();
        }
        for &(i,j) in locs.iter() {
            if board1 {self.chess_board1.board[i][j] = Piece::E;}else
            {self.chess_board2.board[i][j] = Piece::E;}
        }
        for &(i,j,t) in vec.iter() {
            if board1 {
                self.chess_board1.board[i][j] = t;
            }else{
                self.chess_board2.board[i][j] = t;
            }
          
        }
        println!("------------------------");
    }

    pub fn new() -> ChessLogic {
        ChessLogic{
            chess_board1: ChessBoard::new(),
            chess_board2: ChessBoard::new(),
            pawn_in_last_turn_b1: None,
            pawn_in_last_turn_b2: None,
            upgrade_to1: Piece::E,
            upgrade_to2: Piece::E,
            white_active_1: true,
            white_active_2: true,
            half_moves_last_capture1: 0,
            half_moves_last_capture2: 0,
            movectr1: 0,
            movectr2: 0,
            board1_white_capture: [0;5],
            board1_black_capture: [0;5],
            board2_white_capture: [0;5],
            board2_black_capture: [0;5],
        }
    }
    pub fn get_legal_moves(&mut self,board1:bool, old_i:usize, old_j:usize)
    -> Vec<(usize,usize)>
    {
        match self.get_piece(board1,old_i,old_j) {
            Piece::P  => {
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
                        if old_j > 0 
                        && self.get_piece(board1,old_i,old_j-1) == Piece::p {
                            match self.get_pawn_in_last_turn(board1) {
                                Some((a,b)) => vec.push((a-1,b)),
                                _ => {},
                            }
                        }
                        if old_j < 7 
                        && self.get_piece(board1,old_i,old_j+1) == Piece::p {
                            match self.get_pawn_in_last_turn(board1) {
                                Some((a,b)) => vec.push((a-1,b)),
                                _ => {},
                            }
                        }
                        vec


                    },
                    (0,_) => {
                       let mut vec =  Vec::new();
                       println!("This indicates pawn upgrade malfunction");
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
            Piece::p  => {
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
                    (4,_) => {
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
                            if old_j > 0 
                            && self.get_piece(board1,old_i,old_j-1) == Piece::P {
                                match self.get_pawn_in_last_turn(board1){
                                    Some((a,b)) => vec.push((a+1,b)),
                                    _ => {},
                                }
                            }
                            if old_j < 7 
                            && self.get_piece(board1,old_i,old_j+1) == Piece::P {
                                match self.get_pawn_in_last_turn(board1) {
                                    Some((a,b)) => vec.push((a+1,b)),
                                    _ => {},
                                }
                            }
                            vec
    
                    },
                    (7,_) => {
                        let mut vec = Vec::new();
                        println!("This indicates pawn upgrade malfunction");
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
            Piece::R | Piece::r | Piece::Ur | Piece::UR => self.cross_mov(board1,old_i,old_j),
            Piece::B | Piece::b | Piece::UB | Piece::Ub=> self.x_mov(board1,old_i,old_j),
            Piece::Q | Piece::q | Piece::UQ | Piece::Uq=> 
            { let mut vec = self.cross_mov(board1,old_i,old_j);
             vec.append(&mut self.x_mov(board1,old_i,old_j));
             vec },
            Piece::N | Piece::n |Piece::UN | Piece::Un=> self.horse_jump(board1,old_i,old_j),
            Piece::K | Piece::k => self.king_move(board1,old_i,old_j),
             _ => Vec::new(),
        }
    }

    fn is_empty(&self, board1:bool, i:usize, j:usize) -> bool {
        if board1 {
            match self.chess_board1.board[i][j] {
                Piece::E => true, 
                _ => false,
            }
        }else{
            match self.chess_board2.board[i][j] {
                Piece::E => true, 
                _ => false,
            }
        }
    }

    fn is_enemy(&self, board1:bool,white:bool, i:usize, j:usize) -> bool {
        if board1 {
            if white {
                match self.chess_board1.board[i][j] {
                    Piece::E => false,
                    Piece::L => false,
                    Piece::P  => false,
                    Piece::R | Piece::UR => false,
                    Piece::N | Piece::UN => false,
                    Piece::B | Piece::UB => false,
                    Piece::Q | Piece::UQ => false,
                    Piece::K => false,
                    _ => true,
    
                }
            }else {
                match self.chess_board1.board[i][j] {
                    Piece::E => false,
                    Piece::L => false,
                    Piece::p  => false,
                    Piece::r | Piece::Ur => false,
                    Piece::n | Piece::Un => false,
                    Piece::b | Piece::Ub => false,
                    Piece::q | Piece::Uq => false,
                    Piece::k => false,
                    _ => true,
    
                }
            }
        }else{
            if white {
                match self.chess_board2.board[i][j] {
                    Piece::E => false,
                    Piece::L => false,
                    Piece::P  => false,
                    Piece::R | Piece::UR => false,
                    Piece::N | Piece::UN => false,
                    Piece::B | Piece::UB => false,
                    Piece::Q | Piece::UQ => false,
                    Piece::K => false,
                    _ => true,
    
                }
            }else {
                match self.chess_board2.board[i][j] {
                    Piece::E => false,
                    Piece::L => false,
                    Piece::p  => false,
                    Piece::r | Piece::Ur => false,
                    Piece::n | Piece::Un => false,
                    Piece::b | Piece::Ub => false,
                    Piece::q | Piece::Uq => false,
                    Piece::k => false,
                    _ => true,
    
                }
            }
        }
       
    }

    fn is_white(&self, board1:bool, i:usize, j:usize) -> bool {
        if board1 {
            match self.chess_board1.board[i][j] {
                Piece::E => false,
                Piece::L => false,
                Piece::p  => false,
                Piece::r | Piece::Ur => false,
                Piece::n | Piece::Un => false,
                Piece::b | Piece::Ub => false,
                Piece::q | Piece::Uq => false,
                Piece::k  => false,
                _ => true,
            }
        }else{
            match self.chess_board2.board[i][j] {
                Piece::E => false,
                Piece::L => false,
                Piece::p  => false,
                Piece::r | Piece::Ur => false,
                Piece::n | Piece::Un => false,
                Piece::b | Piece::Ub => false,
                Piece::q | Piece::Uq => false,
                Piece::k  => false,
                _ => true,
            }
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
                if self.is_enemy(board1,self.is_white(board1,i,j),ic,j) {
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
        while ic < 7 && jc < 7 {
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
        while ic > 0 && jc < 7 {
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
        while ic < 7 && jc > 0 {
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

    pub fn get_piece_w_en(&self, iswhite:bool, piece:Piece) -> Piece {
        match piece {
            Piece::p | Piece::P => if iswhite {Piece::p} else {Piece::P} ,
            Piece::r | Piece::R | Piece::Ur | Piece::UR => if iswhite {Piece::r} else {Piece::R} ,
            Piece::b | Piece::B | Piece::UB | Piece::Ub => if iswhite {Piece::b} else {Piece::B} ,
            Piece::k | Piece::K => if iswhite {Piece::k} else {Piece::K} ,
            Piece::q | Piece::Q | Piece::Uq | Piece::UQ => if iswhite {Piece::q} else {Piece::Q} ,
            Piece::n | Piece::N | Piece::Un | Piece::UN => if iswhite {Piece::n} else {Piece::N} ,
            Piece::L => Piece::L,
            Piece::E => Piece::E,

        }
    }

    pub fn get_piece_w_upgrade_en(&self, iswhite:bool, piece:Piece) -> Piece {
        match piece {
            Piece::p | Piece::P => if iswhite {Piece::p} else {Piece::P} ,
            Piece::r | Piece::R | Piece::Ur | Piece::UR => if iswhite {Piece::Ur} else {Piece::UR} ,
            Piece::b | Piece::B | Piece::UB | Piece::Ub => if iswhite {Piece::Ub} else {Piece::UB} ,
            Piece::k | Piece::K => if iswhite {Piece::k} else {Piece::K} ,
            Piece::q | Piece::Q | Piece::Uq | Piece::UQ => if iswhite {Piece::Uq} else {Piece::UQ} ,
            Piece::n | Piece::N | Piece::Un | Piece::UN => if iswhite {Piece::Un} else {Piece::UN} ,
            Piece::L => Piece::L,
            Piece::E => Piece::E,

        }
    }

    //iswhite <=> the unit on the square is white
    pub fn is_attacked(&mut self, board1:bool, iswhite:bool, i:usize,j:usize) -> bool {
        //remove yourself from board

 
        
        //check for pawns

        //check for rook|queen on vertical

        //check for rook|queen on horizontal

        //check for bishop on x

        //check for horses

        //check for enemy king 

        let mut ic = (i as i32);
        let mut jc = (j as i32);
            //check for pawns
            if iswhite {
                if self.valid(ic-1,jc-1) {
                    let a = (ic-1) as usize;
                    let b = (jc-1) as usize;
                    if board1 {
                        if self.chess_board1.board[a][b]==Piece::p
                        {
                   
                            return true
                        }
                    }else{
                        if self.chess_board2.board[a][b]==Piece::p
                        {
                   
                            return true
                        }
                    } 
                }
                if self.valid(ic-1,jc+1) {
                    let a = (ic-1) as usize;
                    let b = (jc+1) as usize;
                    if board1 {
                        if self.chess_board1.board[a][b]==Piece::p
                       {
                     
                            return true
                        }
                    }else{
                        if self.chess_board2.board[a][b]==Piece::p
                        {
          
                            return true
                        }
                    } 
                }
            }else{
                ic = (i as i32);
                jc = (j as i32);
                if self.valid(ic+1,jc-1) && self.valid(ic,jc) {
                    let a = (ic+1) as usize;
                    let b = (jc-1) as usize;
                    if board1 {
                        if self.chess_board1.board[a][b]==Piece::P
                        {
                            println!("{},{}: is a piyon",a,b);
                            return true
                        }
                    }else{
                        if self.chess_board2.board[a][b]==Piece::P
                        {
                   
                            return true
                        }
                    } 
                }
                if self.valid(ic+1,jc+1) && self.valid(ic,jc) {
                    let a = (ic+1) as usize;
                    let b = (jc+1) as usize;
                    if board1 {
                        if self.chess_board1.board[a][b]==Piece::P
                        {
                     
                            return true
                        }
                    }else{
                        if self.chess_board2.board[a][b]==Piece::P
                        {
          
                            return true
                        }
                    } 
                }
            }
           
            //check for horizontal line
            jc = j as i32;
            ic = i as i32;
            let mut jx = jc+1;
            while jx < 8 {
                if self.check_for_piece(board1,self.get_piece_w_en(iswhite,Piece::r),ic,jx) || 
                self.check_for_piece(board1,self.get_piece_w_en(iswhite,Piece::q),ic,jx) ||
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::r),ic,jx) || 
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::q),ic,jx){

                    return true
                }else if !self.check_for_piece(board1,Piece::E, ic,jx){
                    break;
                }
                jx+=1;
            }
            jx = jc-1;
            while jx >= 0{
                if self.check_for_piece(board1,self.get_piece_w_en(iswhite,Piece::r),ic,jx) || 
                self.check_for_piece(board1,self.get_piece_w_en(iswhite,Piece::q),ic,jx) ||
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::r),ic,jx) || 
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::q),ic,jx) {
                    return true
                }else if !self.check_for_piece(board1,Piece::E, ic,jx){
                    break;
                }
                jx -=1;
            }
            //check for vertical
            let mut ix = ic+1;
            while ix <8 {
                if self.check_for_piece(board1,self.get_piece_w_en(iswhite,Piece::r),ix,jc) || 
                self.check_for_piece(board1,self.get_piece_w_en(iswhite,Piece::q),ix,jc) ||
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::r),ix,jc) || 
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::q),ix,jc)
             {
                    return true
                }else if !self.check_for_piece(board1,Piece::E, ix,jc){
                    break;
                }
                ix+=1;
            }
            ix = ic-1;
            while ix >=0 {
                if self.check_for_piece(board1,self.get_piece_w_en(iswhite,Piece::r),ix,jc) || 
                self.check_for_piece(board1,self.get_piece_w_en(iswhite,Piece::q),ix,jc) ||
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::r),ix,jc) || 
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::q),ix,jc)
                {
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
                if self.check_for_piece(board1,self.get_piece_w_en(iswhite,Piece::q),ic,jc) ||
                self.check_for_piece(board1,self.get_piece_w_en(iswhite,Piece::b),ic,jc) ||
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::q),ic,jc) ||
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::b),ic,jc) {
            
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
                if self.check_for_piece(board1,self.get_piece_w_en(iswhite,Piece::q),ic,jc) ||
                self.check_for_piece(board1,self.get_piece_w_en(iswhite,Piece::b),ic,jc) ||
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::q),ic,jc) ||
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::b),ic,jc){
              
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
                if self.check_for_piece(board1,self.get_piece_w_en(iswhite,Piece::q),ic,jc) ||
                self.check_for_piece(board1,self.get_piece_w_en(iswhite,Piece::b),ic,jc) ||
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::q),ic,jc) ||
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::b),ic,jc){
                 
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
                if self.check_for_piece(board1,self.get_piece_w_en(iswhite,Piece::q),ic,jc) ||
                self.check_for_piece(board1,self.get_piece_w_en(iswhite,Piece::b),ic,jc) ||
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::q),ic,jc) ||
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::b),ic,jc) {
              
                    return true
                }else if !self.check_for_piece(board1,Piece::E, ic,jc){
                    break;
                }
            }
            //check for horses
            ic = i as i32;
            jc = j as i32;
            let a = [-2,2];
            let b = [-1,1];
            for i_off in a.iter() {
                for j_off in b.iter() {
                    if self.valid(ic+i_off,jc+j_off)  {
                        if board1 {
                            if self.chess_board1.board[(ic+i_off) as usize][(jc+j_off) as usize] == 
                            self.get_piece_w_en(iswhite,Piece::n) ||
                            self.chess_board1.board[(ic+i_off) as usize][(jc+j_off) as usize] == 
                            self.get_piece_w_upgrade_en(iswhite,Piece::n){
                              
                                return true
                            }
                        }else{
                            if self.chess_board2.board[(ic+i_off) as usize][(jc+j_off) as usize] == 
                            self.get_piece_w_en(iswhite,Piece::n) ||
                            self.chess_board1.board[(ic+i_off) as usize][(jc+j_off) as usize] == 
                            self.get_piece_w_upgrade_en(iswhite,Piece::n){
                           
                                return true
                            }
                        }
                    }
                }
            }
            for j_off in a.iter() {
                for i_off in b.iter() {
                    if self.valid(ic+i_off,jc+j_off)  {
                        if board1 {
                            if self.chess_board1.board[(ic+i_off) as usize][(jc+j_off) as usize] == 
                            self.get_piece_w_en(iswhite,Piece::n) || 
                            self.chess_board1.board[(ic+i_off) as usize][(jc+j_off) as usize] == 
                            self.get_piece_w_upgrade_en(iswhite,Piece::n){
                               
                                return true
                            }
                        }else{
                            if self.chess_board2.board[(ic+i_off) as usize][(jc+j_off) as usize] == 
                            self.get_piece_w_en(iswhite,Piece::n) ||
                            self.chess_board2.board[(ic+i_off) as usize][(jc+j_off) as usize] == 
                            self.get_piece_w_upgrade_en(iswhite,Piece::n) {
                              
                                return true
                            }
                        }
                    }
                }
            }
            //check for enemy king
            let c = [-1,0,1];
            let d = [-1,0,1];
            for i_off in c.iter() {
                for j_off in d.iter() {
                    if self.valid(ic+i_off,jc+j_off) && !(*i_off==0 && *j_off==0) {
                        if self.chess_board1.board[(ic+i_off) as usize][(jc+j_off) as usize] == 
                        self.get_piece_w_en(iswhite,Piece::k) {
                    
                            return true
                        }
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

    fn king_move(&mut self, board1:bool, i:usize,j:usize) -> Vec<(usize,usize)>{
        let mut a = [-1,0,1];
        let mut b = [-1,0,1];
        let mut vec = Vec::new();
        let ix = i as i32;
        let jx = j as i32;
        let mut piece = Piece::E;
        let mut wayt = true;
        if board1 {
            piece = self.chess_board1.board[i][j];
            self.chess_board1.board[i][j] = Piece::E;
        }else{
            piece = self.chess_board2.board[i][j];
            self.chess_board2.board[i][j] = Piece::E;
        }
        if piece == Piece::K {
            wayt = true;
        }else{
            wayt = false;
        }
        for i_off in a.iter(){
            for j_off in b.iter(){
                if self.valid(i_off+ix,j_off+jx) && !(*i_off==0 && *j_off==0){
                    let mut ic = i_off+ix;
                    let mut jc = j_off+jx;
                    //if !self.is_attacked(board1,wayt,ic as usize,jc as usize)  --keep this in case we want legal moves only
                    if true 
                    {
                        if board1 {
                            if self.chess_board1.board[ic as usize][jc as usize] == Piece::E 
                            || self.is_enemy(board1,wayt,ic as usize, jc as usize) {
                                vec.push((ic as usize,jc as usize));
                            }
                        }else{
                            if self.chess_board2.board[ic as usize][jc as usize] == Piece::E
                            || self.is_enemy(board1,wayt,ic as usize, jc as usize) {
                                vec.push((ic as usize,jc as usize));
                            }
                        }
                    }
                }

            }
        }
        if board1 {
            self.chess_board1.board[i][j] = piece;
        }else{
            self.chess_board2.board[i][j] = piece;
        }
        //check for castling
        if board1 {
            if wayt {
                if !self.chess_board1.white_k_moved 
                && self.chess_board1.board[7][7] == Piece::R 
                && self.chess_board1.board[7][6] == Piece::E
                && self.chess_board1.board[7][5] == Piece::E
                && !self.is_attacked(board1,wayt,7,6) 
                && !self.is_attacked(board1,wayt,7,5)
                && !self.is_attacked(board1,wayt,7,4)
                {
                    vec.push((7,6))
                }
                if !self.chess_board1.white_k_moved 
                && self.chess_board1.board[7][0] == Piece::R 
                && self.chess_board1.board[7][1] == Piece::E
                && self.chess_board1.board[7][2] == Piece::E
                && self.chess_board1.board[7][3] == Piece::E
                && !self.is_attacked(board1,wayt,7,3) 
                && !self.is_attacked(board1,wayt,7,2)
                && !self.is_attacked(board1,wayt,7,4)
                && !self.is_attacked(board1,wayt,7,1) {
                    vec.push((7,2))
                }
            }else{
                if !self.chess_board1.black_k_moved 
                && self.chess_board1.board[0][7] == Piece::r 
                && self.chess_board1.board[0][6] == Piece::E
                && self.chess_board1.board[0][5] == Piece::E
                && !self.is_attacked(board1,wayt,0,6) 
                && !self.is_attacked(board1,wayt,0,5)
                && !self.is_attacked(board1,wayt,0,4)
                {
                    vec.push((0,6))
                }
                if !self.chess_board1.black_k_moved 
                && self.chess_board1.board[0][0] == Piece::r 
                && self.chess_board1.board[0][1] == Piece::E
                && self.chess_board1.board[0][2] == Piece::E
                && self.chess_board1.board[0][3] == Piece::E
                && !self.is_attacked(board1,wayt,0,3) 
                && !self.is_attacked(board1,wayt,0,2)
                && !self.is_attacked(board1,wayt,0,4)
                && !self.is_attacked(board1,wayt,0,1) {
                    vec.push((0,2))
                }
            }
        }else{
            if wayt {
                if !self.chess_board2.white_k_moved 
                && self.chess_board2.board[7][7] == Piece::R 
                && self.chess_board2.board[7][6] == Piece::E
                && self.chess_board2.board[7][5] == Piece::E
                && !self.is_attacked(board1,wayt,7,6) 
                && !self.is_attacked(board1,wayt,7,5)
                && !self.is_attacked(board1,wayt,7,4)
                {
                    vec.push((7,6))
                }
                if !self.chess_board2.white_k_moved 
                && self.chess_board2.board[7][0] == Piece::R 
                && self.chess_board2.board[7][1] == Piece::E
                && self.chess_board2.board[7][2] == Piece::E
                && self.chess_board2.board[7][3] == Piece::E
                && !self.is_attacked(board1,wayt,7,3) 
                && !self.is_attacked(board1,wayt,7,2)
                && !self.is_attacked(board1,wayt,7,4)
                && !self.is_attacked(board1,wayt,7,1) {
                    vec.push((7,2))
                }
            }else{
                if !self.chess_board2.black_k_moved 
                && self.chess_board2.board[0][7] == Piece::r 
                && self.chess_board2.board[0][6] == Piece::E
                && self.chess_board2.board[0][5] == Piece::E
                && !self.is_attacked(board1,wayt,0,6) 
                && !self.is_attacked(board1,wayt,0,5)
                && !self.is_attacked(board1,wayt,0,4)
                {
                    vec.push((0,6))
                }
                if !self.chess_board2.black_k_moved 
                && self.chess_board2.board[0][0] == Piece::r 
                && self.chess_board2.board[0][1] == Piece::E
                && self.chess_board2.board[0][2] == Piece::E
                && self.chess_board2.board[0][3] == Piece::E
                && !self.is_attacked(board1,wayt,0,3) 
                && !self.is_attacked(board1,wayt,0,2)
                && !self.is_attacked(board1,wayt,0,4)
                && !self.is_attacked(board1,wayt,0,1) {
                    vec.push((0,2))
                }
            }
            
        }
        vec
    }

    pub fn get_board(&mut self, board1:bool )-> &mut ChessBoard{
        match board1 {
            true => &mut self.chess_board1,
            false => &mut self.chess_board2,
        }
    }

    //rly important detail
    //in tandem you can move your pinned piece, then the enemy can capture 
    //your rook
    pub fn legality_check(&mut self, board1:bool, i_old:usize,j_old:usize,i:usize,j:usize) -> bool{
        
        match &self.get_board(board1).board[i_old][j_old] {
            Piece::E | Piece::L => false,
            _ => { 
                let vec = self.get_legal_moves(board1,i_old,j_old);
                for (a,b) in vec.iter() {
                    if *a == i && *b == j {
                        return true
                    }
                }
                return false
            },
        }
    }


    //do not forget the side effects while testing
    pub fn movemaker(&mut self, board1:bool, i_old:usize,j_old:usize,i:usize,j:usize) -> bool{

        if self.legality_check(board1,i_old,j_old,i,j) {
            if board1 {
                match self.chess_board1.board[i_old][j_old] {
                    Piece::K => 
                    {
                        if !self.white_active_1 {
                            return false 
                        }else{
                      
                        if i_old==7 && j_old==4 && i== 7 && (j==6 || j==2)
                        {
                            //we are going castle
                            self.chess_board1.board[i_old][j_old]=Piece::E;
                            self.chess_board1.board[i][j] = Piece::K;
                            if j==6 {
                                self.chess_board1.board[i][j-1] = Piece::R;
                                self.chess_board1.board[7][7] = Piece::E;
                            }else{
                                self.chess_board1.board[i][j+1] = Piece::R;
                                self.chess_board1.board[7][0] = Piece::E;
                            }
                            self.chess_board1.white_k_moved = true;
                            self.white_active_1 = !self.white_active_1;

                            self.half_moves_last_capture1 += 1;
                            return true 
                        }
                        self.chess_board1.white_k_moved = true;
                        }
                    },
                    Piece::k => 
                    {
                        if self.white_active_1 {
                            return false 
                        }else{
                          
                        if i_old==0 && j_old==4
                        && i== 0 && (j==6 || j==2)
                        {
                            //we are going castle
                            self.chess_board1.board[i_old][j_old]=Piece::E;
                            self.chess_board1.board[i][j] = Piece::K;
                            if j==6 {
                                self.chess_board1.board[i][j-1] = Piece::R;
                                self.chess_board1.board[0][7] = Piece::E;
                            }else{
                                self.chess_board1.board[i][j+1] = Piece::R;
                                self.chess_board1.board[0][0] = Piece::E;
                            }
                            self.chess_board1.black_k_moved = true;
                            self.white_active_1 = !self.white_active_1;

                            self.half_moves_last_capture1 += 1;
                            self.movectr1+=1;
                            return true 
                        }
                     
                        self.chess_board1.black_k_moved = true;
                        }
                    },
                    Piece::P => 
                    {
                        if !self.white_active_1 {
                            return false
                        }else{
                        if i==0 {
                           
                            if self.upgrade_to1 == Piece::Q ||  self.upgrade_to1 == Piece::R  || self.upgrade_to1 == Piece::B  ||self.upgrade_to1 == Piece::N {
                                self.half_moves_last_capture1=0;
                                match self.box_index(self.chess_board1.board[i][j]) {
                                    None => {},
                                    Some(x) => {self.board2_black_capture[x] += 1},
                                }
                            }
                            
                           
                            match self.upgrade_to1 {
                                Piece::Q => { self.upgrade_to1 = Piece::E; self.chess_board1.board[i][j] = Piece::UQ;
                                    self.chess_board1.board[i_old][j_old] = Piece::E; 
                                    self.white_active_1 = !self.white_active_1; return true},
                                Piece::R => { self.upgrade_to1 = Piece::E; self.chess_board1.board[i][j] = Piece::UR;
                                    self.chess_board1.board[i_old][j_old] = Piece::E;
                                    self.white_active_1 = !self.white_active_1;return true},
                                Piece::B => { self.upgrade_to1 = Piece::E;self.chess_board1.board[i][j] = Piece::UB;
                                    self.chess_board1.board[i_old][j_old] = Piece::E;
                                    self.white_active_1 = !self.white_active_1;return true},
                                Piece::N => { self.upgrade_to1 = Piece::E;self.chess_board1.board[i][j] = Piece::UN;
                                    self.chess_board1.board[i_old][j_old] = Piece::E;
                                    self.white_active_1 = !self.white_active_1;return true },
                                _ =>  {println!("Precondition not fulfilled!"); return false},
                           } 
                        }
                        }
                    },
                    Piece::p => 
                    {
                        if self.white_active_1 {
                            return false
                        }else {
                       if i==7 {

                        if self.upgrade_to1 == Piece::q ||  self.upgrade_to1 == Piece::r  || self.upgrade_to1 == Piece::b  ||self.upgrade_to1 == Piece::n {
                            self.half_moves_last_capture1=0;
                            match self.box_index(self.chess_board1.board[i][j]) {
                                None => {},
                                Some(x) => {self.board2_white_capture[x] += 1},
                            }
                        }

                            match self.upgrade_to1 {
                                Piece::q => {self.upgrade_to1 = Piece::E;self.chess_board1.board[i][j] = Piece::Uq;
                                self.chess_board1.board[i_old][j_old] = Piece::E;
                                self.white_active_1 = !self.white_active_1; return true},
                                Piece::r => {self.upgrade_to1 = Piece::E;self.chess_board1.board[i][j] = Piece::Ur; 
                                    self.chess_board1.board[i_old][j_old] = Piece::E;
                                    self.white_active_1 = !self.white_active_1;return true},
                                Piece::b => {self.upgrade_to1 = Piece::E;self.chess_board1.board[i][j] = Piece::Ub; 
                                    self.chess_board1.board[i_old][j_old] = Piece::E;
                                    self.white_active_1 = !self.white_active_1;return true},
                                Piece::n => {self.upgrade_to1 = Piece::E;self.chess_board1.board[i][j] = Piece::Un;
                                    self.chess_board1.board[i_old][j_old] = Piece::E; 
                                    self.white_active_1 = !self.white_active_1;return true},
                                _ =>  {println!("Precondition not fulfilled!"); return false},
                            } 
                       }
                        }
                    },
                    _ => {},
                }

               
                if !(( self.white_active_1 && self.is_white(board1,i_old,j_old)) 
                || (!self.white_active_1 && !self.is_white(board1,i_old,j_old))) {
                    //not your turn!!!
                    println!("Siktir git hamlen degil!");
                    return false
                }
                let tmp = self.chess_board1.board[i][j];

                //check if any piece is captured 
                if tmp!=Piece::E {
                    self.half_moves_last_capture1=0;
                }else {
                    self.half_moves_last_capture1+=1;
                }

                //check if pawn is moved
                if self.chess_board1.board[i_old][j_old]==Piece::P {
                    self.half_moves_last_capture1=0;
                }else if self.chess_board1.board[i_old][j_old]==Piece::p {
                    self.half_moves_last_capture1=0;
                }

                //check if black has moved
                if !self.is_white(board1,i_old,j_old){
                    self.movectr1+=1;
                }

                //check if game  should end
                if tmp==Piece::K || tmp==Piece::k {
                    self.finish_up();
                }

                match self.box_index(tmp) {
                    None => {}, //nothing to di
                    Some(a) => {
                        if self.is_white(board1,i,j) {
                            self.board2_white_capture[a]+=1;
                        }else{
                            self.board2_black_capture[a]+=1;
                        }
                    },
                }
                self.chess_board1.board[i][j]=self.chess_board1.board[i_old][j_old];
                self.chess_board1.board[i_old][j_old]=Piece::E;
                self.white_active_1 = !self.white_active_1;
                return true

            }else{
                match self.chess_board2.board[i_old][j_old] {
                    Piece::K => 
                    {
                        if !self.white_active_2 {
                            return false
                        }else{
                        if i_old==7 && j_old==4
                        && i== 7 && (j==6 || j==2)
                        {
                            //we are going castle
                            self.chess_board2.board[i_old][j_old]=Piece::E;
                            self.chess_board2.board[i][j] = Piece::K;
                            if j==6 {
                                self.chess_board2.board[i][j-1] = Piece::R;
                                self.chess_board2.board[7][7] = Piece::E;
                            }else{
                                self.chess_board2.board[i][j+1] = Piece::R;
                                self.chess_board2.board[7][0] = Piece::E;
                            }
                            self.chess_board2.white_k_moved = true;
                            self.white_active_2 = !self.white_active_2;

                            self.half_moves_last_capture2 += 1;
                            return true 
                        }
                        self.chess_board2.white_k_moved = true;
                        }
                    },
                    Piece::k => 
                    {
                        if self.white_active_2 {
                            return false
                        }else{
                        if i_old==0 && j_old==4
                        && i== 0 && (j==6 || j==2)
                        {
                            //we are going castle
                            self.chess_board2.board[i_old][j_old]=Piece::E;
                            self.chess_board2.board[i][j] = Piece::K;
                            if j==6 {
                                self.chess_board2.board[i][j-1] = Piece::R;
                                self.chess_board2.board[0][7] = Piece::E;
                            }else{
                                self.chess_board2.board[i][j+1] = Piece::R;
                                self.chess_board2.board[0][0] = Piece::E;
                            }
                            self.chess_board2.black_k_moved = true;
                            self.white_active_2 = !self.white_active_2;

                            self.half_moves_last_capture2 += 1;
                            self.movectr2+=1;
                            return true 
                        }
                        
                        self.chess_board2.black_k_moved = true;
                        }
                    },
                    Piece::P => 
                    {
                        if !self.white_active_2 {
                            return false
                        }else{
                        if i==0 {

                            if self.upgrade_to2 == Piece::Q ||  self.upgrade_to2 == Piece::R  
                            || self.upgrade_to2 == Piece::B || self.upgrade_to2 == Piece::N {
                                self.half_moves_last_capture2=0;
                                match self.box_index(self.chess_board2.board[i][j]) {
                                    None => {},
                                    Some(x) => {self.board1_black_capture[x] += 1},
                                }
                            }
                            match self.upgrade_to2 {
                                Piece::Q => { self.upgrade_to2 = Piece::E;self.chess_board2.board[i][j] = Piece::UQ;
                                    self.chess_board2.board[i_old][j_old] = Piece::E;
                                    self.white_active_2 = !self.white_active_2;return true},
                                Piece::R => { self.upgrade_to2 = Piece::E;self.chess_board2.board[i][j] = Piece::UR;
                                    self.chess_board2.board[i_old][j_old] = Piece::E;
                                    self.white_active_2 = !self.white_active_2;return true},
                                Piece::B => { self.upgrade_to2 = Piece::E;self.chess_board2.board[i][j] = Piece::UB;
                                    self.chess_board2.board[i_old][j_old] = Piece::E;
                                    self.white_active_2 = !self.white_active_2;return true},
                                Piece::N => { self.upgrade_to2 = Piece::E;self.chess_board2.board[i][j] = Piece::UN;
                                    self.chess_board2.board[i_old][j_old] = Piece::E;
                                    self.white_active_2 = !self.white_active_2;return true },
                                _ =>  {println!("Precondition not fulfilled!"); return false},
                           } 
                        }
                        }
                    },
                    Piece::p => 
                    {
                        if self.white_active_2 {
                            return false
                        }else{
                       if i==7 {
                            if self.upgrade_to2 == Piece::q || self.upgrade_to2 == Piece::r 
                             || self.upgrade_to2 == Piece::b || self.upgrade_to2 == Piece::n {
                                self.half_moves_last_capture2=0;
                                match self.box_index(self.chess_board2.board[i][j]) {
                                    None => {},
                                    Some(x) => {self.board1_white_capture[x] += 1},
                                }
                            }

                            match self.upgrade_to2 {
                                Piece::q => { self.upgrade_to2 = Piece::E;self.chess_board2.board[i][j] = Piece::Uq;
                                self.chess_board2.board[i_old][j_old] = Piece::E; 
                                self.white_active_2 = !self.white_active_2;return true},
                                Piece::r => { self.upgrade_to2 = Piece::E;self.chess_board2.board[i][j] = Piece::Ur; 
                                    self.chess_board2.board[i_old][j_old] = Piece::E;
                                    self.white_active_2 = !self.white_active_2;return true},
                                Piece::b => { self.upgrade_to2 = Piece::E;self.chess_board2.board[i][j] = Piece::Ub; 
                                    self.chess_board2.board[i_old][j_old] = Piece::E;
                                    self.white_active_2 = !self.white_active_2;return true},
                                Piece::n => { self.upgrade_to2 = Piece::E;self.chess_board2.board[i][j] = Piece::Un;
                                    self.chess_board2.board[i_old][j_old] = Piece::E; 
                                    self.white_active_2 = !self.white_active_2;return true},
                                _ =>  {println!("Precondition not fulfilled!"); return false},
                            } 
                       }
                    }
                    },
                    _ => {},
                }

                if !( (self.white_active_2 && self.is_white(board1,i_old,j_old)) 
                || (!self.white_active_2 && !self.is_white(board1,i_old,j_old))) {
                    //not your turn!!!
                    println!("Siktir git hamlen degil!");
                    return false
                }


                let tmp = self.chess_board2.board[i][j];

                //check if any piece is captured 
                if tmp!=Piece::E {
                    self.half_moves_last_capture2=0;
                }else {
                    self.half_moves_last_capture2+=1;
                }

                //check if pawn is moved
                if self.chess_board2.board[i_old][j_old]==Piece::P {
                    self.half_moves_last_capture2=0;
                }else if self.chess_board2.board[i_old][j_old]==Piece::p {
                    self.half_moves_last_capture2=0;
                }

                //check if black has moved
                if !self.is_white(board1,i_old,j_old){
                    self.movectr2+=1;
                }

                //check if game  should end
                if tmp==Piece::K || tmp==Piece::k {
                    self.finish_up();
                }

                match self.box_index(tmp) {
                    None => {}, //nothing to di
                    Some(a) => {
                        if self.is_white(board1,i,j) {
                            self.board1_white_capture[a]+=1;
                        }else{
                            self.board1_black_capture[a]+=1;
                        }
                    },
                }
                self.chess_board2.board[i][j]=self.chess_board2.board[i_old][j_old];
                self.chess_board2.board[i_old][j_old]=Piece::E;
                self.white_active_2 = !self.white_active_2;
                return true
            }
        }else{
            println!("Move not legal!");
            false
        }
    }

    pub fn box_index(&self, piece:Piece) -> Option<usize> {
        //P-R-N-B-Q
        match piece {
            Piece::P | Piece::p  => Some(0),
            Piece::K | Piece::k | Piece::L  | Piece::E => None ,
            Piece::N | Piece::n | Piece::Un | Piece::UN => Some(2),
            Piece::R | Piece::r | Piece::Ur | Piece::UR => Some(1),
            Piece::B | Piece::b | Piece::Ub | Piece::UB => Some(3),
            Piece::Q | Piece::q | Piece::Uq | Piece::UQ => Some(4),

        }
    }

    fn finish_up(&self){
        println!("This is a stub, todo: game done");
    }

}

