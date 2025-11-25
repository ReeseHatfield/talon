use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::process::Command;
use std::time::Duration;
use serenity::all::{ChannelId, Http};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tokio::time::sleep;

use crate::{send_image, send_to_discord};

use anyhow::anyhow;

use std::sync::Arc;

pub async fn live_bird_feed(http: Arc<Http>, channel: ChannelId) {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut buffer = String::new();

    let _raw = termion::raw::RawTerminal::from(stdout);

    for key in stdin.keys() {
        let key = key.unwrap();

        match key {
            // enter key
            Key::Char('\n') | Key::Char('\r') => {
                println!("sending buffer {} to discord", buffer);

                // just clone everything :sob:
                let msg = buffer.clone();
                let http = http.clone();
                let channel = channel;

                take_picture().unwrap();

                sleep(Duration::from_millis(100)).await;


                tokio::spawn(async move {
                    // send_to_discord(&http, channel, &msg).await;

                    let _ = send_image(&http, channel, "img/photo.jpg", &msg).await;
                });

                buffer.clear();


            }
            // any other basic char
            Key::Char(c) => {
                buffer.push(c);
                print!("{}", c);
            }
            // bird shouldnt be able to hit this?
            Key::Ctrl('c') => break,

            // anythhing else -> cntr, alt, etc
            _ => {}
        }

        std::io::stdout().flush().unwrap();
    }
}


fn take_picture() -> Result<(), Box<dyn Error>> {

    let status = Command::new("bash")
        .arg("take-pic.sh")
        .arg("2")
        .status()
        .expect("could not take picture from bash script");


    Ok(())

}
