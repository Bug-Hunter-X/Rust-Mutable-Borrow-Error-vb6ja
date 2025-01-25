fn main() {
    let mut x = 5;
    { //Using a block to limit the scope of y
        let y = &mut x; 
        *y = 10; 
    }
    let z = &mut x; 
    *z = 15;
}
