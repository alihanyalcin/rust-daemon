#[macro_export]
macro_rules! new {
    ($name:expr, $description:expr) => {{
        daemon::new($name, $description, vec![])
    }};

    ($name:expr, $description:expr, $dependency:expr $(, $dependencies:expr)* $(,)*) => {{
        daemon::new($name, $description, vec![$dependency$(,$dependencies)*])
    }};
}

#[macro_export]
macro_rules! no_args {
    () => {{
        vec![]
    }};
}

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
