mod config;
mod constants;

use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use discord_rich_presence::{
    activity::{Activity, Assets, Button},
    DiscordIpc, DiscordIpcClient,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting on client ID {}", constants::CONFIG.configuration.client_id);
    let mut client = DiscordIpcClient::new(&constants::CONFIG.configuration.client_id)?;

    client.connect()?;

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        println!("Exiting the RPC....");
        r.store(false, Ordering::Relaxed);
    })?;

    'outer: loop {
        for (i, (k, v)) in constants::CONFIG.statuses.iter().enumerate() {
            if !running.load(Ordering::Relaxed) {
                break 'outer;
            }

            println!("({:?}/{:?}) Currently on {:?}", i + 1, constants::CONFIG.statuses.len(), k);

            let activity = Activity::new()
                .state(&v.state)
                .details(&v.details)
                .assets(Assets::new().large_image(&v.large_image.as_deref().unwrap_or_default())
                                     .small_image(&v.small_image.as_deref().unwrap_or_default()))
                .buttons(v.buttons.as_ref().map(|btns| btns.iter().map(|button| Button::new(&button.label, &button.url)).collect::<Vec<_>>()).unwrap_or_default());

            client.set_activity(activity)?;

            thread::sleep(Duration::from_secs(constants::CONFIG.configuration.time_between));
        }
    }

    client.close()?;

    Ok(())
}
