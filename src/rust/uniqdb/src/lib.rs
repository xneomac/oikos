pub mod github;
// pub mod mongodb;

use serde::{de::DeserializeOwned, Serialize};
pub trait UniqDb<T>
where
    T: DeserializeOwned + Serialize + Clone,
{
    type Error;

    fn get_all(&self) -> Result<Vec<T>, Self::Error>;
    fn create(&self, id: &str, name: &str, model: &T) -> Result<T, Self::Error>;
    fn get(&self, id: &str) -> Result<T, Self::Error>;
    fn update(&self, id: &str, model: &T) -> Result<T, Self::Error>;
    fn delete(&self, id: &str) -> Result<(), Self::Error>;
}
