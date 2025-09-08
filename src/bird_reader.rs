use std::io::{stdin, stdout, Write};
use serenity::all::{ChannelId, Http};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::send_to_discord;

use std::sync::Arc;

pub fn live_bird_feed(http: Arc<Http>, channel: ChannelId) {
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

                tokio::spawn(async move {
                    send_to_discord(&http, channel, &msg).await;
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
