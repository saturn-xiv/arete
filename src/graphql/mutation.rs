use futures::executor;
use juniper::FieldResult;

use super::super::plugins::nut;
use super::{context::Context, OK};

pub struct Mutation;

#[juniper::object(
    Context = Context,
    description = "Writable operations!"
)]
impl Mutation {
    #[graphql(description = "Sign in by email/nick-name & password.")]
    fn usersSignIn(context: &Context, form: nut::graphql::users::SignIn) -> FieldResult<String> {
        let token = form.execute(context)?;
        Ok(token)
    }
    #[graphql(description = "Sign up an email account.")]
    fn usersSignUp(context: &Context, form: nut::graphql::users::SignUp) -> FieldResult<OK> {
        executor::block_on(form.execute(context))?;
        Ok(OK::default())
    }
    #[graphql(description = "Resend active email.")]
    fn usersConfirm(context: &Context, form: nut::graphql::users::EmailForm) -> FieldResult<OK> {
        executor::block_on(form.confirm(context))?;
        Ok(OK::default())
    }
    #[graphql(description = "Active your account.")]
    fn usersConfirmToken(
        context: &Context,
        form: nut::graphql::users::TokenForm,
    ) -> FieldResult<OK> {
        form.confirm(context)?;
        Ok(OK::default())
    }
    #[graphql(description = "Resend unlock email.")]
    fn usersUnlock(context: &Context, form: nut::graphql::users::EmailForm) -> FieldResult<OK> {
        executor::block_on(form.unlock(context))?;
        Ok(OK::default())
    }
    #[graphql(description = "Unlock your account.")]
    fn usersUnlockToken(
        context: &Context,
        form: nut::graphql::users::TokenForm,
    ) -> FieldResult<OK> {
        form.unlock(context)?;
        Ok(OK::default())
    }
    #[graphql(description = "Forgot your password")]
    fn usersForgotPassword(
        context: &Context,
        form: nut::graphql::users::EmailForm,
    ) -> FieldResult<OK> {
        executor::block_on(form.forgot_password(context))?;
        Ok(OK::default())
    }
    #[graphql(description = "Reset your password.")]
    fn usersResetPassword(
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
    fn usersChangePassword(
        context: &Context,
        form: nut::graphql::users::ChangePassword,
    ) -> FieldResult<OK> {
        form.execute(context)?;
        Ok(OK::default())
    }
    #[graphql(description = "Sign out.")]
    fn usersSignOut(context: &Context) -> FieldResult<OK> {
        nut::graphql::users::SignOut::execute(context)?;
        Ok(OK::default())
    }
}
