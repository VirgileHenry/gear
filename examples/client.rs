use std::mem::size_of;

use Gear::*;
use cgmath::Vector3;
use gear_macros_derive::{NetworkSerializable};

fn main() {
    // create the engine with the window
    let mut engine = Engine::new() // creates the engine
        .with_gl_window(None); // with a window

    // create a renderer and give shaders to it
    let mut renderer = DefaultOpenGlRenderer::new();
    let program = ShaderProgram::simple_program(
        MONOCHROME_LIT_FRAG_SHADER,
        DEFAULT_VERT_SHADER
    ).expect("Unable to compile shaders !");

    let client_handler: MyClient = MyClient{player_shader: ShaderProgramRef::new(&program)};

    // register the shader program in the renderer
    renderer.register_shader_program(program);
    // assign the renderer to the window
    let mut aspect_ratio = 1.0;
    match engine.get_gl_window_mut() {
        Some(window) => {
            window.set_new_renderer(Box::new(renderer));
            aspect_ratio = window.aspect_ratio();
        },
        None => {},
    }

    // create cube and camera entity
    let world = engine.get_world();

    let mut camera_component = CameraComponent::new_perspective_camera(80.0, aspect_ratio, 0.1, 100.0);
    camera_component.set_as_main(&mut world.components);
    let _camera = create_entity!(&mut world.components; Transform::origin().translated(Vector3::new(0.0, 1.5, 5.0)), camera_component);
    let _sun = create_entity!(&mut world.components; Transform::origin().translated(Vector3::new(-4.0, -4.0, -6.0)), MainLight::new(Color::from_rgb(1.0, 0.8, 0.7), Color::from_rgb(0.2, 0.2, 0.2)));


    let mut client = Client::<NetworkMessages, MyClient>::new(client_handler, std::net::SocketAddr::V4(std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(127, 0, 0, 1), 31415)));
    client.try_connect_tcp();
    let server_system = System::new(Box::new(client), UpdateFrequency::Fixed(0.05));
    world.register_system(server_system, 20);

    // start main loop
    engine.main_loop();
}

#[derive(NetworkSerializable)]
#[derive(Debug)]
pub enum NetworkMessages {
    ErrorMessage,
    SpawnPlayer(usize, bool, f32), // player id, is_mine, position (height)
    MovePlayer(usize, f32), // player id, new_position (height)
    DeletePlayer(usize), // player_id
}


struct MyClient {
    player_shader: ShaderProgramRef,
}

impl ClientHandler<NetworkMessages> for MyClient {
    fn on_connected(&mut self, components: &mut ComponentTable) -> Vec<NetworkMessages> {
        // nothing to do
        Vec::new()
    }

    fn on_connection_failed(&mut self, components: &mut ComponentTable) {
        // print it out ?
        println!("Unable to connect to server.");
    }

    fn on_disconected(&mut self, reason: DisconnectReason, components: &mut ComponentTable) {
        // idkkk
    }

    fn update(&mut self, components: &mut ComponentTable, delta: f32) -> Vec<NetworkMessages> {
        // check for input, and send own position to server // todo

        for (event_listener, transform, player) in iterate_over_component_mut!(components; PlayerController, Transform, Player) {
            if player.mine {
                transform.translate(Vector3::new(0.0, event_listener.input * 3.0 * delta, 0.0));
            }
            if event_listener.input != 0.0 {
                // tell the server we moved
                return vec![
                    NetworkMessages::MovePlayer(player.id, transform.position().y)
                ];
            }
        }

        Vec::new()
    }

    fn handle_tcp_message(&mut self, message: NetworkMessages, components: &mut ComponentTable) -> Vec<NetworkMessages> {
        // check for messages cases, and send nothing back
        match message {
            NetworkMessages::SpawnPlayer(player_id, mine, position) => {
                let new_player = create_entity!(components; Player{id: player_id, mine: mine, position: position},
                    Transform::origin().translated(Vector3::new(player_id as f32 - 3.0, position, 0.0)),
                    MeshRenderer::new(Mesh::sphere(0.3, 25),  Material::from_ref(self.player_shader, Box::new(MonochromeMaterialProperties{color: Color::from_rgb(0.4, 0.8, 1.0)}))));
                if mine {
                    components.add_component(new_player, PlayerController{input: 0.0});
                }
            },
            NetworkMessages::DeletePlayer(player_id) => {
                // todo : destroy player (foundry doesn't fully support it yet...)
                // let's deactivate it for now
                let mut to_remove = None;
                for (ent, player) in iterate_over_component!(components; EntityRef, Player) {
                    if player.id == player_id {
                        to_remove = Some(*ent);
                    }
                }
                match to_remove {
                    Some(ent) => components.destroy_entity(ent),
                    _ => {},
                }
            }
            NetworkMessages::MovePlayer(player_id, position) => {
                // find the said player
                for (player, transform) in iterate_over_component_mut!(components; Player, Transform) {
                    if player.id == player_id {
                        transform.translate(Vector3::new(0.0, position - transform.position().y, 0.0))
                    }
                }
            }
            _ => {
                println!("Nothing to do with incoming message : {:?}", message);    
            }
        };

        Vec::new()
    }

}

struct Player {
    id: usize,
    mine: bool,
    position: f32,
}

struct PlayerController {
    input: f32,
}
