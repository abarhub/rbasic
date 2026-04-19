mod ast;
mod parser;
mod interpreter;

fn main() {
    let source = r#"
10 LET X = 42
20 LET Y = 10
30 PRINT "Hello, BASIC!"
40 PRINT X
50 PRINT Y
"#
    .trim();

    match parser::parse(source) {
        Ok(program) => {
            println!("--- AST ---");
            for line in &program.lines {
                println!("{:?}", line);
            }
            println!("--- Execution ---");
            interpreter::run(&program);
        }
        Err(errors) => {
            for e in errors {
                eprintln!("Parse error: {:?}", e);
            }
        }
    }
}
