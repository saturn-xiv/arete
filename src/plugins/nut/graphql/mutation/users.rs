use std::fmt;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum Action {
    SignIn,
    Confirm,
    Unlock,
    ResetPassword,
}

impl fmt::Display for Action {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::SignIn => fmt.write_str("sign-in"),
            Action::Confirm => fmt.write_str("confirm"),
            Action::Unlock => fmt.write_str("unlock"),
            Action::ResetPassword => fmt.write_str("reset-password"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub uid: String,
    pub act: Action,
    pub nbf: i64,
    pub exp: i64,
    pub roles: Vec<String>,
}
