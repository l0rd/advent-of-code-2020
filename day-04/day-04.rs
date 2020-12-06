use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut valid_passports_1=0;
    let mut valid_passports_2=0;
    if let Ok(lines) = read_lines("./input") {
        let mut current_passport = String::new();
        for line in lines {
            if let Ok(passport_line) = line {
                if passport_line.is_empty() {
                    if is_valid1(&current_passport) {
                        valid_passports_1 += 1;                    
                    }

                    if is_valid2(&current_passport) {
                        valid_passports_2 += 1;
                    }
                    current_passport = String::from("");
                }
                current_passport = format!("{} {}", current_passport, passport_line);
            }
        }
        
        if is_valid1(&current_passport) {
            valid_passports_1 += 1;
        }
        if is_valid2(&current_passport) {
            valid_passports_2 += 1;
        }

        println!("{}", valid_passports_1);
        println!("{}", valid_passports_2);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn is_valid1(passport: &String)  -> bool {
    let required_fields = ["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"];

    for field in required_fields.iter() {
        if !passport.contains(field) {
            return false
        }
    }
    
    return true;
}

fn is_valid2(passport: &String)  -> bool {

    let required_fields = ["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"];

    for field in required_fields.iter() {
        if !passport.contains(field) {
            return false
        }
    }

    for field in passport.split_whitespace() {

        if field.starts_with("byr") {
            let value = field.split_at(4).1;
            if !is_valid_byr(value.to_string()) {
                return false
            }
        }
        
        if field.starts_with("iyr") {
            let value = field.split_at(4).1;
            if !is_valid_iyr(value.to_string()) {
                return false
            }
        }

        if field.starts_with("eyr") {
            let value = field.split_at(4).1;
            if !is_valid_eyr(value.to_string()) {
                return false
            }
        }

        if field.starts_with("hgt") {
            let value = field.split_at(4).1;
            if !is_valid_hgt(value.to_string()) {
                return false
            }
        }

        if field.starts_with("hcl") {
            let value = field.split_at(4).1;
            if !is_valid_hcl(value.to_string()) {
                return false
            }
        }

        if field.starts_with("ecl") {
            let value = field.split_at(4).1;
            if !is_valid_ecl(value.to_string()) {
                return false
            }
        }

        if field.starts_with("pid") {
            let value = field.split_at(4).1;
            if !is_valid_pid(value.to_string()) {
                return false
            }
        }

    }

    return true;
}

fn is_valid_byr(value: String) -> bool {
    if value.len() != 4 {
        return false;
    }
    
    if let Ok(value_num) = value.parse::<u32>() {
        if value_num < 1920 || value_num > 2002 {
            return false;
        }
    } else {
        return false;
    }

    return true;    
}

fn is_valid_iyr(value: String) -> bool {
    if value.len() != 4 {
        return false;
    }
    
    if let Ok(value_num) = value.parse::<u32>() {
        if value_num < 2010 || value_num > 2020 {
            return false;
        }
    } else {
        return false;
    }

    return true;    
}

fn is_valid_eyr(value: String) -> bool {
    if value.len() != 4 {
        return false;
    }
    
    if let Ok(value_num) = value.parse::<u32>() {
        if value_num < 2020 || value_num > 2030 {
            return false;
        }
    } else {
        return false;
    }

    return true;    
}

fn is_valid_hgt(value: String) -> bool {
    if value.ends_with("cm") {
        if let Ok(value_num) = value.replace("cm", "").parse::<u32>() {
            if value_num < 150 || value_num > 193 {
                return false;
            }
        } else {
            return false;
        }
    } else if value.ends_with("in") {
        if let Ok(value_num) = value.replace("in", "").parse::<u32>() {
            if value_num < 59 || value_num > 76 {
                return false;
            }
        } else {
            return false;
        }
    } else {
        return false;
    }

    return true;    
}

fn is_valid_hcl(value: String) -> bool {
    if !value.starts_with("#") {
        return false;
    }

    if value.len() != 7 {
        return false;
    }

    let valid_chars="1234567890abcdef";
    for char in value[1..].chars() {
        if !valid_chars.contains(char) {
            return false;
        }
    }

    return true;  
}

fn is_valid_ecl(value: String) -> bool {
    if !value.eq("amb") && 
       !value.eq("blu") && 
       !value.eq("brn") && 
       !value.eq("gry") && 
       !value.eq("grn") && 
       !value.eq("hzl") && 
       !value.eq("oth"){
        return false;
    }
    return true;
}

fn is_valid_pid(value: String) -> bool {
    if value.len() != 9 {
        return false;
    }

    let valid_chars="1234567890";
    for char in value.chars() {
        if !valid_chars.contains(char) {
            return false;
        }
    }

    return true;
}

