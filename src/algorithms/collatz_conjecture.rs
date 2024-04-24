pub fn collatz_conjecture(mut n: i32) -> u32 {
    let mut len: u32 = 1;

    while n > 1 {   
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1};
        len += 1;
    }

    len
}

#[test]
fn test_collatz_conjecture() {
    assert_eq!(collatz_conjecture(100), 25)
}
 