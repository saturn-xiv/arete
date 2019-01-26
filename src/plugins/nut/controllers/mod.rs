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
                api::install,
                api::locales::index,
                api::leave_words::create,
                api::ueditor::post,
                api::admin::locales::index,
                api::admin::locales::create,
                api::admin::locales::show,
                api::admin::locales::update,
                api::admin::locales::destory,
                api::admin::leave_words::index,
                api::admin::leave_words::destory,
                api::admin::site::clear_cache,
                api::admin::site::send_test_email,
                api::admin::site::status::get,
                api::admin::site::info::get,
                api::admin::site::info::post,
                api::admin::site::author::get,
                api::admin::site::author::post,
                api::admin::site::seo::get,
                api::admin::site::seo::post,
                api::admin::site::smtp::get,
                api::admin::site::smtp::post,
                api::admin::tags::index,
                api::admin::tags::create,
                api::admin::tags::show,
                api::admin::tags::update,
                api::admin::tags::destory,
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
