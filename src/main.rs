mod config;

use clap::Parser;
use config::{Rule, load_config};
use std::path::Path;
use std::process::Command;

#[derive(Parser)]
#[command(name = "lat")]
#[command(about = "cat for LLMs")]
struct Args {
    /// 파일 경로 (예: big.json 또는 big.json:data.a_big_array)
    file: String,

    /// 최대 읽을 글자수
    #[arg(short, long)]
    upto: Option<usize>,

    /// 확대할 경로들 (예: data.items,data.users)
    #[arg(short, long, value_delimiter = ',')]
    focus: Option<Vec<String>>,
}

fn substitute_args(
    args: &[String],
    file: &str,
    upto: Option<usize>,
    focus: Option<&[String]>,
) -> Vec<String> {
    let focus_str = focus.map(|f| f.join(",")).unwrap_or_default();
    let upto_str = upto.map(|u| u.to_string());

    args.iter()
        .flat_map(|arg| match arg.as_str() {
            "$FILE" => vec![file.to_string()],
            "$UPTO" => upto_str.clone().map(|s| vec![s]).unwrap_or_default(),
            "$FOCUS" => {
                if focus_str.is_empty() {
                    vec![]
                } else {
                    vec![focus_str.clone()]
                }
            }
            _ => vec![arg.clone()],
        })
        .collect()
}

fn run_rule(
    rule: &Rule,
    file: &str,
    upto: Option<usize>,
    focus: Option<&[String]>,
) -> Result<(), String> {
    let args = substitute_args(&rule.args, file, upto, focus);

    let status = Command::new(&rule.command)
        .args(&args)
        .status()
        .map_err(|e| format!("failed to execute {}: {}", rule.command, e))?;

    if !status.success() {
        return Err(format!(
            "{} exited with status: {}",
            rule.command,
            status.code().unwrap_or(-1)
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod substitute_args_tests {
        use super::*;

        #[test]
        fn substitute_file() {
            let args = vec!["$FILE".to_string()];
            let result = substitute_args(&args, "test.json", None, None);
            assert_eq!(result, vec!["test.json"]);
        }

        #[test]
        fn substitute_upto() {
            let args = vec![
                "$FILE".to_string(),
                "--upto".to_string(),
                "$UPTO".to_string(),
            ];
            let result = substitute_args(&args, "test.json", Some(100), None);
            assert_eq!(result, vec!["test.json", "--upto", "100"]);
        }

        #[test]
        fn substitute_upto_none() {
            let args = vec!["$FILE".to_string(), "$UPTO".to_string()];
            let result = substitute_args(&args, "test.json", None, None);
            assert_eq!(result, vec!["test.json"]);
        }

        #[test]
        fn substitute_focus() {
            let args = vec!["$FILE".to_string(), "$FOCUS".to_string()];
            let focus = vec!["data.items".to_string(), "data.users".to_string()];
            let result = substitute_args(&args, "test.json", None, Some(&focus));
            assert_eq!(result, vec!["test.json", "data.items,data.users"]);
        }

        #[test]
        fn substitute_focus_empty() {
            let args = vec!["$FILE".to_string(), "$FOCUS".to_string()];
            let result = substitute_args(&args, "test.json", None, None);
            assert_eq!(result, vec!["test.json"]);
        }

        #[test]
        fn substitute_mixed() {
            let args = vec![
                "--file".to_string(),
                "$FILE".to_string(),
                "--limit".to_string(),
                "$UPTO".to_string(),
            ];
            let result = substitute_args(&args, "data.json", Some(500), None);
            assert_eq!(result, vec!["--file", "data.json", "--limit", "500"]);
        }
    }
}

fn main() {
    let args = Args::parse();
    let file_path = Path::new(&args.file);

    // config 로드
    let config = match load_config(file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    };

    // 파일명으로 rule 찾기
    let filename = file_path.file_name().and_then(|n| n.to_str()).unwrap_or("");

    let rule = match config.find_rule(filename) {
        Some(r) => r,
        None => {
            eprintln!("no rule found for: {}", filename);
            std::process::exit(1);
        }
    };

    // upto: CLI 값 우선, 없으면 rule의 default
    let upto = rule.upto(args.upto);

    let focus = args.focus.as_deref();

    if let Err(e) = run_rule(rule, &args.file, upto, focus) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
