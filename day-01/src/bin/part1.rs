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
    let normal_line = find_first_number(line);
    let rev: String = line.chars().rev().collect();
    let reversed_line = find_first_number(&rev);

    return normal_line * 10 + reversed_line;
}

fn find_first_number(line: &str) -> i32 {
    let mut value: Option<i32> = None;
    for c in line.chars() {
        if c.is_numeric() && value == None {
            value = Some(c.to_digit(10).unwrap() as i32);
        }
    }
    if let Some(v) = value {
        return v;
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let file_content = String::from(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );

        let lines: Vec<&str> = file_content.lines().collect();
        let mut ans = 0;

        for item in lines.iter() {
            ans += find_value_for_line(item);
        }

        assert_eq!(ans, 142);
    }
}
