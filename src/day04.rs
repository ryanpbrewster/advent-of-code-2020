use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

type Passport = HashMap<String, String>;

lazy_static! {
    static ref PASSPORT: Regex = Regex::new(r"([[:alpha:]]+):([^[[:space:]]]+)").unwrap();
    static ref YEAR: Regex = Regex::new(r"^[[:digit:]]{4}$").unwrap();
    static ref BYR: Regex = Regex::new(r"^19[2-9][0-9]|200[0-2]$").unwrap();
    static ref IYR: Regex = Regex::new(r"^201[0-9]|2020$").unwrap();
    static ref EYR: Regex = Regex::new(r"^202[0-9]|2030$").unwrap();
    static ref HGT: Regex = Regex::new(r"^1([5-8][0-9]|9[0-3])cm|(59|6[0-9]|7[0-6])in$").unwrap();
    static ref HCL: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref ECL: Regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
    static ref PID: Regex = Regex::new(r"^[[:digit:]]{9}$").unwrap();
}
fn parse_passports(input: &str) -> Vec<Passport> {
    let mut passports = vec![];
    let mut cur = HashMap::new();
    for line in input.lines().map(|s| s.trim()) {
        if line.is_empty() {
            if !cur.is_empty() {
                passports.push(cur);
                cur = HashMap::new();
            }
        } else {
            for m in PASSPORT.captures_iter(line) {
                let k = m.get(1).unwrap().as_str().to_owned();
                let v = m.get(2).unwrap().as_str().to_owned();
                cur.insert(k, v);
            }
        }
    }
    if !cur.is_empty() {
        passports.push(cur);
    }
    passports
}

lazy_static! {
    static ref REQUIRED: HashSet<String> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .into_iter()
        .map(String::from)
        .collect();
}
fn has_required_fields(passport: &Passport) -> bool {
    let ks: HashSet<String> = passport.keys().map(String::from).collect();
    ks.is_superset(&REQUIRED)
}

fn validate(passport: &Passport) -> Result<(), &str> {
    let check = |key: &'static str, pattern: &Regex| {
        passport.get(key).filter(|v| pattern.is_match(v)).ok_or(key)
    };
    check("byr", &BYR)?;
    check("iyr", &IYR)?;
    check("eyr", &EYR)?;
    check("hgt", &HGT)?;
    check("hcl", &HCL)?;
    check("ecl", &ECL)?;
    check("pid", &PID)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::{has_required_fields, parse_passports, validate};

    const SAMPLE_INPUT: &str = r"
            ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
            byr:1937 iyr:2017 cid:147 hgt:183cm
            
            iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
            hcl:#cfa07d byr:1929
            
            hcl:#ae17e1 iyr:2013
            eyr:2024
            ecl:brn pid:760753108 byr:1931
            hgt:179cm
            
            hcl:#cfa07d eyr:2025 pid:166559648
            iyr:2011 ecl:brn hgt:59in
        ";

    #[test]
    fn small1() {
        let parsed = parse_passports(SAMPLE_INPUT);
        let valid_count = parsed.into_iter().filter(has_required_fields).count();
        assert_eq!(valid_count, 2);
    }

    #[test]
    fn normal1() {
        let raw = std::fs::read_to_string("data/day04.input").unwrap();
        let parsed = parse_passports(&raw);
        let valid_count = parsed.into_iter().filter(has_required_fields).count();
        assert_eq!(valid_count, 222);
    }

    #[test]
    fn small2() {
        let valid = r"
            pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f

            eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

            hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022

            iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
        ";
        for p in parse_passports(valid) {
            assert!(validate(&p).is_ok(), "should be valid: {:?}", p);
        }

        let invalid = r"
            eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

            iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946

            hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

            hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007
        ";
        for p in parse_passports(invalid) {
            assert!(validate(&p).is_err(), "should be invalid: {:?}", p);
        }
    }

    #[test]
    fn normal2() {
        let raw = std::fs::read_to_string("data/day04.input").unwrap();
        let parsed = parse_passports(&raw);
        let valid_count = parsed.into_iter().filter(|p| validate(p).is_ok()).count();
        assert_eq!(valid_count, 140);
    }
}
