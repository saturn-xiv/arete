use juniper::FieldResult;

use super::super::plugins::nut;
use super::{context::Context, Pager, ID};

pub struct Query;

#[juniper::object(
    Context = Context,
    description = "Readonly operations!",
)]
impl Query {
    #[graphql(description = "System information.")]
    fn about(context: &Context) -> FieldResult<nut::graphql::About> {
        let it = nut::graphql::About::new(context)?;
        Ok(it)
    }

    #[graphql(description = "Current user's logs.")]
    fn usersLogs(context: &Context, pager: Pager) -> FieldResult<nut::graphql::users::Logs> {
        Ok(nut::graphql::users::Logs::new(context, &pager)?)
    }
    #[graphql(description = "Current user's information.")]
    fn currentUser(context: &Context) -> FieldResult<nut::graphql::users::CurrentUser> {
        let it = nut::graphql::users::CurrentUser::new(context)?;
        Ok(it)
    }
    #[graphql(description = "List all user")]
    fn indexUser(context: &Context) -> FieldResult<Vec<nut::graphql::users::User>> {
        let items = nut::graphql::users::User::index(context)?;
        Ok(items)
    }
    #[graphql(description = "List all user's policies")]
    fn indexUserPolicies(
        context: &Context,
        id: ID,
    ) -> FieldResult<Vec<nut::graphql::users::Policy>> {
        let items = nut::graphql::users::Policy::index(context, id)?;
        Ok(items)
    }

    #[graphql(description = "Get site author")]
    fn getSiteAuthor(context: &Context) -> FieldResult<nut::graphql::site::Author> {
        let it = nut::graphql::site::Author::get(context)?;
        Ok(it)
    }
    #[graphql(description = "Get site seo configuration")]
    fn getSiteSeo(context: &Context) -> FieldResult<nut::graphql::site::Seo> {
        let it = nut::graphql::site::Seo::get(context)?;
        Ok(it)
    }
    #[graphql(description = "Get site SMTP configuration")]
    fn getSiteSmtp(context: &Context) -> FieldResult<nut::tasks::send_email::Config> {
        let it = nut::tasks::send_email::Config::get(context)?;
        Ok(it)
    }
    #[graphql(description = "Get site current status")]
    fn getSiteStatus(context: &Context) -> FieldResult<nut::graphql::site::status::Status> {
        let it = nut::graphql::site::status::Status::new(context)?;
        Ok(it)
    }

    #[graphql(description = "All locale items")]
    fn indexLocale(context: &Context) -> FieldResult<Vec<nut::graphql::locales::Locale>> {
        let items = nut::graphql::locales::Locale::index(context)?;
        Ok(items)
    }

    #[graphql(description = "List attachments")]
    fn indexAttachment(
        context: &Context,
    ) -> FieldResult<Vec<nut::graphql::attahments::Attachment>> {
        let items = nut::graphql::attahments::Attachment::index(context)?;
        Ok(items)
    }

    #[graphql(description = "List cards")]
    fn indexCard(context: &Context) -> FieldResult<Vec<nut::graphql::cards::Card>> {
        let items = nut::graphql::cards::Card::index(context)?;
        Ok(items)
    }
    #[graphql(description = "Show card")]
    fn showCard(context: &Context, id: ID) -> FieldResult<nut::graphql::cards::Card> {
        let it = nut::graphql::cards::Card::show(context, id)?;
        Ok(it)
    }

    #[graphql(description = "List categories")]
    fn indexCategory(context: &Context) -> FieldResult<Vec<nut::graphql::categories::Category>> {
        let items = nut::graphql::categories::Category::index(context)?;
        Ok(items)
    }
    #[graphql(description = "Show catagory")]
    fn showCategory(context: &Context, id: ID) -> FieldResult<nut::graphql::categories::Category> {
        let it = nut::graphql::categories::Category::show(context, id)?;
        Ok(it)
    }
}
