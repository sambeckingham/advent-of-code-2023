use std::io::{self, BufRead, BufReader};
use std::fs::File;

pub fn read_input(filename: &str) -> io::Result<Vec<String>> {
    let input = File::open(filename)?;
    let reader = BufReader::new(input);
    reader.lines().collect()
}

#[cfg(test)]
mod tests {
    use crate::read_input;

    #[test]
    fn test_input_reader() {
        let result = read_input("src/test_input.txt").unwrap();

        assert_eq!(result, vec!["abc", "123", "{}{"]);
    }
}
