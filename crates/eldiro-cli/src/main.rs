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

        match run(input.trim(), &mut env) {
            Ok(()) => {}
            Err(msg) => {
                writeln!(stderr, "{}", msg)?;
                stderr.flush()?;
            }
        }

        input.clear();
    }
}

fn run(input: &str, env: &mut eldiro::Env) -> Result<(), String> {
    let parse = eldiro::parse(input).map_err(|msg| format!("Parse error: {}", msg))?;

    let evaluated = parse
        .eval(env)
        .map_err(|msg| format!("Evaluation error: {}", msg))?;

    if evaluated != eldiro::Val::Unit {
        dbg!(evaluated);
    }

    Ok(())
}
