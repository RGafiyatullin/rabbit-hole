use std::ffi::OsString;

pub fn args(input: impl Into<String>) -> Vec<OsString> {
    let mut args: Vec<OsString> = vec!["alice".into()];
    args.extend(input.into().split_whitespace().map(Into::into));
    // eprintln!("args: {:#?}", args);
    args
}
