pub mod binding_def;
pub mod expr;
pub mod stmt;
pub mod val;

mod env;
mod utils;

pub struct Parse(stmt::Stmt);

pub fn parse(s: &str) -> Result<Parse, String> {
    let (s, stmt) = stmt::Stmt::new(s)?;

    if s.is_empty() {
        Ok(Parse(stmt))
    } else {
        Err("input was not consumed fully by parser".to_string())
    }
}
