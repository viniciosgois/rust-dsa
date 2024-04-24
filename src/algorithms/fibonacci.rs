pub fn fibonacci(n: u32) -> u32 {
    if n < 2 {
        return n;
    } else { 
        return fibonacci(n - 1) + fibonacci(n-2);
    }
}

#[test]
fn test_fibonacci() {
    assert_eq!(fibonacci(10), 35)
}