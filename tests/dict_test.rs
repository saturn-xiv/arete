extern crate arete;

const KEYWORD: &'static str = "戒";

#[test]
fn it_stardict() {
    let dict = arete::dict::stardict::StarDict::new(
        &arete::plugins::cbeta::api::dictionaries::ROOT.as_path(),
    )
    .unwrap();

    for it in dict.list().unwrap() {
        println!("{:?}", it);
    }

    for it in dict.search(KEYWORD).unwrap() {
        println!("{:?}", it);
    }
}
