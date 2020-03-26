pub mod parser {
    ///Parses an input such as "e2-e4" and converts into a form that Chesslogic will understand
    /// 
    /// Input is of form: column index,row index,-,new column index,new row index
    pub fn parse(input: &String) -> Option<[usize; 4]> {

        let mut rt = [8;4];
        let mut rtc = 1;
        let mut splt = input.split("-");

        for s in splt {
            let mut f = true;
            for c in s.chars() {
                if c!='\n' {
                    if f {
                        if let Some(x) = char2ind(c) {
                            rt[rtc] = x;
                        }else{
                            return None
                        }
                        rtc -=1;
                        f = false;
                    }else{
                        if let Some(x) = line2line(c) {
                            rt[rtc] = x;
                        }else{
                            return None
                        }
                        rtc +=3;
                    }
                }
               
            }

        }

        Some(rt)
    }

    ///Converts an SAN column index to array column index
    pub fn char2ind(a: char) -> Option<usize> {
        match a {
            'a' | 'A' => Some(0),
            'b' | 'B'  => Some(1),
            'c' | 'C'  => Some(2),
            'd' | 'D'  => Some(3),
            'e' | 'E'  => Some(4),
            'f' | 'F'  => Some(5),
            'g' | 'G'  => Some(6),
            'h' | 'H'  => Some(7),
            _ => None,
        }
    }

    ///Converts an array index column to SAN column index
    pub fn ind2char(a: usize) -> Option<char> {
        match a {
            0 => Some('a'),
            1 => Some('b'),
            2 => Some('c'),
            3 => Some('d'),
            4 => Some('e'),
            5 => Some('f'),
            6 => Some('g'),
            7 => Some('h'),
            _ => None,
        }
    }

    ///convert a SAN line index to an array line index
    pub fn line2line(a: char) -> Option<usize> {
        match a {
            '1' => Some(7),
            '2' => Some(6),
            '3' => Some(5),
            '4' => Some(4),
            '5' => Some(3),
            '6' => Some(2),
            '7' => Some(1),
            '8' => Some(0),
            _ =>  None,
        }
    }

    //converts array line  index to SAN line index
    pub fn ind2line(a: usize) -> Option<char> {
        match a {
            0 => Some('8'),
            1 => Some('7'),
            2 => Some('6'),
            3 => Some('5'),
            4 => Some('4'),
            5 => Some('3'),
            6 => Some('2'),
            7 => Some('1'),
            _ => None,
        }
    }

}