use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    let mut input = String::new();
    let mut env = eldiro::Env::default();

    loop {
        write!(stdout, "→ ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        match eldiro::parse(input.trim()) {
            Ok(parse) => match parse.eval(&mut env) {
                Ok(val) => {
                    dbg!(val);
                }
                Err(msg) => {
                    writeln!(stderr, "Evaluation error: {}", msg)?;
                    stderr.flush()?;
                }
            },
            Err(msg) => {
                writeln!(stderr, "Parse error: {}", msg)?;
                stderr.flush()?;
            }
        }

        input.clear();
    }
}
