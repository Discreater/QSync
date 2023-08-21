use dbm::UserId;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

static KEYS: Lazy<Keys> = Lazy::new(|| {
  let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
  Keys::new(secret.as_bytes())
});

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Claims {
  pub(crate) id: UserId,
  pub(crate) exp: usize,
}

impl Claims {
  pub fn encode(&self) -> Result<String, jsonwebtoken::errors::Error> {
    encode(&Header::default(), &self, &KEYS.encoding)
  }

  pub fn decode(bearer: &str) -> Result<Self, jsonwebtoken::errors::Error> {
    decode::<Self>(bearer, &KEYS.decoding, &Validation::default()).map(|t| t.claims)
  }
}

struct Keys {
  encoding: EncodingKey,
  decoding: DecodingKey,
}

impl Keys {
  fn new(secret: &[u8]) -> Self {
    Self {
      encoding: EncodingKey::from_secret(secret),
      decoding: DecodingKey::from_secret(secret),
    }
  }
}
