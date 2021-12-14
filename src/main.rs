use std::{
	sync::Mutex, thread, time
};
use kabletop_ckb_sdk::p2p::Server;
use lazy_static::*;

mod types;
mod methods;
use types::RelayServer;

lazy_static! {
	pub static ref RELAY_SERVER: Mutex<RelayServer> = Mutex::new(RelayServer::new());
}

fn main() {
	let server = Server::new("0.0.0.0:11550")
		.register("register_client", methods::register_client)
		.register("unregister_client", methods::unregister_client)
		.register("fetch_clients", methods::fetch_clients)
		.register("connect_client", methods::connect_client)
		.register("disconnect_client", methods::disconnect_client)
		.register("prepare_kabletop_channel", methods::prepare_kabletop_channel)
		.register("open_kabletop_channel", methods::open_kabletop_channel)
		.register("close_kabletop_channel", methods::close_kabletop_channel)
		.register("switch_round", methods::switch_round)
		.register("sync_operation", methods::sync_operation)
		.register("sync_p2p_message", methods::sync_p2p_message)
		.register("notify_game_over", methods::notify_game_over)
		.register_call("propose_connection")
		.register_call("partner_disconnect")
		.register_call("prepare_kabletop_channel")
		.register_call("open_kabletop_channel")
		.register_call("close_kabletop_channel")
		.register_call("switch_round")
		.register_call("sync_operation")
		.register_call("sync_p2p_message")
		.register_call("notify_game_over")
		.listen(50, 0, |client_id, connected| {
			if connected {
				println!("[RELAY] client {} connected", client_id);
			} else {
				// callback function cannot be blocked
				thread::spawn(move || {
					RELAY_SERVER.lock().unwrap().disconnect(client_id);
					println!("[RELAY] client {} disconnected", client_id);
				});
			}
		})
		.expect("start relay server");

	RELAY_SERVER.lock().unwrap().set_serverclient(server);
	println!("[RELAY] relay server started at 0:0:0:0:11550");
	println!("[RELAY] waiting connections...");

	// interrupt main thread
	thread::sleep(time::Duration::from_secs(u64::MAX));
}
