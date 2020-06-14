// TODO: Should I switch away extern crate and instead make this strictly an app?

extern crate yamlisp;

use yamlisp::LispIR;

fn main() {
    let src = r#"
    ((name . "John Doe")
     (age . 43)
     (phones "+44 1234567" "+44 2345678"))
    "#;
    println!("{}", src);
    let val = lexpr::from_str(&src).unwrap();
    println!("{}", serde_yaml::to_string(&LispIR(&val)).unwrap());
}
