fn main() {
    let mut x = 5;
    let y = &mut x; 
    let z = &mut x; 
    *y = 6; 
    *z = 7; //Data race here
    println!("x = {}", x);
}