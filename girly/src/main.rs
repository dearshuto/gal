use std::cell::RefCell;
use std::rc::Rc;

use gal::ITrianguratedMeshProvider;
use kiss3d::light::Light;
use kiss3d::nalgebra::{UnitQuaternion, Vector3};
use kiss3d::window::Window;

fn main() {
    let mut window = Window::new("Girly: I'm a viewer for gal");
    window.set_light(Light::StickToCamera);

    // Generate a teapot mesh
    let teapot = gal::prim::Builder::teapot().triangulated();
    let coords = teapot
        .vertices()
        .map(|(x, y, z)| kiss3d::nalgebra::Point3::new(x as f32, y as f32, z as f32))
        .collect();
    let faces = teapot
        .triangles()
        .map(|(x, y, z)| kiss3d::nalgebra::Point3::new(x as u16, y as u16, z as u16))
        .collect();
    let mesh = kiss3d::resource::Mesh::new(
        coords, faces, None,  /*normals*/
        None,  /*uvs*/
        false, /*dynamic_draw*/
    );
    let mut c = window.add_mesh(
        Rc::new(RefCell::new(mesh)),
        kiss3d::nalgebra::Vector3::new(1.0, 1.0, 1.0),
    );

    // Z-up に回転
    let fixed_rotation =
        UnitQuaternion::from_axis_angle(&Vector3::x_axis(), -std::f32::consts::PI / 2.0);
    c.set_local_rotation(fixed_rotation);

    // ワイヤーフレーム描画
    c.set_points_size(10.0);
    c.set_lines_width(1.0);
    c.set_surface_rendering_activation(false);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::z_axis(), 0.014);

    while window.render() {
        c.prepend_to_local_rotation(&rot);
    }
}
