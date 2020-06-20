extern crate arete;

use arete::plugins::cbeta;

#[test]
fn it_bookcase() {
    println!("load index");
    let md = cbeta::models::metadata::Metadata::new().unwrap();
    println!("{:?}", md);

    println!("load {}", md.catalog.value);
    let items = md.catalog().unwrap();
    // for it in &items {
    //     println!("{:?}", it);
    // }
    println!("total {} items", items.len());

    println!("load {}", md.bookdata.value);
    let items = md.bookdata().unwrap();
    // for it in &items {
    //     println!("{:?}", it);
    // }
    println!("total {} items", items.len());

    println!("load {}", md.spine.value);
    let items = md.spine().unwrap();
    // for it in &items {
    //     println!("{:?}", it);
    // }
    println!("total {} items", items.len());

    println!("load {}", md.nav.value);
    let menu = cbeta::models::nav::menu::Html::new(&md.nav.value).unwrap();
    println!("{:?}", menu);

    println!("load simple");
    let simple = cbeta::models::nav::simple::Html::new("simple_nav.xhtml").unwrap();
    println!("{:?}", simple);

    // for it in menu.body.nav.items {
    //     match it {
    //         cbeta::models::nav::Item::Link { href, title } => {
    //             parse_nav(href, title);
    //         }
    //         cbeta::models::nav::Item::Li { item } => {
    //             parse_nav(item.href, item.title);
    //         }
    //         _ => {}
    //     }
    // }
}

// fn parse_nav(href: String, title: String) {
//     println!("par4se file {} {}", href, title);
//     let it = cbeta::models::nav::Html::new(&href).unwrap();
//     println!("{}", it.head.title.value);
// }
