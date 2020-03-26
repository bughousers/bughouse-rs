use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub enum Piece {
    P,R,N,B,Q,K,E,L,
    p,r,n,b,q,k,
    UR,UN,UB,UQ,
    Ur,Un,Ub,Uq,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Piece::P => write!(f, "P"),
            Piece::p => write!(f, "p"),
            Piece::R => write!(f, "R"),
            Piece::N => write!(f, "N"),
            Piece::B => write!(f, "B"),
            Piece::Q => write!(f, "Q"),
            Piece::K => write!(f, "K"),
            Piece::E => write!(f, " "),
            Piece::L => write!(f, "L"), 
            Piece::r => write!(f, "r"),
            Piece::n => write!(f, "n"),
            Piece::b => write!(f, "b"),
            Piece::q => write!(f, "q"),
            Piece::k => write!(f, "k"),
            Piece::UR => write!(f, "R"),
            Piece::UN => write!(f, "N"),
            Piece::UB => write!(f, "B"),
            Piece::UQ => write!(f, "Q"),
            Piece::Ur => write!(f, "r"),
            Piece::Un => write!(f, "n"),
            Piece::Ub => write!(f, "b"),
            Piece::Uq => write!(f, "q"),
            //_ => write!(f, "shouldn't happen")
        }
    }
}

pub struct ChessBoard {
    pub board: [[Piece; 8]; 8], //chess board
    pub white_k_moved: bool,  //castle condition
    pub black_k_moved: bool, //castle condition
    pub white_rook_q_moved: bool, //castle queenside condition
    pub white_rook_k_moved: bool, //castle kingside condition
    pub black_rook_q_moved: bool, //castle queenside condition
    pub black_rook_k_moved: bool, //castle kingside condition
}


impl ChessBoard {

    //default constructor for a chess game
    pub fn new() -> ChessBoard {
        let arr = get_init_array();
        ChessBoard {
            board: arr,
            white_k_moved: false,
            black_k_moved: false,
            white_rook_q_moved: false,
            white_rook_k_moved: false,
            black_rook_q_moved: false,
            black_rook_k_moved: false,
        }
        
    }

    //moves piece from old i old j to i j, does not check anything
    pub fn move_piece(&mut self,old_i: usize, old_j: usize, i:usize, j:usize) {
        self.board[i][j] = self.board[old_i][old_j];
        self.board[old_i][old_j] = Piece::E;
    }

    //prints the chess board
    pub fn print_board(&self) {
        for i in 0..8 {
            for j in 0..8 {
                print!("[{}]",self.board[i][j]);
            }
            println!("");
        }
        
    }


    //sets the board with the initial positions for pieces
    pub fn set_init_array(&mut self){
        for i in 0..8 {
            self.board[1][i] = Piece::p;
            self.board[6][i] = Piece::P;
        }

            self.board[7][0] = Piece::R;
            self.board[7][1] = Piece::N;
            self.board[7][2] = Piece::B;
            self.board[7][3] = Piece::Q;
            self.board[7][4] = Piece::K;
            self.board[7][5] = Piece::B;
            self.board[7][6] = Piece::N;
            self.board[7][7] = Piece::R;
            self.board[0][0] = Piece::r;
            self.board[0][1] = Piece::n;
            self.board[0][2] = Piece::b;
            self.board[0][3] = Piece::q;
            self.board[0][4] = Piece::k;
            self.board[0][5] = Piece::b;
            self.board[0][6] = Piece::n;
            self.board[0][7] = Piece::r;
    }

}

//initial array positions board is returned
pub fn get_init_array() -> [[Piece; 8]; 8] {
    let mut arr = [[Piece::E; 8]; 8];
    for i in 0..8 {
        arr[1][i] = Piece::p;
        arr[6][i] = Piece::P;
    }
        arr[7][0] = Piece::R;
        arr[7][1] = Piece::N;
        arr[7][2] = Piece::B;
        arr[7][3] = Piece::Q;
        arr[7][4] = Piece::K;
        arr[7][5] = Piece::B;
        arr[7][6] = Piece::N;
        arr[7][7] = Piece::R;
        arr[0][0] = Piece::r;
        arr[0][1] = Piece::n;
        arr[0][2] = Piece::b;
        arr[0][3] = Piece::q;
        arr[0][4] = Piece::k;
        arr[0][5] = Piece::b;
        arr[0][6] = Piece::n;
        arr[0][7] = Piece::r;

    arr
}





