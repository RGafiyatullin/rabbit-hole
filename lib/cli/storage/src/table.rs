use std::marker::PhantomData;

use common_interop::curve_select::CurveSelect;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::{AnyError, Storage};

#[derive(Debug, Clone)]
pub struct Table<N, T = N> {
    pub(crate) storage: Storage,
    pub(crate) tree: sled::Tree,

    pub(crate) _pd: PhantomData<(N, T)>,
}

impl<N, T> Table<N, T> {
    pub fn open(storage: impl Into<Storage>) -> Result<Self, AnyError> {
        let storage = storage.into();
        let tree_name = tree_name::<N>();
        let tree = storage.sled_db.open_tree(tree_name)?;
        let table = Self { storage, tree, _pd: Default::default() };

        Ok(table)
    }

    pub fn open_for_curve(
        storage: impl Into<Storage>,
        curve: CurveSelect,
    ) -> Result<Self, AnyError> {
        let storage = storage.into();

        let tree_name = tree_name_for_curve::<N>(curve);
        let tree = storage.sled_db.open_tree(tree_name)?;
        let table = Self { storage, tree, _pd: Default::default() };

        Ok(table)
    }

    pub fn remove(&self, id: &str) -> Result<Option<T>, AnyError>
    where
        T: Serialize,
        T: DeserializeOwned,
    {
        let json_opt = self.tree.remove(id)?;
        let entry_opt = json_opt.map(|json| self.storage.deserialize(&json)).transpose()?;

        Ok(entry_opt)
    }

    pub fn insert(&self, id: &str, entry: &T) -> Result<Option<T>, AnyError>
    where
        T: Serialize,
        T: DeserializeOwned,
    {
        let json = self.storage.serialize(entry)?;
        let json_opt = self.tree.insert(id, json)?;
        let entry_opt = json_opt.map(|json| self.storage.deserialize(&json)).transpose()?;

        Ok(entry_opt)
    }

    pub fn get(&self, key_id: &str) -> Result<Option<T>, AnyError>
    where
        T: DeserializeOwned,
    {
        let Some(json) = self.tree.get(key_id)? else {return Ok(None)};
        let entry = self.storage.deserialize(json)?;
        Ok(Some(entry))
    }

    pub fn select(&self, prefix: &str) -> impl Iterator<Item = Result<(String, T), AnyError>> + '_
    where
        T: DeserializeOwned,
    {
        self.tree.scan_prefix(prefix).map(|result| {
            result.map_err(AnyError::from).and_then(|(key, value)| {
                let key = String::from_utf8(key.as_ref().to_owned())?;
                let value: T = self.storage.deserialize(value.as_ref())?;

                Ok((key, value))
            })
        })
    }

    pub fn dump(&self) -> Result<(), AnyError> {
        eprintln!("Dumping: {:?}", std::str::from_utf8(self.tree.name().as_ref()));
        for k in self.tree.iter() {
            let (k, v) = k?;

            eprintln!(
                "{:?} -> {:?}",
                std::str::from_utf8(k.as_ref()),
                std::str::from_utf8(v.as_ref()),
            );
        }
        eprintln!("Done!");
        Ok(())
    }
}

fn tree_name_for_curve<N>(curve: CurveSelect) -> String {
    format!("{}/{}", curve, std::any::type_name::<N>())
}
fn tree_name<N>() -> String {
    format!("{}", std::any::type_name::<N>())
}
