const AUTH_PATH: &str = "/auth";
const USERS_PATH: &str = "/users";

pub enum RoutePath {
    AUTH,
    USERS,
}

impl RoutePath {
    pub fn get_path(&self) -> &'static str {
        match self {
            RoutePath::AUTH => AUTH_PATH,
            RoutePath::USERS => USERS_PATH,
        }
    }
}
