pub fn is_armstrong_number(num: u32) -> bool {
    let i = no_of_digits(num);
    let ans = armstrong(i, num);
    num == ans
}

fn armstrong(i: u32, num: u32) -> u32 {
    if num > 0 {
        let d = num % 10;
        armstrong(i, num / 10) + d.pow(i)
    } else {
        0
    }
}

fn no_of_digits(num: u32) -> u32 {
    if num / 10 > 0 {
        no_of_digits(num / 10) + 1
    } else {
        1
    }
}
