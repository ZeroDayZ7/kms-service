pub enum ApiRoute {
    Login,
    Register,
    Profile,
}

impl ApiRoute {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Login => "auth.login",
            Self::Register => "auth.register",
            Self::Profile => "user.profile",
        }
    }
}
