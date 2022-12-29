use std::f32::consts::PI;
use bevy::{
    prelude::*,
    render::mesh::Indices,
    render::mesh::PrimitiveTopology,
};

fn hexagon_mesh() -> Mesh {
    let center = ([0., 0., 0.], [0., 0., 1.], [0., 0.]);

    let x = |root: f32| (root * 2. * PI / 6.).cos();
    let y = |root: f32| (root * 2. * PI / 6.).sin();

    let spike0 = ([1., 0., 0.], [0., 0., 1.], [0., 0.]);
    let spike1 = ([x(1.), y(1.), 0.], [0., 0., 1.], [0., 0.]);
    let spike2 = ([x(2.), y(2.), 0.], [0., 0., 1.], [0., 0.]);
    let spike3 = ([x(3.), y(3.), 0.], [0., 0., 1.], [0., 0.]);
    let spike4 = ([x(4.), y(4.), 0.], [0., 0., 1.], [0., 0.]);
    let spike5 = ([x(5.), y(5.), 0.], [0., 0., 1.], [0., 0.]);
    let vertices = [center, spike0, spike1, spike2, spike3, spike4, spike5];
    let mut positions = Vec::with_capacity(6);
    let mut normals = Vec::with_capacity(6);
    let mut uvs = Vec::with_capacity(6);

    for (position, normal, uv) in vertices.iter() {
        positions.push(*position);
        normals.push(*normal);
        uvs.push(*uv);
    }

    let indices = Indices::U32(vec![0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5, 0, 5, 6, 0, 6, 1]);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(indices));
    mesh
}
