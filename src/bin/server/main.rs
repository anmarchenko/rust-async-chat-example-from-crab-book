use std::sync::Arc;

use async_chat::utils::ChatResult;
use async_std::net::TcpListener;
use async_std::task::block_on;
use async_std::{prelude::*, task};

mod connection;
mod group;
mod group_table;

fn main() -> ChatResult<()> {
    let address = std::env::args().nth(1).expect("Usage: server ADDRESS");

    let chat_group_table = Arc::new(group_table::GroupTable::new());

    block_on(async {
        let listener = TcpListener::bind(address).await?;

        let mut new_connections = listener.incoming();

        while let Some(socket_result) = new_connections.next().await {
            let socket = socket_result?;
            let groups = chat_group_table.clone();

            task::spawn(async {
                // log_error(connection::serve)
            })
        }

        Ok(())
    })
}
