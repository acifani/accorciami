static CHARSET: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z',
];

static BASE: u64 = 62;

pub fn encode_in_base62(v: u64) -> String {
    let mut n = v;

    if n == 0 {
        return "0".to_string();
    }

    let mut stack: Vec<char> = vec![];
    while n > 0 {
        let r = (n % BASE) as usize;
        n /= BASE;
        stack.push(CHARSET[r]);
    }

    stack.reverse();
    return stack.into_iter().collect();
}
