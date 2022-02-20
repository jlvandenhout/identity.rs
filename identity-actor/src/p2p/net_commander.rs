// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use futures::channel::mpsc;
use futures::channel::oneshot;
use futures::future::poll_fn;

use libp2p::request_response::InboundFailure;
use libp2p::request_response::OutboundFailure;
use libp2p::request_response::RequestId;
use libp2p::request_response::ResponseChannel;
use libp2p::Multiaddr;
use libp2p::PeerId;
use libp2p::TransportError;

use super::messages::RequestMessage;
use super::messages::ResponseMessage;

#[derive(Clone)]
pub struct NetCommander {
  command_sender: mpsc::Sender<SwarmCommand>,
}

impl NetCommander {
  pub fn new(command_sender: mpsc::Sender<SwarmCommand>) -> Self {
    NetCommander { command_sender }
  }

  pub async fn send_request(
    &mut self,
    peer: PeerId,
    request: RequestMessage,
  ) -> Result<ResponseMessage, OutboundFailure> {
    let (sender, receiver) = oneshot::channel();
    let command = SwarmCommand::SendRequest {
      peer,
      request,
      response_channel: sender,
    };
    self.send_command(command).await;
    receiver.await.expect("sender was dropped")
  }

  pub async fn send_response(
    &mut self,
    data: Vec<u8>,
    channel: ResponseChannel<ResponseMessage>,
    request_id: RequestId,
  ) -> Result<(), InboundFailure> {
    let (sender, receiver) = oneshot::channel();
    let command = SwarmCommand::SendResponse {
      response: data,
      cmd_response_channel: sender,
      response_channel: channel,
      request_id,
    };
    self.send_command(command).await;
    receiver.await.expect("sender was dropped")
  }

  pub async fn start_listening(&mut self, address: Multiaddr) -> Result<(), TransportError<std::io::Error>> {
    let (sender, receiver) = oneshot::channel();
    let command = SwarmCommand::StartListening {
      address,
      response_channel: sender,
    };
    self.send_command(command).await;
    receiver.await.unwrap()
  }

  pub async fn add_address(&mut self, peer: PeerId, address: Multiaddr) {
    self.send_command(SwarmCommand::AddAddress { peer, address }).await;
  }

  pub async fn get_addresses(&mut self) -> Vec<Multiaddr> {
    let (sender, receiver) = oneshot::channel();
    self
      .send_command(SwarmCommand::GetAddresses {
        response_channel: sender,
      })
      .await;
    receiver.await.expect("sender was dropped")
  }

  pub async fn stop_listening(&mut self) {
    let (sender, receiver) = oneshot::channel();
    self
      .send_command(SwarmCommand::StopListening {
        response_channel: sender,
      })
      .await;
    receiver.await.expect("sender was dropped")
  }

  async fn send_command(&mut self, command: SwarmCommand) {
    let _ = poll_fn(|cx| self.command_sender.poll_ready(cx)).await;
    let _ = self.command_sender.start_send(command);
  }
}

pub enum SwarmCommand {
  SendRequest {
    peer: PeerId,
    request: RequestMessage,
    response_channel: oneshot::Sender<Result<ResponseMessage, OutboundFailure>>,
  },
  SendResponse {
    response: Vec<u8>,
    cmd_response_channel: oneshot::Sender<Result<(), InboundFailure>>,
    response_channel: ResponseChannel<ResponseMessage>,
    request_id: RequestId,
  },
  StartListening {
    address: Multiaddr,
    response_channel: oneshot::Sender<Result<(), TransportError<std::io::Error>>>,
  },
  AddAddress {
    peer: PeerId,
    address: Multiaddr,
  },
  GetAddresses {
    response_channel: oneshot::Sender<Vec<Multiaddr>>,
  },
  StopListening {
    response_channel: oneshot::Sender<()>,
  },
}