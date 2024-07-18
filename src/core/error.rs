use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::format;
use std::string::FromUtf8Error;
use std::io;
use colored::Colorize;
use tracing::{trace, debug, info, warn, error};

#[macro_export]
macro_rules! mme_err {
    ($($arg:tt)*) => { MmeError::new().msg(format!( $($arg)*)) };
}

pub type MmeResult<T> = Result<T, MmeError>;

pub trait MmeResultTrait<T> {
    fn critical(self) -> T;
}

pub trait IntoMmeResult<T, S> {
    fn mme_result(self) -> MmeResult<T>;
    fn mme_result_msg(self, msg: S) -> MmeResult<T> where S: std::fmt::Display ;
}

#[derive(Debug, Clone)]
pub struct MmeError {
    pub category: Vec<String>,
    pub code: u32,
    pub messages: Vec<String>,
    pub code_location: Option<MmeCodeLocation>,
}

#[derive(Debug, Clone)]
pub struct MmeCodeLocation {
    file: String,
    line: u32,
    column: u32,
}

impl Display for MmeCodeLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "file: {} line: {} column: {}", self.file, self.line, self.column)
    }
}

impl From<&std::panic::Location <'_>> for MmeCodeLocation {
    fn from(panic_location: &std::panic::Location) -> MmeCodeLocation {
        let file = panic_location.file().to_string();
        let line = panic_location.line();
        let column = panic_location.column();
        MmeCodeLocation {file, line, column}
    }
}

impl MmeError {
    #[track_caller]
    pub fn code(code: u32) -> MmeError {
        let caller_location = std::panic::Location::caller();

        get_error_by_code(code, caller_location)
    }

    pub fn set_code(mut self, code: u32) -> MmeError {
        self.code = code;
        return self;
    }

    #[track_caller]
    pub fn new() -> MmeError {
        let caller_location = std::panic::Location::caller();

        return MmeError {
            category: Vec::new(),
            code: 0,
            messages: Vec::new(),
            code_location: Some(caller_location.into()),
        }
    }

    pub fn msg<E>(mut self, msg: E) -> MmeError 
        where E: std::fmt::Display
    {
        self.messages.push(format!("{}", msg));
        return self;
    }

    pub fn category<T>(mut self, category: T) -> MmeError where T: Into<String> {
        self.category.push(category.into());
        return self;
    }
    
    fn send(self) -> MmeError {
        self
    }

    fn add_to_err_item(self) -> MmeError {
        self
    }

    pub fn log(self) -> MmeError {
        error!("MmeError envountered!");
        if let Some(ref location) = self.code_location {
            error!("[ {} ] {}", "LOCATION".yellow(), location);
        };
        for msg in &self.messages {
            error!("[ {} ] {}", "MSG".yellow(), msg);
        }
        self
    }

    pub fn handle(self) -> MmeError {
        return self.send().add_to_err_item().log();
    }

    pub fn critical(self){
        let mut msg_iter = self.messages.iter();
        match msg_iter.next() {
            None => {
                error!("{} MmeError with code: {}", "CRITICAL".red(), self.code);
            },
            Some(msg) => {
                error!("{} MmeError with code: {} - {}", "CRITICAL".red(), self.code, msg);
            },
        }
        for msg in msg_iter {
            error!("{} {}", "MSG".red(), msg)
        }
        if let Some(location) = self.code_location {
            error!("{} {}", "LOCATION".red(), location)
        }
        panic!();
    }

    pub fn location(mut self, location: MmeCodeLocation) -> MmeError {

        self.code_location = Some(location);

        self
    }

}

impl<T: Display> From<T> for MmeError {
    #[track_caller]
    fn from(value: T) -> Self {
        let caller_location = std::panic::Location::caller();
        MmeError::new()
            .msg(format!("From {}: {}", std::any::type_name_of_val(&value), value))
            .location(caller_location.into())
    }
}

impl <T> MmeResultTrait<T> for MmeResult<T> {
    fn critical(self) -> T {
        match self {
            Ok(val) => val,
            Err(err) => {
                err.critical();
                unreachable!()
            },
        }
    }
}

impl<T, E, S> IntoMmeResult<T, S> for Result<T, E> where E: std::fmt::Display {
    #[track_caller]
    fn mme_result(self) -> MmeResult<T> {
        let caller_location = std::panic::Location::caller();

        match self {
            Ok(val) => MmeResult::Ok(val),
            Err(err) => MmeResult::Err(MmeError::new()
                .category("misc")
                .msg(format!("From {}: {}", std::any::type_name_of_val(&err), err))),
        }
    }
    #[track_caller]
    fn mme_result_msg(self, msg: S) -> MmeResult<T> where S: std::fmt::Display {
        let caller_location = std::panic::Location::caller();
        match self {
            Ok(val) => MmeResult::Ok(val),
            Err(err) => MmeResult::Err(MmeError::new()
                .category("misc")
                .msg(format!("{}", msg))
                .msg(format!("From {}: {}", std::any::type_name_of_val(&err), err))),
        }
    }
}


fn get_error_by_code(code: u32, caller_location: &std::panic::Location) -> MmeError {
    let err = MmeError {
        category: Vec::new(),
        code: 0,
        messages: Vec::new(),
        code_location: Some(caller_location.into()),
    };

    match code {
        109 => err.set_code(109).category("decoding").msg("conversion from utf8 failed"),
        108 => err.set_code(108).category("decoding").msg("serde deserialisation failed"),
        _ => err,
    }
}



