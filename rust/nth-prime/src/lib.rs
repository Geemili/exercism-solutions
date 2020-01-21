pub fn nth(mut n: u32) -> u32 {
    let mut current_num = 2;
    while n != 0 {
        n -= 1;
        current_num += 1;
        while !is_prime(current_num) {
            current_num += 1;
        }
    }
    current_num
}

fn is_prime(n: u32) -> bool {
    let max_to_check = (n as f32).sqrt().floor() as u32;
    for x in 2..=max_to_check {
        if n % x == 0 {
            return false;
        }
    }
    return true;
}
