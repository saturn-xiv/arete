use chrono::NaiveDate;
use juniper::FieldResult;
use validator::Validate;

use super::super::{
    i18n,
    plugins::{forum, nut},
};
use super::{Context, Handler, I16, I64};

pub struct Mutation;

graphql_object!(
    Mutation: Context | &self | {
        field install(&executor, real_name: String, email: String, password: String) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::Install{
                real_name: real_name.clone(),
                email: email.clone(),
                password: password.clone(),
            })
        }

        field author(&executor, name: String, email: String) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::site::Author{
                name: name.clone(),
                email: email.clone(),
            })
        }
        field smtp(&executor, host: String, email: String, password: String) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::tasks::send_email::Config{
                host: host.clone(),
                email: email.clone(),
                password: password.clone(),
            })
        }
        field seo(&executor, google_verify_id: Option<String>, baidu_verify_id: Option<String>) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::site::Seo{
                google: google_verify_id.clone().map(|x| nut::graphql::site::Google{
                    verify_id: x,
                }),
                baidu: baidu_verify_id.clone().map(|x| nut::graphql::site::Baidu{
                    verify_id: x,
                }),
            })
        }
        field clearCache(&executor) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::site::ClearCache{})
        }
        field sendTestEmail(&executor) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::tasks::send_email::Test{})
        }

        field saveLocale(&executor, lang: String, code: String, message: String) -> FieldResult<Option<String>> {
            __graphql!(executor, &i18n::graphql::Save{
                lang: lang.clone(),
                code: code.clone(),
                message: message.clone(),
            })
        }
        field destroyLocale(&executor, id: I64) -> FieldResult<Option<String>> {
            __graphql!(executor, &i18n::graphql::Destroy{id: id.0})
        }

        field userSignIn(&executor, login: String, password: String) -> FieldResult<String> {
            __graphql!(executor, &nut::graphql::users::SignIn{
                login: login.clone(),
                password: password.clone(),
            })
        }
        field userSignUp(&executor, real_name: String, nick_name: String, email: String, password: String, home: String) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::users::SignUp{
                nick_name: nick_name.clone(),
                real_name: real_name.clone(),
                email: email.clone(),
                password: password.clone(),
                home: home.clone(),
            })
        }
        field userConfirm(&executor, email: String, home: String) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::users::Confirm{
                email: email.clone(),
                home: home.clone(),
            })
        }
        field userConfirmToken(&executor, token: String) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::users::ConfirmToken{
                token: token.clone(),
            })
        }
        field userUnlock(&executor, email: String, home: String) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::users::Unlock{
                email: email.clone(),
                home: home.clone(),
            })
        }
        field userUnlockToken(&executor, token: String) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::users::UnlockToken{
                token: token.clone(),
            })
        }
        field userForgotPassword(&executor, email: String, home: String) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::users::ForgotPassword{
                email: email.clone(),
                home: home.clone(),
            })
        }
        field userResetPassword(&executor, password: String, token: String) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::users::ResetPassword{
                password: password.clone(),
                token: token.clone(),
            })
        }
        field userChangePassword(&executor, current_password: String, new_password: String) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::users::ChangePassword{
                current_password: current_password.clone(),
                new_password: new_password.clone(),
            })
        }
        field userProfile(&executor, real_name: String, nick_name: String, email: String, logo: String) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::users::Profile{
                real_name: real_name.clone(),
                nick_name: nick_name.clone(),
                email: email.clone(),
                logo: logo.clone(),
            })
        }
        field userSignOut(&executor) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::users::SignOut{})
        }
        field setUserAuthority(&executor, uid: String, role: String, resource: Option<String>, nbf: NaiveDate, exp: NaiveDate) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::users::SetAuthority{
                uid: uid.clone(),
                authority: nut::graphql::users::Authority{
                    role: role.clone(),
                    resource: resource.clone(),
                    nbf: nbf.clone(),
                    exp: exp.clone(),
                }
            })
        }

        field destroyAttachment(&executor, id: I64) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::attachments::Destroy{id: id.0})
        }

        field createLeaveWord(&executor, body: String, media_type: String) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::leave_words::Create{
                body: body.clone(),
                media_type: media_type.clone(),
            })
        }
        field destroyLeaveWord(&executor, id: I64) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::leave_words::Destroy{id: id.0})
        }

        field updateVote(&executor, resource_type: String, resource_id: I64, like: bool) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::votes::Update{
                resource_type: resource_type.clone(),
                resource_id: resource_id.clone(),
                like: like,
            })
        }
        field destroyVote(&executor, id: I64) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::votes::Destroy{id: id.0})
        }

        field createCard(&executor, lang: String, title: String, logo: String, body: String, media_type: String, href: String, action: String, loc: String, position: I16) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::cards::Create{
                lang: lang.clone(),
                title: title.clone(),
                logo: logo.clone(),
                body: body.clone(),
                media_type: media_type.clone(),
                href: href.clone(),
                action: action.clone(),
                loc: loc.clone(),
                position: position.clone(),
            })
        }
        field updateCard(&executor, id: I64, lang: String, title: String, logo: String, body: String, media_type: String, href: String, action: String, loc: String, position: I16) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::cards::Update{
                id: id.clone(),
                lang: lang.clone(),
                title: title.clone(),
                logo: logo.clone(),
                body: body.clone(),
                media_type: media_type.clone(),
                href: href.clone(),
                action: action.clone(),
                loc: loc.clone(),
                position: position.clone(),
            })
        }
        field destroyCard(&executor, id: I64) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::cards::Destroy{id: id.0})
        }

        field createLink(&executor, lang: String, label: String, href: String, loc: String, x: I16, y: I16) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::links::Create{
                lang: lang.clone(),
                label: label.clone(),
                href: href.clone(),
                loc: loc.clone(),
                x: x.clone(),
                y: y.clone(),
            })
        }
        field updateLink(&executor, id: I64, lang: String, label: String, href: String, loc: String, x: I16, y: I16) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::links::Update{
                id: id.clone(),
                lang: lang.clone(),
                label: label.clone(),
                href: href.clone(),
                loc: loc.clone(),
                x: x.clone(),
                y: y.clone(),
            })
        }
        field destroyLink(&executor, id: I64) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::links::Destroy{id: id.0})
        }

        field createFriendLink(&executor, home: String, title: String, logo: String, position: I16) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::friend_links::Create{
                home: home.clone(),
                title: title.clone(),
                logo: logo.clone(),
                position: position.clone(),
            })
        }
        field updateFriendLink(&executor, id: I64, home: String, title: String, logo: String, position: I16) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::friend_links::Update{
                id: id.clone(),
                home: home.clone(),
                title: title.clone(),
                logo: logo.clone(),
                position: position.clone(),
            })
        }
        field destroyFriendLink(&executor, id: I64) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::friend_links::Destroy{id: id.0})
        }

        field createTag(&executor, name: String, icon: String, color: String) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::tags::Create{
                name: name.clone(),
                icon: icon.clone(),
                color: color.clone(),
            })
        }
        field updateTag(&executor, id: I64, name: String, icon: String, color: String) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::tags::Update{
                id: id.clone(),
                name: name.clone(),
                icon: icon.clone(),
                color: color.clone(),
            })
        }
        field destroyTag(&executor, id: I64) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::tags::Destroy{id: id.0})
        }

        field createCategory(&executor, name: String, icon: String, color: String, parent: Option<I64>, position: I16) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::categories::Create{
                name: name.clone(),
                icon: icon.clone(),
                color: color.clone(),
                parent: parent.clone(),
                position: position.clone(),
            })
        }
        field updateCategory(&executor, id: I64, name: String, icon: String, color: String, parent: Option<I64>, position: I16) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::categories::Update{
                id: id.clone(),
                name: name.clone(),
                icon: icon.clone(),
                color: color.clone(),
                parent: parent.clone(),
                position: position.clone(),
            })
        }
        field destroyCategory(&executor, id: I64) -> FieldResult<Option<String>> {
            __graphql!(executor, &nut::graphql::categories::Destroy{id: id.0})
        }

        field createForumPost(&executor, topic: I64, post: Option<I64>, body: String, media_type: String) -> FieldResult<Option<String>> {
            __graphql!(executor, &forum::graphql::posts::Create{
                topic: topic.clone(),
                post: post.clone(),
                body: body.clone(),
                media_type: media_type.clone(),
            })
        }
        field updateForumPost(&executor, id: I64, body: String, media_type: String) -> FieldResult<Option<String>> {
            __graphql!(executor, &forum::graphql::posts::Update{
                id: id.clone(),
                body: body.clone(),
                media_type: media_type.clone(),
            })
        }
        field destroyForumPost(&executor, id: I64) -> FieldResult<Option<String>> {
            __graphql!(executor, &forum::graphql::posts::Destroy{id: id.0})
        }
        field createForumTopic(&executor, title: String, body: String, media_type: String, tags: Vec<I64>, categories: Vec<I64>) -> FieldResult<Option<String>> {
            __graphql!(executor, &forum::graphql::topics::Create{
                title: title.clone(),
                body: body.clone(),
                media_type: media_type.clone(),
                tags: tags.clone(),
                categories: categories.clone(),
            })
        }
        field updateForumTopic(&executor, id: I64, title: String, body: String, media_type: String, tags: Vec<I64>, categories: Vec<I64>) -> FieldResult<Option<String>> {
            __graphql!(executor, &forum::graphql::topics::Update{
                id: id.clone(),
                title: title.clone(),
                body: body.clone(),
                media_type: media_type.clone(),
                tags: tags.clone(),
                categories: categories.clone(),
            })
        }
        field destroyForumTopic(&executor, id: I64) -> FieldResult<Option<String>> {
            __graphql!(executor, &forum::graphql::topics::Destroy{id: id.0})
        }

    }
);
