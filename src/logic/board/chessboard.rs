use crate::logic::board::aux;

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
    
    pub white_long_castle_rights: bool,
    pub white_short_castle_rights: bool, 
    pub black_long_castle_rights: bool, 
    pub black_short_castle_rights: bool,
    pub current_turn: Color,

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
            white_long_castle_rights: false,
            white_short_castle_rights: false,
            black_long_castle_rights: false,
            black_short_castle_rights: false,
            current_turn: Color::White,
            white_deployable: [0,0,0,0,0],
            black_deployable: [0,0,0,0,0]
        }
    }
}

impl ChessBoard {
    pub fn reset(&mut self) {
        aux::set_starting_pos(&mut self.board);
        self.white_long_castle_rights = false;
        self.white_short_castle_rights = false; 
        self.black_long_castle_rights = false;
        self.black_short_castle_rights = false;

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


    pub fn set_piece(&mut self,piece_type:PieceType, color:Color, upgraded:bool, i:usize, j:usize){
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
                    _ => return vec
                }
            }
        }
    }

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


