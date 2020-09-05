use crate::logic::board::chessboard::{Field, Piece, PieceType, Color};
use std::fmt;

pub fn set_starting_pos(arr: &mut [[Field; 8]; 8]){
    for i in 0..8 {
        arr[1][i] = Field::Piece(Piece{piece_type: PieceType::Pawn, color:Color::Black, upgraded:false});
        arr[6][i] = Field::Piece(Piece{piece_type: PieceType::Pawn, color:Color::White, upgraded:false});
    }

    for i in 2..5 {
        for j in 0..8 {
            arr[i][j] = Field::Empty;
        }
    }

        arr[7][0] = Field::Piece(Piece{piece_type: PieceType::Rook, color:Color::White, upgraded:false});
        arr[7][1] = Field::Piece(Piece{piece_type: PieceType::Knight, color:Color::White, upgraded:false});
        arr[7][2] = Field::Piece(Piece{piece_type: PieceType::Bishop, color:Color::White, upgraded:false});
        arr[7][3] = Field::Piece(Piece{piece_type: PieceType::King, color:Color::White, upgraded:false});
        arr[7][4] = Field::Piece(Piece{piece_type: PieceType::Queen, color:Color::White, upgraded:false});
        arr[7][5] = Field::Piece(Piece{piece_type: PieceType::Bishop, color:Color::White, upgraded:false});
        arr[7][6] = Field::Piece(Piece{piece_type: PieceType::Knight, color:Color::White, upgraded:false});
        arr[7][7] = Field::Piece(Piece{piece_type: PieceType::Rook, color:Color::White, upgraded:false});
        arr[0][0] = Field::Piece(Piece{piece_type: PieceType::Rook, color:Color::Black, upgraded:false});
        arr[0][1] = Field::Piece(Piece{piece_type: PieceType::Knight, color:Color::Black, upgraded:false});
        arr[0][2] = Field::Piece(Piece{piece_type: PieceType::Bishop, color:Color::Black, upgraded:false});
        arr[0][3] = Field::Piece(Piece{piece_type: PieceType::King, color:Color::Black, upgraded:false});
        arr[0][4] = Field::Piece(Piece{piece_type: PieceType::Queen, color:Color::Black, upgraded:false});
        arr[0][5] = Field::Piece(Piece{piece_type: PieceType::Bishop, color:Color::Black, upgraded:false});
        arr[0][6] = Field::Piece(Piece{piece_type: PieceType::Knight, color:Color::Black, upgraded:false});
        arr[0][7] = Field::Piece(Piece{piece_type: PieceType::Rook, color:Color::Black, upgraded:false});
}

pub fn gen_starting_pos() -> [[Field; 8]; 8]{
    let mut tmp_arr: [[Field; 8]; 8] = [[Field::Empty; 8]; 8];
    set_starting_pos(&mut tmp_arr);
    return tmp_arr
}

pub fn apply_modifications(c : char, color: Color, upgraded: bool) -> String {
    let s : String;
    if color==Color::White {
        s = c.to_uppercase().collect();
    }else{
        s = c.to_string();
    }

    if upgraded {
        let mut pre = String::from("U");
        pre.push_str(&s);
        return pre
    }else{
        return s;
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Field::Empty => write!(f," "),
            Field::Legal => write!(f,"L"),
            Field::Piece(Piece{piece_type:a,color:b,upgraded:c}) => {
                match a {
                    PieceType::Pawn => write!(f,"{}", apply_modifications('p',*b,*c)),
                    PieceType::Bishop => write!(f,"{}", apply_modifications('b',*b,*c)),
                    PieceType::King => write!(f,"{}", apply_modifications('k',*b,*c)),
                    PieceType::Queen => write!(f,"{}", apply_modifications('q',*b,*c)),
                    PieceType::Knight => write!(f,"{}", apply_modifications('n',*b,*c)),
                    PieceType::Rook => write!(f,"{}", apply_modifications('r',*b,*c)) 
                }
            }
        }
    }
}

pub fn print_board(arr: &[[Field; 8]; 8]) {
    for i in 0..8 {
        for j in 0..8 {
            print!("[{}]",arr[i][j]);
        }
        println!("");
    }
}

///Returns true if the given vector of tuples contains the tuples
pub fn contains(vec: &Vec<(usize,usize)>,(i,j): (usize,usize)) -> bool {
    for (a,b) in vec.iter() {
        if *a==i && j==*b {
            return true
        }
    }
    return false
}

///Get the index of the piece on the captured pieces array
/// # Arguments 
/// * `piece` - the index of the given piece
/// 
/// The upgraded pieces will be sent over as pawns,
pub fn box_index(piece : &Piece) -> Option<usize> {
    //P-R-N-B-Q
    match piece.piece_type {
        PieceType::Pawn => Some(0),
        PieceType::Rook => Some(1),
        PieceType::Knight => Some(2),
        PieceType::Bishop => Some(3),
        PieceType::Queen => Some(4),
        _ => None
    }
}
