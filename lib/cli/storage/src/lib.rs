pub type AnyError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type BoxedStorage = Box<dyn Storage>;

pub trait StorageOpen: Storage + Sized {
    fn open(arg: &str) -> Result<Self, AnyError>;
}

pub trait StorageOpenBoxed {
    fn open_boxed(arg: &str) -> Result<BoxedStorage, AnyError>;
}

pub trait Storage: Send + Sync + 'static {
    fn flush(&self) -> Result<(), AnyError>;
}

impl<T> StorageOpenBoxed for T
where
    T: StorageOpen + Storage,
{
    fn open_boxed(arg: &str) -> Result<Box<dyn Storage>, AnyError> {
        let storage = T::open(arg)?;
        Ok(Box::new(storage))
    }
}
