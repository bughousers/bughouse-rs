pub mod parser {
    pub fn parse(input: &String) -> [usize; 4] {
        let mut rt = [8;4];
        let mut ct = 0;
        for c in input.chars() {
            println!("{}",c);
            match c {
                'L' | 'l' => {rt[ct] = 8; ct+=1;},
                '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'=> {
                    match c.to_digit(10) {
                        None => {rt[ct] = 10 as usize; ct+=1;},
                        Some(x) => {rt[ct] = x as usize; ct+=1;},
                    }
                },
                _ => {},
            }
        }
        rt
    }
}