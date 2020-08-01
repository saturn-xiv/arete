#[macro_export]
macro_rules! __i18n_e {
    ($d:expr, $l:expr, $c:expr) => {{
        I18n::e($d, $l, $c, &None::<String>).into()
    }};
    ($d:expr, $l:expr, $c:expr, $v:expr) => {{
        I18n::e($d, $l, $c, &Some($v)).into()
    }};
}

#[macro_export]
macro_rules! __i18n_l {
    ($d:expr, $u:expr, $i:expr, $l:expr, $c:expr) => {{
        let m = I18n::t($d, $l, $c, &None::<String>);
        LogDao::add($d, $u, $i, m)
    }};
    ($d:expr, $u:expr, $i:expr, $l:expr, $c:expr, $v:expr) => {{
        let m = I18n::t($d, $l, $c, &Some($v));
        LogDao::add($d, $u, $i, m)
    }};
}

// #[macro_export]
// macro_rules! __html {
//     ($t:expr, $bo:expr, $bu:expr, $se:expr, $ma:expr) => {{
//         let body = match $t {
//             Theme::Bootstrap => $bo.render()?,
//             Theme::Bulma => $bu.render()?,
//             Theme::SemanticUi => $se.render()?,
//             Theme::Materialize => $ma.render()?,
//         };
//         HttpResponse::Ok()
//             .content_type(TEXT_HTML_UTF_8.to_string())
//             .body(body)
//     }};
// }
