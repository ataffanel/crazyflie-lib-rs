use std::time::Duration;

use async_std::future::timeout;

// Example scans for Crazyflies, connect the first one and print the log and param variables TOC.
#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let link_context = crazyflie_link::LinkContext::new(async_executors::AsyncStd);

    // Scann for Crazyflies on the default address
    let found = link_context.scan([0xE7; 5]).await?;

    if let Some(uri) = found.first() {
        println!("Connecting to {} ...", uri);

        let cf = crazyflie_lib::Crazyflie::connect_from_uri(
            async_executors::AsyncStd,
            &link_context,
            uri,
        )
        .await?;

        let mut console_stream = cf.console.get_stream().await;

        while let Ok(Ok(line)) = timeout(Duration::from_secs(1), console_stream.next()).await {
            println!("{}", line);
        }

        cf.disconnect().await;
    } else {
        println!("No Crazyflie found, exiting!");
    }

    Ok(())
}