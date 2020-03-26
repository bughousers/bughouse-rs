pub mod board;
pub mod tests;
use crate::logic::board::ChessBoard;
use crate::logic::board::Piece;
use std::cmp;



#[derive(Clone, Copy, PartialEq)]
///Enum class to keep track of the winner
///
///It can be W,B from Board 1 or 2, None or Stalemate(Patt)
pub enum Winner {
    W1,B1,N,P,W2,B2,
}

#[derive(Clone, Copy, PartialEq)]
///Enum class for possible movement errors, could be usefull for calls from a server
/// 
///NotLegal -> move is not a legal move
///NotTurn -> not the turn of the caller
///CannotDeploy -> the given piece cannot be deployed, e.g. the cell is not empty
///NoPieceInPool -> the pool for needed piece is empty
///PromotionProblem -> the promotion booleans are not set (the boolean has to be set to promote pawns)
pub enum MoveError {
    NotLegal,NotTurn,CannotDeploy,AlreadyOver,NoPieceInPool,
    PromotionProblem,
}

///Chesslogic struct has everything needed for a Bughouse game
pub struct ChessLogic {
    ///Chessboard of game 1, aka. board1
    pub chess_board1: ChessBoard, 
    ///Chessboard of game 2, aka. board2
    pub chess_board2: ChessBoard, 
    ///The last moved pawn of board1, None if another piece has moved
    pawn_in_last_turn_b1: Option<(usize,usize)>,
    ///The last moved pawn of board2, None if another piece has moved
    pawn_in_last_turn_b2: Option<(usize,usize)>,
    ///True if white is active on board1, else false
    white_active_1: bool, 
    ///True if white is active on board2, else false
    white_active_2: bool, 
    ///The piece to upgrade to in the next pawn, board1
    pub upgrade_to1: Piece, 
    ///The piece to upgrade to in the next pawn, board2
    pub upgrade_to2: Piece, 
    ///Count of half moves since last pawn move or piece capture, board1
    half_moves_last_capture1: usize, 
    ///Same but for board2
    half_moves_last_capture2: usize, 
    ///Number of moves in board1
    movectr1: usize, 
    ///Same but for board2
    movectr2: usize, 

   
    ///Pieces that can be deployed on board1 (white pieces),the order is P-R-N-B-Q
    board1_white_capture: [u8;5], 
    ///Same but for board1,black
    board1_black_capture: [u8;5], 
    ///Same but for board2,white
    board2_white_capture: [u8;5], 
    ///Same but for board2,black
    board2_black_capture: [u8;5],
    ///A field to save the winner 
    winner: Winner,
}

impl ChessLogic {

    ///Sets both games to initial state (none of the games have started)
    pub fn refresh(&mut self){
        self.chess_board1.set_init_array();
       
        self.chess_board1.white_k_moved = false;
        self.chess_board1.black_k_moved = false; 
        self.chess_board1.white_rook_q_moved = false; 
        self.chess_board1.white_rook_k_moved = false; 
        self.chess_board1.black_rook_q_moved = false; 
        self.chess_board1.black_rook_k_moved = false; 

        self.chess_board2.set_init_array();

        self.chess_board2.white_k_moved = false; 
        self.chess_board2.black_k_moved = false; 
        self.chess_board2.white_rook_q_moved = false; 
        self.chess_board2.white_rook_k_moved = false; 
        self.chess_board2.black_rook_q_moved = false; 
        self.chess_board2.black_rook_k_moved = false; 

        self.pawn_in_last_turn_b1 = None;  
        self.pawn_in_last_turn_b2 = None; 
        self.upgrade_to1 = Piece::E; 
        self.upgrade_to2 = Piece::E; 
        self.white_active_1 = true; 
        self.white_active_2 = true; 
        self.half_moves_last_capture1 = 0; 
        self.half_moves_last_capture2 = 0; 
        self.movectr1 = 1; 
        self.movectr2 = 1; 
        self.board1_white_capture = [0;5]; 
        self.board1_black_capture = [0;5]; 
        self.board2_white_capture = [0;5]; 
        self.board2_black_capture = [0;5]; 
        self.winner= Winner::N; 
    }

    ///Prints one of the boards
    /// # Arguments
    /// * `board1` - true if board1, false if board 2
    pub fn print(&self,board1:bool){
        if board1{
            self.chess_board1.print_board();
            println!("------------------------");
        }else{
            self.chess_board2.print_board();
            println!("------------------------");
        }

    }

    ///Return if the king has moved
    /// # Arguments
    /// * `board1` - true if board1, false if board2
    /// * `white` - true if white, false if black

    pub fn get_if_king_moved(&self,board1:bool,white:bool) -> bool{
        if board1 {
            if white{
                self.chess_board1.white_k_moved
            }  else{
                self.chess_board1.black_k_moved
            }         
        }else{
            if white{
               self.chess_board2.white_k_moved
            }else{
                self.chess_board2.black_k_moved
            }
        }
    }

    ///Returns the capture pools
    pub fn get_pools(&self) -> ([u8;5],[u8;5],[u8;5],[u8;5]){
        (
            self.board1_white_capture, 
            self.board1_black_capture, 
            self.board2_white_capture, 
            self.board2_black_capture
        )
    }

    ///Get a bool that is true if white is active
    ///# Arguments
    /// * `board1` - true if board1, false if board2
    pub fn get_white_active(&self, board1:bool) -> bool{
        match board1 {
            true => self.white_active_1,
            false => self.white_active_2,
        }
    }

    ///Get number of played turns
    /// # Arguments
    /// * `board1` - true if board1, false if board2
    pub fn get_movectr(&self, board1:bool) -> usize {
        match board1 {
            true => self.movectr1,
            false => self.movectr2,
        }
    }

    ///Get half moves since last capture or pawn movement
    /// # Arguments
    /// * `board1` - true if board1, false if board2
    pub fn get_half_moves(&self, board1:bool) -> usize {
        match board1 {
            true => self.half_moves_last_capture1,
            false => self.half_moves_last_capture2,
        }
    }

    ///Returns the captured pieces
    ///  # Arguments
    /// * `board1` - true if board1, false if board2
    /// * `white` - true if white, false if black
    /// * `i` - index for the pool, can be generated with box_index
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

        ///Sets the captured pieces after a piece capture
        /// # Arguments
        /// * `board1` - true if board1, false if board2
        /// * `white` - true if white, false if black
        /// * `i` - index for the pool, can be generated with box_index
        /// * `inc` - if inc then the piece count in the pool will be increased by 1 else decreased by 1
        fn set_captured_piece(&mut self, board1:bool,white:bool, i:usize,inc:bool){
            if board1 {
                if white { 
                   if !inc {
                    self.board1_white_capture[i]-=1;
                   } else{
                    self.board1_white_capture[i]+=1;
                   }
                }else{
                    if !inc {
                        self.board1_black_capture[i]-=1;
                       } else{
                        self.board1_black_capture[i]+=1;
                       }
                }
            }else{
                if white {
                    if !inc {
                        self.board2_white_capture[i]-=1;
                       } else{
                        self.board2_white_capture[i]+=1;
                       }
                }else{
                    if !inc {
                        self.board2_black_capture[i]-=1;
                       } else{
                        self.board2_black_capture[i]+=1;
                       }
                }
            }
        }

    ///Returns the last moved pawn, needed for enpassant encoding
    /// # Arguments
    /// * `board1` - true if board1, false if board2
    pub fn get_pawn_in_last_turn(&self, board1:bool) -> Option<(usize,usize)> {
        match board1 {
            true => self.pawn_in_last_turn_b1,
            false => self.pawn_in_last_turn_b2,
        }
    }

    ///Returns the winner type (white,black,none ...)
    /// * `board1` - true if board1, false if board2
    pub fn get_winner(&self, board1:bool) -> Winner {
        self.winner
    }

    ///Sets the pawn_in_last_turn field
    /// 
    /// only for testing, movemaker and deploy piece update this field
    /// # Arguments
    /// * `board1` - true if board1, else false
    /// * `x` - location of the pawn_in_last_turn, can be none
    fn set_pawn_in_last_turn(&mut self, board1:bool, x:Option<(usize,usize)>) {
        match board1 {
            true => self.pawn_in_last_turn_b1 = x,
            false => self.pawn_in_last_turn_b2 = x,
        }
    }

    ///Print legal moves with input of legal moves
    /// # Arguments
    /// * `board1` - true if board1, else false
    /// * `locs` - vector of legal moves
    pub fn print_w_legal(&mut self,board1:bool,locs: &Vec<(usize,usize)>){
        //its highly probably that a piece will have 8-16 legal moves
        let mut vec = Vec::with_capacity(8);
        for &(i,j) in locs.iter() {
            if self.get_piece(board1,i,j)== Piece::E {
                //legal moves are encoded with 'L'
                if board1 {
                    self.chess_board1.board[i][j] = Piece::L;
                }else{
                    self.chess_board2.board[i][j] = Piece::L;
                }
            }else{
                vec.push((i,j,self.get_piece(board1,i,j)));
                //legal moves are encoded with 'L'
                if board1{
                    self.chess_board1.board[i][j] = Piece::L;
                }else{
                    self.chess_board2.board[i][j] = Piece::L;
                }
               
            }
        }
        //print the boards with legal moves as L
        if board1 {
            self.chess_board1.print_board();
        }else{
            self.chess_board2.print_board();
        }

        //set back the legal moves 'L' with their old pieces
        for &(i,j) in locs.iter() {
            // L -> E
            if board1 {self.chess_board1.board[i][j] = Piece::E;} 
            else {self.chess_board2.board[i][j] = Piece::E;}
        }
        for &(i,j,t) in vec.iter() {
            // non L -> revert to old Piece
            if board1 {
                self.chess_board1.board[i][j] = t;
            }else{
                self.chess_board2.board[i][j] = t;
            }
          
        }
        println!("------------------------");
    }

    /// Default constructor for chesslogic
    /// 
    /// Initializes the game with initial positions 
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
            movectr1: 1,
            movectr2: 1,
            board1_white_capture: [0;5],
            board1_black_capture: [0;5],
            board2_white_capture: [0;5],
            board2_black_capture: [0;5],
            winner: Winner::N,
        }
    }

    ///Returns a vector of legal moves for the board, and for the location
    /// # Arguments
    /// * `board1` - true if board1, else false
    /// * `old_i` - row index of the piece
    /// * `old_j` - col index of the piece
    pub fn get_legal_moves(&mut self,board1:bool, old_i:usize, old_j:usize)
    -> Vec<(usize,usize)>
    {
        match self.get_piece(board1,old_i,old_j) {
            Piece::P  => {
                match (old_i,old_j) {
                    //unmoved (insert double move too -- Double move)
                    (6,_) =>  {
                        //max 4 moves
                        let mut vec = Vec::with_capacity(2);
                        if self.is_empty(board1,old_i-1,old_j) {
                            vec.push((old_i-1,old_j));
                            // -2 only if -1 is empty
                            if self.is_empty(board1,old_i-2,old_j) {
                                vec.push((old_i-2,old_j));
                            }
                        }
                        //enemy piece on -1,-1
                        if old_j > 0 && self.is_enemy(board1,true,old_i-1,old_j-1) {
                            vec.push((old_i-1,old_j-1));
                        }
                        //enemy piece on -1,+1
                        if old_j < 7 && self.is_enemy(board1,true,old_i-1,old_j+1) {
                            vec.push((old_i-1,old_j+1));
                        }
                        vec
                    },
                    //6th lane with en passant
                    (3,_) => {
                        //max 5 moves
                        let mut vec = Vec::with_capacity(2);
                        if self.is_empty(board1,old_i-1,old_j) {
                            vec.push((old_i-1,old_j));
                        }
                        if old_j > 0 && self.is_enemy(board1,true,old_i-1,old_j-1) {
                            vec.push((old_i-1,old_j-1));
                        }
                        if old_j < 7 && self.is_enemy(board1,true,old_i-1,old_j+1) {
                            vec.push((old_i-1,old_j+1));
                        }
                        //en passant from left
                        if old_j > 0 && self.get_piece(board1,old_i,old_j-1) == Piece::p {
                            match self.get_pawn_in_last_turn(board1) {
                                Some((a,b)) => vec.push((a-1,b)),
                                _ => (),
                            }
                        }
                        //en passant from right
                        if old_j < 7 && self.get_piece(board1,old_i,old_j+1) == Piece::p {
                            match self.get_pawn_in_last_turn(board1) {
                                Some((a,b)) => vec.push((a-1,b)),
                                _ => (),
                            }
                        }
                        vec


                    },
                    //there can be no white pawns on line
                    (0,_) => {
                       let mut vec =  Vec::new();
                       println!("This indicates pawn upgrade malfunction");
                       vec
                    },
                    //no special moves
                    (_,_) => {
                        //max 4 moves
                        let mut vec = Vec::with_capacity(2);
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
                //same for the black
                match (old_i,old_j) {
                    //double moves
                    (1,_) =>  {
                        //max 4 moves
                        let mut vec = Vec::with_capacity(2);
                        //W double move
                        if self.is_empty(board1,old_i+1,old_j) {
                            vec.push((old_i+1,old_j));
                            if self.is_empty(board1,old_i+2,old_j) {
                                vec.push((old_i+2,old_j));
                            }
                        }
                        //enemy left above
                        if old_j > 0 && self.is_enemy(board1,false,old_i+1,old_j-1) {
                            vec.push((old_i+1,old_j-1));
                        }
                        //enemy right above
                        if old_j < 7 && self.is_enemy(board1,false,old_i+1,old_j+1) {
                            vec.push((old_i+1,old_j+1));
                        }
                        vec
                    },
                    //4th line with enpassant 
                    (4,_) => {
                            //max 5 moves
                            let mut vec = Vec::with_capacity(2);
                            //front empty
                            if self.is_empty(board1,old_i+1,old_j) {
                                vec.push((old_i+1,old_j));
                            }
                            //enemy left above
                            if old_j > 0 && self.is_enemy(board1,true,old_i+1,old_j-1) {
                                vec.push((old_i+1,old_j-1));
                            }
                            //enemy right above
                            if old_j < 7 && self.is_enemy(board1,true,old_i+1,old_j+1) {
                                vec.push((old_i+1,old_j+1));
                            }
                            //en passant, enemy left
                            if old_j > 0 && self.get_piece(board1,old_i,old_j-1) == Piece::P {
                                match self.get_pawn_in_last_turn(board1){
                                    Some((a,b)) => vec.push((a+1,b)),
                                    _ => (),
                                }
                            }
                            //en passant, enemy right
                            if old_j < 7 && self.get_piece(board1,old_i,old_j+1) == Piece::P {
                                match self.get_pawn_in_last_turn(board1) {
                                    Some((a,b)) => vec.push((a+1,b)),
                                    _ => (),
                                }
                            }
                            vec
    
                    },
                    //no pawn can be at the 7th line, promotion!
                    (7,_) => {
                        let mut vec = Vec::new();
                        println!("This indicates pawn upgrade malfunction");
                        vec
                    },  
                    //no special moves
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
            Piece::B | Piece::b | Piece::UB | Piece::Ub => self.x_mov(board1,old_i,old_j),
            Piece::Q | Piece::q | Piece::UQ | Piece::Uq => 
            { let mut vec = self.cross_mov(board1,old_i,old_j);
              vec.append(&mut self.x_mov(board1,old_i,old_j));
              vec },
            Piece::N | Piece::n |Piece::UN | Piece::Un=> self.horse_jump(board1,old_i,old_j),
            Piece::K | Piece::k => self.king_move(board1,old_i,old_j),
            //should not come to here
             _ => {println!("This indicates a bug!"); Vec::new()},
        }
    }

    ///Returns if the location on the given board is empty
    /// # Arguments
    /// * `board1` - true if board1, else false
    /// * `i` - row index
    /// * `j` - col index
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

    ///Returns if the location on the given board is an enemy piece (needs to know if the caller is white)
    /// # Arguments 
    /// * `board1` - true if board1, else false
    /// * `white` - true if white, else false
    /// * `i` - row index of the other piece
    /// * `j` - col index of the other piece
    /// E and L are never the enemies
    fn is_enemy(&self, board1:bool,white:bool, i:usize, j:usize) -> bool {
            if white {
                //true for black, false for white
                match  self.get_board_n(board1).board[i][j] {
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
                //true for white, false for black
                match self.get_board_n(board1).board[i][j] {
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

    ///Return a bool that is true if the given piece is white, else false
    /// # Arguments
    /// * `white` - true if white, else false
    /// * `i` - row index of the other piece
    /// * `j` - col index of the other piece
    fn is_white(&self, board1:bool, i:usize, j:usize) -> bool {
            match self.get_board_n(board1).board[i][j] {
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

    ///Gets legal moves on a line
    /// # Argumnets
    /// * `board1` - true if board1 else false
    /// * `i` - row index of the piece
    /// * `j` - col index of the piece
    fn horizontal_mov(&self,board1:bool, i:usize, j:usize) -> Vec<(usize,usize)> {
        let mut vec = Vec::with_capacity(4);
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

    ///Gets legal moves on a column, horizontal_move but jc is changed with ic
    /// # Arguments
    /// * `board1` - true if board1 else false
    /// * `i` - row index of the piece
    /// * `j` - col index of the piece
    fn vertical_mov(&self,board1:bool, i:usize, j:usize)  -> Vec<(usize,usize)>{
        let mut vec = Vec::with_capacity(4);
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

    ///Makes a cross with vertical and horizontal moves 
    /// # Arguments
    /// * `board1` - true if board1 else false
    /// * `i` - row index of the piece
    /// * `j` - col index of the piece
    fn cross_mov(&self,board1:bool, i:usize, j:usize) -> Vec<(usize,usize)> {
        let mut vec = self.vertical_mov(board1,i,j);
        vec.append(&mut self.horizontal_mov(board1,i,j));
        vec
    }

    ///Iterates for [+n,-n] ; [+n,-n] combinations for 1..7 and gets legal moves
    /// # Arguments
    /// * `board1` - true if board1 else false
    /// * `i` - row index of the piece
    /// * `j` - col index of the piece
    fn x_mov(&self,board1:bool, i:usize, j:usize)  -> Vec<(usize,usize)>{
        let mut vec = Vec::with_capacity(4);
        let mut ic = i;
        let mut jc = j;
        let w = self.is_white(board1,i,j);

        //lower right
        while ic < 7 && jc < 7 {
            ic+=1;
            jc+=1;
            if self.is_empty(board1,ic,jc) {
                vec.push((ic,jc));
            }else{
                if self.is_enemy(board1,w,ic,jc){
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
                if self.is_enemy(board1,w,ic,jc){
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
                if self.is_enemy(board1,w,ic,jc){
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
                if self.is_enemy(board1,w,ic,jc){
                    vec.push((ic,jc));
                }
                break;
            }
        }
        vec
    }

    ///Get possible horse moves, combinations of [+2,-2] ; [-1,+1] for all of the valid locations
    /// # Arguments
    /// * `board1` - true if board1 else false
    /// * `i` - row index of the piece
    /// * `j` - col index of the piece
    fn horse_jump(&self, board1:bool, i:usize, j:usize) -> Vec<(usize,usize)> {
        //all possible combinations of +2, -2, +1, -1 for each square
        let twoway = [-2,2];
        let oneway = [1,-1];
        let mut vec = Vec::with_capacity(2);
        let w = self.is_white(board1,i,j);

        //first -2,2 then 1,-1
        for i_off in twoway.iter() {
            for j_off in oneway.iter() {
                let mut i_signed = i as i32;
                let mut j_signed = j as i32;
                i_signed += i_off;
                j_signed += j_off;
                //check if the indices are valid
                if i_signed >= 0 && i_signed <= 7 && j_signed >= 0 && j_signed <= 7 {
                    //cast back to usize
                    let itmp = i_signed as usize;
                    let jtmp = j_signed as usize;
                    
                    if self.is_empty(board1,itmp,jtmp) || 
                    self.is_enemy(board1,w,itmp,jtmp) {
                        vec.push((itmp,jtmp));
                    }
                   
                }
            }
        }
        //first 1,-1 then -2,2
        for i_off in oneway.iter() {
            for j_off in twoway.iter() {
                let mut i_signed = i as i32;
                let mut j_signed = j as i32;
                i_signed += i_off;
                j_signed += j_off;
                if i_signed >= 0 && i_signed <= 7 && j_signed >= 0 && j_signed <= 7 {
                    let itmp = i_signed as usize;
                    let jtmp = j_signed as usize;
                    if self.is_empty(board1,itmp,jtmp) || 
                    self.is_enemy(board1,w,itmp,jtmp) {
                        vec.push((itmp,jtmp));
                    }
                }
            }
        }
        vec
    }

    ///Returns the piece in given board and location
    /// # Arguments
    /// * `board1` - true if board1 else false
    /// * `i` - row index of the piece
    /// * `j` - col index of the piece
    pub fn get_piece(&self, chessboard1:bool, i:usize, j:usize) -> Piece {
        match chessboard1 {
            true => self.chess_board1.board[i][j],
            false => self.chess_board2.board[i][j],
        }
    }

    ///Converts the input piece to right color
    /// # Arguments
    /// * `board1` - true if board1 else false
    /// * `iswhite` - true if the piece is to be converted to white else false
    /// * `piece` - input piece
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

    ///Converts the input piece to right color, but returns the upgraded if it exists
    /// # Arguments
    /// * `board1` - true if board1 else false
    /// * `iswhite` - true if the piece is to be converted to white else false
    /// * `piece` - input piece
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

    ///Checks wether the given location is under attack
    /// # Arguments
    /// * `board1` - true if board1 else false
    /// * `iswhite` - true if the piece if checking for white else false
    /// * `i` - row index
    /// * `j` - col index
    pub fn is_attacked(&mut self, board1:bool, iswhite:bool, i:usize,j:usize) -> bool {
        //remove yourself from board (you could be blocking other places)
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
                        if self.get_board_n(board1).board[a][b]==Piece::p
                        {
                   
                            return true
                        }
                }
                if self.valid(ic-1,jc+1) {
                    let a = (ic-1) as usize;
                    let b = (jc+1) as usize;
                        if self.get_board_n(board1).board[a][b]==Piece::p
                        {
                     
                            return true
                        }
                }
            }else{
                ic = (i as i32);
                jc = (j as i32);
                if self.valid(ic+1,jc-1) && self.valid(ic,jc) {
                    let a = (ic+1) as usize;
                    let b = (jc-1) as usize;
                        if self.get_board_n(board1).board[a][b]==Piece::P
                        {
                            println!("{},{}: is a piyon",a,b);
                            return true
                        }
                }
                if self.valid(ic+1,jc+1) && self.valid(ic,jc) {
                    let a = (ic+1) as usize;
                    let b = (jc+1) as usize;
                        if self.get_board_n(board1).board[a][b]==Piece::P
                        {
                     
                            return true
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
                    //blocked
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
                    //blocked
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
                    //blocked 
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
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::b),ic,jc) {
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
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::b),ic,jc) {
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
        
        //nothing attacks this place
        false
    }

    ///checks if the given indices are in the legal bounds
    /// # Arguments
    /// * `a` - row index
    /// * `b` - col index
    fn valid(&self, a: i32, b: i32) -> bool {
        if a >= 0 && a <= 7 && b >= 0 && b<= 7 {
            return true
        }else {
            return false
        }
    }

    ///Checks if the given location has the given piece
    /// # Arguments
    /// * `piece` - piece to check
    /// * `a` - row index
    /// * `b` - col index
    fn check_for_piece(&self,board1:bool, piece:Piece, i : i32, j:i32) -> bool {
        if !self.valid(i,j){
            println!("The index is not valid, this idicates a bug");
            return false
        }else{
                if self.get_board_n(board1).board[i as usize][j as usize]==piece
                {
                    return true
                }else {
                    return false
                }
        }
    }

    ///Empties the whole board
    /// # Arguments
    /// * `board1` - true if board1 else false
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

  
    ///Sets the piece on the given location
    /// # Arguments
    /// * `board1` - true if board1 else false
    /// * `piece` - the piece to set
    /// * `i` - row index
    /// * `j` - col index
    fn set_piece(&mut self,board1:bool,piece:Piece,i:usize,j:usize){
        if board1{
            self.chess_board1.board[i][j] = piece;
        }else{
            self.chess_board2.board[i][j] = piece;
        }
    }

    ///Checks if the king can  
    /// # Arguments
    /// * `board1` - true if board1 else false
    /// * `i` - row index
    /// * `j` - col index
    /// 
    /// In Bughouse kind move to a location which is attacked
    fn king_move(&mut self, board1:bool, i:usize,j:usize) -> Vec<(usize,usize)>{
        let mut a = [-1,0,1];
        let mut b = [-1,0,1];
        let mut vec = Vec::with_capacity(2);
        let ix = i as i32;
        let jx = j as i32;
        let mut piece = Piece::E;
        let mut wayt = true;

        //remove yourself for is_attacked check if needed
        if board1 {
            piece = self.chess_board1.board[i][j];
            self.chess_board1.board[i][j] = Piece::E;
        }else{
            piece = self.chess_board2.board[i][j];
            self.chess_board2.board[i][j] = Piece::E;
        }

        //set white
        if piece == Piece::K {
            wayt = true;
        }else{
            wayt = false;
        }

        //check for all possible combinations
        for i_off in a.iter(){
            for j_off in b.iter(){
                if self.valid(i_off+ix,j_off+jx) && !(*i_off==0 && *j_off==0){
                    let mut ic = i_off+ix;
                    let mut jc = j_off+jx;
                    //if !self.is_attacked(board1,wayt,ic as usize,jc as usize)  
                    //keep this in case we want legal moves only
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

        //set yourself back on the field -- needed only for is_attacked tho
        if board1 {
            self.chess_board1.board[i][j] = piece;
        }else{
            self.chess_board2.board[i][j] = piece;
        }
      
        //check for castling
            if wayt {
                if !self.get_board(board1).white_k_moved 
                && self.get_board(board1).board[7][7] == Piece::R 
                && self.get_board(board1).board[7][6] == Piece::E
                && self.get_board(board1).board[7][5] == Piece::E
                && !self.is_attacked(board1,wayt,7,6) 
                && !self.is_attacked(board1,wayt,7,5)
                && !self.is_attacked(board1,wayt,7,4)
                && !self.get_board(board1).white_rook_k_moved
                {
                    vec.push((7,6))
                }
                if !self.get_board(board1).white_k_moved 
                && self.get_board(board1).board[7][0] == Piece::R 
                && self.get_board(board1).board[7][1] == Piece::E
                && self.get_board(board1).board[7][2] == Piece::E
                && self.get_board(board1).board[7][3] == Piece::E
                && !self.is_attacked(board1,wayt,7,3) 
                && !self.is_attacked(board1,wayt,7,2)
                && !self.is_attacked(board1,wayt,7,4)
                && !self.is_attacked(board1,wayt,7,1)
                && !self.get_board(board1).white_rook_q_moved {
                    vec.push((7,2))
                }
            }else{
                if !self.get_board(board1).black_k_moved 
                && self.get_board(board1).board[0][7] == Piece::r 
                && self.get_board(board1).board[0][6] == Piece::E
                && self.get_board(board1).board[0][5] == Piece::E
                && !self.is_attacked(board1,wayt,0,6) 
                && !self.is_attacked(board1,wayt,0,5)
                && !self.is_attacked(board1,wayt,0,4)
                && !self.get_board(board1).black_rook_k_moved
                {
                    vec.push((0,6))
                }
                if !self.get_board(board1).black_k_moved 
                && self.get_board(board1).board[0][0] == Piece::r 
                && self.get_board(board1).board[0][1] == Piece::E
                && self.get_board(board1).board[0][2] == Piece::E
                && self.get_board(board1).board[0][3] == Piece::E
                && !self.is_attacked(board1,wayt,0,3) 
                && !self.is_attacked(board1,wayt,0,2)
                && !self.is_attacked(board1,wayt,0,4)
                && !self.is_attacked(board1,wayt,0,1)
                && !self.get_board(board1).black_rook_q_moved {
                    vec.push((0,2))
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

    pub fn get_board_n(&self, board1:bool )-> &ChessBoard{
        match board1 {
            true => &self.chess_board1,
            false => &self.chess_board2,
        }
    }

    ///Checks if you can move your piece
    /// # Arguments
    /// * `board1` - true if board1 else false
    /// * `old_i` - oldrow index
    /// * `old_j` - old col index
    /// * `i` - row index, where the piece wants to move
    /// * `j` - col index, same as above
    ///in tandem you can move your pinned piece, then the enemy can capture your king
    ///checks if you can move from i_old, j_old to i,j
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

    ///Deploys a piece on your field as a turn returns an Error if you cant deploy 
    /// # Arguments
    /// * `board1` - true if to deploy on board1 else false
    /// * `white` - true if the piece to be deployed white else false
    /// * `p` - the piece to deploy
    /// * `i` - the row to deploy
    /// * `j` - the col to deploy
    /// 
    /// Deploys a piece at the location (i,j) if the location (i,j) is within bounds, empty and it is legal to deploy at index (i,j)
    /// For example a white pawn, P cannot be deployed to (0,_). It also checks if the deployable piece pool has
    /// enough pieces of the type p (>=1 p:Piece).
    /// It updates: pools, winner, count of turns, count of half-turns since last capture.
    /// A king cannot be deployed but if the rook is deployed on the initial position it is possible to castle, it also updated by the deploy piece.
    /// A deploy cannot terminate the game, since you cannot capture the king with a deploy. The game terminates only when a king is captured, a plyer resigns, or stalemate occurs which is prob. never going to happen.
    pub fn deploy_piece(&mut self,board1:bool,white:bool,p:Piece,i:usize,j:usize) -> Result<bool,MoveError> {
        //deploy the piece only if it is legal to play
        if self.winner!=Winner::N {
            return Ok(true)
        }

        //check for i j bounds
        if !self.valid(i as i32,j as i32) {
            return Err(MoveError::NotLegal)
        }

        //check if the location is empty
        if board1 {
            if self.get_board(board1).board[i][j] != Piece::E {
               return Err(MoveError::CannotDeploy)
            }

            if white {
                if !self.white_active_1 {  
                    return Err(MoveError::NotTurn)
                } else {
            
                    if let Some(ind) = self.box_index(p) {
                        if self.board1_white_capture[ind]>0 {
                            self.board1_white_capture[ind]-=1;
                        }else{
                            return Err(MoveError::NoPieceInPool)
                        }

                        self.chess_board1.board[i][j] = p;
                        self.white_active_1 = !self.white_active_1;

                        if p == Piece::P {
                            if i==0 {
                                return Err(MoveError::NotLegal);
                            }

                            self.pawn_in_last_turn_b1 = Some((i,j));
                            self.half_moves_last_capture2=0;
                        }else{
                            self.pawn_in_last_turn_b1 = None;
                            self.half_moves_last_capture1+=1;
                        }

                        if p == Piece::R {
                            if i==7 && j==7 {
                                self.get_board(board1).white_rook_k_moved = false;
                            }
                            if i==7 && j==7 {
                                self.get_board(board1).white_rook_q_moved = false;
                            }
                        }

                        return Ok(true)
                    } else {return Err(MoveError::CannotDeploy)}
                }
            }else{
                if self.white_active_1 {
                     return Err(MoveError::CannotDeploy)
                }else {

                    if  let Some(ind) = self.box_index(p) {
                        if self.board1_black_capture[ind]>0 {
                            self.board1_black_capture[ind]-=1;
                        }else{
                            return Err(MoveError::NoPieceInPool)
                        }

                        self.chess_board1.board[i][j] = p;
                        self.white_active_1 = !self.white_active_1;

                        if p == Piece::p {
                            if i==7 {
                                return Err(MoveError::NotLegal);
                            }

                            self.pawn_in_last_turn_b1 = Some((i,j));
                            self.half_moves_last_capture1=0;
                        }else{
                            self.pawn_in_last_turn_b1 = None;
                            self.half_moves_last_capture1+=1;
                        }

                        if p == Piece::r {
                            if i==0 && j==7 {
                                self.get_board(board1).black_rook_k_moved = false;
                            }
                            if i==0 && j==0 {
                                self.get_board(board1).black_rook_q_moved = false;
                            }
                        }
                        self.movectr1+=1;
                        return Ok(true)
                    } else {return Err(MoveError::CannotDeploy)}
                }   
            } 
        }else {
            if self.chess_board2.board[i][j] != Piece::E {
                return Err(MoveError::CannotDeploy)
            }

            if white {
                if !self.white_active_2 {  
                    return Err(MoveError::CannotDeploy)
                } else {
             
                    if let Some(ind) = self.box_index(p) {
                        if self.board2_white_capture[ind]>0 {
                            self.board2_white_capture[ind]-=1;
                        }else{
                           return Err(MoveError::NoPieceInPool)
                        }

                        self.chess_board2.board[i][j] = p;
                        self.white_active_2 = !self.white_active_2;

                        self.chess_board2.board[i][j] = p;
                        self.white_active_2 = !self.white_active_2;
    
                        if p == Piece::P {
                            if i==0 {
                                return Err(MoveError::NotLegal);
                            }

                            self.pawn_in_last_turn_b2 = Some((i,j));
                            self.half_moves_last_capture2=0;
                        }else{
                            self.pawn_in_last_turn_b2 = None;
                            self.half_moves_last_capture2+=1;
                        }

                        if p == Piece::R {
                            if i==7 && j==7 {
                                self.chess_board2.white_rook_k_moved = false;
                            }
                            if i==7 && j==0 {
                                self.chess_board2.white_rook_q_moved = false;
                            }
                        }
                        return Ok(true)
                    }else { 
                        return Err(MoveError::CannotDeploy)
                    }
                }
            }else{
                if self.white_active_2 { 
                    return Err(MoveError::NotTurn)
                } else {
                  
                    if let Some(ind) = self.box_index(p) {
                        if self.board2_black_capture[ind]>0 {
                            self.board2_black_capture[ind]-=1;
                        }else{
                           return Err(MoveError::NotTurn)
                        }

                        self.chess_board2.board[i][j] = p;
                        self.white_active_2 = !self.white_active_2;
                        
                        if p == Piece::p {
                            if i==7 {
                                return Err(MoveError::NotLegal);
                            }

                            self.pawn_in_last_turn_b2 = Some((i,j));
                            self.half_moves_last_capture2=0;
                        }else{
                            self.pawn_in_last_turn_b2 = None;
                            self.half_moves_last_capture2+=1;
                        }

                        if p == Piece::r {
                            if i==0 && j==7 {
                                self.chess_board2.black_rook_k_moved = false;
                            }
                            if i==0 && j==0 {
                                self.chess_board2.black_rook_q_moved = false;
                            }
                        }
                        self.movectr2+=1;
                       return Ok(true)
                    } else {
                        return Err(MoveError::CannotDeploy)
                    }
                }
            }
        }
        
        return Err(MoveError::CannotDeploy)
    }



    ///Moves the piece from i_old,j_old to i,j if the move is legal
    /// # Arguments
    /// * `board1` - true if to deploy on board1 else false  
    /// * `i_old` - the row before the move
    /// * `j_old` - the col before the move
    /// * `i` - the row to move
    /// * `j` - the col to move
    /// 
    /// Moves the piece p from (i_old,j_old) to (i,j) if (i_old,j_old) has a piece and (i,j) is within bounds and legal moving to.
    /// Pinned pieces can move in bughouse, and the king can be captured, meaning that the king can move into a square that is attacked
    /// Function will return an error if a winner was already set (meaning not equal to Winner::N). 
    /// The function updates castling rights on both sides for both colors, if the callers color is not equal to the active color
    /// the function will return an error. It also updates half-turns and total turns. 
    /// In case of a promotion the fields upgrade_to1 or upgrade_to2 has to be set BEFORE, after a successful promotion the corresponding field
    /// will be reset (set to Piece::E), if the field is not set to a legal piece the function will return an Error
    /// So the promotion precondition has to be fulfilled before calling the movemaker function. A captures piece is automatically sent to the 
    /// teammates deployable pieces pool
    pub fn movemaker(&mut self, board1:bool, i_old:usize,j_old:usize,i:usize,j:usize) -> Result<bool,MoveError> {
        if self.winner!=Winner::N {
            return Err(MoveError::AlreadyOver)
        }
        
        //check of move is legal
        if self.legality_check(board1,i_old,j_old,i,j) {
            if board1 {
                match self.get_board(board1).board[i_old][j_old] {
                    Piece::R => {
                        if !self.white_active_1 {
                            return Err(MoveError::NotTurn)
                        }else{
                            if i_old == 7 && j_old == 7 {
                                self.get_board(board1).white_rook_k_moved=true;
                            }
                            if i_old == 7 && j_old == 0 {
                                self.get_board(board1).white_rook_q_moved=true;
                            }
                        }
                    },
                    Piece::r => {
                        if self.white_active_1 {
                            return Err(MoveError::NotTurn)
                        }else{
                            if i_old == 0 && j_old == 7 {
                                self.get_board(board1).black_rook_k_moved=true;
                            }
                            if i_old == 0 && j_old == 0 {
                                self.get_board(board1).black_rook_q_moved=true;
                            }
                        }
                    },

                    Piece::K => 
                    {
                        if !self.white_active_1 {
                            return Err(MoveError::NotTurn)
                        }else{
                            self.pawn_in_last_turn_b1 = None;
                           
                            //check if the move is a castling move
                            if i_old==7 && j_old==4 && i== 7 && (j==6 || j==2)
                            && !self.chess_board1.white_k_moved {
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
                                return Ok(true)
                            }
                            self.chess_board1.white_k_moved = true;
                        }
                    },
                    Piece::k => 
                    {
                        if self.white_active_1 {
                            return Err(MoveError::NotTurn)
                        }else{
                            self.pawn_in_last_turn_b1 = None;

                            if i_old==0 && j_old==4 && i== 0 && (j==6 || j==2)
                            && !self.chess_board1.black_k_moved
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
                                return Ok(true)
                            }
                        
                            self.chess_board1.black_k_moved = true;
                        }
                    },
                    Piece::P => 
                    {
                        if !self.white_active_1 {
                            return Err(MoveError::NotTurn)
                        }else{
                            self.pawn_in_last_turn_b1 = Some((i,j));
                            if i==0 {
                                if self.upgrade_to1 == Piece::Q ||  self.upgrade_to1 == Piece::R  || self.upgrade_to1 == Piece::B  ||self.upgrade_to1 == Piece::N {
                                    self.half_moves_last_capture1=0;
                                    match self.box_index(self.chess_board1.board[i][j]) {
                                        None => {},
                                        Some(x) => {self.board2_black_capture[x] += 1},
                                    }
                                }else{
                                    return Err(MoveError::PromotionProblem);
                                }
                            
                                match self.upgrade_to1 {
                                    Piece::Q => { self.upgrade_to1 = Piece::E; self.chess_board1.board[i][j] = Piece::UQ;
                                        self.chess_board1.board[i_old][j_old] = Piece::E; 
                                        self.white_active_1 = !self.white_active_1; return Ok(true)},
                                    Piece::R => { self.upgrade_to1 = Piece::E; self.chess_board1.board[i][j] = Piece::UR;
                                        self.chess_board1.board[i_old][j_old] = Piece::E;
                                        self.white_active_1 = !self.white_active_1; return Ok(true)},
                                    Piece::B => { self.upgrade_to1 = Piece::E;self.chess_board1.board[i][j] = Piece::UB;
                                        self.chess_board1.board[i_old][j_old] = Piece::E;
                                        self.white_active_1 = !self.white_active_1; return Ok(true)},
                                    Piece::N => { self.upgrade_to1 = Piece::E;self.chess_board1.board[i][j] = Piece::UN;
                                        self.chess_board1.board[i_old][j_old] = Piece::E;
                                        self.white_active_1 = !self.white_active_1; return Ok(true)},
                                    _ =>  {return Err(MoveError::PromotionProblem)},
                            } 
                            }
                        }
                    },
                    Piece::p => 
                    {
                        if self.white_active_1 {
                            return Err(MoveError::NotTurn)
                        }else {
                            self.pawn_in_last_turn_b1 = Some((i,j));
                            if i==7 {
                                if self.upgrade_to1 == Piece::q ||  self.upgrade_to1 == Piece::r  || self.upgrade_to1 == Piece::b  ||self.upgrade_to1 == Piece::n {
                                    self.half_moves_last_capture1=0;
                                    match self.box_index(self.chess_board1.board[i][j]) {
                                        None => {},
                                        Some(x) => {self.board2_white_capture[x] += 1},
                                    }
                                }else{
                                    return Err(MoveError::PromotionProblem);
                                }

                                match self.upgrade_to1 {
                                    Piece::q => {self.upgrade_to1 = Piece::E;self.chess_board1.board[i][j] = Piece::Uq;
                                    self.chess_board1.board[i_old][j_old] = Piece::E;
                                    self.white_active_1 = !self.white_active_1; return Ok(true)},
                                    Piece::r => {self.upgrade_to1 = Piece::E;self.chess_board1.board[i][j] = Piece::Ur; 
                                        self.chess_board1.board[i_old][j_old] = Piece::E;
                                        self.white_active_1 = !self.white_active_1;return Ok(true)},
                                    Piece::b => {self.upgrade_to1 = Piece::E;self.chess_board1.board[i][j] = Piece::Ub; 
                                        self.chess_board1.board[i_old][j_old] = Piece::E;
                                        self.white_active_1 = !self.white_active_1;return Ok(true)},
                                    Piece::n => {self.upgrade_to1 = Piece::E;self.chess_board1.board[i][j] = Piece::Un;
                                        self.chess_board1.board[i_old][j_old] = Piece::E; 
                                        self.white_active_1 = !self.white_active_1;return Ok(true)},
                                    _ =>  {return Err(MoveError::PromotionProblem)},
                                } 
                            }
                        }
                    },
                    _ => {},
                }

               
                if !((self.white_active_1 && self.is_white(board1,i_old,j_old)) 
                || (!self.white_active_1 && !self.is_white(board1,i_old,j_old))) {
                    //not your turn!!!
                    //println!("Siktir git hamlen degil!");
                    return Err(MoveError::NotTurn)
                }
                //check to move location
                let tmp = self.chess_board1.board[i][j];

                //check if any piece is captured 
                if tmp!=Piece::E {
                    self.half_moves_last_capture1=0;
                }else {
                    self.half_moves_last_capture1+=1;
                }
               

                //check if pawn is moved
                if self.chess_board1.board[i_old][j_old]==Piece::P {

                    self.pawn_in_last_turn_b1 = Some((i,j));
                    self.half_moves_last_capture1=0;

                }else if self.chess_board1.board[i_old][j_old]==Piece::p {

                    self.pawn_in_last_turn_b1 = Some((i,j));
                    self.half_moves_last_capture1=0;

                //if the king has moved change king has moved flag
                }else if self.chess_board1.board[i_old][j_old]==Piece::K{
                    
                    self.pawn_in_last_turn_b1 = None;
                    self.chess_board1.white_k_moved = true;
                
                }else if self.chess_board1.board[i_old][j_old]==Piece::k{

                    self.pawn_in_last_turn_b1 = None;
                    self.chess_board1.black_k_moved = true;
           
                }else{
                    //no king no pawn movement -> pawn in last turn = None
                    self.pawn_in_last_turn_b1 = None;
                }

              

                //check if black has moved
                if !self.is_white(board1,i_old,j_old){
                    self.movectr1+=1;
                }

                //check if game  should end
                if tmp==Piece::K || tmp==Piece::k  {
                    self.finish_up(tmp,board1);
                }


                if self.check_patt(board1,true) || self.check_patt(board1,false) {
                    self.winner = Winner::P;
                    return Err(MoveError::AlreadyOver)
                }


                //send captured piece to your ally
                match self.box_index(tmp) {
                    None => (), //nothing to di
                    Some(a) => {
                        if self.is_white(board1,i,j) {
                            self.board2_white_capture[a]+=1;
                        }else{
                            self.board2_black_capture[a]+=1;
                        }
                    },
                }

                //apply move
                self.chess_board1.board[i][j]=self.chess_board1.board[i_old][j_old];
                self.chess_board1.board[i_old][j_old]=Piece::E;
                self.white_active_1 = !self.white_active_1;
                return Ok(true)

            }else{
                match self.chess_board2.board[i_old][j_old] {
                    Piece::R => {
                        if !self.white_active_2 {
                            return Err(MoveError::NotTurn)
                        }else{
                            if i_old == 7 && j_old == 7 {
                                self.chess_board2.white_rook_k_moved=true;
                            }
                            if i_old == 7 && j_old == 0 {
                                self.chess_board2.white_rook_q_moved=true;
                            }
                        }
                    },
                    Piece::r => {
                        if self.white_active_2 {
                            return Err(MoveError::NotTurn) 
                        }else{
                            if i_old == 0 && j_old == 7 {
                                self.chess_board2.black_rook_k_moved=true;
                            }
                            if i_old == 0 && j_old == 0 {
                                self.chess_board2.black_rook_q_moved=true;
                            }
                        }
                    },

                    Piece::K => 
                    {
                        if !self.white_active_2 {
                            return Err(MoveError::NotTurn)
                        }else{
                            self.pawn_in_last_turn_b2 = None;

                            if i_old==7 && j_old==4
                            && i== 7 && (j==6 || j==2)
                            && !self.chess_board2.white_k_moved
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
                                return Ok(true)
                            }
                            self.chess_board2.white_k_moved = true;
                        }
                    },
                    Piece::k => 
                    {
                        if self.white_active_2 {
                            return Err(MoveError::NotTurn)
                        }else{
                            self.pawn_in_last_turn_b2 = None;
                            if i_old==0 && j_old==4
                            && i== 0 && (j==6 || j==2)
                            && !self.chess_board2.black_k_moved
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
                                return Ok(true)
                        }
                        
                        self.chess_board2.black_k_moved = true;
                        }
                    },
                    Piece::P => 
                    {
                        if !self.white_active_2 {
                            return Err(MoveError::NotTurn)
                        }else{
                            if i==0 {
                                self.pawn_in_last_turn_b2 = Some((i,j));
                                if self.upgrade_to2 == Piece::Q ||  self.upgrade_to2 == Piece::R  
                                || self.upgrade_to2 == Piece::B || self.upgrade_to2 == Piece::N {
                                    self.half_moves_last_capture2=0;
                                    match self.box_index(self.chess_board2.board[i][j]) {
                                        None => {},
                                        Some(x) => {self.board1_black_capture[x] += 1},
                                    }
                                }else{
                                    return Err(MoveError::PromotionProblem);
                                }

                                match self.upgrade_to2 {
                                    Piece::Q => { self.upgrade_to2 = Piece::E;self.chess_board2.board[i][j] = Piece::UQ;
                                        self.chess_board2.board[i_old][j_old] = Piece::E;
                                        self.white_active_2 = !self.white_active_2;return Ok(true)},
                                    Piece::R => { self.upgrade_to2 = Piece::E;self.chess_board2.board[i][j] = Piece::UR;
                                        self.chess_board2.board[i_old][j_old] = Piece::E;
                                        self.white_active_2 = !self.white_active_2;return Ok(true)},
                                    Piece::B => { self.upgrade_to2 = Piece::E;self.chess_board2.board[i][j] = Piece::UB;
                                        self.chess_board2.board[i_old][j_old] = Piece::E;
                                        self.white_active_2 = !self.white_active_2;return Ok(true)},
                                    Piece::N => { self.upgrade_to2 = Piece::E;self.chess_board2.board[i][j] = Piece::UN;
                                        self.chess_board2.board[i_old][j_old] = Piece::E;
                                        self.white_active_2 = !self.white_active_2;return Ok(true)},
                                    _ =>  {return Err(MoveError::PromotionProblem)},
                            } 
                            }
                        }
                    },
                    Piece::p => 
                    {
                        if self.white_active_2 {
                            return Err(MoveError::NotTurn)
                        }else{
                            self.pawn_in_last_turn_b2 = Some((i,j));
                            if i==7 {
                                if self.upgrade_to2 == Piece::q || self.upgrade_to2 == Piece::r 
                                || self.upgrade_to2 == Piece::b || self.upgrade_to2 == Piece::n {
                                    self.half_moves_last_capture2=0;
                                    match self.box_index(self.chess_board2.board[i][j]) {
                                        None => {},
                                        Some(x) => {self.board1_white_capture[x] += 1},
                                    }
                                }else{
                                    return Err(MoveError::PromotionProblem);
                                }

                                match self.upgrade_to2 {
                                    Piece::q => { self.upgrade_to2 = Piece::E;self.chess_board2.board[i][j] = Piece::Uq; 
                                        self.chess_board2.board[i_old][j_old] = Piece::E;
                                        self.white_active_2 = !self.white_active_2;return Ok(true)},
                                    Piece::r => { self.upgrade_to2 = Piece::E;self.chess_board2.board[i][j] = Piece::Ur; 
                                            self.chess_board2.board[i_old][j_old] = Piece::E;
                                            self.white_active_2 = !self.white_active_2;return Ok(true)},
                                    Piece::b => { self.upgrade_to2 = Piece::E;self.chess_board2.board[i][j] = Piece::Ub; 
                                        self.chess_board2.board[i_old][j_old] = Piece::E;
                                        self.white_active_2 = !self.white_active_2;return Ok(true)},
                                    Piece::n => { self.upgrade_to2 = Piece::E;self.chess_board2.board[i][j] = Piece::Un;
                                        self.chess_board2.board[i_old][j_old] = Piece::E; 
                                        self.white_active_2 = !self.white_active_2;return Ok(true)},
                                    _ =>  {return Err(MoveError::PromotionProblem)},
                                } 
                        }
                    }
                    },
                    _ => {},
                }

                if !( (self.white_active_2 && self.is_white(board1,i_old,j_old)) 
                || (!self.white_active_2 && !self.is_white(board1,i_old,j_old))) {
                    //not your turn!!!
                    //println!("Siktir git hamlen degil!");
                    return Err(MoveError::NotTurn)
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
                    self.pawn_in_last_turn_b2 = Some((i,j));
                    self.half_moves_last_capture2=0;
                }else if self.chess_board2.board[i_old][j_old]==Piece::K {
                    self.pawn_in_last_turn_b2 = None;
                    self.chess_board2.white_k_moved = true;
                }else if self.chess_board2.board[i_old][j_old]==Piece::k {
                    self.pawn_in_last_turn_b2 = None;
                    self.chess_board2.black_k_moved = true;
                }else{
                    self.pawn_in_last_turn_b2 = None;
                }

                //check if black has moved
                if !self.is_white(board1,i_old,j_old){
                    self.movectr2+=1;
                }

 

                //check if game  should end
                if tmp==Piece::K || tmp==Piece::k  {
                    self.finish_up(tmp,board1);
                }

                if self.check_patt(board1,true) || self.check_patt(board1,false) {
                    self.winner = Winner::P;
                    return Err(MoveError::AlreadyOver)
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
                return Ok(true)
            }
        }else{
            println!("Move not legal!");
            return Err(MoveError::NotLegal)
        }
    }

    ///Get the index of the piece on the captured pieces array
    /// # Arguments 
    /// * `piece` - the index of the given piece
    /// 
    /// The upgraded pieces will be sent over as pawns,
    /// Did not want to import hashtable for such a trivial example
    pub fn box_index(&self, piece:Piece) -> Option<usize> {
        //P-R-N-B-Q
        match piece {
            Piece::P | Piece::p  => Some(0),
            Piece::K | Piece::k | Piece::L  | Piece::E => None ,
            Piece::N | Piece::n  => Some(2),
            Piece::R | Piece::r  => Some(1),
            Piece::B | Piece::b  => Some(3),
            Piece::Q | Piece::q  => Some(4),
            Piece::Un | Piece::UN
            | Piece::Ur | Piece::UR
            | Piece::Ub | Piece::UB
            | Piece::Uq | Piece::UQ => Some(0),

        }
    }


    ///Sets the winner, should only be called through movemaker/deploy piece
    /// # Arguments 
    /// * `p` - a Piece to indicate which King is captures
    /// * `board1` - true if board1 else false
    fn finish_up(&mut self, p:Piece, board1:bool){
        if p==Piece::K {
            if board1 {
                self.winner=Winner::B1;
            }else{
                self.winner=Winner::B2;
            }
        }else if p==Piece::k {
            if board1 {
                self.winner=Winner::W1;
            }else{
                self.winner=Winner::W2;
            }
        }
    }

    /// A function for a player to resign
    /// # Arguments
    /// * `board1` - true if a player from board1 resigns else false
    /// * `white` - true if white resigns, else false
    pub fn resign(&mut self,board1:bool,white:bool){
        if board1 {
            if white {
                self.winner=Winner::B1;
            }else{
                self.winner=Winner::W1;
            }
        }else{
            if white {
                self.winner=Winner::B2;
            }else{
                self.winner=Winner::W2;
            }
        }
    }

    ///A function to check if there is a stalemate and sets the winner to P(att)
    /// # Arguments
    /// * `board1` - true if a  board1 else false 
    /// * `white` - true if white  else false
    pub fn check_patt(&mut self,board1:bool, white:bool) -> bool {
        let mut pic = Piece::E;
        if white { pic = Piece::K; } else { pic = Piece::k; }

        for i in 0..8 {
            for j in 0..8 {
                if white {
                    if self.is_white(board1,i,j) && self.get_board(board1).board[i][j]!=Piece::K {
                        if self.get_legal_moves(board1,i,j)!=Vec::new() {
                            return false
                        }
                        
                    }
                }else{
                    if !self.is_white(board1,i,j) &&  self.get_board(board1).board[i][j]!=Piece::k {
                        if self.get_legal_moves(board1,i,j)!=Vec::new() {
                            return false
                        }
                    }
                }
            }
        }

        match self.find_piece(pic,board1) {
            None => return false,
            Some((i,j)) => {
                if !self.is_attacked(board1,white,i,j) && 
                self.pool_empty(board1,white){
                    self.winner=Winner::P;
                    return true
                }
            },
        }
        return false
    }


    ///A function to check if the pool for a player is empty 
    ///Needed for stalemate check
    /// # Arguments
    /// * `board1` - true if a  board1 else false 
    /// * `white` - true if white  else false
    fn pool_empty(&self, board1:bool,white:bool) -> bool {
        for i in 0..5 {
            if board1 {
                if white {
                   if self.board1_white_capture[i]!=0 {return false}
                }else{
                   if self.board1_black_capture[i]!=0 {return false}
                }
            }else{
                if white {
                    if self.board2_white_capture[i]!=0 {return false}
                }else{
                    if self.board2_black_capture[i]!=0 {return false}
                }
            }
        }
        true
    }

    ///A function to send a piece from one teammate to another
    ///Needed for stalemate check
    /// # Arguments
    /// * `board1` - true if the player is from board1 else false 
    /// * `white` - true if player is white  else false
    fn recv_piece(&mut self, board1:bool, white:bool,p:Piece){
        let i = self.box_index(p);
        if let Some(x) = i { 
            if board1 {
                if white { 
                    self.board1_white_capture[x]+=1;
                }else{
                    self.board1_black_capture[x]+=1;
                }
            }else{
                if white {
                    self.board2_white_capture[x]+=1;
                }else{
                    self.board2_black_capture[x]+=1;
                }
            }
        }
    }

    ///A function to find a piece on a board, returns the first piece found if there are more than 1 of the same type
    ///Inteded use is to find the kings
    /// # Arguments
    /// * `p` - the piece to be searched
    /// * `board1` - true if the player is from board1 else false 
    pub fn find_piece(&self, p: Piece, board1:bool) -> Option<(usize,usize)> {
        for i in 0..8 {
            for j in 0..8 {
                if board1 {
                    if self.chess_board1.board[i][j] == p {
                        return Some((i,j))
                     }
                }else{
                    if self.chess_board2.board[i][j] == p {
                        return Some((i,j))
                    }
                }
              
            }
        }
        None 
    }

    ///Returns the castling rights
    /// # Arguments
    /// * `board1` - true if board1 else false 
    pub fn get_castling_rights(&self,board1:bool) -> [bool;4] {
        let mut x = [false;4];
        x[0] = self.get_board_n(board1).white_rook_k_moved;
        x[1] = self.get_board_n(board1).white_rook_q_moved;
        x[2] = self.get_board_n(board1).black_rook_k_moved;
        x[3] = self.get_board_n(board1).black_rook_q_moved;
        return x
    }

    ///A function the set the promotion field (to_upgrade1-2), corrects type into some extent
    /// # Arguments
    /// * `board1` - true if board1 else false 
    /// * `p` - the piece to be promoted to pawn -> p:Piece
    pub fn set_promotion(&mut self, board1:bool, p:Piece) -> bool {
        if board1 {
            match p {
                Piece::Q | Piece::UQ => self.upgrade_to1 = Piece::Q, 
                Piece::B | Piece::UB => self.upgrade_to1 = Piece::B, 
                Piece::R | Piece::UR => self.upgrade_to1 = Piece::R, 
                Piece::N | Piece::UN => self.upgrade_to1 = Piece::N, 
                _ => {return false},
            }
        }else{
            match p {
                Piece::q | Piece::Uq => self.upgrade_to2 = Piece::q, 
                Piece::b | Piece::Ub => self.upgrade_to2 = Piece::b, 
                Piece::r | Piece::Ur => self.upgrade_to2 = Piece::r, 
                Piece::n | Piece::Un => self.upgrade_to2 = Piece::n, 
                _ => {return false},
            }
        }
        return true 
    }

    /// A function to reset the promotion to Piece::E
    /// # Arguments
    /// * `board1` - true if board1 else false 
    pub fn reset_promotion(&mut self, board1:bool) {
        if board1 {
            self.upgrade_to1=Piece::E;
        }else{
            self.upgrade_to2=Piece::E;
        }
    }
    
}







