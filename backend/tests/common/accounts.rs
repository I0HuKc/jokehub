pub trait TestUser {
    fn get_username(&self) -> &str;
    fn get_password(&self) -> &str;
}

// Тестовый пользователь с уровнем доступа Padawan
#[allow(dead_code)]
pub struct TestPadawan<'a> {
    username: &'a str,
    password: &'a str,
}

impl<'a> TestPadawan<'a> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        TestPadawan {
            username: "upadawan",
            password: "password2022",
        }
    }
}

impl<'a> TestUser for TestPadawan<'a> {
    fn get_username(&self) -> &str {
        return self.username;
    }

    fn get_password(&self) -> &str {
        return self.password;
    }
}
