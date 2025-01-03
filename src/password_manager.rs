use std::collections::HashMap;
use std::io::{self, Write};
use std::fs::{self};
use serde::{Deserialize, Serialize};
use rpassword::read_password;
use ring::aead::{self, LessSafeKey, UnboundKey, Aad, Nonce, NONCE_LEN};
use ring::rand::{SystemRandom, SecureRandom};
use base64::{engine::general_purpose, Engine};

const STORAGE_FILE: &str = "passwords.json";
const KEY_FILE: &str = "key.bin";

#[derive(Serialize, Deserialize)]
pub struct PasswordData {
    pub service: String,
    pub nonce: String,
    pub encrypted_password: String,
}

pub struct PasswordManager {
    pub passwords: HashMap<String, PasswordData>,
    key: LessSafeKey,
}

impl PasswordManager {
    pub fn new() -> Self {
        let key = PasswordManager::load_or_generate_key();

        let passwords = PasswordManager::load_passwords_from_file();

        PasswordManager { passwords, key }
    }

    fn load_passwords_from_file() -> HashMap<String, PasswordData> {
        if let Ok(data) = fs::read_to_string(STORAGE_FILE) {
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            HashMap::new()
        }
    }

    // Čuva lozinke u fajlu
    fn save_passwords_to_file(&self) {
        let data = serde_json::to_string(&self.passwords).unwrap();
        fs::write(STORAGE_FILE, data).unwrap();
    }

    fn load_or_generate_key() -> LessSafeKey {
        if let Ok(key_bytes) = fs::read(KEY_FILE) {
            let unbound_key = UnboundKey::new(&aead::AES_256_GCM, &key_bytes).unwrap();
            LessSafeKey::new(unbound_key)
        } else {
            let mut key_bytes = [0u8; 32];
            SystemRandom::new().fill(&mut key_bytes).unwrap();
            let unbound_key = UnboundKey::new(&aead::AES_256_GCM, &key_bytes).unwrap();

            fs::write(KEY_FILE, &key_bytes).unwrap();

            LessSafeKey::new(unbound_key)
        }
    }

    pub fn add_password(&mut self) {
        print!("Enter the service name: ");
        io::stdout().flush().unwrap();
        let mut service = String::new();
        io::stdin().read_line(&mut service).unwrap();
        let service = service.trim();

        print!("Enter the password: ");
        io::stdout().flush().unwrap();
        let password = read_password().unwrap();

        let nonce = self.generate_nonce();
        let encrypted_password = self.encrypt_password(&password, &nonce);

        let password_data = PasswordData {
            service: service.to_string(),
            nonce: general_purpose::STANDARD.encode(&nonce),
            encrypted_password,
        };

        self.passwords.insert(service.to_string(), password_data);
        self.save_passwords_to_file();
        println!("Password added successfully.");
    }

    pub fn get_password(&self) {
        print!("Enter the service name: ");
        io::stdout().flush().unwrap();
        let mut service = String::new();
        io::stdin().read_line(&mut service).unwrap();
        let service = service.trim();

        if let Some(data) = self.passwords.get(service) {
            let nonce = general_purpose::STANDARD.decode(&data.nonce).unwrap();
            match self.decrypt_password(&data.encrypted_password, &nonce) {
                Some(password) => println!("Password: {}", password),
                None => println!("Failed to decrypt password."),
            }
        } else {
            println!("No password found for the given service.");
        }
    }

    fn encrypt_password(&self, password: &str, nonce: &[u8]) -> String {
        let mut password_bytes = password.as_bytes().to_vec();
        let nonce = Nonce::try_assume_unique_for_key(nonce).unwrap();
        self.key.seal_in_place_append_tag(nonce, Aad::empty(), &mut password_bytes).unwrap();
        general_purpose::STANDARD.encode(&password_bytes)
    }

    fn decrypt_password(&self, encrypted_password: &str, nonce: &[u8]) -> Option<String> {
        let mut encrypted_password_bytes = general_purpose::STANDARD.decode(encrypted_password).unwrap();
        let nonce = Nonce::try_assume_unique_for_key(nonce).unwrap();
        let decrypted_data = self.key.open_in_place(nonce, Aad::empty(), &mut encrypted_password_bytes).ok()?;
        Some(String::from_utf8_lossy(decrypted_data).to_string())
    }


    fn generate_nonce(&self) -> Vec<u8> {
        let mut nonce = vec![0u8; NONCE_LEN];
        SystemRandom::new().fill(&mut nonce).unwrap();
        nonce
    }
}
