use crate::env::{EmptyEnvironment, Environment};
use crate::types::JWTServiceStorage;

use candid::Principal;
use hmac::{Hmac, Mac};
use ic_cdk::caller;
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;
use std::collections::HashMap;

/// Implements the JWTService interface
pub struct JWTService {
    pub env: Box<dyn Environment>,
    pub jwt_users: HashMap<Principal, String>,
    pub owner: Principal,
    pub jwt_secret: String,
}

impl Default for JWTService {
    fn default() -> Self {
        JWTService {
            env: Box::new(EmptyEnvironment {}),
            jwt_users: HashMap::new(),
            owner: Principal::anonymous(),
            jwt_secret: String::from("some-secret"),
        }
    }
}

impl From<JWTServiceStorage> for JWTService {
    fn from(stable: JWTServiceStorage) -> JWTService {
        JWTService {
            env: Box::new(EmptyEnvironment {}),
            jwt_users: HashMap::new(),
            owner: stable.owner,
            jwt_secret: stable.jwt_secret,
        }
    }
}

/// Implements the JWTService interface
impl JWTService {
    pub fn generate_jwt(&mut self) -> String {
        let caller_user: String = caller().to_text();
        let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
        let exp_at: String = (self.env.now_secs() + (1000 * 60 * 60 * 24 * 7)).to_string();
        let mut claims: BTreeMap<&str, &str> = BTreeMap::new();
        claims.insert("sub", "canister login");
        claims.insert("iss", &caller_user);
        claims.insert("exp", &exp_at);
        let token_str = claims.sign_with_key(&key).unwrap();
        self.jwt_users.insert(caller(), token_str.clone());
        return token_str;
    }

    /// Return the user JWT, if one exists
    pub fn get_user_jwt(&self, user_principal: Principal) -> Result<String, String> {
        if self.owner == caller() {
            let jwt_token = self
                .jwt_users
                .get(&user_principal)
                .ok_or_else(|| format!("No jwt with principal {} exists", user_principal))?;
            Ok(jwt_token.clone())
        } else {
            Err(String::from("caller error"))
        }
    }

    /// Return the canister owner
    pub fn get_owner(&self) -> Principal {
        self.owner
    }

    pub fn get_jwt_secret(&self) -> Result<String, String> {
        if self.owner == caller() {
            Ok(self.jwt_secret.clone())
        } else {
            Err(String::from("caller error"))
        }
    }
}