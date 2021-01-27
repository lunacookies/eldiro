#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let parse = parser::parse(s);
        let syntax = parse.syntax();
        let _validation_errors = ast::validation::validate(&syntax);
        let root = ast::Root::cast(syntax).unwrap();
        let (_database, _stmts) = hir::lower(root);
    }
});
