use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher,
};

pub struct Password {
    password: String,
    pub secure_hash: String,
}

impl Password {
    pub fn new(password: &String) -> Password {
        Password {
            password: password.clone(),
            secure_hash: calculate_secure_hash(password),
        }
    }
    pub fn is_secure(&self) -> bool {
        let mut alphabet_size: u8 = 0;

        if self.has_lowercase() {
            alphabet_size += 26;
        }
        if self.has_uppercase() {
            alphabet_size += 26;
        }
        if self.has_numbers() {
            alphabet_size += 10;
        }
        if self.has_special_characters() {
            alphabet_size += 32;
        }

        let entropy = self.calculate_entropy(alphabet_size);

        if entropy < 100_f64 {
            return false;
        }

        return true;
    }

    fn calculate_entropy(&self, alphabet_size: u8) -> f64 {
        let password_length = self.password.len();

        println!("Alphabet size: {}", alphabet_size);
        println!("Password length: {}", password_length);

        let tmp = alphabet_size as f64;
        let entropy = tmp.powf(password_length as f64);
        let bits_of_entropy = entropy.log2();

        bits_of_entropy
    }

    fn has_lowercase(&self) -> bool {
        self.password.chars().any(|c| c.is_lowercase())
    }

    fn has_uppercase(&self) -> bool {
        self.password.chars().any(|c| c.is_uppercase())
    }

    fn has_numbers(&self) -> bool {
        self.password.chars().any(|c| c.is_numeric())
    }

    fn has_special_characters(&self) -> bool {
        self.password.chars().any(|c| !c.is_alphanumeric())
    }
}

fn calculate_secure_hash(password: &String) -> String {
    let salt: SaltString = SaltString::generate(&mut OsRng);
    let argon2: Argon2 = Argon2::default();

    let password_hash: PasswordHash = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password");

    password_hash.to_string()
}
