use std::io;
use std::io::{StderrLock, StdinLock, StdoutLock};

pub trait IO {
    type Stdout: io::Write;
    type Stderr: io::Write;
    type Stdin: io::Read;

    fn stdout(&self) -> Self::Stdout;
    fn stderr(&self) -> Self::Stderr;
    fn stdin(&self) -> Self::Stdin;
}

impl<I> IO for &I
where
    I: IO,
{
    type Stdout = I::Stdout;
    type Stderr = I::Stderr;
    type Stdin = I::Stdin;

    fn stdout(&self) -> Self::Stdout {
        <I as IO>::stdout(*self)
    }
    fn stderr(&self) -> Self::Stderr {
        <I as IO>::stderr(*self)
    }
    fn stdin(&self) -> Self::Stdin {
        <I as IO>::stdin(*self)
    }
}

pub struct ProcessIO;

impl IO for ProcessIO {
    type Stdout = StdoutLock<'static>;
    type Stderr = StderrLock<'static>;
    type Stdin = StdinLock<'static>;

    fn stdout(&self) -> Self::Stdout {
        std::io::stdout().lock()
    }

    fn stderr(&self) -> Self::Stderr {
        std::io::stderr().lock()
    }

    fn stdin(&self) -> Self::Stdin {
        std::io::stdin().lock()
    }
}
