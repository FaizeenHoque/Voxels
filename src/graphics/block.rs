use cgmath::Vector3;
use wgpu::{Buffer, RenderPass, RenderPipeline};

pub struct Block {
    pub name: String,
    pub vertex: Buffer,
    pub indices: Buffer,
}

impl Block {
    pub fn new(name: String, vertex: Buffer, indices: Buffer) -> Block {
        Block {
            name: name,
            vertex: vertex,
            indices: indices,
        }
    }

    pub fn draw(&mut self, mut render_pass: RenderPass, _position: Vector3<i32>) {
        render_pass.set_vertex_buffer(0, self.vertex.slice(..));
        render_pass.set_index_buffer(self.indices.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..(self.indices.size() / 2) as u32, 0, 0..1);
    }
}
