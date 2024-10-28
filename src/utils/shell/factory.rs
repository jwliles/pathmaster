use super::handlers::ShellHandler;
use super::handlers::{
    BashHandler, FishHandler, GenericHandler, KshHandler, TcshHandler, ZshHandler,
};
use std::env;

pub fn get_shell_handler() -> Box<dyn ShellHandler> {
    let shell = env::var("SHELL").unwrap_or_default();

    match shell.as_str() {
        s if s.contains("zsh") => Box::new(ZshHandler::new()),
        s if s.contains("bash") => Box::new(BashHandler::new()),
        s if s.contains("fish") => Box::new(FishHandler::new()),
        s if s.contains("tcsh") || s.contains("csh") => Box::new(TcshHandler::new()),
        s if s.contains("ksh") => Box::new(KshHandler::new()),
        _ => Box::new(GenericHandler::new()),
    }
}
