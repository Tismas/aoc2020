use crate::utils::numeric::is_between;
use regex::Regex;
use std::ops::{Index, IndexMut};

pub struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Index<&'_ str> for Passport {
    type Output = Option<String>;
    fn index(&self, s: &str) -> &Option<String> {
        match s {
            "byr" => &self.byr,
            "iyr" => &self.iyr,
            "eyr" => &self.eyr,
            "hgt" => &self.hgt,
            "hcl" => &self.hcl,
            "ecl" => &self.ecl,
            "pid" => &self.pid,
            "cid" => &self.cid,
            _ => panic!("unknown field: {}", s),
        }
    }
}

impl IndexMut<&'_ str> for Passport {
    fn index_mut(&mut self, s: &str) -> &mut Option<String> {
        match s {
            "byr" => &mut self.byr,
            "iyr" => &mut self.iyr,
            "eyr" => &mut self.eyr,
            "hgt" => &mut self.hgt,
            "hcl" => &mut self.hcl,
            "ecl" => &mut self.ecl,
            "pid" => &mut self.pid,
            "cid" => &mut self.cid,
            _ => panic!("unknown field: {}", s),
        }
    }
}

impl Default for Passport {
    fn default() -> Passport {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }
}

impl Passport {
    fn is_byr_valid(&self) -> bool {
        return is_between(
            self.byr.as_ref().unwrap().parse::<i32>().unwrap(),
            1920,
            2002,
        );
    }
    fn is_iyr_valid(&self) -> bool {
        return is_between(
            self.iyr.as_ref().unwrap().parse::<i32>().unwrap(),
            2010,
            2020,
        );
    }
    fn is_eyr_valid(&self) -> bool {
        return is_between(
            self.eyr.as_ref().unwrap().parse::<i32>().unwrap(),
            2020,
            2030,
        );
    }
    fn is_hgt_valid(&self) -> bool {
        let hgt = self.hgt.as_ref().unwrap();
        if hgt.contains("cm") {
            return is_between(hgt.split("cm").nth(0).unwrap().parse().unwrap(), 150, 193);
        } else {
            return is_between(hgt.split("in").nth(0).unwrap().parse().unwrap(), 59, 76);
        }
    }
    fn is_hcl_valid(&self) -> bool {
        let color_regex = Regex::new(r"#[a-fA-F0-9]{6}").unwrap();
        return color_regex.is_match(self.hcl.as_ref().unwrap());
    }
    fn is_ecl_valid(&self) -> bool {
        return ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .contains(&self.ecl.as_ref().unwrap().as_str());
    }
    fn is_pid_valid(&self) -> bool {
        return self.pid.as_ref().unwrap().len() == 9;
    }

    pub fn is_valid(&self) -> bool {
        return self.is_byr_valid()
            && self.is_iyr_valid()
            && self.is_eyr_valid()
            && self.is_hgt_valid()
            && self.is_hcl_valid()
            && self.is_ecl_valid()
            && self.is_pid_valid();
    }
}
