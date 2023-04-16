pub fn find(items: &[i32], target: i32) -> Option<usize> {
    let mut lower: usize = 0;
    let mut upper: usize = items.len();
    if items.is_empty() {
        return None;
    }
    while lower <= upper {
        let middle = (lower + upper) / 2;
        let item = items[middle];
        if item == target {
            return Some(middle);
        } else if target < item {
            if middle == 0 {
                return None;
            }
            upper = middle - 1;
        } else {
            if middle >= items.len() - 1 {
                return None;
            }
            lower = middle + 1;
        }
    }
    None
}
