use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    match parse_input(&args) {
        Ok(n) => {
            let roman = int_to_roman(n);
            println!("{roman}");
        }
        Err(err) => {
            eprintln!("error: {err}");
            eprintln!("usage: {} <integer 1..=3999>", args.get(0).map(String::as_str).unwrap_or("roman-converter"));
            process::exit(2);
        }
    }
}

fn parse_input(args: &[String]) -> Result<u16, String> {
    if args.len() != 2 {
        return Err("expected exactly 1 argument".to_string());
    }

    let raw = args[1].trim();
    if raw == "--help" || raw == "-h" {
        return Err("help requested".to_string());
    }

    let n: u16 = raw
        .parse()
        .map_err(|_| format!("invalid integer: '{raw}'"))?;

    if !(1..=3999).contains(&n) {
        return Err(format!("integer out of range (1..=3999): {n}"));
    }

    Ok(n)
}

fn int_to_roman(mut n: u16) -> String {
    // Standard roman numeral table for 1..=3999
    const TABLE: &[(u16, &str)] = &[
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];

    let mut out = String::new();
    for &(value, symbol) in TABLE {
        while n >= value {
            out.push_str(symbol);
            n -= value;
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_basic_values() {
        assert_eq!(int_to_roman(1), "I");
        assert_eq!(int_to_roman(4), "IV");
        assert_eq!(int_to_roman(9), "IX");
        assert_eq!(int_to_roman(58), "LVIII");
        assert_eq!(int_to_roman(1994), "MCMXCIV");
        assert_eq!(int_to_roman(3999), "MMMCMXCIX");
    }

    #[test]
    fn rejects_out_of_range() {
        let args = vec!["roman-converter".to_string(), "0".to_string()];
        assert!(parse_input(&args).unwrap_err().contains("out of range"));

        let args = vec!["roman-converter".to_string(), "4000".to_string()];
        assert!(parse_input(&args).unwrap_err().contains("out of range"));
    }

    #[test]
    fn rejects_non_integer() {
        let args = vec!["roman-converter".to_string(), "12.3".to_string()];
        assert!(parse_input(&args).unwrap_err().contains("invalid integer"));
    }
}
