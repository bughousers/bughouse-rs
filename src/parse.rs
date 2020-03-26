pub mod parser {
    //parse method only works if the input is correct
    pub fn parse(input: &String) -> [usize; 4] {

        let mut rt = [8;4];
        let mut rtc = 1;
        let mut splt = input.split("-");

        for s in splt {
            let mut f = true;
            for c in s.chars() {
                if c!='\n' {
                    if f {
                        rt[rtc] = char2ind(c);
                        rtc -=1;
                        f = false;
                    }else{
                        rt[rtc] = line2line(c);
                        rtc +=3;
                    }
                }
               
            }

        }

        rt
    }

    //converts chess column to array column
    pub fn char2ind(a: char) -> usize {
        match a {
            'a' | 'A' => 0,
            'b' | 'B'  => 1,
            'c' | 'C'  => 2,
            'd' | 'D'  => 3,
            'e' | 'E'  => 4,
            'f' | 'F'  => 5,
            'g' | 'G'  => 6,
            'h' | 'H'  => 7,
            _ => {println!("Nonlegal input, char2ind"); 666},
        }
    }

    //converts array column to chess column
    pub fn ind2char(a: usize) -> char {
        match a {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => {println!("Nonlegal input, ind2char"); 'x'},
        }
    }

    //convert chess line -> array line
    pub fn line2line(a: char) -> usize {
        match a {
            '1' => 7,
            '2' => 6,
            '3' => 5,
            '4' => 4,
            '5' => 3,
            '6' => 2,
            '7' => 1,
            '8' => 0,
            _ => {println!("Nonlegal input, line2line"); 666},
        }
    }

    //converts array line to line 
    pub fn ind2line(a: usize) -> char {
        match a {
            0 => '8',
            1 => '7',
            2 => '6',
            3 => '5',
            4 => '4',
            5 => '3',
            6 => '2',
            7 => '1',
            _ => {println!("Nonlegal input, ind2line"); 'x'},
        }
    }

}