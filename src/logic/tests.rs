#[cfg(test)]
use crate::logic::*;
use crate::parse::parser;
use crate::infoCourier::infoCourier::gen_fen;
use crate::infoCourier::infoCourier::*;
use crate::util::contains;
use crate::logic::MoveError;
use crate::logic::board::get_init_array;



#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn deploying_pieces_W_fen(){
    let mut cl =ChessLogic::new();
    cl.recv_piece(true,true,Piece::P);
    assert!(cl.deploy_piece(true,true,Piece::P,4,4)==Ok(true));
    let mut a = gen_fen(&mut cl);
    cl.print(true);
    let mut st = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPPPPPP/RNBQKBNR b KQkq e3 0 1".to_string();
    match a {
        (b,c) => {
            println!("{}",b);println!("{}",c);
            assert_eq!(st,b); 
            },
        _ => (),
    }
    cl.recv_piece(true,false,Piece::r);
    assert!(cl.deploy_piece(true,false,Piece::r,3,4)==Ok(true));
    let mut a = gen_fen(&mut cl);
    cl.print(true);
    let mut st = "rnbqkbnr/pppppppp/8/4r3/4P3/8/PPPPPPPP/RNBQKBNR w KQkq - 1 2".to_string();
    match a {
        (b,c) => {
            println!("{}",b);println!("{}",c);
            assert_eq!(st,b); 
            },
        _ => (),
    }
}

#[test]
fn read_fn(){
    let mut x = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq e3 0 1".to_string();
    let mut arr = "".to_string();
    let mut a = read_fen(&x,&"".to_string(),&arr,&arr,&arr,&arr);
    x = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b ---- - 0 1".to_string();
    arr = "".to_string();
    if let Some(mut a) = read_fen(&x,&x.to_string(),&arr,&arr,&arr,&arr){
        a.chess_board1.print_board();
        assert!(a.chess_board1.board==get_init_array());
        assert!(a.chess_board2.board==get_init_array());
    }else{
        assert!(false);
    }
  
}

#[test]
fn is_move_legal(){
    let mut cl = ChessLogic::new();
    cl.white_active_1 = false;
    assert!(cl.movemaker(true,1,1,3,1)==Ok(true));
    cl.white_active_1 = false;
    assert!(cl.movemaker(true,1,7,3,7)==Ok(true));
    cl.white_active_1 = false;
    assert!(cl.movemaker(true,0,1,2,2)==Ok(true));
    cl.white_active_1 = false;
    assert!(cl.movemaker(true,1,1,4,1)==Err(MoveError::NotLegal));
    cl.white_active_1 = false;
    assert!(cl.movemaker(true,0,0,1,0)==Err(MoveError::NotLegal));
    cl.white_active_1 = false;
    assert!(cl.movemaker(true,0,0,2,0)==Err(MoveError::NotLegal));
}


#[test]
fn pinned_piece(){
    let mut cl = ChessLogic::new();
    cl.all_empty(true);
    cl.set_piece(true,Piece::R,0,0);
    cl.set_piece(true,Piece::r,4,0);
    cl.set_piece(true,Piece::k,5,0);
    //is allowed bcs we are playing tandem
    cl.white_active_1 = false;
    assert!(cl.movemaker(true,4,0,4,7)==Ok(true));
    cl.all_empty(true);
    cl.set_piece(true,Piece::R,0,0);
    cl.set_piece(true,Piece::r,4,0);
    cl.set_piece(true,Piece::k,5,0);
    cl.white_active_1 = false;
    assert!(cl.movemaker(true,5,0,5,1)==Ok(true));
    let vec = cl.get_legal_moves(true,5,0);
    cl.print_w_legal(true,&vec);
    cl.all_empty(true);
    cl.set_piece(true,Piece::R,0,0);
    cl.set_piece(true,Piece::r,4,0);
    cl.set_piece(true,Piece::k,5,0);
    cl.white_active_1 = false;
    assert!(cl.movemaker(true,5,0,6,1)==Ok(true));
    cl.all_empty(true);
    cl.set_piece(true,Piece::R,0,0);
    cl.set_piece(true,Piece::r,4,0);
    cl.set_piece(true,Piece::k,5,0);
    cl.white_active_1 = false;
    assert!(cl.movemaker(true,5,0,4,0)==Err(MoveError::NotLegal));
}


#[test]
fn promotion() {
    let mut cl = ChessLogic::new();
    cl.all_empty(true);
    cl.set_piece(true,Piece::P,1,7);
    cl.upgrade_to1 = Piece::Q;
    cl.movemaker(true,1,7,0,7);
    cl.print(true);
    assert!(cl.chess_board1.board[0][7]==Piece::UQ);
    assert!(cl.upgrade_to1==Piece::E);

    cl.all_empty(false);
    cl.set_piece(false,Piece::p,6,7);
    cl.upgrade_to2 = Piece::r;
    cl.white_active_2 = false;
    cl.movemaker(false,6,7,7,7);
    cl.print(false);
    assert!(cl.chess_board2.board[7][7]==Piece::Ur);
    assert!(cl.upgrade_to2==Piece::E);

    let mut cl = ChessLogic::new();
    cl.all_empty(true);
    cl.set_piece(true,Piece::P,1,0);
    cl.upgrade_to1 = Piece::N;
    cl.movemaker(true,1,0,0,0);
    cl.print(true);
    assert!(cl.chess_board1.board[0][0]==Piece::UN);
    assert!(cl.upgrade_to1==Piece::E);
}

#[test]
fn castling_white_side_effect4(){
    let mut cl = ChessLogic::new();
    cl.all_empty(false);
    cl.set_piece(false,Piece::k,0,4);
    cl.set_piece(false,Piece::r,0,7);
    cl.set_piece(false,Piece::r,0,0);
    let vec = cl.get_legal_moves(false,0,4);
    cl.print_w_legal(false,&vec);
    assert!(contains(&vec,(0,6)));
    assert!(contains(&vec,(0,2)));
    cl.white_active_2 = false;
    cl.movemaker(false,0,4,0,2);
    assert!(cl.chess_board2.board[0][0]==Piece::E);
    assert!(cl.chess_board2.board[0][4]==Piece::E);
    assert!(cl.chess_board2.board[0][2]==Piece::K);
    assert!(cl.chess_board2.board[0][3]==Piece::R);
}

#[test]
fn capture_piece(){
     //the order is P-R-N-B-Q
    let mut cl = ChessLogic::new();
    cl.all_empty(true);
    cl.set_piece(true,Piece::R,0,0);
    cl.set_piece(true,Piece::r,4,0);
    cl.white_active_1 = false; 
    assert!(cl.movemaker(true,4,0,0,0)==Ok(true));
    println!("{}== num of capture rooks on board1 which are white",
    cl.get_captured_piece(false,true,1));
    assert!(cl.get_captured_piece(false,true,1)==1);
    cl.set_piece(true,Piece::Q,0,7);
    let vec = cl.get_legal_moves(true,0,0);
    cl.print_w_legal(true,&vec);
    cl.white_active_1 = false; 
    assert!(cl.movemaker(true,0,0,0,7)==Ok(true));
    
    assert!(cl.get_captured_piece(false,true,4)==1);
    cl.set_piece(true,Piece::Q,7,7);
    cl.white_active_1 = false; 
    assert!(cl.movemaker(true,0,7,7,7)==Ok(true));
    assert!(cl.get_captured_piece(false,true,4)==2);
    cl.set_piece(true,Piece::K,6,7);
    cl.movemaker(true,6,7,7,7);
}

   
#[test]
fn capture_piece2(){
    let mut cl = ChessLogic::new();
    cl.all_empty(false);
    cl.set_piece(false,Piece::R,0,0);
    cl.set_piece(false,Piece::r,4,0);
    cl.white_active_2 = false; 
    let vec2 = cl.get_legal_moves(false,4,0);

    cl.print_w_legal(false,&vec2);
    assert!(cl.movemaker(false,4,0,0,0)==Ok(true));
    println!("{}",cl.get_captured_piece(true,true,1));
    assert!(cl.get_captured_piece(true,true,1)==1);

    cl.all_empty(true);
    cl.set_piece(true,Piece::R,0,0);
    cl.set_piece(true,Piece::r,4,0);
    assert!(cl.white_active_2);
    let vec3 = cl.get_legal_moves(true,0,0);
    cl.print_w_legal(true,&vec3);
    assert!(cl.movemaker(true,0,0,4,0)==Ok(true));
    assert!(cl.get_captured_piece(false,false,1)==1);

    cl.all_empty(false);
    cl.set_piece(false,Piece::R,0,0);
    cl.set_piece(false,Piece::r,4,0);
    assert!(cl.white_active_2);
    let vec = cl.get_legal_moves(false,0,0);
    cl.print_w_legal(false,&vec);
    assert!(cl.movemaker(false,0,0,4,0)==Ok(true));
    assert!(cl.get_captured_piece(true,false,1)==1);
}

#[test]
fn king1(){
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
    vecbyhand.push((5,5));
    vecbyhand.push((6,4));
    vecbyhand.push((6,6));
    vecbyhand.push((7,4));
    vecbyhand.push((7,5));
    vecbyhand.push((7,6));
    let mut vec_norm = normalize(&vec);
    let mut vecbyhand_norm = normalize(&vecbyhand);
    vec_norm.sort();
    vecbyhand_norm.sort();


    assert_eq!(vec_norm,vecbyhand_norm);
}

#[test]
fn illegal_rook(){
    let mut cl = ChessLogic::new();
    cl.all_empty(true);
    cl.set_piece(true,Piece::K,7,4);
    cl.set_piece(true,Piece::R,7,7);
    cl.set_piece(true,Piece::r,0,0);
    let board1 = true;
    assert!(cl.movemaker(board1,7,7,6,7)==Ok(true));
    assert!(cl.movemaker(board1,0,0,1,0)==Ok(true));
    assert!(cl.movemaker(board1,6,7,7,7)==Ok(true));
    assert!(cl.movemaker(board1,1,0,1,1)==Ok(true));
    assert!(cl.movemaker(board1,7,4,7,6)==Err(MoveError::NotLegal));
}
#[test]
fn rook2(){
    let mut cl = ChessLogic::new();
    cl.all_empty(true);
    cl.set_piece(true,Piece::q,5,0);
    cl.set_piece(true,Piece::k,3,0);
    cl.set_piece(true,Piece::Ur,4,0);
    cl.set_piece(true,Piece::b,4,1);

    assert_eq!(
        cl.get_legal_moves(true,4,0),Vec::new()
    );

    cl.all_empty(true);
    cl.set_piece(true,Piece::UQ,5,0);
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
fn attacks1(){
    let mut cl = ChessLogic::new();
  
    cl.set_piece(true,Piece::P,5,4);
    cl.set_piece(true,Piece::p,5,5);
    cl.set_piece(true,Piece::R,4,2);
    cl.set_piece(true,Piece::q,4,3);
    cl.set_piece(true,Piece::E,1,7);
    for i in 0..8 {
        cl.set_piece(true,Piece::E,6,i);
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
    cl.set_piece(true,Piece::N,1,4);
    cl.set_piece(true,Piece::R,5,7);
    cl.set_piece(true,Piece::p,1,6);
    cl.set_piece(true,Piece::k,1,7);

    let vec = cl.get_legal_moves(true,1,7);
    cl.print_w_legal(true,&vec);
    assert_eq!(vec,[(0, 6), (0, 7), (2, 6), (2, 7)])

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
    assert_eq!(vec,[(0,6)])
}

#[test]
fn fools_mate(){
    let mut cl = ChessLogic::new();
    cl.chess_board1.board[6][6]=Piece::E;
    cl.chess_board1.board[6][5]=Piece::E;
    cl.chess_board1.board[4][6]=Piece::P;
    cl.chess_board1.board[5][5]=Piece::P;
    cl.chess_board1.board[4][7]=Piece::Uq;

    let vec = cl.get_legal_moves(true,7,4);
    cl.print_w_legal(true,&vec);
    assert_eq!(vec,[(6, 5)])
}

#[test]
fn rook_move(){
    let mut cl = ChessLogic::new();

    cl.all_empty(true);
    cl.set_piece(true,Piece::R,4,4);
    cl.set_piece(true,Piece::R,4,6);
    let vec = cl.get_legal_moves(true,4,4);
    cl.print_w_legal(true,&vec);
    let mut vecbyhand = Vec::new();

    vecbyhand.push((4,5));
    vecbyhand.push((3,4));
    vecbyhand.push((2,4));
    vecbyhand.push((1,4));
    vecbyhand.push((0,4));
    vecbyhand.push((5,4));
    vecbyhand.push((6,4));
    vecbyhand.push((7,4));
    vecbyhand.push((4,3));
    vecbyhand.push((4,2));
    vecbyhand.push((4,1));
    vecbyhand.push((4,0));

    let mut vec_norm = normalize(&vec);
    let mut vecbyhand_norm = normalize(&vecbyhand);
    vec_norm.sort();
    vecbyhand_norm.sort();
    
    assert_eq!(vec_norm,vecbyhand_norm);

    cl.all_empty(true);
    cl.set_piece(true,Piece::r,4,4);
    cl.set_piece(true,Piece::r,4,6);
    let vec2 = cl.get_legal_moves(true,4,4);
    cl.print_w_legal(true,&vec2);
    let mut vec_norm2 = normalize(&vec2);
    vec_norm2.sort();
    assert_eq!(vec_norm2,vecbyhand_norm);
}

#[test]
fn bishop_move(){
    let mut cl = ChessLogic::new();

    cl.all_empty(true);
    cl.set_piece(true,Piece::b,4,4);
    cl.set_piece(true,Piece::b,5,5);
    let mut vec = cl.get_legal_moves(true,4,4);
    cl.print_w_legal(true,&vec);
    let mut vecbyhand = Vec::new();
    vecbyhand.push((0,0));
    vecbyhand.push((1,1));
    vecbyhand.push((2,2));
    vecbyhand.push((3,3));

    vecbyhand.push((5,3));
    vecbyhand.push((6,2));
    vecbyhand.push((7,1));

    vecbyhand.push((3,5));
    vecbyhand.push((2,6));
    vecbyhand.push((1,7));

    let mut vec_norm = normalize(&vec);
    let mut vecbyhand_norm = normalize(&vecbyhand);
    vec_norm.sort();
    vecbyhand_norm.sort();
    
    assert_eq!(vec_norm,vecbyhand_norm);

    cl.all_empty(true);
    cl.set_piece(true,Piece::B,4,4);
    cl.set_piece(true,Piece::B,5,5);
    vec = cl.get_legal_moves(true,4,4);
    cl.print_w_legal(true,&vec);

    let mut vec_norm = normalize(&vec);
    let mut vecbyhand_norm = normalize(&vecbyhand);
    vec_norm.sort();
    vecbyhand_norm.sort();
    
    assert_eq!(vec_norm,vecbyhand_norm);
}

#[test]
fn queen_move(){
    let mut cl = ChessLogic::new();
    cl.all_empty(true);
    cl.set_piece(true,Piece::Q,0,0);
    cl.set_piece(true,Piece::Uq,7,7);
    let mut vec_queen = cl.get_legal_moves(true,7,7);
    let mut vecbyhand_queen = Vec::new();


    vecbyhand_queen.push((0,0));
    vecbyhand_queen.push((1,1));
    vecbyhand_queen.push((2,2));
    vecbyhand_queen.push((3,3));
    vecbyhand_queen.push((4,4));
    vecbyhand_queen.push((5,5));
    vecbyhand_queen.push((6,6));

    vecbyhand_queen.push((6,7));
    vecbyhand_queen.push((5,7));
    vecbyhand_queen.push((4,7));
    vecbyhand_queen.push((3,7));
    vecbyhand_queen.push((2,7));
    vecbyhand_queen.push((1,7));
    vecbyhand_queen.push((0,7));

    vecbyhand_queen.push((7,6));
    vecbyhand_queen.push((7,5));
    vecbyhand_queen.push((7,4));
    vecbyhand_queen.push((7,3));
    vecbyhand_queen.push((7,2));
    vecbyhand_queen.push((7,1));
    vecbyhand_queen.push((7,0));

    let mut vec_norm = normalize(&vec_queen);
    let mut vecbyhand_norm = normalize(&vecbyhand_queen);
    vec_norm.sort();
    vecbyhand_norm.sort();
    
    assert_eq!(vec_norm,vecbyhand_norm);
}

#[test]
fn cool_checkmate1(){
    let mut cl = ChessLogic::new();
    cl.all_empty(true);
    cl.set_piece(true,Piece::k,2,4);
    cl.set_piece(true,Piece::Q,3,2);
    cl.set_piece(true,Piece::P,2,2);
    cl.set_piece(true,Piece::K,4,5);
    cl.set_piece(true,Piece::n,0,0);
    cl.set_piece(true,Piece::R,0,5);

    let mut vec= cl.get_legal_moves(true,2,4);
    let mut vecbyhand = [(1, 3), (1, 4), (1, 5), (2, 3), (2, 5), (3, 3), (3, 4), (3, 5)];
    cl.print_w_legal(true,&vec);
    assert_eq!(vec,vecbyhand);
}

#[test]
fn pawn_attacks(){
    let mut cl = ChessLogic::new();
    let vec = cl.get_legal_moves(true,6,7);
    cl.print_w_legal(true,&vec);
    assert_eq!(false,cl.is_attacked(true,true,5,7));
    assert_eq!(false,cl.is_attacked(true,true,5,6));
    assert_eq!(false,cl.is_attacked(true,true,5,0));

    assert_eq!(true,cl.is_attacked(true,false,5,7));
    assert_eq!(true,cl.is_attacked(true,false,5,6));
    assert_eq!(true,cl.is_attacked(true,false,5,0));

    assert_eq!(true,cl.is_attacked(true,true,2,1));
    assert_eq!(true,cl.is_attacked(true,true,2,0));
    assert_eq!(true,cl.is_attacked(true,true,2,7));
    assert_eq!(true,cl.is_attacked(true,true,2,3));

    assert_eq!(false,cl.is_attacked(true,true,4,0));
    assert_eq!(false,cl.is_attacked(true,true,4,7));
    assert_eq!(false,cl.is_attacked(true,true,3,7));
    assert_eq!(false,cl.is_attacked(true,true,3,0));
    assert_eq!(false,cl.is_attacked(true,true,3,4));
}

#[test]
fn enpassant(){
    let mut cl = ChessLogic::new();
    cl.all_empty(true);
    cl.set_piece(true,Piece::p,4,0);
    cl.set_piece(true,Piece::P,4,1);
    cl.set_pawn_in_last_turn(true, Some((4,1)));
    let mut vec = cl.get_legal_moves(true,4,0);

    let mut vecbyhand = Vec::new();
    vecbyhand.push((5,0));
    vecbyhand.push((5,1));
    assert_eq!(vec,vecbyhand);

    cl.all_empty(true);
    cl.set_piece(true,Piece::p,4,7);
    cl.set_piece(true,Piece::P,4,6);
    cl.set_pawn_in_last_turn(true, Some((4,6)));
    vec = cl.get_legal_moves(true,4,7);

    vecbyhand = Vec::new();
    vecbyhand.push((5,7));
    vecbyhand.push((5,6));
    assert_eq!(vec,vecbyhand);      
    
    cl.all_empty(true);
    cl.set_piece(true,Piece::p,4,6);
    cl.set_piece(true,Piece::P,4,7);
    cl.set_pawn_in_last_turn(true, Some((4,7)));
    vec = cl.get_legal_moves(true,4,6);

    vecbyhand = Vec::new();
    vecbyhand.push((5,6));
    vecbyhand.push((5,7));
    assert_eq!(vec,vecbyhand);

    cl.all_empty(true);
    cl.set_piece(true,Piece::p,3,6);
    cl.set_piece(true,Piece::P,3,7);
    cl.set_pawn_in_last_turn(true,Some((3,6)));
    vec = cl.get_legal_moves(true,3,7);

    vecbyhand = Vec::new();
    vecbyhand.push((2,7));
    vecbyhand.push((2,6));
    assert_eq!(vec,vecbyhand);

    cl.all_empty(true);
    cl.set_piece(true,Piece::p,3,0);
    cl.set_piece(true,Piece::P,3,1);
    cl.set_pawn_in_last_turn(true,Some((3,0)));
    vec = cl.get_legal_moves(true,3,1);

    vecbyhand = Vec::new();
    vecbyhand.push((2,1));
    vecbyhand.push((2,0));
    assert_eq!(vec,vecbyhand);

    cl.all_empty(true);
    cl.set_piece(true,Piece::p,3,0);
    cl.set_piece(true,Piece::P,3,1);
    cl.set_pawn_in_last_turn(true,None);
    vec = cl.get_legal_moves(true,3,1);

    vecbyhand = Vec::new();
    vecbyhand.push((2,1));
    assert_eq!(vec,vecbyhand);
}

#[test]
fn horse_bishop_checkmate(){
    let mut cl = ChessLogic::new();
    cl.all_empty(true);
    cl.set_piece(true,Piece::k,1,7);
    cl.set_piece(true,Piece::K,1,5);
    cl.set_piece(true,Piece::UB,0,5);
    cl.set_piece(true,Piece::N,3,4);

    let vec = cl.get_legal_moves(true,1,7);
    cl.print_w_legal(true,&vec);
    assert_eq!(vec,[(0, 6), (0, 7), (1, 6), (2, 6), (2, 7)]);

    let mut cl = ChessLogic::new();
    cl.all_empty(true);
    cl.set_piece(true,Piece::k,0,0);
    cl.set_piece(true,Piece::K,2,1);
    cl.set_piece(true,Piece::UB,2,2);
    cl.set_piece(true,Piece::N,2,0);

    let vec = cl.get_legal_moves(true,0,0);
    assert_eq!(vec,[(0, 1), (1, 0), (1, 1)]);
}

#[test]
fn legality_check(){
    let mut cl = ChessLogic::new();
    //pawns
    assert_eq!(true,cl.legality_check(true, 1,1,3,1));
    assert_eq!(true,cl.legality_check(true, 1,1,2,1));
    assert_eq!(false,cl.legality_check(true, 1,1,4,1));
    assert_eq!(true,cl.legality_check(true, 6,7,5,7));
    assert_eq!(false,cl.legality_check(true, 6,7,6,7));
    assert_eq!(false,cl.legality_check(true, 6,7,5,6));
}

#[test]
fn board2_1(){
    let mut cl = ChessLogic::new();
    cl.all_empty(false);
    cl.set_piece(false,Piece::q,5,0);
    cl.set_piece(false,Piece::k,3,0);
    cl.set_piece(false,Piece::r,4,0);
    cl.set_piece(false,Piece::b,4,1);
    let vecc = cl.get_legal_moves(false,4,0);
    cl.print_w_legal(false,&vecc);
    assert_eq!(
        vecc,Vec::new()
    );

    cl.all_empty(false);
    cl.set_piece(false,Piece::Q,5,0);
    cl.set_piece(false,Piece::K,3,0);
    cl.set_piece(false,Piece::r,4,0);
    cl.set_piece(false,Piece::B,4,1);

    let vec = cl.get_legal_moves(false,4,0);
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

#[test]
fn board2_2(){
    let mut cl = ChessLogic::new();
    cl.all_empty(false);
    cl.set_piece(false,Piece::k,2,4);
    cl.set_piece(false,Piece::Q,3,2);
    cl.set_piece(false,Piece::P,2,2);
    cl.set_piece(false,Piece::K,4,5);
    cl.set_piece(false,Piece::n,0,0);
    cl.set_piece(false,Piece::R,0,5);

    let mut vec= cl.get_legal_moves(false,2,4);
    let mut vecbyhand = Vec::new();
    vecbyhand.push((1,3));
    vecbyhand.push((1,4));
    vecbyhand.push((1,5));
    vecbyhand.push((2,3));
    vecbyhand.push((2,5));
    vecbyhand.push((3,3));
    vecbyhand.push((3,4));
    vecbyhand.push((3,5));
    cl.print_w_legal(false,&vec);
    assert_eq!(vec,vecbyhand);
}

#[test]
fn cool_checkmate2(){
    let mut cl = ChessLogic::new();
    cl.all_empty(true);
    cl.set_piece(true,Piece::k,2,4);
    cl.set_piece(true,Piece::P,2,3);
    cl.set_piece(true,Piece::P,3,4);
    cl.set_piece(true,Piece::B,4,2);
    cl.set_piece(true,Piece::p,3,5);
    cl.set_piece(true,Piece::Un,1,3);
    
    let mut vec= cl.get_legal_moves(true,2,4);
    let mut vecbyhand = [(1, 4), (1, 5), (2, 3), (2, 5), (3, 3), (3, 4)];
    cl.print_w_legal(true,&vec);
    assert_eq!(vec,vecbyhand);
}

#[test] 
fn chincough(){
    let mut cl = ChessLogic::new();
    cl.all_empty(true);
    cl.set_piece(true,Piece::K,0,0);
    cl.set_piece(true,Piece::P,0,1);
    cl.set_piece(true,Piece::P,1,0);
    cl.set_piece(true,Piece::P,1,1);
    cl.set_piece(true,Piece::n,1,2);

    let mut vec= cl.get_legal_moves(true,0,0);
    cl.print_w_legal(true,&vec);
    assert_eq!(vec,Vec::new());
    let mut vec2=cl.get_legal_moves(true,1,0);
    cl.print_w_legal(true,&vec2);
    assert_eq!(vec2,Vec::new());
    let mut vec3=cl.get_legal_moves(true,0,1);
    cl.print_w_legal(true,&vec3);
    assert_eq!(vec3,Vec::new());
}




#[test]
fn parse(){
    if let Some(vec) = parser::parse(&String::from("e2-e4")) {
        assert_eq!([6,4,4,4],vec);
    }else{
        assert!(false);
    }

    if let  Some(vec) = parser::parse(&String::from("a1-a3")) {
        assert_eq!([7,0,5,0],vec);
    }else{
        assert!(false);
    }
    if let Some(vec) = parser::parse(&String::from("d2-d4")) {
        assert_eq!([6,3,4,3],vec);
    }else{
        assert!(false);
    }
    if let Some(vec) = parser::parse(&String::from("h8-h1")) {
        assert_eq!([0,7,7,7],vec);
    }else{
        assert!(false);
    }
 
}

#[test]
fn castling_black(){
    let mut cl = ChessLogic::new();
    cl.all_empty(true);
    cl.set_piece(true,Piece::k,0,4);
    cl.set_piece(true,Piece::r,0,7);
    cl.set_piece(true,Piece::r,0,0);
    let vec = cl.get_legal_moves(true,0,4);
    cl.print_w_legal(true,&vec);
    assert!(contains(&vec,(0,6)));
    assert!(contains(&vec,(0,2)));

    cl.set_piece(true,Piece::R,7,1);
    let vec = cl.get_legal_moves(true,0,4);
    assert!(contains(&vec,(0,6)));
    assert!(!contains(&vec,(0,2)));

    cl.set_piece(true,Piece::R,7,6);
    let vec = cl.get_legal_moves(true,0,4);
    assert!(!contains(&vec,(0,6)));
    assert!(!contains(&vec,(0,2)));

    cl.set_piece(true,Piece::E,7,1);
    cl.set_piece(true,Piece::E,7,6);
    cl.set_piece(true,Piece::Q,7,4);
    assert!(!contains(&vec,(0,6)));
    assert!(!contains(&vec,(0,2)));
}

#[test]
fn castling_white(){
    let mut cl = ChessLogic::new();
    cl.all_empty(true);
    cl.set_piece(true,Piece::K,7,4);
    cl.set_piece(true,Piece::R,7,7);
    cl.set_piece(true,Piece::R,7,0);
    let vec = cl.get_legal_moves(true,7,4);
    cl.print_w_legal(true,&vec);
    assert!(contains(&vec,(7,6)));
    assert!(contains(&vec,(7,2)));

    cl.set_piece(true,Piece::r,0,1);
    let vec = cl.get_legal_moves(true,7,4);
    assert!(contains(&vec,(7,6)));
    assert!(!contains(&vec,(7,2)));

    cl.set_piece(true,Piece::r,0,6);
    let vec = cl.get_legal_moves(true,7,4);
    assert!(!contains(&vec,(7,6)));
    assert!(!contains(&vec,(7,2)));

    cl.set_piece(true,Piece::E,0,1);
    cl.set_piece(true,Piece::E,0,6);
    cl.set_piece(true,Piece::q,0,4);
    assert!(!contains(&vec,(7,6)));
    assert!(!contains(&vec,(7,2)));
}

#[test]
fn castling_white_side_effect1(){
    let mut cl = ChessLogic::new();
    cl.all_empty(true);
    cl.set_piece(true,Piece::K,7,4);
    cl.set_piece(true,Piece::R,7,7);
    cl.set_piece(true,Piece::R,7,0);
    let vec = cl.get_legal_moves(true,7,4);
    cl.print_w_legal(true,&vec);
    assert!(contains(&vec,(7,6)));
    assert!(contains(&vec,(7,2)));

    cl.movemaker(true,7,4,7,6);
    cl.print(true);
    assert!(cl.chess_board1.board[7][7]==Piece::E);
    assert!(cl.chess_board1.board[7][4]==Piece::E);
    assert!(cl.chess_board1.board[7][6]==Piece::K);
    assert!(cl.chess_board1.board[7][5]==Piece::R);
}

#[test]
fn castling_white_side_effect2(){
    let mut cl = ChessLogic::new();
    cl.all_empty(true);
    cl.set_piece(true,Piece::K,7,4);
    cl.set_piece(true,Piece::R,7,7);
    cl.set_piece(true,Piece::R,7,0);
    let vec = cl.get_legal_moves(true,7,4);
    cl.print_w_legal(true,&vec);
    assert!(contains(&vec,(7,6)));
    assert!(contains(&vec,(7,2)));

    cl.movemaker(true,7,4,7,2);
    assert!(cl.chess_board1.board[7][0]==Piece::E);
    assert!(cl.chess_board1.board[7][4]==Piece::E);
    assert!(cl.chess_board1.board[7][2]==Piece::K);
    assert!(cl.chess_board1.board[7][3]==Piece::R);
}

#[test]
fn castling_white_side_effect3(){
    let mut cl = ChessLogic::new();
    cl.all_empty(false);
    cl.set_piece(false,Piece::K,7,4);
    cl.set_piece(false,Piece::R,7,7);
    cl.set_piece(false,Piece::R,7,0);
    let vec = cl.get_legal_moves(false,7,4);
    cl.print_w_legal(false,&vec);
    assert!(contains(&vec,(7,6)));
    assert!(contains(&vec,(7,2)));

    cl.movemaker(false,7,4,7,2);
    assert!(cl.chess_board2.board[7][0]==Piece::E);
    assert!(cl.chess_board2.board[7][4]==Piece::E);
    assert!(cl.chess_board2.board[7][2]==Piece::K);
    assert!(cl.chess_board2.board[7][3]==Piece::R);
}


#[test]
fn up(){
    let mut cl = ChessLogic::new();
    cl.all_empty(true);
    cl.set_piece(true,Piece::Ur,0,7);
    let vec = cl.get_legal_moves(true,0,7);
    cl.print_w_legal(true,&vec);
    assert!(!contains(&vec,(1,1)));
}


#[test]
fn trivial_yfen(){
    let mut cl =ChessLogic::new();
    let mut a = gen_fen(&mut cl);
    let mut st = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
    match a {
        (b,c) => {
            println!("{}",b);println!("{}",c);
            assert_eq!(st,b); 
            assert_eq!(st,c);},
        _ => {},
    }

    cl.movemaker(true,6,4,4,4);
    a = gen_fen(&mut cl);
    st = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1".to_string();
    match a {
        (b,c) => {
            println!("{}",b);println!("{}",c);
            println!("{:?}",cl.get_pawn_in_last_turn(true));
            assert_eq!(st,b); 
           },
        _ => {},
    }

    cl.movemaker(true,1,2,3,2);
    a = gen_fen(&mut cl);
    st = "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2".to_string();
    match a {
        (b,c) => {
            println!("{}",b);println!("{}",c);
            println!("{:?}",cl.get_pawn_in_last_turn(true));
            assert_eq!(st,b); 
           },
        _ => {},
    }

    cl.movemaker(true,7,6,5,5);
    a = gen_fen(&mut cl);
    st = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2".to_string();
    match a {
        (b,c) => {
            println!("{}",b);println!("{}",c);
            println!("{:?}",cl.get_pawn_in_last_turn(true));
            assert_eq!(st,b); 
           },
        _ => {},
    }
}
