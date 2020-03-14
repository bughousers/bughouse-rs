#![allow(warnings)] 
mod logic;

#[cfg(test)]
mod tests {
    use crate::logic::board::Piece;
    use crate::logic::ChessLogic;

    #[test]
    fn test_trivial() {
        assert_eq!(1,1);
    }
    #[test]
    fn test_king1(){
        let mut cl = ChessLogic::new();

        cl.all_empty(true);
        cl.set_piece(true,Piece::q,7,7);
        cl.set_piece(true,Piece::r,0,3);
        cl.set_piece(true,Piece::K,6,5);
        let vec = cl.get_legal_moves(true,6,5);
        cl.print_w_legal(true,&vec);
        let mut vecbyhand = Vec::new();

        vecbyhand.push((5,6));
        vecbyhand.push((5,4));
        vecbyhand.push((6,4));

        let mut vec_norm = normalize(&vec);
        let mut vecbyhand_norm = normalize(&vecbyhand);
        vec_norm.sort();
        vecbyhand_norm.sort();


        assert_eq!(vec_norm,vecbyhand_norm);
    }

    #[test]
    fn test_rook2(){
        let mut cl = ChessLogic::new();
        cl.all_empty(true);
        cl.set_piece(true,Piece::q,5,0);
        cl.set_piece(true,Piece::k,3,0);
        cl.set_piece(true,Piece::r,4,0);
        cl.set_piece(true,Piece::b,4,1);

        assert_eq!(
            cl.get_legal_moves(true,4,0),Vec::new()
        );

        cl.all_empty(true);
        cl.set_piece(true,Piece::Q,5,0);
        cl.set_piece(true,Piece::K,3,0);
        cl.set_piece(true,Piece::r,4,0);
        cl.set_piece(true,Piece::B,4,1);

        let vec = cl.get_legal_moves(true,4,0);
        let mut vecbyhand = Vec::new();

        vecbyhand.push((5,0));
        vecbyhand.push((3,0));
        vecbyhand.push((4,1));

        let mut vec_norm = normalize(&vec);
        let mut vecbyhand_norm = normalize(&vecbyhand);
        vec_norm.sort();
        vecbyhand_norm.sort();


        assert_eq!(vec_norm,vecbyhand_norm)
    }

    pub fn normalize(vec: &Vec<(usize,usize)>) -> Vec<usize> {
        let mut ret = Vec::new();
        for (a,b) in vec.iter() {  
            ret.push(a*10+b);
        }
        ret
    }

    #[test]
    fn test_attacks1(){
        let mut cl = ChessLogic::new();
        cl.chess_board1.board[5][4] = Piece::P;
        cl.chess_board1.board[5][5] = Piece::p;
        cl.chess_board1.board[4][2] = Piece::R;
        cl.chess_board1.board[4][3] = Piece::q;
        cl.chess_board1.board[1][7] = Piece::E;
        for i in 0..8 {
            cl.chess_board1.board[6][i] = Piece::E;
        }
        
        cl.print(true);
        assert_eq!(true,cl.is_attacked(true,true,6,6));
        assert_eq!(true,cl.is_attacked(true,true,7,7));
        assert_eq!(true,cl.is_attacked(true,true,3,4));
        assert_eq!(true,cl.is_attacked(true,true,2,3));
        assert_eq!(true,cl.is_attacked(true,true,0,5));
        assert_eq!(true,cl.is_attacked(true,true,6,4));
        assert_eq!(true,cl.is_attacked(true,true,4,7));
        assert_eq!(true,cl.is_attacked(true,true,7,0));
        assert_eq!(false,cl.is_attacked(true,true,5,1))
    }

    #[test]
    fn checkmate1(){
        let mut cl = ChessLogic::new();
        cl.all_empty(true);
        cl.chess_board1.board[1][4]=Piece::N;
        cl.chess_board1.board[5][7]=Piece::R;
        cl.chess_board1.board[1][6]=Piece::p;
        cl.chess_board1.board[1][7]=Piece::k;

        let vec = cl.get_legal_moves(true,1,7);
        cl.print_w_legal(true,&vec);
        assert_eq!(vec,Vec::new())

    }

    #[test]
    fn checkmate2(){
        let mut cl = ChessLogic::new();
        cl.all_empty(true);
        cl.chess_board1.board[0][0]=Piece::Q;
        cl.chess_board1.board[0][7]=Piece::k;
        cl.chess_board1.board[1][7]=Piece::p;
        cl.chess_board1.board[1][6]=Piece::p;

        let vec = cl.get_legal_moves(true,0,7);
        cl.print_w_legal(true,&vec);
        assert_eq!(vec,Vec::new())
    }

    #[test]
    fn fools_mate(){
        let mut cl = ChessLogic::new();
        cl.chess_board1.board[6][6]=Piece::E;
        cl.chess_board1.board[6][5]=Piece::E;
        cl.chess_board1.board[4][6]=Piece::P;
        cl.chess_board1.board[5][5]=Piece::P;
        cl.chess_board1.board[4][7]=Piece::q;

        let vec = cl.get_legal_moves(true,7,4);
        cl.print_w_legal(true,&vec);
        assert_eq!(vec,Vec::new())
    }
}