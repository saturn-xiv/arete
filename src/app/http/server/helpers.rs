handlebars_helper!(lower: |s: str| s.to_lowercase());
handlebars_helper!(upper: |s: str| s.to_uppercase());
handlebars_helper!(hex: |v: i64| format!("0x{:x}", v));
handlebars_helper!(money: |v: i64, {cur: str="$"}| format!("{}{}.00", cur, v));
