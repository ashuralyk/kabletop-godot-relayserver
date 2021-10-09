use std::collections::{
	HashMap, HashSet
};
use kabletop_godot_sdk::p2p::protocol_relay::types::{
	ClientInfo, request, response
};
use kabletop_ckb_sdk::p2p::{
	ServerClient, Caller
};

pub struct RelayServer {
	p2p_serverclient: Option<ServerClient>,
	partial_clients:  HashMap<i32, ClientInfo>,
	channels:         HashSet<(ClientInfo, ClientInfo)>,
	events:           HashMap<&'static str, Box<dyn Fn() + Send>>
}

impl RelayServer {
	pub fn new() -> Self {
		RelayServer {
			p2p_serverclient: None,
			partial_clients:  HashMap::new(),
			channels:         HashSet::new(),
			events:           HashMap::new()
		}
	}

	fn call_event(&self, event: &str) {
		if let Some(callback) = self.events.get(event) {
			callback();
		}
	}

	pub fn set_serverclient(&mut self, serverclient: ServerClient) {
		self.p2p_serverclient = Some(serverclient);
	}

	pub fn get_serverclient(&mut self, client_id: i32) -> &mut ServerClient {
		self.p2p_serverclient.as_mut().unwrap().set_id(client_id)
	}

	pub fn listen_event(&mut self, event: &'static str, callback: Box<dyn Fn() + Send>) {
		if let Some(value) = self.events.get_mut(event) {
			*value = callback;
		} else {
			self.events.insert(event, callback);
		}
	}

	pub fn add_partial_client(&mut self, client_id: i32, client: ClientInfo) -> bool {
		if let None = self.partial_clients.get(&client_id) {
			self.partial_clients.insert(client_id, client);
			self.call_event("add_partial_client");
			true
		} else {
			false
		}
	}

	pub fn remove_partial_client(&mut self, client_id: i32) -> bool {
		if let Some(_) = self.partial_clients.get(&client_id) {
			self.partial_clients.remove(&client_id);
			self.call_event("remove_partial_client");
			true
		} else {
			false
		}
	}

	pub fn get_partial_clients(&self) -> Vec<ClientInfo> {
		self.partial_clients
			.iter()
			.map(|(_, client)| client.clone())
			.collect::<Vec<_>>()
	}

	pub fn get_partner_client(&self, client_id: i32) -> Option<ClientInfo> {
		let mut partner_info = None;
		for (client, partner) in &self.channels {
			if client.id == client_id {
				partner_info = Some(partner.clone());
				break
			} else if partner.id == client_id {
				partner_info = Some(client.clone());
				break
			}
		}
		partner_info
	}

	pub fn connect(&mut self, client: ClientInfo, partial_id: i32) -> bool {
		let mut connected = false;
		if let None = self.partial_clients.get(&client.id) {
			if let Some(partial) = self.partial_clients.get(&partial_id) {
				self.channels.insert((client, partial.clone()));
				connected = true;
			}
		}
		if connected {
			self.remove_partial_client(partial_id);
			self.call_event("connect");
		}
		connected
	}

	pub fn client_disconnect(&mut self, client_id: i32) {
		let mut found = false;
		if let Some(client) = self.partial_clients.get(&client_id) {
			println!("[RELAY] partial client {}({}/{}) disconnected", client.nickname, client.staking_ckb, client.bet_ckb);
			found = true;
		}
		if found {
			self.partial_clients.remove(&client_id);
			self.call_event("disconnect");
		}
		if let Some(partner) = self.get_partner_client(client_id) {
			let _: response::PartnerDisconnect = self.get_serverclient(client_id)
				.call("partner_disconnect", request::PartnerDisconnect { client_id })
				.expect("disconnect");
			if let Some(client) = self.get_partner_client(partner.id) {
				println!("[RELAY] channel {}({}) <=> {}({}) disconnected", client.nickname, client.id, partner.nickname, partner.id);
				self.channels.remove(&(client, partner));
			} else {
				panic!("broken channel of {} <=> {}", client_id, partner.id)
			}
		}
	}
}
