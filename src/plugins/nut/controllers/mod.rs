pub mod api;
pub mod html;
pub mod seo;

use rocket::Route;

lazy_static! {
    pub static ref ROUTES: Vec<(&'static str, Vec<Route>)> = {
        let mut items = Vec::new();
        items.push((
            "/api",
            routes![
                api::users::sign_in,
                api::users::sign_up,
                api::users::confirm,
                api::users::forgot_password,
                api::users::unlock,
                api::users::reset_password,
                api::users::logs,
                api::users::get_profile,
                api::users::post_profile,
                api::users::change_password,
                api::users::sign_out,
                api::about,
                api::locales::index,
                api::leave_words::create,
                api::ueditor::post,
                api::admin::locales::update,
                api::admin::locales::destory,
                api::admin::leave_words::index,
                api::admin::leave_words::destory,
            ],
        ));

        items.push((
            "/",
            routes![
                seo::robots::txt,
                seo::sitemap::xml_gz,
                seo::rss::atom,
                html::users::confirm_token,
                html::users::unlock_token,
                html::home::index,
            ],
        ));
        items
    };
}
