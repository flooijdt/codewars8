fn main() {
    println!("Hello, world!");
    println!("{:?}", solution(10));
}

fn solution(num: i32) -> i32 {
    for i in 1..num {
        println!("{:?}", i);
    }
    0
}
