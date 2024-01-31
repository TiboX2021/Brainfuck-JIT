/// Returns true if the input char is one of the 8 valid brainfuck instructions
pub fn is_brainfuck_code(c: char) -> bool {
    matches!(c, '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']')
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
