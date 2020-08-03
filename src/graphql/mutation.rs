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
    fn setSiteInfo(context: &Context, form: nut::graphql::site::Info) -> FieldResult<OK> {
        form.execute(context)?;
        Ok(OK::default())
    }
    #[graphql(description = "Update site author")]
    fn setSiteAuthor(context: &Context, email: String, name: String) -> FieldResult<OK> {
        nut::graphql::site::Author::set(context, &email, &name)?;
        Ok(OK::default())
    }
    #[graphql(description = "Update site seo configuration")]
    fn setSiteSeo(
        context: &Context,
        keywords: Vec<String>,
        google_verify_code: Option<String>,
        baidu_verify_code: Option<String>,
    ) -> FieldResult<OK> {
        let form = nut::graphql::site::Seo {
            keywords,
            google: google_verify_code.map(|it| nut::graphql::site::Google { verify_id: it }),
            baidu: baidu_verify_code.map(|it| nut::graphql::site::Baidu { verify_id: it }),
        };
        form.set(context)?;
        Ok(OK::default())
    }
    #[graphql(description = "Update site SMTP configuration")]
    fn setSiteSmtp(
        context: &Context,
        host: String,
        email: String,
        password: String,
    ) -> FieldResult<OK> {
        let form = nut::tasks::send_email::Config {
            host,
            email,
            password,
        };
        form.set(context)?;
        Ok(OK::default())
    }
    #[graphql(description = "Test site SMTP configuration")]
    fn testSiteSmtp(context: &Context) -> FieldResult<OK> {
        executor::block_on(nut::tasks::send_email::Config::test(context))?;
        Ok(OK::default())
    }
    #[graphql(description = "Clear site cache")]
    fn clearSiteCache(context: &Context) -> FieldResult<OK> {
        nut::graphql::site::ClearCache::execute(context)?;
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

    #[graphql(description = "Destory attachment")]
    fn destoryAttachment(context: &Context, id: ID) -> FieldResult<OK> {
        nut::graphql::attahments::Destory::execute(context, id)?;
        Ok(OK::default())
    }

    #[graphql(description = "Create card")]
    fn createCard(context: &Context, form: nut::graphql::cards::Form) -> FieldResult<OK> {
        form.create(context)?;
        Ok(OK::default())
    }
    #[graphql(description = "Update card")]
    fn updateCard(context: &Context, id: ID, form: nut::graphql::cards::Form) -> FieldResult<OK> {
        form.update(context, id)?;
        Ok(OK::default())
    }
    #[graphql(description = "Destory card")]
    fn destoryCard(context: &Context, id: ID) -> FieldResult<OK> {
        nut::graphql::cards::Destory::execute(context, id)?;
        Ok(OK::default())
    }

    #[graphql(description = "Create category")]
    fn createCategory(context: &Context, form: nut::graphql::categories::Form) -> FieldResult<OK> {
        form.create(context)?;
        Ok(OK::default())
    }
    #[graphql(description = "Update category")]
    fn updateCategory(
        context: &Context,
        id: ID,
        form: nut::graphql::categories::Form,
    ) -> FieldResult<OK> {
        form.update(context, id)?;
        Ok(OK::default())
    }
    #[graphql(description = "Destory category")]
    fn destoryCategory(context: &Context, id: ID) -> FieldResult<OK> {
        nut::graphql::categories::Destory::execute(context, id)?;
        Ok(OK::default())
    }
}
