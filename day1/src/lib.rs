fn part1(filename: &str) -> i32 {
    use input_reader::read_input;

    let mut result = 0;
    let input = read_input(filename).unwrap();

    for line in input {
        let (mut first_number, mut last_number) = (10,0);
  
        for char in line.chars() {
            if char.is_numeric() {
                if first_number == 10 {
                    first_number = char.to_digit(10).unwrap();
                }

                last_number = char.to_digit(10).unwrap();
            }
        }

        let calibration_value = format!("{}{}", first_number, last_number).parse::<i32>().unwrap();
        result += calibration_value;
    };

    result
}


fn part2(filename: &str) -> i32 {
    use input_reader::read_input;
    use std::collections::HashMap;

    let mut result = 0;
    let map = HashMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2), 
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)]);
    let input = read_input(filename).unwrap();

    for line in input {
        let (mut first_number, mut last_number) = (10,0);
        let mut i = 0;

        let characters = line.chars().collect::<Vec<char>>();

        while i < line.len() {
            if characters[i].is_numeric() {
                if first_number == 10 {
                    first_number = characters[i].to_digit(10).unwrap();
                }

                last_number = characters[i].to_digit(10).unwrap();
            }

            for (&name, &number) in &map {
                if line[i..].starts_with(name) {
                    if first_number == 10 {
                        first_number = number;
                    }

                    last_number = number;
                }
            }
            
            i += 1;
        }
        
        let calibration_value = format!("{}{}", first_number, last_number).parse::<i32>().unwrap();
        result += calibration_value;
    };

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_input() {
        let result = part1("src/test_input1.txt");

        assert_eq!(result, 142);
    }

    #[test]
    fn part1_real_input() {
        let result = part1("src/real_input.txt");

        assert_eq!(result, 54951);
    }

    #[test]
    fn part2_test_input() {
        let result = part2("src/test_input2.txt");

        assert_eq!(result, 281);
    }

    #[test]
    fn part2_real_input() {
        let result = part2("src/real_input.txt");

        assert_eq!(result, 55218);
    }
}