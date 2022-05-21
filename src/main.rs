
#[test]
fn line_access() {
    type Line = [i32; 10];
    let mut line: Line = [0;10];
    line [0] = 1;
    assert_eq! (line [0], 1);
}
    
fn main() {
    println!("Hello, world!");
}
