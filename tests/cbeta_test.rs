extern crate arete;

use arete::plugins::cbeta;

#[test]
fn it_bookcase() {
    println!("load index");
    let md = cbeta::models::metadata::Metadata::new().unwrap();
    println!("{:?}", md);

    println!("load catalog");
    let items = md.catalog().unwrap();
    // for it in &items {
    //     println!("{:?}", it);
    // }
    println!("total {} items", items.len());

    println!("load bookdata");
    let items = md.bookdata().unwrap();
    // for it in &items {
    //     println!("{:?}", it);
    // }
    println!("total {} items", items.len());

    println!("load spine");
    let items = md.spine().unwrap();
    // for it in &items {
    //     println!("{:?}", it);
    // }
    println!("total {} items", items.len());
}
