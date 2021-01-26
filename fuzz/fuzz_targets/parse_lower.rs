#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let parse = parser::parse(s);
        let root = ast::Root::cast(parse.syntax()).unwrap();
        let (_database, _stmts) = hir::lower(root);
    }
});
