use cgmath::{Vector3, Vector4};

use crate::{buffers::{BufferObject, VertexArray}, Mesh, ui_vertex::UIVertex, Vertex};

pub struct BoundingBox {
    pub points: [Vector3<f32>; 8]
}

impl BoundingBox {
    pub fn new(radius: Vector3<f32>) -> Self {
        let mut points = [radius; 8];
        let mut sign = vec![1., 1., 1.];
        for i in 0..8 {
            for j in 0..3 {
                points[i][j] *= sign[j];
            }
            for j in 0..2 {
                if sign[2 - j] == -1. {
                    sign[3 - j] *= -1.;
                }
            }
            sign[0] *= -1.;
        }
        BoundingBox {points}
    }

}


#[allow(unused)] // technically we are not using vbo and ebo, but keep them alive do avoid dropping them
pub struct MeshRenderingBuffers {
    vao: VertexArray, // holds rendering parameter data and pointer to vbo + ebo
    vbo: BufferObject, // holds vertex data
    ebo: BufferObject, // holds index data
    tri_count: usize, // number of triangles in mesh
    pub bounding_box: Option<BoundingBox>,
}

impl MeshRenderingBuffers {
    pub fn new(vao: VertexArray, vbo: BufferObject, ebo: BufferObject, tri_count: usize, bounding_box_radius: Option<Vector3<f32>>) -> MeshRenderingBuffers {
        MeshRenderingBuffers {
            vao,
            vbo,
            ebo,
            tri_count,
            bounding_box: if let Some(radius) = bounding_box_radius { Some(BoundingBox::new(radius)) } else { None }
        }
    }

    pub fn from(mesh: &Mesh) -> MeshRenderingBuffers {
        // create and bind the vao
        let vao = VertexArray::new();
        vao.bind();
        // create the vbo, bind it, upload data to it and give the vertex the attrib pointers
        let vbo = BufferObject::new(gl::ARRAY_BUFFER);
        vbo.bind();
        vbo.upload_data(&mesh.vertices, gl::STATIC_DRAW);
        Vertex::vertex_attrib_pointer();
        // create the ebo, bind it, upload data to it
        let ebo = BufferObject::new(gl::ELEMENT_ARRAY_BUFFER);
        ebo.bind();
        ebo.upload_data(&mesh.triangles, gl::STATIC_DRAW);
        // unbind the vao first, otherwise we will unbind the vbo and ebo from the vao
        vao.unbind();
        // ubind vbo and ebo
        ebo.unbind();
        vbo.unbind();

        MeshRenderingBuffers::new(vao, vbo, ebo, mesh.triangles.len(), None)
    }

    pub fn bind(&self) {
        self.vao.bind()
    }

    pub unsafe fn draw(&self) {
        gl::DrawElements(
            gl::TRIANGLES, // mode
            self.tri_count as i32, // starting index in the enabled arrays
            gl::UNSIGNED_INT,
            0 as *const std::ffi::c_void, // number of indices to be rendered
        );
    }

    pub fn ui_quad_buffer() -> MeshRenderingBuffers {

        let vertices = vec![
            UIVertex::new(0., 0., 0., 0.),
            UIVertex::new(0., 1., 0., 1.),
            UIVertex::new(1., 1., 1., 1.),
            UIVertex::new(1., 0., 1., 0.),
        ];
        let triangles = vec![
            0, 1, 2,
            2, 3, 0,
        ];

        // create and bind the vao
        let vao = VertexArray::new();
        vao.bind();
        // create the vbo, bind it, upload data to it and give the vertex the attrib pointers
        let vbo = BufferObject::new(gl::ARRAY_BUFFER);
        vbo.bind();
        vbo.upload_data(&vertices, gl::STATIC_DRAW);
        UIVertex::vertex_attrib_pointer();
        // create the ebo, bind it, upload data to it
        let ebo = BufferObject::new(gl::ELEMENT_ARRAY_BUFFER);
        ebo.bind();
        ebo.upload_data(&triangles, gl::STATIC_DRAW);
        // unbind the vao first, otherwise we will unbind the vbo and ebo from the vao
        vao.unbind();
        // ubind vbo and ebo
        ebo.unbind();
        vbo.unbind();

        MeshRenderingBuffers::new(vao, vbo, ebo, triangles.len(), None)
    }
}
