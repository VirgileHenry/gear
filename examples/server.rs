use std::mem::size_of;

use Gear::*;
use cgmath::Vector3;
use gear_macros_derive::{NetworkSerializable};

fn main() {
    // create the engine with the window
    let mut engine = Engine::new(); // with a window


    // create cube and camera entity
    let world = engine.get_world();

    let server_handler: MyServer = MyServer{};
    let server = Server::<NetworkMessages, MyServer>::new(server_handler, 31415, 10).unwrap();
    let server_system = System::new(Box::new(server), UpdateFrequency::PerFrame);
    world.register_system(server_system, 20);

    // start main loop
    engine.main_loop();
}

#[derive(NetworkSerializable)]
pub enum NetworkMessages {
    ErrorMessage,
    SpawnPlayer(usize, bool, f32), // player id, is_mine, position (height)
    MovePlayer(usize, f32), // player id, new_position (height)
    DeletePlayer(usize), // player_id
}


struct MyServer {
    
}

impl ServerHandler<NetworkMessages> for MyServer {
    fn on_client_connected(&mut self, client: usize, components: &mut ComponentTable) -> Vec<ServerReturnMessage<NetworkMessages>> {
        // for all clients : send new player arrived with given id and position 0
        let mut result = vec![
            ServerReturnMessage::TcpToExcept(client, NetworkMessages::SpawnPlayer(client, false, 0.0)),
            ServerReturnMessage::TcpToClient(client, NetworkMessages::SpawnPlayer(client, true, 0.0)),
        ];

        // send to new client positions of all players
        // add message of spawning all existing players
        for current_player in iterate_over_component!(components; Player) {
            result.push(ServerReturnMessage::TcpToClient(client, NetworkMessages::SpawnPlayer(current_player.id, false, current_player.position)));
        }

        // create the player entity in the server's world
        create_entity!(components; Player{id: client, position: 0.0});

        result
    }

    fn on_client_disconnected(&mut self, client: usize, components: &mut ComponentTable) -> Vec<ServerReturnMessage<NetworkMessages>> {
        // remove player component
        // todo : not yet handled by the foundry
        
        // tell all clients to destroy disconnected player
        vec![
            ServerReturnMessage::TcpToAll(NetworkMessages::DeletePlayer(client))
        ]
    }

    fn update(&mut self, components: &mut ComponentTable, delta: f32) -> Vec<ServerReturnMessage<NetworkMessages>> {
        // nothing to do in this system... no physics to run or whatever
        Vec::new()
    }

    fn handle_tcp_message(&mut self, client: usize, message: NetworkMessages, components: &mut ComponentTable) -> Vec<ServerReturnMessage<NetworkMessages>> {
        // only message we handle : if the player moved
        match message {
            NetworkMessages::MovePlayer(player_id, new_pos) => {
                // assert the client is asking to move hiw own player
                if client == player_id {
                    // change the intern representation
                    for player in iterate_over_component_mut!(components; Player) {
                        if player.id == client {
                            player.position = new_pos;
                        }
                    }
                    // send to all other client the player moved
                    vec![
                        ServerReturnMessage::TcpToExcept(client, NetworkMessages::MovePlayer(client, new_pos))
                    ]
                }
                else {
                    // don't send anything
                    Vec::new()
                }
            }
            _ => Vec::new(),
        }
    }

}

struct Player {
    id: usize,
    position: f32,
}