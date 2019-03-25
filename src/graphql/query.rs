use juniper::FieldResult;
use validator::Validate;

use super::super::{
    env::VERSION,
    i18n,
    plugins::{forum, nut},
};
use super::{Context, Handler, I64};

pub struct Query;

graphql_object!(Query: Context |&self| {

    field apiVersion() -> &str {
        VERSION
    }

    field author(&executor) -> FieldResult<nut::graphql::site::Author> {
        __graphql!(executor, &nut::graphql::site::GetAuthor{})
    }
    field smtp(&executor) -> FieldResult<nut::tasks::send_email::Config> {
        __graphql!(executor, &nut::tasks::send_email::Get{})
    }
    field seo(&executor) -> FieldResult<nut::graphql::site::Seo> {
        __graphql!(executor, &nut::graphql::site::GetSeo{})
    }
    field status(&executor) -> FieldResult<nut::graphql::site::status::Status> {
        __graphql!(executor, &nut::graphql::site::status::Get{})
    }

    field listLocaleByLang(&executor, lang: String) -> FieldResult<Vec<i18n::graphql::Locale>> {
        __graphql!(executor, &i18n::graphql::ByLang{lang: lang.clone()})
    }
    field showLocale(&executor, id: I64) -> FieldResult<i18n::graphql::Locale> {
        __graphql!(executor, &i18n::graphql::Show{id: id.0})
    }

    field availableLanguage(&executor) -> FieldResult<Vec<String>> {
        __graphql!(executor, &i18n::graphql::Languages)
    }
    field currentUser(&executor) -> FieldResult<Option<nut::graphql::users::Info>> {
        __graphql!(executor, &nut::graphql::users::Current)
    }

    field userLogs(&executor, limit: I64) -> FieldResult<Vec<nut::graphql::users::Log>> {
        __graphql!(executor, &nut::graphql::users::Logs{limit: limit.0})
    }
    field indexUser(&executor) -> FieldResult<Vec<nut::graphql::users::Info>> {
        __graphql!(executor, &nut::graphql::users::Index{})
    }
    field showUser(&executor, uid: String) -> FieldResult<nut::graphql::users::Info> {
        __graphql!(executor, &nut::graphql::users::Show{uid: uid.clone()})
    }
    field getUserAuthority(&executor, uid: String) -> FieldResult<Vec<nut::graphql::users::Authority>> {
        __graphql!(executor, &nut::graphql::users::GetAuthority{uid: uid.clone()})
    }

    field indexAttachment(&executor) -> FieldResult<Vec<nut::graphql::attachments::Attachment>> {
        __graphql!(executor, &nut::graphql::attachments::Index{})
    }
    field showAttachment(&executor, id: I64) -> FieldResult<nut::graphql::attachments::Attachment> {
        __graphql!(executor, &nut::graphql::attachments::Show{id: id.0})
    }

    field indexLeaveWord(&executor, limit: I64) -> FieldResult<Vec<nut::graphql::leave_words::LeaveWord>> {
        __graphql!(executor, &nut::graphql::leave_words::Index{limit: limit.0})
    }

    field indexVote(&executor) -> FieldResult<Vec<nut::graphql::votes::Vote>> {
        __graphql!(executor, &nut::graphql::votes::Index{})
    }

    field indexCard(&executor) -> FieldResult<Vec<nut::graphql::cards::Card>> {
        __graphql!(executor, &nut::graphql::cards::Index{})
    }
    field showCard(&executor, id: I64) -> FieldResult<nut::graphql::cards::Card> {
        __graphql!(executor, &nut::graphql::cards::Show{id: id.0})
    }

    field indexLink(&executor) -> FieldResult<Vec<nut::graphql::links::Link>> {
        __graphql!(executor, &nut::graphql::links::Index{})
    }
    field showLink(&executor, id: I64) -> FieldResult<nut::graphql::links::Link> {
        __graphql!(executor, &nut::graphql::links::Show{id: id.0})
    }

    field indexFriendLink(&executor) -> FieldResult<Vec<nut::graphql::friend_links::FriendLink>> {
        __graphql!(executor, &nut::graphql::friend_links::Index{})
    }
    field showFriendLink(&executor, id: I64) -> FieldResult<nut::graphql::friend_links::FriendLink> {
        __graphql!(executor, &nut::graphql::friend_links::Show{id: id.0})
    }

    field indexTag(&executor) -> FieldResult<Vec<nut::graphql::tags::Tag>> {
        __graphql!(executor, &nut::graphql::tags::Index{})
    }
    field showTag(&executor, id: I64) -> FieldResult<nut::graphql::tags::Tag> {
        __graphql!(executor, &nut::graphql::tags::Show{id: id.0})
    }

    field indexCategory(&executor) -> FieldResult<Vec<nut::graphql::categories::Category>> {
        __graphql!(executor, &nut::graphql::categories::Index{})
    }
    field showCategory(&executor, id: I64) -> FieldResult<nut::graphql::categories::Category> {
        __graphql!(executor, &nut::graphql::categories::Show{id: id.0})
    }

    field indexForumPost(&executor) -> FieldResult<Vec<forum::graphql::posts::Post>> {
        __graphql!(executor, &forum::graphql::posts::Index{})
    }
    field showForumPost(&executor, id: I64) -> FieldResult<forum::graphql::posts::Post> {
        __graphql!(executor, &forum::graphql::posts::Show{id: id.0})
    }
    field indexForumTopic(&executor) -> FieldResult<Vec<forum::graphql::topics::Topic>> {
        __graphql!(executor, &forum::graphql::topics::Index{})
    }
    field showForumTopic(&executor, id: I64) -> FieldResult<forum::graphql::topics::Topic> {
        __graphql!(executor, &forum::graphql::topics::Show{id: id.0})
    }

});
