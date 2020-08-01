use futures::executor;
use juniper::FieldResult;

use super::super::plugins::nut;
use super::{context::Context, ID, OK};

pub struct Mutation;

#[juniper::object(
    Context = Context,
    description = "Writable operations!"
)]
impl Mutation {
    #[graphql(description = "Initial database.")]
    fn install(context: &Context, form: nut::graphql::Install) -> FieldResult<OK> {
        form.execute(context)?;
        Ok(OK::default())
    }

    #[graphql(description = "Sign in by email/nick-name & password.")]
    fn signInUser(context: &Context, form: nut::graphql::users::SignIn) -> FieldResult<String> {
        let token = form.execute(context)?;
        Ok(token)
    }
    #[graphql(description = "Sign up an email account.")]
    fn signUpUser(context: &Context, form: nut::graphql::users::SignUp) -> FieldResult<OK> {
        executor::block_on(form.execute(context))?;
        Ok(OK::default())
    }
    #[graphql(description = "Resend active email.")]
    fn confirmUser(context: &Context, form: nut::graphql::users::EmailForm) -> FieldResult<OK> {
        executor::block_on(form.confirm(context))?;
        Ok(OK::default())
    }
    #[graphql(description = "Active your account.")]
    fn confirmUserToken(
        context: &Context,
        form: nut::graphql::users::TokenForm,
    ) -> FieldResult<OK> {
        form.confirm(context)?;
        Ok(OK::default())
    }
    #[graphql(description = "Resend unlock email.")]
    fn unlockUser(context: &Context, form: nut::graphql::users::EmailForm) -> FieldResult<OK> {
        executor::block_on(form.unlock(context))?;
        Ok(OK::default())
    }
    #[graphql(description = "Unlock your account.")]
    fn unnlockUserToken(
        context: &Context,
        form: nut::graphql::users::TokenForm,
    ) -> FieldResult<OK> {
        form.unlock(context)?;
        Ok(OK::default())
    }
    #[graphql(description = "Forgot your password")]
    fn forgotUserPassword(
        context: &Context,
        form: nut::graphql::users::EmailForm,
    ) -> FieldResult<OK> {
        executor::block_on(form.forgot_password(context))?;
        Ok(OK::default())
    }
    #[graphql(description = "Reset your password.")]
    fn resetUserPassword(
        context: &Context,
        form: nut::graphql::users::ResetPassword,
    ) -> FieldResult<OK> {
        form.execute(context)?;
        Ok(OK::default())
    }
    #[graphql(description = "Update your profile.")]
    fn usersUpdateProfile(
        context: &Context,
        form: nut::graphql::users::Profile,
    ) -> FieldResult<OK> {
        form.execute(context)?;
        Ok(OK::default())
    }
    #[graphql(description = "Change your password.")]
    fn changeUserPassword(
        context: &Context,
        form: nut::graphql::users::ChangePassword,
    ) -> FieldResult<OK> {
        form.execute(context)?;
        Ok(OK::default())
    }
    #[graphql(description = "Sign out.")]
    fn signOutUser(context: &Context) -> FieldResult<OK> {
        nut::graphql::users::SignOut::execute(context)?;
        Ok(OK::default())
    }
    #[graphql(description = "Lock a account.")]
    fn lockUser(context: &Context, id: ID) -> FieldResult<OK> {
        nut::graphql::users::Lock::execute(context, id)?;
        Ok(OK::default())
    }
    #[graphql(description = "Apply policy to user.")]
    fn applyUserPolicy(
        context: &Context,
        id: ID,
        form: nut::graphql::users::Apply,
    ) -> FieldResult<OK> {
        form.execute(context, id)?;
        Ok(OK::default())
    }
    #[graphql(description = "Deny policy from user.")]
    fn denyUserPolicy(
        context: &Context,
        id: ID,
        form: nut::graphql::users::Deny,
    ) -> FieldResult<OK> {
        form.execute(context, id)?;
        Ok(OK::default())
    }

    #[graphql(description = "Update site info")]
    fn updateSiteInfo(context: &Context, form: nut::graphql::site::Info) -> FieldResult<OK> {
        form.execute(context)?;
        Ok(OK::default())
    }
    #[graphql(description = "Update site author")]
    fn updateSiteAuthor(context: &Context, form: nut::graphql::site::Author) -> FieldResult<OK> {
        form.execute(context)?;
        Ok(OK::default())
    }

    #[graphql(description = "Set locale")]
    fn updateLocale(context: &Context, form: nut::graphql::locales::Update) -> FieldResult<OK> {
        form.execute(context)?;
        Ok(OK::default())
    }
    #[graphql(description = "Remove locale")]
    fn destoryLocale(context: &Context, id: ID) -> FieldResult<OK> {
        nut::graphql::locales::Destory::execute(context, id)?;
        Ok(OK::default())
    }
}
