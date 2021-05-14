#[macro_export]
macro_rules! command {
    ($program:expr, $arg:expr $(, $args:expr)* $(,)*) => {
        {
            tokio::process::Command::new($program).arg(&$arg)$(.arg(&$args))*
        }
    };
}

#[macro_export]
macro_rules! command_output {
    ($program:expr, $arg:expr $(, $args:expr)* $(,)*) => {
        {
            crate::command!($program, $arg $(, $args)*).output().await
        }
    };
}

#[macro_export]
macro_rules! command_status {
    ($program:expr, $arg:expr $(, $args:expr)* $(,)*) => {
        {
            crate::command!($program, $arg $(, $args)*).status().await
        }
    };
}

#[macro_export]
macro_rules! path_exist {
    ($path:expr) => {{
        tokio::fs::metadata($path).await.is_ok()
    }};
}
