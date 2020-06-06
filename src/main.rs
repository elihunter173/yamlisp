// TODO: Should I switch away extern crate and instead make this strictly an app?

extern crate yamlisp;

use yamlisp::LispIR;

fn main() {
    let src = r#"
        (test "foo" 2 -2 2.0)
    "#;
    let val = lexpr::from_str(src).unwrap();
    println!("{:?}", &val);
    let ir = LispIR(&val);
    println!("{}", serde_lexpr::to_string(&ir).unwrap());
}
