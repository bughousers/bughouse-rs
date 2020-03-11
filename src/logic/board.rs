use std::fmt;

#[derive(Clone, Copy)]
pub enum Piece {
    P,R,N,B,Q,K,E,L,
    p,r,n,b,q,k,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Piece::P => write!(f, "P"),
            Piece::R => write!(f, "R"),
            Piece::N => write!(f, "N"),
            Piece::B => write!(f, "B"),
            Piece::Q => write!(f, "Q"),
            Piece::K => write!(f, "K"),
            Piece::E => write!(f, " "),
            Piece::p => write!(f, "p"),
            Piece::r => write!(f, "r"),
            Piece::n => write!(f, "n"),
            Piece::b => write!(f, "b"),
            Piece::q => write!(f, "q"),
            Piece::k => write!(f, "k"),
            Piece::L => write!(f, "L"),
            //_ => write!(f, "shouldn't happen")
        }
    }
}

pub struct ChessBoard {
    pub board: [[Piece; 8]; 8],
}


impl ChessBoard {

    pub fn new() -> ChessBoard {
        let arr = get_init_array();
        ChessBoard {
            board: arr,
        }
        
    }

    pub fn move_piece(&mut self,old_i: usize, old_j: usize, i:usize, j:usize) {
        self.board[i][j] = self.board[old_i][old_j];
        //println!("{}",self.board[i][j]);
        self.board[old_i][old_j] = Piece::E;
    }


    pub fn print_board(&self) {
        for i in 0..8 {
            for j in 0..8 {
                print!("[{}]",self.board[i][j]);
            }
            println!("");
        }
        
    }

}

fn get_init_array() -> [[Piece; 8]; 8] {
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




