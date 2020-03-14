#![allow(warnings)] 
mod logic; 
use crate::logic::ChessLogic;
use crate::logic::board::Piece;
use crate::parse::parser;
mod parse;
//use crate::parser::parser::echo;
use std::io;

fn main() {    
    let mut chess_logic = ChessLogic::new();
    /*
    //let legals = chess_logic.get_legal_moves(true,6,7);
    //chess_logic.print_w_legal(true,&legals);
    //chess_logic.chess_board1.board[1][0] = Piece::E;
    chess_logic.testfoo();
    chess_logic.print_w_legal(true,&chess_logic.get_legal_moves(true,1,0));
    chess_logic.print_w_legal(true,&chess_logic.get_legal_moves(true,7,7));
    chess_logic.print_w_legal(true,&chess_logic.get_legal_moves(true,7,0));
    println!("{}",chess_logic.get_piece(true,4,5));

    chess_logic.print_w_legal(true,&chess_logic.get_legal_moves(true,4,2));
    chess_logic.print_w_legal(true,&chess_logic.get_legal_moves(true,4,3));
    chess_logic.print_w_legal(true,&chess_logic.get_legal_moves(true,0,1));
    chess_logic.print_w_legal(true,&chess_logic.get_legal_moves(true,7,1));
    println!("{}",chess_logic.is_attacked(true,true,6,6));
    println!("{}",chess_logic.is_attacked(true,true,7,7));
    println!("{}",chess_logic.is_attacked(true,true,3,4));
    println!("{}",chess_logic.is_attacked(true,true,2,3));
    println!("{}",chess_logic.is_attacked(true,true,0,5));

    chess_logic.all_empty(true);
    chess_logic.set_piece(true,Piece::r,7,7);
    chess_logic.print_w_legal(true,&chess_logic.get_legal_moves(true,7,7));

    chess_logic.all_empty(true);
    chess_logic.set_piece(true,Piece::q,7,7);
    chess_logic.print_w_legal(true,&chess_logic.get_legal_moves(true,7,7));

    chess_logic.all_empty(true);
    chess_logic.set_piece(true,Piece::q,7,7);
    chess_logic.set_piece(true,Piece::r,0,4);
    chess_logic.set_piece(true,Piece::K,6,5);
    chess_logic.print_w_legal(true,&chess_logic.get_legal_moves(true,6,5));
    */
    while(true){
        let mut input = String::new();
        let mut input_casted = [8;4];
        chess_logic.print(true);

        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                input_casted = parser::parse(&input);
            }
            Err(error) => println!("error: {}", error),
        }

        for i in input_casted.iter(){
            print!("{}, ",i);
            print!("\n");
        }

        if input_casted[0] == 8 {
            let mut vt = chess_logic.get_legal_moves(true,input_casted[1],input_casted[2]);
           chess_logic.print_w_legal(true, &vt);
        }else{
            let pic = chess_logic.chess_board1.board[input_casted[0]][input_casted[1]];
            chess_logic.chess_board1.board[input_casted[0]][input_casted[1]]=Piece::E;
            chess_logic.chess_board1.board[input_casted[2]][input_casted[3]]=pic;
            
            let mut v = chess_logic.get_legal_moves(true,input_casted[2],input_casted[3]);
            chess_logic.print_w_legal(true,&v);
        }
    }

}