fn main() {
    println!("Hello, world!");
    let n = 1000;
    let m = 950;
    let msg = format!("----------- Greatest Common Divisor n {n} m {m}----------", n=n, m=m);
    println!("{}", msg);
    let gcd = greatest_common_divisor(n, m);
    let gcd_msg = format!("------ gcd is {gcd} -----------", gcd=gcd);
    println!("{}", gcd_msg);
}

fn greatest_common_divisor(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    
    while m != 0 {
        if m < n {
            let tmp = m;
            m = n;
            n = tmp;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_greatest_common_divisor(){
    assert_eq!(greatest_common_divisor(14, 15), 1);

    let test_n = 1000;
    let test_m = 50;
    let expected_gcd = 50;
    assert_eq!(greatest_common_divisor(test_n, test_m), expected_gcd);

    let test_n = 2 * 3 * 5 * 11 * 17;
    let test_m = 3 * 7 * 11* 13 * 19;
    let expected_gcd = 33;
    assert_eq!(greatest_common_divisor(test_n, test_m), expected_gcd);
}

#[test]
#[should_panic]
fn test_greatest_common_divisor_zero_n(){
    let test_n = 0;
    let test_m = 2;
    greatest_common_divisor(test_n, test_m);

}


#[test]
#[should_panic]
fn test_greatest_common_divisor_zero_m(){
    let test_n = 2;
    let test_m = 0;
    greatest_common_divisor(test_n, test_m);
}
