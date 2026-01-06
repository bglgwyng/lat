use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UptoUnit {
    Tokens(usize),
    Characters(usize),
    Lines(usize),
}

impl fmt::Display for UptoUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UptoUnit::Tokens(n) => write!(f, "{}t", n),
            UptoUnit::Characters(n) => write!(f, "{}c", n),
            UptoUnit::Lines(n) => write!(f, "{}l", n),
        }
    }
}

pub fn parse_upto(s: &str) -> Result<UptoUnit, String> {
    let s = s.trim();
    if s.is_empty() {
        return Err("empty upto value".to_string());
    }

    let (num_part, unit) = s.split_at(s.len() - 1);
    let num: usize = num_part
        .parse()
        .map_err(|_| format!("invalid number: {}", num_part))?;

    match unit {
        "t" => Ok(UptoUnit::Tokens(num)),
        "c" => Ok(UptoUnit::Characters(num)),
        "l" => Ok(UptoUnit::Lines(num)),
        _ => Err(format!("unknown unit '{}', expected t/c/l", unit)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tokens() {
        let result = parse_upto("100t").unwrap();
        assert!(matches!(result, UptoUnit::Tokens(100)));
    }

    #[test]
    fn parse_characters() {
        let result = parse_upto("500c").unwrap();
        assert!(matches!(result, UptoUnit::Characters(500)));
    }

    #[test]
    fn parse_lines() {
        let result = parse_upto("50l").unwrap();
        assert!(matches!(result, UptoUnit::Lines(50)));
    }

    #[test]
    fn parse_with_whitespace() {
        let result = parse_upto("  200l  ").unwrap();
        assert!(matches!(result, UptoUnit::Lines(200)));
    }

    #[test]
    fn parse_invalid_unit() {
        let result = parse_upto("100x");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("unknown unit"));
    }

    #[test]
    fn parse_invalid_number() {
        let result = parse_upto("abcl");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("invalid number"));
    }

    #[test]
    fn parse_empty() {
        let result = parse_upto("");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty"));
    }
}
