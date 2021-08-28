fn main() {
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    let _c = [3; 10];
    let tup = (8, 8, 8);
    let count: f64 = 128.01;
    let remainder = 43 % 5;
    let space = "   ";
    let c = 'A';
    println!("{} {} {}", count, remainder, c);
    let space = space.len();
    println!("{} {} {} {}", space, tup.0, tup.1, _c[0]);
}
