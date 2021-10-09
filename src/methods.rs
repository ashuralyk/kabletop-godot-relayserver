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
		let ok = relay_server!().connect(value.requester, value.client_id);
		Ok(json!(response_relay::ConnectClient {
			result: ok
		}))
	})
}

pub fn propose_channel_parameter(client_id: i32, value: Value) -> BoxFuture<'static, Result<Value, String>> {
	Box::pin(async move {
		let value: response::ApproveGameParameter = relay_server!()
			.get_serverclient(client_id)
			.set_id(get_partner_id(client_id, "propose_channel_parameter")?)
			.call("propose_channel_parameter", value)
			.map_err(|err| format!("relay propose_channel_parameter error: {}", err))?;
		Ok(json!(value))
	})
}

pub fn prepare_kabletop_channel(client_id: i32, value: Value) -> BoxFuture<'static, Result<Value, String>> {
	Box::pin(async move {
		let value: response::CompleteAndSignChannel = relay_server!()
			.get_serverclient(client_id)
			.set_id(get_partner_id(client_id, "prepare_kabletop_channel")?)
			.call("prepare_kabletop_channel", value)
			.map_err(|err| format!("relay prepare_kabletop_channel error: {}", err))?;
		Ok(json!(value))
	})
}

pub fn open_kabletop_channel(client_id: i32, value: Value) -> BoxFuture<'static, Result<Value, String>> {
	Box::pin(async move {
		let value: response::OpenChannel = relay_server!()
			.get_serverclient(client_id)
			.set_id(get_partner_id(client_id, "open_kabletop_channel")?)
			.call("open_kabletop_channel", value)
			.map_err(|err| format!("relay open_kabletop_channel error: {}", err))?;
		Ok(json!(value))
	})
}

pub fn close_kabletop_channel(client_id: i32, value: Value) -> BoxFuture<'static, Result<Value, String>> {
	Box::pin(async move {
		let value: response::CloseChannel = relay_server!()
			.get_serverclient(client_id)
			.set_id(get_partner_id(client_id, "close_kabletop_channel")?)
			.call("close_kabletop_channel", value)
			.map_err(|err| format!("relay close_kabletop_channel error: {}", err))?;
		Ok(json!(value))
	})
}

pub fn notify_game_over(client_id: i32, value: Value) -> BoxFuture<'static, Result<Value, String>> {
	Box::pin(async move {
		let value: response::CloseGame = relay_server!()
			.get_serverclient(client_id)
			.set_id(get_partner_id(client_id, "notify_game_over")?)
			.call("notify_game_over", value)
			.map_err(|err| format!("relay notify_game_over error: {}", err))?;
		Ok(json!(value))
	})
}

pub fn switch_round(client_id: i32, value: Value) -> BoxFuture<'static, Result<Value, String>> {
	Box::pin(async move {
		let value: response::OpenRound = relay_server!()
			.get_serverclient(client_id)
			.set_id(get_partner_id(client_id, "switch_round")?)
			.call("switch_round", value)
			.map_err(|err| format!("relay switch_round error: {}", err))?;
		Ok(json!(value))
	})
}

pub fn sync_operation(client_id: i32, value: Value) -> BoxFuture<'static, Result<Value, String>> {
	Box::pin(async move {
		let value: response::ApplyOperation = relay_server!()
			.get_serverclient(client_id)
			.set_id(get_partner_id(client_id, "sync_operation")?)
			.call("sync_operation", value)
			.map_err(|err| format!("relay sync_operation error: {}", err))?;
		Ok(json!(value))
	})
}

pub fn sync_p2p_message(client_id: i32, value: Value) -> BoxFuture<'static, Result<Value, String>> {
	Box::pin(async move {
		let value: response::ReplyP2pMessage = relay_server!()
			.get_serverclient(client_id)
			.set_id(get_partner_id(client_id, "sync_p2p_message")?)
			.call("sync_p2p_message", value)
			.map_err(|err| format!("relay sync_p2p_message error: {}", err))?;
		Ok(json!(value))
	})
}
