fn main() {
    let file_content = include_str!("./input1.txt");

    let lines: Vec<&str> = file_content.lines().collect();

    let mut ans = 0;

    for item in lines.iter() {
        ans += find_value_for_line(item);
    }

    println!("{}", ans);
}

fn find_value_for_line(line: &str) -> i32 {
    let normal_line = get_number_from_string(line, StrType::Normal);
    let reversed_line = get_number_from_string(line, StrType::Reversed);

    println!(
        "line {}, first digit {}, last {}",
        line, normal_line, reversed_line
    );
    return normal_line * 10 + reversed_line;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2, get_number_from_string("onetwo", StrType::Reversed));
        assert_eq!(29, find_value_for_line("two1nine"))
    }

    #[test]
    fn test_for_multiple_line() {
        let file_content = String::from(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );

        let lines: Vec<&str> = file_content.lines().collect();
        let mut ans = 0;

        for item in lines.iter() {
            ans += find_value_for_line(item);
        }

        assert_eq!(ans, 281);
    }
}
#[derive(PartialEq)]
enum StrType {
    Normal,
    Reversed,
}

fn get_number_from_string(line: &str, str_type: StrType) -> i32 {
    let mut substr = String::new();

    let reversed_line: String = if str_type == StrType::Reversed {
        line.chars().rev().collect()
    } else {
        line.to_string()
    };

    for c in reversed_line.chars() {
        if str_type == StrType::Reversed {
            if c.is_numeric() {
                return c.to_digit(10).unwrap() as i32;
            }
            substr = c.to_string() + &substr;
        } else {
            if c.is_numeric() {
                return c.to_digit(10).unwrap() as i32;
            }
            substr.push(c);
        }

        if substr.contains("one") {
            return 1;
        } else if substr.contains("two") {
            return 2;
        } else if substr.contains("three") {
            return 3;
        } else if substr.contains("four") {
            return 4;
        } else if substr.contains("five") {
            return 5;
        } else if substr.contains("six") {
            return 6;
        } else if substr.contains("seven") {
            return 7;
        } else if substr.contains("eight") {
            return 8;
        } else if substr.contains("nine") {
            return 9;
        }
    }
    println!("{}", substr);
    return 0;
}
