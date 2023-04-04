pub fn get_diamond(c: char) -> Vec<String> {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
    let size = alphabet.find(c).unwrap() * 2 + 1;
    let mut diamond: Vec<String> = vec![];
    let center = (size - 1) / 2;
    let mut x_1 = center;
    let mut x_2 = center;
    for i in 0..size {
        let index = if i > center { 2 * center - i } else { i };
        let letter = alphabet.chars().nth(index).unwrap();
        let mut row = String::new();
        for j in 0..size {
            if j == x_1 || j == x_2 {
                row.push(letter);
            } else {
                row.push(' ');
            }
        }
        diamond.push(row);
        if i < center && x_1 > 0 {
            x_1 -= 1;
            x_2 += 1;
        } else if i >= center && x_2 > 0 {
            x_1 += 1;
            x_2 -= 1;
        }
    }
    diamond
}
