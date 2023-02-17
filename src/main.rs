fn main() {
    println!("Hello, world!");
    println!("{:?}", solution(10));
}

fn solution(num: i32) -> i32 {
    let mut sum: i32 = 0;
    if num < 0 {
        return 0;
    } else {
        for i in 1..num {
            if i % 3 == 0 {
                sum += i;
            } else {
                if i % 5 == 0 {
                    sum += i;
                } else {
                    continue;
                }
            }
        }
    }
    sum
}
