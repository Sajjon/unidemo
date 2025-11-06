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

#[derive(Clone, Copy, Debug, thiserror::Error, uniffi::Error)]
pub enum Error {
    #[error("Invalid mnemonic phrase")]
    InvalidMnemonic,
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

    pub fn public_key(&self) -> PublicKey {
        PublicKey::from(self.0.public_key())
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
    uniffi::Object,
    derive_more::Debug,
    derive_more::Display,
)]
#[debug("0x{}", hex::encode(self.as_ref()))]
#[display("0x{}", hex::encode(self.as_ref()))]
#[uniffi::export(Eq, Debug, Display)]
pub struct PublicKey(SubxtPublicKey);

impl Clone for PublicKey {
    fn clone(&self) -> Self {
        PublicKey::from(SubxtPublicKey(self.0.0.clone()))
    }
}

impl Eq for PublicKey {}
impl PartialEq for PublicKey {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ref() == other.0.as_ref()
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
