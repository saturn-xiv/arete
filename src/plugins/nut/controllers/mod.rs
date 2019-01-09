pub mod api;
pub mod html;
pub mod seo;

use rocket::Route;

lazy_static! {
    pub static ref ROUTES: Vec<(&'static str, Vec<Route>)> = {
        let mut items = Vec::new();
        items.push((
            "/api/users",
            routes![
                api::users::sign_in,
                api::users::sign_up,
                api::users::confirm,
                api::users::confirm_token,
                api::users::forgot_password,
                api::users::unlock,
                api::users::unlock_token,
                api::users::reset_password,
                api::users::logs,
                api::users::get_profile,
                api::users::post_profile,
                api::users::change_password,
                api::users::sign_out
            ],
        ));
        items.push((
            "/api/locales",
            routes![
                api::locales::index,
                api::locales::save,
                api::locales::destory,
            ],
        ));
        items.push((
            "/api/leave-words",
            routes![
                api::leave_words::index,
                api::leave_words::create,
                api::leave_words::destory,
            ],
        ));
        items.push((
            "/",
            routes![
                seo::robots::txt,
                seo::sitemap::xml_gz,
                seo::rss::atom,
                html::home,
                html::ueditor,
            ],
        ));
        items
    };
}
