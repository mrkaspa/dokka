use std::io;
use std::process::Command;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum CmdError {
    UtfError(FromUtf8Error),
    IoError(io::Error),
}

impl From<FromUtf8Error> for CmdError {
    fn from(err: FromUtf8Error) -> CmdError {
        CmdError::UtfError(err)
    }
}

impl From<io::Error> for CmdError {
    fn from(err: io::Error) -> CmdError {
        CmdError::IoError(err)
    }
}

pub fn docker_ps() -> Result<String, CmdError> {
    let out = Command::new("docker")
        .arg("ps")
        .arg("-a")
        .arg("--format")
        .arg("'{{ json . }}'")
        .output()?;
    let res = String::from_utf8(out.stdout)?;
    Ok(res)
}
