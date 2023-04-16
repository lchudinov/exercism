// Take a look at the tests, you might have to change the function arguments

pub fn binarySearch(comptime T: type, target: T, items: []const T) ?usize {
    var lower: usize = 0;
    var upper: usize = items.len;
    if (items.len == 0) {
        return null;
    }
    while (lower <= upper) {
        const middle = (lower + upper) / 2;
        const item = items[middle];
        if (item == target) {
            return middle;
        } else if (target < item) {
            if (middle == 0) {
                return null;
            }
            upper = middle - 1;
        } else {
            if (middle >= items.len - 1) {
                return null;
            }
            lower = middle + 1;
        }
    }
    return null;
}
