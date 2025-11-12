mod non_empty_collection;

use serde::{Deserialize, Serialize};
use subxt_signer::sr25519::Keypair as SubxtKeyPair;
use subxt_signer::sr25519::PublicKey as SubxtPublicKey;

#[derive(
    Clone,
    derive_more::Deref,
    derive_more::DerefMut,
    derive_more::From,
    derive_more::Debug,
    derive_more::Display,
    uniffi::Object,
)]
#[debug("KeyPair({:x?})", self.public_key())]
#[display("KeyPair({})", self.public_key())]
pub struct KeyPair(SubxtKeyPair);

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error, uniffi::Error)]
pub enum Error {
    #[error("Invalid mnemonic phrase")]
    InvalidMnemonic,

    #[error("The collection is empty")]
    EmptyCollection,
}

impl KeyPair {
    pub fn from_phrase(phrase: impl AsRef<str>, password: Option<String>) -> Result<Self, Error> {
        let phrase = phrase.as_ref();
        let Ok(mnemonic) = bip39::Mnemonic::parse(phrase) else {
            return Err(Error::InvalidMnemonic);
        };

        let Ok(key_pair) = SubxtKeyPair::from_phrase(&mnemonic, password.as_deref()) else {
            return Err(Error::InvalidMnemonic);
        };
        Ok(Self::from(key_pair))
    }
}
#[uniffi::export]
impl KeyPair {
    #[uniffi::constructor]
    pub fn zoo_wrong() -> Self {
        Self::from_phrase(
            "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong".to_owned(),
            None,
        )
        .expect("zoo...wrong is a valid mnemonic")
    }

    #[uniffi::constructor(name = "from_phrase")]
    pub fn from_phrase_(phrase: String, password: Option<String>) -> Result<Self, Error> {
        Self::from_phrase(phrase, password)
    }

    pub fn public_key(&self) -> PublicKeyInner {
        PublicKeyInner::from(self.0.public_key())
    }
}

impl Eq for KeyPair {}
impl PartialEq for KeyPair {
    fn eq(&self, other: &Self) -> bool {
        self.public_key() == other.public_key()
    }
}

#[derive(
    derive_more::Deref,
    derive_more::DerefMut,
    derive_more::From,
    derive_more::Debug,
    derive_more::Display,
    uniffi::Object,
)]
#[debug("0x{}", hex::encode(self.as_ref()))]
#[display("0x{}", hex::encode(self.as_ref()))]
#[uniffi::export(Eq, Debug, Display)]
pub struct PublicKeyInner(SubxtPublicKey);

use std::sync::Arc;

#[derive(
    Clone,
    PartialEq,
    Eq,
    derive_more::Deref,
    derive_more::From,
    derive_more::Debug,
    derive_more::Display,
    uniffi::Record,
)]
#[uniffi::export(Eq, Debug, Display)]
pub struct PublicKey {
    inner: Arc<PublicKeyInner>,
}

impl Clone for PublicKeyInner {
    fn clone(&self) -> Self {
        PublicKeyInner::from(SubxtPublicKey(self.0.0.clone()))
    }
}

impl Eq for PublicKeyInner {}
impl PartialEq for PublicKeyInner {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ref() == other.0.as_ref()
    }
}

#[derive(uniffi::Record)]
pub struct Person {
    pub name: Name,
    pub year_of_birth: u16,
    pub website: Url,
}

#[derive(uniffi::Record)]
pub struct Name {
    pub first_name: String,
    pub last_name: String,
}

use std::str::FromStr;
pub use url::Url;

uniffi::custom_type!(Url, String, {
    remote,
    try_lift: |s| Ok(Url::from_str(&s).unwrap()),
    lower: |s| s.to_string(),
});

#[derive(uniffi::Record)]

pub struct HolderOfData {
    pub data: Vec<u8>,
}

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait NetworkingAntenna: Send + Sync + 'static {
    async fn get_request(&self, url: Url) -> Result<Vec<u8>, Error>;
}

#[derive(uniffi::Object)]
pub struct ApiClient {
    network_antenna: Arc<dyn NetworkingAntenna>,
}

#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
    #[serde(rename = "funded_txo_sum")]
    amount: u64,
}

#[uniffi::export]
impl ApiClient {
    pub async fn get_dot_balance_by_address(&self, address: String) -> Result<u64, Error> {
        // let url = Url::parse(&format!("https://api.example.com/dot_balance/{}", address))?;
        // let response = self.network_antenna.get_request(url).await?;
        // let token_response: TokenResponse = serde_json::from_slice(&response)?;
        // Ok(token_response.amount)
        todo!()
    }
}

uniffi::setup_scaffolding!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mnemonic() {
        let s = "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong";
        let sut = KeyPair::from_phrase(s, None);
        assert!(sut.is_ok());
        let sut = sut.unwrap();
        let public_key = sut.public_key();
        assert_eq!(
            public_key.to_string(),
            "0xfefd19b87ac6f83c8a3cc4e9603a00d4a9e6f8322a625786300380722550c47d".to_owned()
        );
    }
}
