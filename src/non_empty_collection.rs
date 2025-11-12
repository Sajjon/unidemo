use serde::{Deserialize, Serialize};

use crate::Error;

macro_rules! declare_non_empty_collection_of {
    ($struct_name: ident, $item_type: ty) => {
        #[derive(Clone, Debug, PartialEq, Eq, derive_more::Deref, uniffi::Object)]
        pub struct $struct_name(Vec<$item_type>);

        #[uniffi::export]
        impl $struct_name {
            #[uniffi::constructor]
            pub fn new(items: Vec<$item_type>) -> Result<Self, Error> {
                if items.is_empty() {
                    Err(Error::EmptyCollection)
                } else {
                    Ok(Self(items))
                }
            }

            pub fn head(&self) -> $item_type {
                self.0[0].clone()
            }

            pub fn get(&self, index: u64) -> Option<$item_type> {
                self.0.get(index as usize).cloned()
            }
        }
    };
}

/*
pub type PublicKeys = NonEmptyCollection<PublicKey>;
pub type KeyPairs = NonEmptyCollection<KeyPair>;

*/

declare_non_empty_collection_of!(NonEmptyStringCollection, String);
declare_non_empty_collection_of!(NonEmptyU8Collection, u8);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_empty_string_collection() {
        assert_eq!(
            NonEmptyStringCollection::new(vec![]).unwrap_err(),
            Error::EmptyCollection
        );

        let collection =
            NonEmptyStringCollection::new(vec!["a".to_string(), "b".to_string()]).unwrap();
        assert_eq!(collection.get(0), Some("a".to_string()));
        assert_eq!(collection.get(1), Some("b".to_string()));
        assert_eq!(collection.get(2), None);
    }

    #[test]
    fn test_non_empty_u8_collection() {
        assert_eq!(
            NonEmptyU8Collection::new(vec![]).unwrap_err(),
            Error::EmptyCollection
        );

        let collection = NonEmptyU8Collection::new(vec![1, 2, 3]).unwrap();
        assert_eq!(collection.get(0), Some(1));
        assert_eq!(collection.get(1), Some(2));
        assert_eq!(collection.get(2), Some(3));
        assert_eq!(collection.get(3), None);
    }
}

/*
#[derive(Clone, Debug, PartialEq, Eq, derive_more::Deref)]
pub struct NonEmptyCollection<Item> {
    items: Vec<Item>,
}

impl<Item> NonEmptyCollection<Item> {
    #[allow(dead_code)]
    pub fn new(items: Vec<Item>) -> Result<Self, Error> {
        if items.is_empty() {
            Err(Error::EmptyCollection)
        } else {
            Ok(Self { items })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_empty_collection() {
        assert_eq!(
            NonEmptyCollection::<u8>::new(vec![]).unwrap_err(),
            Error::EmptyCollection
        );
    }

    #[test]
    fn test_non_empty_collection_success() {
        let collection = NonEmptyCollection::new(vec![1, 2, 3]).unwrap();
        assert_eq!(*collection, vec![1, 2, 3]);
    }
}
*/
