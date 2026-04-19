use std::collections::HashMap;
use std::io::{self, Write};
use crate::ast::*;

pub fn run(program: &Program) {
    run_with_output(program, &mut io::stdout());
}

pub fn run_with_output(program: &Program, output: &mut dyn Write) {
    let mut vars: HashMap<String, i64> = HashMap::new();

    for line in &program.lines {
        match &line.statement {
            Statement::Let { var, value } => {
                let v = eval_int(value, &vars);
                vars.insert(var.clone(), v);
            }
            Statement::Print { values } => {
                let parts: Vec<String> = values.iter()
                    .map(|e| format_value(e, &vars))
                    .collect();
                writeln!(output, "{}", parts.join(" ")).unwrap();
            }
        }
    }
}

fn eval_int(expr: &Expr, vars: &HashMap<String, i64>) -> i64 {
    match expr {
        Expr::Integer(n) => *n,
        Expr::Variable(name) => *vars.get(name).unwrap_or(&0),
        Expr::StringLit(_) => panic!("Expected integer, got string"),
    }
}

fn format_value(expr: &Expr, vars: &HashMap<String, i64>) -> String {
    match expr {
        Expr::Integer(n) => n.to_string(),
        Expr::StringLit(s) => s.clone(),
        Expr::Variable(name) => vars.get(name).unwrap_or(&0).to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn run_program(source: &str) -> String {
        let program = parse(source).expect("parse error");
        let mut output = Vec::new();
        run_with_output(&program, &mut output);
        String::from_utf8(output).unwrap()
    }

    #[test]
    fn test_print_string() {
        assert_eq!(run_program(r#"PRINT "Bonjour""#), "Bonjour\n");
    }

    #[test]
    fn test_print_integer() {
        assert_eq!(run_program("PRINT 42"), "42\n");
    }

    #[test]
    fn test_print_variable() {
        assert_eq!(run_program("X = 10\nPRINT X"), "10\n");
    }

    #[test]
    fn test_print_multiple_params() {
        assert_eq!(run_program(r#"PRINT "val", 99"#), "val 99\n");
    }

    #[test]
    fn test_print_empty() {
        assert_eq!(run_program("PRINT"), "\n");
    }

    #[test]
    fn test_assignment_let() {
        assert_eq!(run_program("LET X = 7\nPRINT X"), "7\n");
    }

    #[test]
    fn test_assignment_without_let() {
        assert_eq!(run_program("X = 7\nPRINT X"), "7\n");
    }

    #[test]
    fn test_assignment_overwrite() {
        assert_eq!(run_program("X = 1\nX = 2\nPRINT X"), "2\n");
    }

    #[test]
    fn test_variable_default_zero() {
        assert_eq!(run_program("PRINT X"), "0\n");
    }

    #[test]
    fn test_full_program() {
        let src = "10 LET X = 42\n20 Y = 8\n30 PRINT \"X =\", X\n40 PRINT \"Y =\", Y";
        assert_eq!(run_program(src), "X = 42\nY = 8\n");
    }

    #[test]
    fn test_mixed_numbered_unnumbered_lines() {
        let src = "10 X = 5\nPRINT X";
        assert_eq!(run_program(src), "5\n");
    }
}
