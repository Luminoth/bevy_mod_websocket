# TODO

* Move off [tokio-tungstenite](https://crates.io/crates/tokio-tungstenite) and over to [fastwebsockets](https://crates.io/crates/fastwebsockets)
  * This should be more performant, however it does still bring in the tokio requirement
* Support intential disconnect
  * When disconnecting, need to make sure we despawn the entity
* A lot of cloning from internal Components could be moved to use the same Option::take() pattern used with ConnectWebSocket
