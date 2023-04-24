use std::io::{self, Cursor, Read, Write};
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex, MutexGuard};

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::AnyError;

#[derive(Debug)]
pub struct TestIO {
    stdin: Arc<Mutex<Cursor<Vec<u8>>>>,
    stdout: Arc<Mutex<Vec<u8>>>,
    stderr: Arc<Mutex<Vec<u8>>>,
}

impl TestIO {
    pub fn from_empty_input() -> Self {
        Self::from_bytes_input(vec![])
    }
    pub fn from_bytes_input(stdin: Vec<u8>) -> Self {
        let stdin = Arc::new(Mutex::new(Cursor::new(stdin)));
        let stdout = Default::default();
        let stderr = Default::default();
        Self { stdin, stdout, stderr }
    }
    pub fn from_yaml_stdin<I>(input: I) -> Result<Self, AnyError>
    where
        I: Serialize,
    {
        let mut stdin = Vec::<u8>::new();
        serde_yaml::to_writer(&mut stdin, &input)?;
        Ok(Self::from_bytes_input(stdin))
    }

    pub fn stdout_as_yaml<T>(&self) -> Result<T, AnyError>
    where
        T: DeserializeOwned,
    {
        let stdout = self.stdout.lock().unwrap();
        let output = serde_yaml::from_slice(stdout.as_ref())?;
        Ok(output)
    }
}

pub struct W<T>(T);

impl<'a> crate::caps::IO for &'a TestIO {
    type Stdin = W<MutexGuard<'a, Cursor<Vec<u8>>>>;
    type Stdout = W<MutexGuard<'a, Vec<u8>>>;
    type Stderr = W<MutexGuard<'a, Vec<u8>>>;

    fn stdin(&self) -> Self::Stdin {
        W(self.stdin.lock().unwrap())
    }
    fn stdout(&self) -> Self::Stderr {
        W(self.stdout.lock().unwrap())
    }
    fn stderr(&self) -> Self::Stderr {
        W(self.stderr.lock().unwrap())
    }
}

impl<T> Write for W<T>
where
    T: DerefMut + Deref,
    T::Target: Write,
{
    fn flush(&mut self) -> io::Result<()> {
        (*self.0).flush()
    }
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        (*self.0).write(buf)
    }
}

impl<T> Read for W<T>
where
    T: DerefMut + Deref,
    T::Target: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (*self.0).read(buf)
    }
}
