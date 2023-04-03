pub fn collatz(n: u64) -> Option<u64> {
    let mut steps = 0;
    let mut num = n;
    if n == 0 {
        return None;
    }
    while num != 1 {
        if num % 2 == 0 {
            num /= 2;
        } else if let Some(res) = 3u64.checked_mul(num)?.checked_add(1) {
            num = res;
        } else {
            return None;
        }
        steps += 1;
    }
    Some(steps)
}
