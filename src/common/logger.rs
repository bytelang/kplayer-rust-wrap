use crate::common::string::{BridgeString, StringPoint};


#[link(wasm_import_module = "2.0.0")]
extern {
    fn logger(level: u32, point: StringPoint);
}

pub enum LogLevel {
    Trace,
    Debug,
    Warn,
    Info,
    Error,
}

pub fn logs<T: ToString>(level: LogLevel, msg: T) {
    let bridge_string = BridgeString::new(msg);
    unsafe {
        logger(level as u32, bridge_string.get());
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        {
            use crate::common::logger;
            use crate::kplayer::unit::APP;
            let file_path = file!();
            let line = line!();
            let func_name = module_path!();
            let log_message = format!($($arg)*);
            logger::logs(
                logger::LogLevel::Info,
                format!("app: {}, file_path: {}, file: {}, func_name: {}, message: {}", APP.unwrap(), file_path, line, func_name, log_message)
            )
        };
    }
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        {
            use crate::common::logger;
            use crate::kplayer::unit::APP;
            let file_path = file!();
            let line = line!();
            let func_name = module_path!();
            let log_message = format!($($arg)*);
            logger::logs(
                logger::LogLevel::Warn,
                format!("app: {}, file_path: {}, file: {}, func_name: {}, message: {}", APP.unwrap(), file_path, line, func_name, log_message)
            )
        };
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        {
            use crate::common::logger;
            use crate::kplayer::unit::APP;

            let file_path = file!();
            let line = line!();
            let func_name = module_path!();
            let log_message = format!($($arg)*);
            logger::logs(
                logger::LogLevel::Debug,
                format!("app: {}, file_path: {}, file: {}, func_name: {}, message: {}", APP.unwrap(), file_path, line, func_name, log_message)
            )
        };
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        {
            use crate::common::logger;
            use crate::kplayer::unit::APP;

            let file_path = file!();
            let line = line!();
            let func_name = module_path!();
            let log_message = format!($($arg)*);
            logger::logs(
                logger::LogLevel::Error,
                format!("app: {}, file_path: {}, file: {}, func_name: {}, message: {}", APP.unwrap(), file_path, line, func_name, log_message)
            );
        }
    };
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        {
            use crate::common::logger;
            use crate::kplayer::unit::APP;

            let file_path = file!();
            let line = line!();
            let func_name = module_path!();
            let log_message = format!($($arg)*);
            logger::logs(
                logger::LogLevel::Trace,
                format!("app: {}, file_path: {}, file: {}, func_name: {}, message: {}", APP.unwrap(), file_path, line, func_name, log_message)
            )
        };
    };
}