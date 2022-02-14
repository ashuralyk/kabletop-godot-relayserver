Relay server is a simple server that just relays interactive network interfaces in [kabletop-godot](https://github.com/ashuralyk/kabletop-godot). The interfaces are listed below:

> 1. prepare_kabletop_channel
> 2. open_kabletop_channel
> 3. close_kabletop_channel
> 4. notify_game_over
> 5. switch_round
> 6. sync_operation
> 7. sync_p2p_message

Also, the relay server implements basic room management logic, supporting client connection, disconnection, registration and logout operations.