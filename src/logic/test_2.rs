#[cfg(test)]
mod tests {
    use super::*;
    use super::parse::parser;
    use super::infoCourier::infoCourier::gen_yfen;
    
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn deploying_pieces_W_fen(){
        let mut cl =ChessLogic::new();
        cl.recv_piece(true,true,Piece::P);
        assert!(cl.deploy_piece(true,true,Piece::P,4,4));
        let mut a = super::gen_yfen(&mut cl);
        cl.print(true);
        let mut st = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPPPPPP/RNBQKBNR b ---- e3 0 1".to_string();
        match a {
            (b,c) => {
                println!("{}",b);println!("{}",c);
                assert_eq!(st,b); 
                },
            _ => (),
        }
        cl.recv_piece(true,false,Piece::r);
        assert!(cl.deploy_piece(true,false,Piece::r,3,4));
        let mut a = super::gen_yfen(&mut cl);
        cl.print(true);
        let mut st = "rnbqkbnr/pppppppp/8/4r3/4P3/8/PPPPPPPP/RNBQKBNR w ---- - 1 2".to_string();
        match a {
            (b,c) => {
                println!("{}",b);println!("{}",c);
                assert_eq!(st,b); 
                },
            _ => (),
        }
    }
}
    
   