extern crate mustache;
#[macro_use]
extern crate serde_json;

#[test]
fn it_template() {
    let msg = mustache::compile_str("Hello, {{name}}!")
        .unwrap()
        .render_to_string(&json!({"name":"arete"}))
        .unwrap();
    println!("{}", msg);
    assert_eq!("Hello, arete!", msg);
}
