use password_hash::PasswordHash;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterUserForm {
    #[validate(length(min = 1), required)]
    pub name: Option<String>,

    #[validate(email, required)]
    pub email: Option<String>,

    #[validate(length(min = 1), required)]
    pub password: Option<String>,
}

impl RegisterUserForm {
    pub fn hash_password(self) -> String {
        let binding = self.password.clone().unwrap();
        let hash = PasswordHash::new(&binding).unwrap();
        hash.to_string()
    }
}


// verify a password with this:
// let password = "my_password";
// let hash = PasswordHash::new(password, salt).unwrap();

// let is_valid = PasswordHash::verify(password, hash.clone()).unwrap();