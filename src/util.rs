pub fn contains(vec: &Vec<(usize,usize)>,(i,j): (usize,usize)) -> bool {
    for (a,b) in vec.iter() {
        if *a==i && j==*b {
            return true
        }
    }
    return false
}