use juniper::FieldResult;
use validator::Validate;

use super::super::{
    i18n,
    plugins::{forum, nut},
};
use super::{Context, Handler, I64};

pub struct Mutation;

graphql_object!(Mutation: Context | &self | {
    field install(&executor, form: nut::graphql::Install) -> FieldResult<()> {
        __graphql!(executor, &form)
    }

    field author(&executor, form: nut::graphql::site::Author) -> FieldResult<()> {
        __graphql!(executor, &form)
    }
    field smtp(&executor, form: nut::tasks::send_email::Config) -> FieldResult<()> {
        __graphql!(executor, &form)
    }
    field seo(&executor, form: nut::graphql::site::Seo) -> FieldResult<()> {
        __graphql!(executor, &form)
    }
    field clearCache(&executor) -> FieldResult<()> {
        __graphql!(executor, &nut::graphql::site::ClearCache{})
    }
    field sendTestEmail(&executor) -> FieldResult<()> {
        __graphql!(executor, &nut::tasks::send_email::Test{})
    }

    field saveLocale(&executor, form: i18n::graphql::Save) -> FieldResult<()> {
        __graphql!(executor, &form)
    }
    field destroyLocale(&executor, id: I64) -> FieldResult<()> {
        __graphql!(executor, &i18n::graphql::Destroy{id: id.0})
    }

    field userSignIn(&executor, form: nut::graphql::users::SignIn) -> FieldResult<String> {
        __graphql!(executor, &form)
    }
    field userSignUp(&executor, form: nut::graphql::users::SignUp) -> FieldResult<()> {
        __graphql!(executor, &form)
    }
    field userConfirm(&executor, form: nut::graphql::users::Confirm) -> FieldResult<()> {
        __graphql!(executor, &form)
    }
    field userConfirmToken(&executor, form: nut::graphql::users::ConfirmToken) -> FieldResult<()> {
        __graphql!(executor, &form)
    }
    field userUnlock(&executor, form: nut::graphql::users::Unlock) -> FieldResult<()> {
        __graphql!(executor, &form)
    }
    field userUnlockToken(&executor, form: nut::graphql::users::UnlockToken) -> FieldResult<()> {
        __graphql!(executor, &form)
    }
    field userForgotPassword(&executor, form: nut::graphql::users::ForgotPassword) -> FieldResult<()> {
        __graphql!(executor, &form)
    }
    field userResetPassword(&executor, form: nut::graphql::users::ResetPassword) -> FieldResult<()> {
        __graphql!(executor, &form)
    }
    field userChangePassword(&executor, form: nut::graphql::users::ChangePassword) -> FieldResult<()> {
        __graphql!(executor, &form)
    }
    field userProfile(&executor, form: nut::graphql::users::Profile) -> FieldResult<()> {
        __graphql!(executor, &form)
    }
    field userSignOut(&executor) -> FieldResult<()> {
        __graphql!(executor, &nut::graphql::users::SignOut{})
    }
    field setUserAuthority(&executor, form: nut::graphql::users::SetAuthority) -> FieldResult<()> {
        __graphql!(executor, &form)
    }

    field destroyAttachment(&executor, id: I64) -> FieldResult<()> {
        __graphql!(executor, &nut::graphql::attachments::Destroy{id: id.0})
    }

    field createLeaveWord(&executor, form: nut::graphql::leave_words::Create) -> FieldResult<()> {
        __graphql!(executor, &form)
    }
    field destroyLeaveWord(&executor, id: I64) -> FieldResult<()> {
        __graphql!(executor, &nut::graphql::leave_words::Destroy{id: id.0})
    }

    field updateVote(&executor, form: nut::graphql::votes::Update) -> FieldResult<()> {
        __graphql!(executor, &form)
    }
    field destroyVote(&executor, id: I64) -> FieldResult<()> {
        __graphql!(executor, &nut::graphql::votes::Destroy{id: id.0})
    }

    field createCard(&executor, form: nut::graphql::cards::Create) -> FieldResult<()> {
        __graphql!(executor, &form)
    }
    field updateCard(&executor, form: nut::graphql::cards::Update) -> FieldResult<()> {
        __graphql!(executor, &form)
    }
    field destroyCard(&executor, id: I64) -> FieldResult<()> {
        __graphql!(executor, &nut::graphql::cards::Destroy{id: id.0})
    }

    field createLink(&executor, form: nut::graphql::links::Create) -> FieldResult<()> {
        __graphql!(executor, &form)
    }
    field updateLink(&executor, form: nut::graphql::links::Update) -> FieldResult<()> {
        __graphql!(executor, &form)
    }
    field destroyLink(&executor, id: I64) -> FieldResult<()> {
        __graphql!(executor, &nut::graphql::links::Destroy{id: id.0})
    }

    field createFriendLink(&executor, form: nut::graphql::friend_links::Create) -> FieldResult<()> {
        __graphql!(executor, &form)
    }
    field updateFriendLink(&executor, form: nut::graphql::friend_links::Update) -> FieldResult<()> {
        __graphql!(executor, &form)
    }
    field destroyFriendLink(&executor, id: I64) -> FieldResult<()> {
        __graphql!(executor, &nut::graphql::friend_links::Destroy{id: id.0})
    }

    field createTag(&executor, form: nut::graphql::tags::Create) -> FieldResult<()> {
        __graphql!(executor, &form)
    }
    field updateTag(&executor, form: nut::graphql::tags::Update) -> FieldResult<()> {
        __graphql!(executor, &form)
    }
    field destroyTag(&executor, id: I64) -> FieldResult<()> {
        __graphql!(executor, &nut::graphql::tags::Destroy{id: id.0})
    }

    field createCategory(&executor, form: nut::graphql::categories::Create) -> FieldResult<()> {
        __graphql!(executor, &form)
    }
    field updateCategory(&executor, form: nut::graphql::categories::Update) -> FieldResult<()> {
        __graphql!(executor, &form)
    }
    field destroyCategory(&executor, id: I64) -> FieldResult<()> {
        __graphql!(executor, &nut::graphql::categories::Destroy{id: id.0})
    }

    field createForumPost(&executor, form: forum::graphql::posts::Create) -> FieldResult<()> {
        __graphql!(executor, &form)
    }
    field updateForumPost(&executor, form: forum::graphql::posts::Update) -> FieldResult<()> {
        __graphql!(executor, &form)
    }
    field destroyForumPost(&executor, id: I64) -> FieldResult<()> {
        __graphql!(executor, &forum::graphql::posts::Destroy{id: id.0})
    }
    field createForumTopic(&executor, form: forum::graphql::topics::Create) -> FieldResult<()> {
        __graphql!(executor, &form)
    }
    field updateForumTopic(&executor, form: forum::graphql::topics::Update) -> FieldResult<()> {
        __graphql!(executor, &form)
    }
    field destroyForumTopic(&executor, id: I64) -> FieldResult<()> {
        __graphql!(executor, &forum::graphql::topics::Destroy{id: id.0})
    }

});
