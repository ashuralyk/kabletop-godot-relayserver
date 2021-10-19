use kabletop_godot_sdk::p2p::{
	protocol::types::response, protocol_relay::types::{
		request as request_relay, response as response_relay, ClientInfo
	}
};
use serde_json::{
	json, Value, from_value
};
use futures::future::BoxFuture;
use kabletop_ckb_sdk::p2p::Caller;

macro_rules! relay_server {
	() => {
		crate::RELAY_SERVER.lock().unwrap()
	};
}

fn get_partner_id(client_id: i32, method: &str) -> Result<i32, String> {
	match relay_server!().get_partner_client(client_id) {
		Some(partner) => Ok(partner.id),
		None          => Err(format!("relay {} error: unchained client_id({})", method, client_id))
	}
}

pub fn register_client(client_id: i32, value: Value) -> BoxFuture<'static, Result<Value, String>> {
	Box::pin(async move {
		let value: request_relay::RegisterClient = from_value(value)
			.map_err(|err| format!("deserialize RegisterClient -> {}", err))?;
		println!("client {} registered: {} ({}/{})", client_id, value.nickname, value.staking_ckb, value.bet_ckb);
		let ok = relay_server!().add_partial_client(client_id, ClientInfo {
			id:          client_id,
			nickname:    value.nickname,
			staking_ckb: value.staking_ckb,
			bet_ckb:     value.bet_ckb
		});
		Ok(json!(response_relay::RegisterClient {
			result: ok
		}))
	})
}

pub fn unregister_client(client_id: i32, value: Value) -> BoxFuture<'static, Result<Value, String>> {
	Box::pin(async move {
		let _: request_relay::UnregisterClient = from_value(value)
			.map_err(|err| format!("deserialize UnregisterClient -> {}", err))?;
		println!("client {} unregistered", client_id);
		let ok = relay_server!().remove_partial_client(client_id);
		Ok(json!(response_relay::UnregisterClient {
			result: ok
		}))
	})
}

pub fn fetch_clients(_: i32, value: Value) -> BoxFuture<'static, Result<Value, String>> {
	Box::pin(async {
		let _: request_relay::FetchClients = from_value(value)
			.map_err(|err| format!("deserialize FetchClients -> {}", err))?;
		let clients = relay_server!().get_partial_clients();
		Ok(json!(response_relay::FetchClients {
			clients
		}))
	})
}

pub fn connect_client(client_id: i32, value: Value) -> BoxFuture<'static, Result<Value, String>> {
	Box::pin(async move {
		let mut value: request_relay::ConnectClient = from_value(value)
			.map_err(|err| format!("deserialize ConnectClient -> {}", err))?;
		value.requester.id = client_id;
		let mut ok = relay_server!().connect(value.requester, value.client_id);
		if ok {
			let value: response_relay::ProposeConnection = relay_server!()
				.get_serverclient(value.client_id)
				.call("propose_connection", request_relay::ProposeConnection {})
				.map_err(|err| format!("relay connect_client error: {}", err))?;
			ok = value.result;
		}
		Ok(json!(response_relay::ConnectClient {
			result: ok
		}))
	})
}

pub fn disconnect_client(client_id: i32, value: Value) -> BoxFuture<'static, Result<Value, String>> {
	Box::pin(async move {
		let _: request_relay::DisconnectClient = from_value(value)
			.map_err(|err| format!("deserialize DisconnectClient -> {}", err))?;
		relay_server!().disconnect(client_id);
		Ok(json!(response_relay::DisconnectClient {}))
	})
}

pub fn prepare_kabletop_channel(client_id: i32, value: Value) -> BoxFuture<'static, Result<Value, String>> {
	Box::pin(async move {
		let partner_id = get_partner_id(client_id, "prepare_kabletop_channel")?;
		let value: response::CompleteAndSignChannel = relay_server!()
			.get_serverclient(partner_id)
			.call("prepare_kabletop_channel", value)
			.map_err(|err| format!("relay prepare_kabletop_channel error: {}", err))?;
		Ok(json!(value))
	})
}

pub fn open_kabletop_channel(client_id: i32, value: Value) -> BoxFuture<'static, Result<Value, String>> {
	Box::pin(async move {
		let partner_id = get_partner_id(client_id, "open_kabletop_channel")?;
		let value: response::OpenChannel = relay_server!()
			.get_serverclient(partner_id)
			.call("open_kabletop_channel", value)
			.map_err(|err| format!("relay open_kabletop_channel error: {}", err))?;
		Ok(json!(value))
	})
}

pub fn close_kabletop_channel(client_id: i32, value: Value) -> BoxFuture<'static, Result<Value, String>> {
	Box::pin(async move {
		let partner_id = get_partner_id(client_id, "close_kabletop_channel")?;
		let value: response::CloseChannel = relay_server!()
			.get_serverclient(partner_id)
			.call("close_kabletop_channel", value)
			.map_err(|err| format!("relay close_kabletop_channel error: {}", err))?;
		Ok(json!(value))
	})
}

pub fn notify_game_over(client_id: i32, value: Value) -> BoxFuture<'static, Result<Value, String>> {
	Box::pin(async move {
		let partner_id = get_partner_id(client_id, "notify_game_over")?;
		let value: response::CloseGame = relay_server!()
			.get_serverclient(partner_id)
			.call("notify_game_over", value)
			.map_err(|err| format!("relay notify_game_over error: {}", err))?;
		Ok(json!(value))
	})
}

pub fn switch_round(client_id: i32, value: Value) -> BoxFuture<'static, Result<Value, String>> {
	Box::pin(async move {
		let partner_id = get_partner_id(client_id, "switch_round")?;
		let value: response::OpenRound = relay_server!()
			.get_serverclient(partner_id)
			.call("switch_round", value)
			.map_err(|err| format!("relay switch_round error: {}", err))?;
		Ok(json!(value))
	})
}

pub fn sync_operation(client_id: i32, value: Value) -> BoxFuture<'static, Result<Value, String>> {
	Box::pin(async move {
		let partner_id = get_partner_id(client_id, "sync_operation")?;
		let value: response::ApplyOperation = relay_server!()
			.get_serverclient(partner_id)
			.call("sync_operation", value)
			.map_err(|err| format!("relay sync_operation error: {}", err))?;
		Ok(json!(value))
	})
}

pub fn sync_p2p_message(client_id: i32, value: Value) -> BoxFuture<'static, Result<Value, String>> {
	Box::pin(async move {
		let partner_id = get_partner_id(client_id, "sync_p2p_message")?;
		let value: response::ReplyP2pMessage = relay_server!()
			.get_serverclient(partner_id)
			.call("sync_p2p_message", value)
			.map_err(|err| format!("relay sync_p2p_message error: {}", err))?;
		Ok(json!(value))
	})
}
