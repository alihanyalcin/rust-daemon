use anyhow::Result;
use env_logger;
use log::{error, info, warn};

#[tokio::main]
pub async fn main() -> Result<()> {
    env_logger::init();

    let daemon = match daemon::new("name", "description", vec!["d1", "d2"]) {
        Ok(daemon) => daemon,
        Err(err) => panic!("{}", err),
    };

    match daemon.install(vec!["args1", "args2"]).await {
        Ok(()) => info!("installed"),
        Err(err) => error!("install error: {}", err),
    }

    match daemon.start().await {
        Ok(()) => info!("started"),
        Err(err) => error!("start error: {}", err),
    }

    match daemon.status().await {
        Ok(status) => match status {
            true => info!("status active"),
            false => warn!("status not active"),
        },
        Err(err) => error!("status error: {}", err),
    }

    match daemon.stop().await {
        Ok(()) => info!("stopped"),
        Err(err) => error!("stop error: {}", err),
    }

    match daemon.remove().await {
        Ok(()) => info!("removed"),
        Err(err) => error!("remove error: {}", err),
    }

    Ok(())
}
