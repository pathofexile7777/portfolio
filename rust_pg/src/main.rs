fn my_move() -> i32 {
    let num = 5;
    let add_num = move |x: i32| x + num;
    add_num(2)
}
fn main() {
    let x = my_move();
    let y = x;
    println!("{}", y);
}