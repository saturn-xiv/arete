use juniper::FieldResult;
use validator::Validate;

use super::super::{i18n, plugins::nut};
use super::{BigSerial, Context, Handler};

pub struct Mutation;

graphql_object!(
    Mutation: Context | &self | {
        field saveLocale(&executor, form: i18n::graphql::Save) -> FieldResult<()> {
            __graphql!(executor, &form)
        }

        field userSignIn(&executor, form: nut::graphql::mutation::users::SignIn) -> FieldResult<String> {
            __graphql!(executor, &form)
        }
        field userSignUp(&executor, form: nut::graphql::mutation::users::SignUp) -> FieldResult<()> {
            __graphql!(executor, &form)
        }
        field userConfirm(&executor, form: nut::graphql::mutation::users::Confirm) -> FieldResult<()> {
            __graphql!(executor, &form)
        }
        field userConfirmToken(&executor, form: nut::graphql::mutation::users::ConfirmToken) -> FieldResult<()> {
            __graphql!(executor, &form)
        }
        field userUnlock(&executor, form: nut::graphql::mutation::users::Unlock) -> FieldResult<()> {
            __graphql!(executor, &form)
        }
        field userUnlockToken(&executor, form: nut::graphql::mutation::users::UnlockToken) -> FieldResult<()> {
            __graphql!(executor, &form)
        }
        field userForgotPassword(&executor, form: nut::graphql::mutation::users::ForgotPassword) -> FieldResult<()> {
            __graphql!(executor, &form)
        }
        field userResetPassword(&executor, form: nut::graphql::mutation::users::ResetPassword) -> FieldResult<()> {
            __graphql!(executor, &form)
        }
        field userLogs(&executor, limit: BigSerial) -> FieldResult<Vec<nut::graphql::mutation::users::Log>> {
            __graphql!(executor, &nut::graphql::mutation::users::Logs{limit: limit.0})
        }
        field userChangePassword(&executor, form: nut::graphql::mutation::users::ChangePassword) -> FieldResult<()> {
            __graphql!(executor, &form)
        }
        field userProfile(&executor, form: nut::graphql::mutation::users::Profile) -> FieldResult<()> {
            __graphql!(executor, &form)
        }
        field userSignOut(&executor) -> FieldResult<()> {
            __graphql!(executor, &nut::graphql::mutation::users::SignOut{})
        }
    }
);
