#[macro_export]
macro_rules! command {
    ($program:expr, $arg:expr $(, $args:expr)* $(,)*) => {
        {
            std::process::Command::new($program).arg(&$arg)$(.arg(&$args))*
        }
    };
}

#[macro_export]
macro_rules! command_output {
    ($program:expr, $arg:expr $(, $args:expr)* $(,)*) => {
        {
            crate::command!($program, $arg $(, $args)*).output()
        }
    };
}

#[macro_export]
macro_rules! command_status {
    ($program:expr, $arg:expr $(, $args:expr)* $(,)*) => {
        {
            crate::command!($program, $arg $(, $args)*).status()
        }
    };
}
