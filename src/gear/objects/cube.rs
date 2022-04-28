
struct SpiningComponent {
    // component that spins its object
    is_active: bool,
    rotating_speed: f32,
}


/*
let cube_mesh = Mesh::from_data(
            vec![
                Vertex::new(cgmath::Vector3::new(-0.5, -0.5, -0.5), Color::from_primitive(ColorPrimitives::Red)),
                Vertex::new(cgmath::Vector3::new(-0.5, -0.5, 0.5), Color::from_primitive(ColorPrimitives::White)),
                Vertex::new(cgmath::Vector3::new(0.5, -0.5, -0.5), Color::from_primitive(ColorPrimitives::Cyan)),
                Vertex::new(cgmath::Vector3::new(0.5, -0.5, 0.5), Color::from_primitive(ColorPrimitives::Yellow)),
                Vertex::new(cgmath::Vector3::new(0.5, 0.5, -0.5), Color::from_primitive(ColorPrimitives::Blue)),
                Vertex::new(cgmath::Vector3::new(0.5, 0.5, 0.5), Color::from_primitive(ColorPrimitives::Black)),
                Vertex::new(cgmath::Vector3::new(-0.5, 0.5, -0.5), Color::from_primitive(ColorPrimitives::Magenta)),
                Vertex::new(cgmath::Vector3::new(-0.5, 0.5, 0.5), Color::from_primitive(ColorPrimitives::Green)) ],
            vec![
                0, 2, 3,   0, 3, 1, // front
                4, 6, 7,   4, 7, 5, // back
                3, 2, 4,   3, 4, 5, // right
                7, 6, 0,   7, 0, 1, // left
                6, 4, 2,   6, 2, 0, // bottom 
                1, 3, 5,   1, 5, 7  // top
            ],
        ); 
*/
