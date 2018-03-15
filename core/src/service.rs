// Copyright 2018 Kodebox, Inc.
// This file is part of CodeChain.
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
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::sync::Arc;

use cbytes::Bytes;
use cio::{IoHandler, IoService, IoContext};

use super::client::Client;
use super::error::Error;

/// Client service setup.
pub struct ClientService {
    io_service: Arc<IoService<ClientIoMessage>>,
    client: Arc<Client>,
}

impl ClientService {
    pub fn start() -> Result<ClientService, Error> {
        let io_service = IoService::<ClientIoMessage>::start()?;
        let client = Client::new(io_service.channel())?;

        let client_io = Arc::new(ClientIoHandler {
            client: client.clone(),
        });
        io_service.register_handler(client_io)?;

        client.engine().register_client(Arc::downgrade(&client) as _);

        Ok(ClientService {
            io_service: Arc::new(io_service),
            client,
        })
    }
}

/// Message type for external and internal events
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ClientIoMessage {
    /// New consensus message received.
    NewConsensusMessage(Bytes)
}

/// IO interface for the Client handler
struct ClientIoHandler {
    client: Arc<Client>,
}

impl IoHandler<ClientIoMessage> for ClientIoHandler {
    fn message(&self, _io: &IoContext<ClientIoMessage>, net_message: &ClientIoMessage) {
        match *net_message {
            ClientIoMessage::NewConsensusMessage(ref message) => if let Err(e) = self.client.engine().handle_message(message) {
                trace!(target: "poa", "Invalid message received: {}", e);
            },
            _ => {} // ignore other messages
        }
    }
}
