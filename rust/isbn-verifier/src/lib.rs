// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let (count, total, valid) = isbn.chars().fold((0, 0, true), |(count, total, valid), c| {
        let multiplier = 10 - count;
        match c {
            '-' => (count, total, valid),
            'X' => (count + 1, total + multiplier * 10, valid && count == 9),
            '0'...'9' => (
                count + 1,
                total + multiplier * c.to_digit(10).unwrap() as usize,
                valid && true,
            ),
            _ => (0, 0, false),
        }
    });
    total % 11 == 0 && count == 10 && valid
}
