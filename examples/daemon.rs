use anyhow::{bail, Result};
use clap::{App, Arg};
use env_logger;
use log::{error, info};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::signal::ctrl_c;

#[tokio::main]
pub async fn main() -> Result<()> {
    env_logger::init();

    let config = App::new("Example Service")
        .arg(
            Arg::with_name("addr")
                .help("tcp listener address")
                .short("a")
                .long("addr")
                .default_value("127.0.0.1:8080"),
        )
        .get_matches();

    let addr = config.value_of("addr").unwrap();

    let daemon = match daemon::new!("name", "description").await {
        Ok(daemon) => daemon,
        Err(err) => panic!("{}", err),
    };

    match daemon.install(daemon::no_args!()).await {
        Ok(()) => {
            info!("installed");
        }
        Err(err) => bail!("install error: {}", err),
    };

    match daemon.start().await {
        Ok(()) => {
            info!("started");
        }
        Err(err) => bail!("start error: {}", err),
    };

    match daemon.status().await {
        Ok(status) => {
            info!("active: {}", status);
        }
        Err(err) => bail!("status error: {}", err),
    }

    let listener = TcpListener::bind(&addr).await?;
    info!("listening on: {}", addr);

    tokio::select! {
        res = run(listener) => {
            if let Err(err) = res {
                error!("failed to accept: {}", err);
            }
        }
        _ = ctrl_c() => {
            info!("shutting down");

            match daemon.stop().await {
                Ok(()) => {
                    info!("stopped");
                }
                Err(err) => bail!("stop error: {}", err),
            };


            match daemon.remove().await {
                Ok(()) => {
                    info!("removed");
                }
                Err(err) => bail!("remove error: {}", err),
            };
        }
    }

    Ok(())
}

async fn run(listener: TcpListener) -> Result<()> {
    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if n == 0 {
                    return;
                }

                socket
                    .write_all(&buf[0..n])
                    .await
                    .expect("failed to write data to socket");
            }
        });
    }
}
