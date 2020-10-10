/*
 * This file is part of the Shuttlecraft distribution (https://github.com/shuttlecraft).
 * Copyright (c) 2015 Aravinth Manivannan.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

#![warn(rust_2018_idioms)]
use pretty_env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

use futures::FutureExt;
use std::env;
use std::error::Error;

lazy_static! {
    static ref SECRET: String = env::var("CONN_RES_SECRET")
        .expect("Please set CONN_RES_SECRET to the secret that you wish to send");
}

static MESSAGE: &[u8; 22] = b"You are buying me food";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    let addr = env::var("CONN_RES_ADDR")
        .expect("Please set CONN_RES_ADDR to the address:port that you wish to listen to");

    let secret_bytes = SECRET.as_bytes();

    info!("Listening on: http://{}", addr);

    let mut listener = TcpListener::bind(addr).await?;
    loop {
        while let Ok((inbound, _)) = listener.accept().await {
            let transfer = transfer(inbound, &secret_bytes).map(|r| {
                if let Err(e) = r {
                    info!("Failed to transfer; error={}", e);
                }
            });

            tokio::spawn(transfer);
        }
    }
}

async fn transfer(mut inbound: TcpStream, secret: &[u8]) -> Result<(), Box<dyn Error>> {
    info!(
        "{}",
        inbound.peer_addr().map_err(|_| "Couldn't get address")?
    );

    info!("Sending stream");
    for _ in 0..50_000 {
        inbound.write(MESSAGE).await?;
    }
    inbound.write(secret).await?;

    for _ in 0..50_000 {
        inbound.write(MESSAGE).await?;
    }

    info!("Connection reset");
    Ok(())
}
