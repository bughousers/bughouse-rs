use crate::logic::board::aux;

#[derive(Clone, Copy, PartialEq)]
///Enum class for possible movement errors, could be usefull for calls from a server
/// 
///NotLegal -> move is not a legal move
///NotTurn -> not the turn of the caller
///CannotDeploy -> the given piece cannot be deployed, e.g. the cell is not empty
///NoPieceInPool -> the pool for needed piece is empty
///PromotionProblem -> the promotion booleans are not set (the boolean has to be set to promote pawns)
pub enum MoveError {
    NotLegal,NotTurn,AlreadyOver,NoPieceInPool,
    PromotionProblem,NotEmpty
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceType {
    Pawn, Rook, Knight, Bishop, Queen, King
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    White, Black
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
    pub upgraded: bool,
}

impl Piece {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for Piece {
    fn default() -> Self {
        Piece {
            piece_type: PieceType::Rook,
            color: Color::White,
            upgraded: false
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Field {
    Empty, Piece(Piece), Legal
}

pub struct ChessBoard {
    pub board: [[Field; 8]; 8],
    
    pub white_king_moved: bool,
    pub white_rook_short_moved: bool, 
    pub white_rook_long_moved: bool,
    pub black_king_moved: bool,
    pub black_rook_short_moved: bool, 
    pub black_rook_long_moved: bool, 
    pub last_moved_pawn: (usize,usize),
    pub current_turn: Color,
    pub counter: usize,
    pub pattcounter: usize,
    pub over: bool,

    white_deployable: [u8; 5],
    black_deployable: [u8; 5],
}

impl ChessBoard {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for ChessBoard {
    fn default() -> Self {
        ChessBoard {
            board: aux::gen_starting_pos(),
            white_king_moved: false,
            white_rook_short_moved: false,
            white_rook_long_moved: false,
            black_king_moved: false,
            black_rook_short_moved: false,
            black_rook_long_moved: false,
            last_moved_pawn: (8,8),
            current_turn: Color::White,
            counter: 0,
            pattcounter: 0,
            over: false,
            white_deployable: [0,0,0,0,0],
            black_deployable: [0,0,0,0,0]
        }
    }
}

impl ChessBoard {
    pub fn reset(&mut self) {
        aux::set_starting_pos(&mut self.board);
        self.white_king_moved = false;
        self.white_rook_short_moved = false;
        self.white_rook_long_moved = false;
        self.black_king_moved = false;
        self.black_rook_short_moved = false;
        self.black_rook_long_moved = false;
        self.last_moved_pawn = (8,8);
        self.counter = 0;
        self.over = false;
        self.pattcounter = 0;

        self.white_deployable.iter_mut().for_each(|x| *x = 0);
        self.black_deployable.iter_mut().for_each(|x| *x = 0);
    }

    fn empty(&self, ic:usize, jc:usize) -> bool {
        match self.board[ic][jc] {
            Field::Empty => return true,
            _ => return false
        }
    }

    fn enemy(&self, i:usize, j:usize,ic:usize, jc:usize) -> bool {
        match self.board[ic][jc] {
            Field::Empty => return false,
            Field::Legal => return false,
            Field::Piece(Piece{piece_type:_, color:c, upgraded:_}) => {
               match self.board[i][j] {
                    Field::Piece(Piece{piece_type:_, color:c2, upgraded:_}) => {
                        if c2 != c {
                            return true
                        }else{
                            return false
                        }
                   },
                   _ => return false,
               }
            }
        }
    }

    ///Gets legal moves on a line
    /// # Argumnets
    /// * `i` - row index of the piece
    /// * `j` - col index of the piece
    fn horizontal_move(&self, i:usize, j:usize) -> Vec<(usize,usize)> {
        let mut vec = Vec::with_capacity(4);
        let mut jc = j;
        while jc > 0 {
            jc -= 1;
            let mut ret = self.empty(i,jc);
            if ret {
                vec.push((i,jc));
            }
            ret = self.enemy(i,j,i,jc);
            if ret {
                vec.push((i,jc));
                break;
            }
        }
        jc = j;
        while jc < 7 {
            jc += 1;
            let mut ret = self.empty(i,jc);
            if ret {
                vec.push((i,jc));
            }
            ret = self.enemy(i,j,i,jc);
            if ret {
                vec.push((i,jc));
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
    fn vertical_move(&self, i:usize, j:usize)  -> Vec<(usize,usize)>{
        let mut vec = Vec::with_capacity(4);
        let mut ic = i;
        while ic > 0 {
            ic -= 1;
            let mut ret = self.empty(ic,j);
            if ret {
                vec.push((ic,j));
            }
            ret = self.enemy(i,j,ic,j);
            if ret {
                vec.push((ic,j));
                break;
            }
        }
        ic = i;
        while ic < 7 {
            ic += 1;
            let mut ret = self.empty(ic,j);
            if ret {
                vec.push((ic,j));
            }
            ret = self.enemy(i,j,ic,j);
            if ret {
                vec.push((ic,j));
                break;
            }
        }
        vec
    }
    ///Makes a cross with vertical and horizontal moves 
    /// # Arguments
    /// * `i` - row index of the piece
    /// * `j` - col index of the piece
    fn cross_move(&self, i:usize, j:usize) -> Vec<(usize,usize)> {
        let mut vec = self.vertical_move(i,j);
        vec.append(&mut self.horizontal_move(i,j));
        vec
    }

    ///Iterates for [+n,-n] ; [+n,-n] combinations for 1..7 and gets legal moves
    /// # Arguments
    /// * `i` - row index of the piece
    /// * `j` - col index of the piece
    fn x_move(&self, i:usize, j:usize)  -> Vec<(usize,usize)> {
        let mut vec = Vec::with_capacity(4);
        let mut ic = i;
        let mut jc = j;

        //lower right
        while ic < 7 && jc < 7 {
            ic+=1;
            jc+=1;
            let mut ret = self.empty(ic,jc);
            if ret {
                vec.push((ic,jc));
            }
            ret = self.enemy(i,j,ic,jc);
            if ret {
                vec.push((ic,jc));
                break;
            }
        }
        //upper right
        ic=i;
        jc=j;
        while ic > 0 && jc < 7 {
            ic-=1;
            jc+=1;
            let mut ret = self.empty(ic,jc);
            if ret {
                vec.push((ic,jc));
            }
            ret = self.enemy(i,j,ic,jc);
            if ret {
                vec.push((ic,jc));
                break;
            }
        }
        //upper left 
        ic=i;
        jc=j;
        while ic > 0 && jc > 0 {
            ic-=1;
            jc-=1;
            let mut ret = self.empty(ic,jc);
            if ret {
                vec.push((ic,jc));
            }
            ret = self.enemy(i,j,ic,jc);
            if ret {
                vec.push((ic,jc));
                break;
            }
        }
        //lower left
        ic=i;
        jc=j;
        while ic < 7 && jc > 0 {
            ic+=1;
            jc-=1;
            let mut ret = self.empty(ic,jc);
            if ret {
                vec.push((ic,jc));
            }
            ret = self.enemy(i,j,ic,jc);
            if ret {
                vec.push((ic,jc));
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
    fn horse_jump(&self, i:usize, j:usize) -> Vec<(usize,usize)> {
        //all possible combinations of +2, -2, +1, -1 for each square
        let twoway = [-2,2];
        let oneway = [1,-1];
        let mut vec = Vec::with_capacity(4);

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
                    
                    if self.empty(itmp,jtmp) || 
                    self.enemy(i,j,itmp,jtmp) {
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
                    if self.empty(itmp,jtmp) || 
                    self.enemy(i,j,itmp,jtmp) {
                        vec.push((itmp,jtmp));
                    }
                }
            }
        }
        vec
    }

    ///Checks wether the given location is under attack
    /// # Arguments
    /// * `i` - row index
    /// * `j` - col index
    pub fn is_attacked(&self, i:usize,j:usize) -> bool {
        //remove yourself from board (you could be blocking other places)
        //check for pawns
        //check for rook|queen on vertical
        //check for rook|queen on horizontal
        //check for bishop on x
        //check for horses
        //check for enemy king 
        let mut ic = i as i32;
        let mut jc = j as i32;
        let f = self.board[i][j];
        let color : Color;
        match f {
            Field::Piece(Piece{piece_type:_, color:c, upgraded:_})  =>  {color = c;},
            _ => panic!("Not a piece on the field")
        }

        //check for pawns
        match color {
            Color::White => {
                if self.valid(ic-1,jc-1) {
                    let a = (ic-1) as usize;
                    let b = (jc-1) as usize;
                    match self.board[a][b] {
                        Field::Piece(Piece{piece_type:t, color:c, upgraded:_}) => {
                            if t==PieceType::Pawn && c==Color::Black {
                                return true
                            }
                        }
                        _ => {}
                    }    
                }
                if self.valid(ic-1,jc+1) {
                    let a = (ic-1) as usize;
                    let b = (jc+1) as usize;
                    match self.board[a][b] {
                        Field::Piece(Piece{piece_type:t, color:c, upgraded:_}) => {
                            if t==PieceType::Pawn && c==Color::Black {
                                return true
                            }
                        }
                        _ => {}
                    } 
                }
            },
            Color::Black => {
                ic = i as i32;
                jc = j as i32;
                if self.valid(ic+1,jc-1) && self.valid(ic,jc) {
                    let a = (ic+1) as usize;
                    let b = (jc-1) as usize;
                    match self.board[a][b] {
                        Field::Piece(Piece{piece_type:t, color:c, upgraded:_}) => {
                            if t==PieceType::Pawn && c==Color::White {
                                return true
                            }
                        }
                        _ => {}
                    } 
                }
                if self.valid(ic+1,jc+1) && self.valid(ic,jc) {
                    let a = (ic+1) as usize;
                    let b = (jc+1) as usize;
                    match self.board[a][b] {
                        Field::Piece(Piece{piece_type:t, color:c, upgraded:_}) => {
                            if t==PieceType::Pawn && c==Color::White {
                                return true
                            }
                        }
                        _ => {}
                    } 
                }
            }
        }

        //check for horizontal line
        jc = j as i32;
        ic = i as i32;
        let mut jx = jc+1;
        while jx < 8{
            if self.line_movable_enemy(i,j,ic as usize,jx as usize) {
                return true
            }else if self.board[ic as usize][jx as usize]!=Field::Empty {
                break;
            }
            jx += 1;
        }
        jx = jc - 1;
        while jx >= 0{
            if self.line_movable_enemy(i,j,ic as usize,jx as usize) {
                return true
            }else if self.board[ic as usize][jx as usize]!=Field::Empty {
                break;
            }
            jx -= 1;
        }
        let mut ix = ic+1;
        while ix <8 {
            if self.line_movable_enemy(i,j,ic as usize,jx as usize) {
                return true
            }else if self.board[ic as usize][jx as usize]!=Field::Empty {
                break;
            }
            ix += 1;
        }
        ix = ic-1;
        while ix >=0 {
            if self.line_movable_enemy(i,j,ic as usize,jx as usize) {
                return true
            }else if self.board[ic as usize][jx as usize]!=Field::Empty {
                break;
            }
            ix-=1;
        }


        //check for x 
        ic=i as i32;
        jc=j as i32;
        //lower right
        while ic < 7 && jc < 7 {
            ic+=1;
            jc+=1;
            if self.diagonal_movable_enemy(i,j,ic as usize,jc as usize) {
                return true
            }else if self.board[ic as usize][jc as usize]!=Field::Empty {
                break;
            }

        }
        //upper right
        ic=i as i32;
        jc=j as i32;
        while ic > 0 && jc < 7 {
            ic-=1;
            jc+=1;
            if self.diagonal_movable_enemy(i,j,ic as usize,jc as usize) {
                return true
            }else if self.board[ic as usize][jc as usize]!=Field::Empty {
                break;
            }       
        }
        //upper left 
        ic=i as i32;
        jc=j as i32;
        while ic > 0 && jc > 0 {
            ic-=1;
            jc-=1;
            if self.diagonal_movable_enemy(i,j,ic as usize,jc as usize) {
                return true
            }else if self.board[ic as usize][jc as usize]!=Field::Empty {
                break;
            }
        }
        //lower left
        ic=i as i32;
        jc=j as i32;
        while ic < 7 && jc > 0 {
            ic+=1;
            jc-=1;
            if self.diagonal_movable_enemy(i,j,ic as usize,jc as usize) {
                return true
            }else if self.board[ic as usize][jc as usize]!=Field::Empty {
                break;
            }
        }  
        
        //check for knights
        ic = i as i32;
        jc = j as i32;
        let a = [-2,2];
        let b = [-1,1];
        for i_off in a.iter() {
            for j_off in b.iter() {
                if self.valid(ic+i_off,jc+j_off) {
                    if self.enemy_knight(i,j,(ic+i_off) as usize, (jc+j_off) as usize) {
                        return true
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
                    if self.enemy_king(i,j,(ic+i_off) as usize, (jc+j_off) as usize) {
                        return true
                    }
                }
            }
        }        
        return false
    }

    ///Moves for the king, be careful rules for bughouse apply king can move to somewhere that is
    ///under attack
    fn king_move(&self, i:usize, j:usize) ->  Vec<(usize,usize)> {
        let a = [-1,0,1];
        let b = [-1,0,1];
        let mut vec = Vec::with_capacity(2);
        let ix = i as i32;
        let jx = j as i32;

        for i_off in a.iter(){
            for j_off in b.iter(){
                if self.valid(i_off+ix,j_off+jx) && !(*i_off==0 && *j_off==0){
                    let ic = (i_off+ix) as usize;
                    let jc = (j_off+jx) as usize;

                    if self.enemy(i,j,ic,jc) || self.board[ic][jc]==Field::Empty {
                        vec.push((ic,jc));
                    }
                }
            }
        }

        //check for castling
        match self.board[i][j] {
            Field::Piece(Piece{piece_type:PieceType::King, color:c, upgraded:_}) => {
                match c {
                    Color::White => {
                        if !self.white_king_moved {
                            if !self.white_rook_long_moved && self.board[7][3]==Field::Empty &&
                            self.board[7][2]==Field::Empty && self.board[7][1]==Field::Empty {
                                vec.push((7,2));
                            }
                            if !self.white_rook_short_moved && self.board[7][5]==Field::Empty &&
                            self.board[7][6]==Field::Empty {
                                vec.push((7,6));
                            }
                        }
                    },
                    Color::Black => {
                        if !self.black_king_moved {
                            if !self.black_rook_long_moved && self.board[0][3]==Field::Empty &&
                            self.board[0][2]==Field::Empty && self.board[0][1]==Field::Empty{
                                vec.push((0,2));
                            }
                            if !self.white_rook_short_moved && self.board[0][5]==Field::Empty &&
                            self.board[0][6]==Field::Empty {
                                  vec.push((0,6));
                            }  
                        }
                    }
                }
            },
            _ => panic!("board[i][j] is not the king!")
        }

        return vec
    }

    pub fn pawn_move(&self, i:usize, j:usize) -> Vec<(usize,usize)> {
        let mut vec = Vec::with_capacity(2);
        match self.board[i][j] {
            Field::Piece(Piece{piece_type:PieceType::Pawn, color:c, upgraded:_}) => {
                match c {
                    Color::White => {
                        match (i,j) {
                            (6,_) =>  {
                                //max 4 moves
                                let mut vec = Vec::with_capacity(2);
                                if self.board[(i-1) as usize][j]==Field::Empty {
                                    vec.push((i-1,j));
                                    // -2 only if -1 is empty
                                    if self.board[(i-2) as usize][j]==Field::Empty  {
                                        vec.push((i-2,j));
                                    }
                                }
                                //enemy piece on -1,-1
                                if j > 0 && self.enemy(i,j,i-1,j-1) {
                                    vec.push((i-1,j-1));
                                }
                                //enemy piece on -1,+1
                                if j < 7 && self.enemy(i,j,i-1,j+1) {
                                    vec.push((i-1,j+1));
                                }
                                return vec
                            },
                            //6th lane with en passant
                            (3,_) => {
                                //max 5 moves
                                let mut vec = Vec::with_capacity(2);
                                if self.empty(i-1,j) {
                                    vec.push((i-1,j));
                                }
                                if j > 0 && self.enemy(i,j,i-1,j-1) {
                                    vec.push((i-1,j-1));
                                }
                                if j < 7 && self.enemy(i,j,i-1,j+1) {
                                    vec.push((i-1,j+1));
                                }
                                //en passant from left
                                if j > 0 {
                                    match self.board[i][j-1] {
                                        Field::Piece(Piece{piece_type:PieceType::Pawn, color:Color::Black, upgraded:_}) => {
                                            if self.last_moved_pawn == (i,j-1) {
                                                vec.push((i-1,j-1))
                                            }
                                        },
                                        _ => {}
                                    }
                                }
                                //en passant from right
                                if j < 7 {
                                    match self.board[i][j-1] {
                                        Field::Piece(Piece{piece_type:PieceType::Pawn, color:Color::Black, upgraded:_}) => {
                                            if self.last_moved_pawn == (i,j+1) {
                                                vec.push((i-1,j+1))
                                            }
                                        },
                                        _ => {}
                                    }
                                }
                                return vec
                            },
                            //there can be no white pawns on line
                            (0,_) => {
                                panic!("This indicates pawn upgrade malfunction");
                            },
                            //no special moves
                            (_,_) => {
                                //max 4 moves
                                if self.empty(i-1,j) {
                                    vec.push((i-1,j));
                                }
                                if j > 0 && self.enemy(i,j,i-1,j-1) {
                                    vec.push((i-1,j-1));
                                }
                                if j < 7 && self.enemy(i,j,i-1,j+1) {
                                    vec.push((i-1,j+1));
                                }
                                return vec
                            },
                        }  
                    },
                    Color::Black => {
                        match (i,j) {
                            (1,_) => {
                                //W double move
                                if self.empty(i+1,j) {
                                    vec.push((i+1,j));
                                    if self.empty(i+2,j) {
                                        vec.push((i+2,j));
                                    }
                                }
                                //enemy left above
                                if j > 0 && self.enemy(i,j,i+1,j-1) {
                                    vec.push((i+1,j-1));
                                }
                                //enemy right above
                                if j < 7 && self.enemy(i,j,i+1,j+1) {
                                    vec.push((i+1,j+1));
                                }
                                return vec
                            }
                            (4,_) => {
                                //front empty
                                if self.empty(i+1,j) {
                                    vec.push((i+1,j));
                                }
                                //enemy left above
                                if j > 0 && self.enemy(i,j,i+1,j-1) {
                                    vec.push((i+1,j-1));
                                }
                                //enemy right above
                                if j < 7 && self.enemy(i,j,i+1,j+1) {
                                    vec.push((i+1,j+1));
                                }
                                //en passant, enemy left
                                if j > 0  {
                                    match self.board[i][j+1] {
                                        Field::Piece(Piece{piece_type:PieceType::Pawn, color:Color::White, upgraded:_}) => {
                                            vec.push((i+1,j+1));
                                        },
                                        _ => {}
                                    }
                                }
                                //en passant, enemy right
                                if j < 7 {
                                    match self.board[i][j-1] {
                                        Field::Piece(Piece{piece_type:PieceType::Pawn, color:Color::White, upgraded:_}) => {
                                            vec.push((i+1,j-1));
                                        },
                                        _ => {}
                                    }
                                }
                                return vec
                            },
                            (7,0) => {
                                panic!("No black pawn can be in this line")
                            }
                            (_,_) => {
                                if self.empty(i+1,j) {
                                    vec.push((i+1,j));
                                }
                                if j > 0 && self.enemy(i,j,i+1,j-1) {
                                    vec.push((i+1,j-1));
                                }
                                if j < 7 && self.enemy(i,j,i+1,j+1) {
                                    vec.push((i+1,j+1));
                                }
                                return vec
                            }
                        }
                    }
                }
            },
            _ => panic!("board[i][j] is not a pawn")
        }
    }

    fn enemy_king(&self, i:usize,j:usize, ie:usize, je:usize) -> bool {
        let col : Color;
        let enemycol : Color;
        
        match self.board[ie][je]{
            Field::Empty => return false,
            Field::Legal => return false,
            Field::Piece(Piece{piece_type:_, color:c, upgraded:_}) => {enemycol = c;},
        }

        match self.board[i][j] {
            Field::Piece(Piece{piece_type:t, color:c, upgraded:_}) => {
                if t == PieceType::King {
                    col = c;
                }else{
                    return false
                }  
            },
            _ => panic!("board[i][j] is not a piece")
        }
        
        return col!=enemycol
    }

    fn enemy_knight(&self, i:usize, j:usize, ie:usize, je:usize) -> bool {
        let col : Color;
        let enemycol : Color;
        
        match self.board[ie][je]{
            Field::Empty => return false,
            Field::Legal => return false,
            Field::Piece(Piece{piece_type:_, color:c, upgraded:_}) => {enemycol = c;},
        }

        match self.board[i][j] {
            Field::Piece(Piece{piece_type:t, color:c, upgraded:_}) => {
                if t == PieceType::Knight {
                    col = c;
                }else{
                    return false
                }  
            },
            _ => panic!("board[i][j] is not a piece")
        }
        
        return col!=enemycol
    }

    fn diagonal_movable_enemy(&self, i:usize, j:usize, ie:usize, je:usize) -> bool {
        let col : Color;
        let enemycol : Color;

        match self.board[ie][je]{
            Field::Empty => return false,
            Field::Legal => return false,
            Field::Piece(Piece{piece_type:_, color:c, upgraded:_}) => {enemycol = c;},
        }
        
        match self.board[i][j] {
            Field::Piece(Piece{piece_type:t, color:c, upgraded:_}) => {
                if t == PieceType::Bishop || t == PieceType::Queen {
                    col = c;
                }else{
                    return false
                }  
            },
            _ => panic!("board[i][j] is not a piece")
        }

        return enemycol != col
    }

    fn line_movable_enemy(&self, i:usize, j:usize, ie:usize, je:usize) -> bool {
        let col : Color;
        let enemycol : Color;
        
        match self.board[ie][je]{
            Field::Empty => return false,
            Field::Legal => return false,
            Field::Piece(Piece{piece_type:_, color:c, upgraded:_}) => {enemycol = c;},
        }
        
        match self.board[i][j] {
            Field::Piece(Piece{piece_type:t, color:c, upgraded:_}) => {
                if t == PieceType::Rook || t == PieceType::Queen {
                    col = c;
                }else{
                    return false
                }  
            },
            _ => panic!("board[i][j] is not a piece")
        }

        return enemycol != col
    }

    ///Deploys a piece on your field as a turn returns an Error if you cant deploy 
    /// # Arguments
    /// * piece - piece to deploy
    /// * `i` - the row to deploy
    /// * `j` - the col to deploy
    /// 
    /// Deploys a piece at the location (i,j) if the location (i,j) is within bounds, empty and it is legal to deploy at index (i,j)
    /// For example a white pawn, P cannot be deployed to (0,_). It also checks if the deployable piece pool has
    /// enough pieces of the type p (>=1 p:Piece).
    /// It updates: pools, winner, count of turns, count of half-turns since last capture.
    /// A king cannot be deployed but if the rook is deployed on the initial position it is possible to castle, it also updated by the deploy piece.
    /// A deploy cannot terminate the game, since you cannot capture the king with a deploy. The game terminates only when a king is captured, a plyer resigns, or stalemate occurs which is prob. never going to happen.
    pub fn deploy_piece(&mut self, piece:&Piece, i:usize,j:usize) -> Result<bool,MoveError> {
            //check for i j bounds
            if !self.valid(i as i32,j as i32) {
                return Err(MoveError::NotLegal)
            }

            if self.board[i][j] != Field::Empty {
                return Err(MoveError::NotEmpty)
            }

            if self.current_turn != piece.color {
                return Err(MoveError::NotTurn)
            }

            match piece.color {
                Color::White => {
                    let biopt = aux::box_index(&piece);
                    let bi = biopt.unwrap();
                    if self.white_deployable[bi] > 0 {
                        self.white_deployable[bi] -= 1;
                        self.board[i][j] = Field::Piece(*piece);
                        self.current_turn = Color::Black;
                        return Ok(true)
                    }else {
                        return Err(MoveError::NoPieceInPool)
                    }
                },
                Color::Black => {
                    let biopt = aux::box_index(&piece);
                    let bi = biopt.unwrap();
                    if self.black_deployable[bi] > 0 {
                        self.black_deployable[bi] -= 1;
                        self.board[i][j] = Field::Piece(*piece);
                        self.current_turn = Color::White;
                        return Ok(true)
                    }else {
                        return Err(MoveError::NoPieceInPool)
                    }
                }
            }
            
    }

    #[inline]
    pub fn valid(&self, a:i32, b:i32) -> bool {
        if a >= 0 && a <= 7 && b >= 0 && b<= 7 {
            return true
        }else {
            return false
        }
    }

    #[inline]
    pub fn get_field(&self, i:usize, j:usize) -> Field {
        return self.board[i][j]
    }

    #[inline]
    pub fn set_piece(&mut self, piece_type:PieceType, color:Color, upgraded:bool, i:usize, j:usize){
        self.board[i][j] = Field::Piece(Piece{piece_type:piece_type,color:color,upgraded:upgraded});
    }

    pub fn all_empty(&mut self) {
        for i in 0..8 {
            for j in 0..8 {
                self.board[i][j] = Field::Empty;
            }
        }
    }

    pub fn legal_moves(&self, i:usize, j:usize) -> Vec<(usize,usize)> {
        let f : &Field = &self.board[i][j];
        let vec : Vec<(usize,usize)>  = Vec::new();
        match f {
            Field::Empty => return vec,
            Field::Legal => return vec,
            Field::Piece(Piece{piece_type:t, color:_c, upgraded:_u}) => {
                match t {
                    PieceType::Rook => {
                        return self.cross_move(i,j)
                    },
                    PieceType::Bishop => {
                        return self.x_move(i,j)
                    },
                    PieceType::Queen => {
                        let mut v = self.x_move(i,j);
                        v.append(& mut self.cross_move(i,j));
                        return v
                    },
                    PieceType::Knight => {
                        return self.horse_jump(i, j);
                    },
                    PieceType::Pawn => {
                        return self.pawn_move(i,j)
                    }
                    PieceType::King => {
                        return self.king_move(i, j)
                    }
                }
            }
        }
    }

    fn check_legal(&self, i_old:usize,j_old:usize,i_new:usize,j_new:usize) -> bool {
        match &self.board[i_old][j_old] {
            Field::Empty | Field::Legal => false,
            _ => { 
                let vec = self.legal_moves(i_old,j_old);
                for (a,b) in vec.iter() {
                    if *a == i_new && *b == j_new {
                        return true
                    }
                }
                return false
            },
        }
    }

    //TODO
    /*
    pub fn movemaker(&mut self, i_old:usize,j_old:usize,i_new:usize,j_new:usize) -> Result<bool,MoveError> {
        if self.over {
            return Err(MoveError::AlreadyOver)
        }


        if self.check_legal(i_old,j_old,i_new,j_new) {
            match self.board[i_old][j_old] {
                Field::Piece(Piece{piece_type:t,color:c,upgraded:u}) => {
                    match c {
                        Color::White => {
                            if self.current_turn==Color::White {
                                self.current_turn=Color::Black;
                            }else{
                                return Err(MoveError::NotTurn)
                            }
                            
                        },
                        Color::Black => {
                            if self.current_turn==Color::Black {
                                self.current_turn = Color::White;
                            }else{
                                return Err(MoveError::NotTurn)
                            }
                        },
                    }
                    match t {
                        PieceType::Pawn => {

                        },
                        PieceType::Rook => {
                            if i_old==7 && j_old==7 && c==Color::White {
                                self.white_rook_short_moved=true;
                            }
                            if i_old==7 && j_old==0 && c==Color::White {
                                self.white_rook_long_moved=true;
                            }
                            if i_old==0 && j_old==7 && c==Color::Black {
                                self.black_rook_short_moved=true;
                            }
                            if i_old==0 && j_old==0 && c==Color::Black {
                                self.black_rook_long_moved=true;
                            }
                        }

                        _ => {}
                    }
                },
                _ => {}
            }
        }else{
            return Err(MoveError::NotLegal);
        }


        return Ok(true)
    }
    */
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_state_print(){
        let cb = ChessBoard::new();
        aux::print_board(&cb.board);
        assert!(true);
    }
}


