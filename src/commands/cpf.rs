use rand::{thread_rng, Rng};
use std::cmp::Ordering;

use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

fn is_format_valid(value: &String) -> bool {
    let reg_exp = Regex::new(r"^\d{11}$").unwrap();

    reg_exp.is_match(value.as_str())
}

fn clean_mask(value: String) -> String {
    let reg_exp = Regex::new(r"[.-]").unwrap();
    let result = reg_exp.replace_all(value.as_str(), "").to_string();
    let is_valid = is_format_valid(&result);

    match is_valid {
        true => result,
        false => panic!("Invalid CPF format"),
    }
}

fn format_cpf(value: String) -> String {
    let reg_exp = Regex::new(r"(\d{3})(\d{3})(\d{3})(\d{2})").unwrap();

    reg_exp.replace(value.as_str(), "$1.$2.$3-$4").to_string()
}

pub fn generate() {
    let mut rng = thread_rng();
    let mut raw_cpf = String::new();

    for _ in 0..9 {
        let n: usize = rng.gen_range(0..=9);

        raw_cpf.push_str(n.to_string().as_str());
    }

    let mut first_digit_sum = 0;
    let mut val_chars = raw_cpf.graphemes(true).collect::<Vec<&str>>();

    for i in 0..9 {
        let num: usize = val_chars[i].parse().unwrap();

        first_digit_sum += num * (10 - i);
    }

    let first_rest = first_digit_sum % 11;
    let first_digit = match first_rest.cmp(&2) {
        Ordering::Less => 0,
        _ => 11 - first_rest,
    };

    let first_digit_string = first_digit.to_string();

    val_chars.push(first_digit_string.as_str());

    let mut second_digit_sum = 0;

    for i in 0..10 {
        let num: usize = val_chars[i].parse().unwrap();

        second_digit_sum += num * (11 - i);
    }

    let second_digit_rest = second_digit_sum % 11;
    let second_digit = match second_digit_rest.cmp(&2) {
        Ordering::Less => 0,
        _ => 11 - second_digit_rest,
    };

    let second_digit_string = second_digit.to_string();
    val_chars.push(second_digit_string.as_str());

    println!("{:?}", format_cpf(val_chars.join("")));
}

pub fn validate(value: &Option<String>) {
    match value {
        Some(val) => {
            let parsed_value = clean_mask(val.to_string());

            let mut first_digit_sum = 0;
            let val_chars = parsed_value.graphemes(true).collect::<Vec<&str>>();

            let mut are_all_digits_equal = true;

            for &c in &val_chars {
                are_all_digits_equal = c == val_chars[0];

                if !are_all_digits_equal {
                    break;
                }
            }

            if are_all_digits_equal {
                panic!("Invalid CPF");
            }

            for i in 0..9 {
                let num: usize = val_chars[i].parse().unwrap();

                first_digit_sum += num * (10 - i);
            }

            let first_rest = first_digit_sum % 11;
            let first_digit = match first_rest.cmp(&2) {
                Ordering::Less => 0,
                _ => 11 - first_rest,
            };

            let current_first_digit: usize = val_chars[9].parse().unwrap();

            if current_first_digit != first_digit {
                panic!("CPF invalid");
            }

            let mut second_digit_sum = 0;

            for i in 0..10 {
                let num: usize = val_chars[i].parse().unwrap();

                second_digit_sum += num * (11 - i);
            }

            let second_digit_rest = second_digit_sum % 11;
            let second_digit = match second_digit_rest.cmp(&2) {
                Ordering::Less => 0,
                _ => 11 - second_digit_rest,
            };

            let current_second_digit: usize = val_chars[10].parse().unwrap();

            if current_second_digit != second_digit {
                panic!("Invalid CPF");
            }

            println!("Valid CPF");
        }
        _ => panic!("CPF not given"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Invalid CPF format")]
    fn test_validate() {
        let cpf = Some(String::from("864.720.875-7"));

        validate(&cpf);
    }
}
