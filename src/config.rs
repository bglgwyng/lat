use kdl::{KdlDocument, KdlNode};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Rule {
    pub patterns: Vec<String>,
    pub command: String,
    pub args: Vec<String>,
    default_upto: Option<usize>,
}

impl Rule {
    pub fn upto(&self, cli_upto: Option<usize>) -> Option<usize> {
        cli_upto.or(self.default_upto)
    }
}

#[derive(Debug)]
pub struct Config {
    rules: Vec<Rule>,
}

impl Config {
    fn from_kdl(doc: &KdlDocument) -> Result<Self, String> {
        let mut rules = Vec::new();

        for node in doc.nodes() {
            if node.name().value() == "rule" {
                rules.push(Self::parse_rule(node)?);
            }
        }

        Ok(Config { rules })
    }

    fn parse_rule(node: &KdlNode) -> Result<Rule, String> {
        // patterns은 rule 노드의 arguments
        let patterns: Vec<String> = node
            .entries()
            .iter()
            .filter(|e| e.name().is_none())
            .filter_map(|e| e.value().as_string().map(|s| s.to_string()))
            .collect();

        if patterns.is_empty() {
            return Err("rule must have at least one pattern".to_string());
        }

        let children = node.children().ok_or("rule must have a body")?;

        // command
        let command = children
            .get("command")
            .and_then(|n| n.entries().first())
            .and_then(|e| e.value().as_string())
            .ok_or("rule must have a command")?
            .to_string();

        // args
        let args: Vec<String> = children
            .get("args")
            .map(|n| {
                n.entries()
                    .iter()
                    .filter(|e| e.name().is_none())
                    .filter_map(|e| e.value().as_string().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        // defaults
        let default_upto = children
            .get("defaults")
            .and_then(|n| n.get("upto"))
            .and_then(|v| v.as_integer())
            .map(|n| n as usize);

        Ok(Rule {
            patterns,
            command,
            args,
            default_upto,
        })
    }

    pub fn find_rule(&self, filename: &str) -> Option<&Rule> {
        for rule in &self.rules {
            for pattern in &rule.patterns {
                if glob_match(pattern, filename) {
                    return Some(rule);
                }
            }
        }
        None
    }
}

fn glob_match(pattern: &str, filename: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    if let Some(ext) = pattern.strip_prefix("*.") {
        return filename.ends_with(&format!(".{}", ext));
    }
    pattern == filename
}

pub fn load_config(start_path: &Path) -> Result<Config, String> {
    // 현재 디렉토리부터 상위로 .lat.kdl 찾기
    let mut dir = if start_path.is_file() {
        start_path.parent().unwrap_or(Path::new("."))
    } else {
        start_path
    };

    loop {
        let config_path = dir.join(".lat.kdl");
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .map_err(|e| format!("failed to read config: {}", e))?;
            let doc: KdlDocument = content
                .parse()
                .map_err(|e| format!("failed to parse config: {}", e))?;
            return Config::from_kdl(&doc);
        }

        match dir.parent() {
            Some(parent) => dir = parent,
            None => break,
        }
    }

    Err("no .lat.kdl config found".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse_tests {
        use super::*;

        #[test]
        fn parse_simple_viewer() {
            let kdl = r#"
                rule "*.json" {
                    command "json-lat"
                    args "$FILE"
                    defaults upto=500
                }
            "#;
            let doc: KdlDocument = kdl.parse().unwrap();
            let config = Config::from_kdl(&doc).unwrap();

            assert_eq!(config.rules.len(), 1);
            assert_eq!(config.rules[0].patterns, vec!["*.json"]);
            assert_eq!(config.rules[0].command, "json-lat");
            assert_eq!(config.rules[0].args, vec!["$FILE"]);
            assert_eq!(config.rules[0].upto(None), Some(500));
        }

        #[test]
        fn parse_multiple_patterns() {
            let kdl = r#"
                rule "*.js" "*.ts" "*.jsx" "*.tsx" {
                    command "js-lat"
                    args "$FILE" "--upto" "$UPTO"
                    defaults upto=1000
                }
            "#;
            let doc: KdlDocument = kdl.parse().unwrap();
            let config = Config::from_kdl(&doc).unwrap();

            assert_eq!(
                config.rules[0].patterns,
                vec!["*.js", "*.ts", "*.jsx", "*.tsx"]
            );
            assert_eq!(config.rules[0].args, vec!["$FILE", "--upto", "$UPTO"]);
        }

        #[test]
        fn parse_without_default_upto() {
            let kdl = r#"
                rule "*.md" {
                    command "cat"
                    args "$FILE"
                }
            "#;
            let doc: KdlDocument = kdl.parse().unwrap();
            let config = Config::from_kdl(&doc).unwrap();

            assert_eq!(config.rules[0].upto(None), None);
        }

        #[test]
        fn parse_multiple_rules() {
            let kdl = r#"
                rule "*.json" {
                    command "json-lat"
                    args "$FILE"
                }
                rule "*.md" {
                    command "cat"
                    args "$FILE"
                }
            "#;
            let doc: KdlDocument = kdl.parse().unwrap();
            let config = Config::from_kdl(&doc).unwrap();

            assert_eq!(config.rules.len(), 2);
        }

        #[test]
        fn find_rule_exact_extension() {
            let kdl = r#"
                rule "*.json" {
                    command "json-lat"
                    args "$FILE"
                }
                rule "*.md" {
                    command "cat"
                    args "$FILE"
                }
            "#;
            let doc: KdlDocument = kdl.parse().unwrap();
            let config = Config::from_kdl(&doc).unwrap();

            let rule = config.find_rule("test.json").unwrap();
            assert_eq!(rule.command, "json-lat");

            let rule = config.find_rule("README.md").unwrap();
            assert_eq!(rule.command, "cat");
        }

        #[test]
        fn find_rule_fallback() {
            let kdl = r#"
                rule "*.json" {
                    command "json-lat"
                    args "$FILE"
                }
                rule "*" {
                    command "cat"
                    args "$FILE"
                }
            "#;
            let doc: KdlDocument = kdl.parse().unwrap();
            let config = Config::from_kdl(&doc).unwrap();

            let rule = config.find_rule("unknown.xyz").unwrap();
            assert_eq!(rule.command, "cat");
        }

        #[test]
        fn find_rule_priority() {
            let kdl = r#"
                rule "*.json" {
                    command "json-lat"
                    args "$FILE"
                }
                rule "*" {
                    command "cat"
                    args "$FILE"
                }
            "#;
            let doc: KdlDocument = kdl.parse().unwrap();
            let config = Config::from_kdl(&doc).unwrap();

            let rule = config.find_rule("data.json").unwrap();
            assert_eq!(rule.command, "json-lat");
        }
    }

    mod glob_match_tests {
        use super::*;

        #[test]
        fn match_wildcard() {
            assert!(glob_match("*", "anything.txt"));
            assert!(glob_match("*", "file"));
        }

        #[test]
        fn match_extension() {
            assert!(glob_match("*.json", "data.json"));
            assert!(glob_match("*.json", "path/to/data.json"));
            assert!(!glob_match("*.json", "data.txt"));
        }

        #[test]
        fn match_exact() {
            assert!(glob_match("Makefile", "Makefile"));
            assert!(!glob_match("Makefile", "makefile"));
        }
    }
}
