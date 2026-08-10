use crate::graphics::Vertex;

pub const CUBEVERTS: &[Vertex] = &[
    Vertex {
        position: [-0.5, -0.5, 0.5],
        color: [1.0, 0.0, 0.0],
    }, // 0
    Vertex {
        position: [0.5, -0.5, 0.5],
        color: [0.0, 1.0, 0.0],
    }, // 1
    Vertex {
        position: [0.5, 0.5, 0.5],
        color: [0.0, 0.0, 1.0],
    }, // 2
    Vertex {
        position: [-0.5, 0.5, 0.5],
        color: [1.0, 1.0, 0.0],
    }, // 3
    Vertex {
        position: [-0.5, -0.5, -0.5],
        color: [1.0, 0.0, 1.0],
    }, // 4
    Vertex {
        position: [0.5, -0.5, -0.5],
        color: [0.0, 1.0, 1.0],
    }, // 5
    Vertex {
        position: [0.5, 0.5, -0.5],
        color: [1.0, 1.0, 1.0],
    }, // 6
    Vertex {
        position: [-0.5, 0.5, -0.5],
        color: [0.0, 0.0, 0.0],
    }, // 7
];

pub const CUBEINDS: &[u16] = &[
    0, 1, 2, 2, 3, 0, // front
    1, 5, 6, 6, 2, 1, // right
    5, 4, 7, 7, 6, 5, // back
    4, 0, 3, 3, 7, 4, // left
    3, 2, 6, 6, 7, 3, // top
    4, 5, 1, 1, 0, 4, // bottom
];

pub const PRISMVERTS: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5, 0.5],
        color: [1.0, 0.0, 0.0],
    }, // 0 top-front
    Vertex {
        position: [-0.5, -0.5, 0.5],
        color: [0.0, 1.0, 0.0],
    }, // 1 bl-front
    Vertex {
        position: [0.5, -0.5, 0.5],
        color: [0.0, 0.0, 1.0],
    }, // 2 br-front
    Vertex {
        position: [0.0, 0.5, -0.5],
        color: [1.0, 1.0, 0.0],
    }, // 3 top-back
    Vertex {
        position: [-0.5, -0.5, -0.5],
        color: [1.0, 0.0, 1.0],
    }, // 4 bl-back
    Vertex {
        position: [0.5, -0.5, -0.5],
        color: [0.0, 1.0, 1.0],
    }, // 5 br-back
];

pub const PRISMINDS: &[u16] = &[
    0, 1, 2, // front tri
    3, 5, 4, // back tri (reversed)
    0, 3, 4, 4, 1, 0, // left side
    1, 4, 5, 5, 2, 1, // bottom side
    2, 5, 3, 3, 0, 2, // right side
];
