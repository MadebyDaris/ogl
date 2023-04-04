use std::vec;

use crate::utils::matrix::ModelMat;
use super::super::utils::app::*;
use super::super::render::*;
use crate::{game::world, utils::{app::App as app}, object::*};
use glium::{glutin::{self, event}};

pub fn environment() {
    let app = App::new();

// TRANSFORM MATRIXES
    let mut model_matrix = ModelMat::new();
    let mut model_matrix_2 = ModelMat::new();
    model_matrix.scale(8.0, 8.0, 8.0);
    model_matrix.translate(0.0, 0.0, -4.4);
    model_matrix_2.scale(8.0, 1.0, 2.0);

    let translation = [1., 1., 1.];
    let mut t = [1., 1., 1.];
    let u_li = (0.1, 0.1, 0.1);
    
// SPHERE
    let sphere_data = load_obj_file("./data/obj/sphere.obj");
    let sphere_shaders = ShaderData { transform_data: model_matrix, tex_filename :  "./data/tex/tex.jpg".to_string(), vertex_shader : "data/glsl/vertex_shader.glsl".to_string(), fragment_shader : "data/glsl/fragment_shader.glsl".to_string()};
    let sphere = Mesh::new(translation , &app.screen , &sphere_data.verts , sphere_shaders);
    
// TERRAIN OBJECT
    let terrain_data = load_obj_file("./data/obj/ground.obj");
    let terrain_shader = ShaderData { transform_data: model_matrix_2, tex_filename :  "./data/tex/tex.jpg".to_string(), vertex_shader : "data/glsl/vertex_shader.glsl".to_string(), fragment_shader : "data/glsl/fragment_shader.glsl".to_string()};
    let terrain = Mesh::new(t , &app.screen , &terrain_data.verts , terrain_shader);


    let mut _camera = Camera::new(&app.screen);
    _camera.update();

    let children:Vec<Mesh> = vec![sphere, terrain];

    let mut w = world::World::new(children, _camera, u_li);

    app::update(app.event_loop, move |events| {

        let camera_mat = CameraMat{ view_mat: _camera.view_matrix(), pers_mat: _camera.get_perspective() };
        _camera.update();
        
        t[1] += 0.005;

// COLISION DETECTION
        {
            let mut a = box_collision_object(&sphere_data.verts);
            a.x.0 += translation[0]; a.x.1 += translation[0];
            a.y.0 += translation[1]; a.y.1 += translation[1];
            a.z.0 += translation[2]; a.z.1 += translation[2];

            let mut b = box_collision_object(&terrain_data.verts);
            b.x.0 += t[0]; b.x.1 += t[0];
            b.y.0 += t[1]; b.y.1 += t[1];
            b.z.0 += t[2]; b.z.1 += t[2];
            
            if a.x.0 < b.x.1 { if a.x.1 > b.x.0{
                if a.y.0 < b.y.1 { if a.y.1 > b.y.0{
                    if a.z.0 < b.z.1 { if a.z.1 > b.z.0{
                        t[1] -= 0.05;
                        model_matrix.translate(0.0, -0.5, 0.0);
                        a.y.0 -= 0.5; a.y.1 -= 0.5;
                        println!("intersecting");
                    }}
                }}
            }}
        }
                                        
// RENDER
        let sphere_uni = MeshUniforms { mod_matrix: model_matrix.matrix, transform: t, u_light : (1., 1., 1.) };
        let terrain_uni = MeshUniforms { mod_matrix: model_matrix_2.matrix, transform: translation, u_light : (1., 1., 1.) };

        let object_render_data = vec![sphere_uni, terrain_uni];

        w.render(object_render_data, &app.screen, &camera_mat, (1., 1., 1.));
        let mut action = Action::Continue;
        for event in events {
            match event {
                event::Event::DeviceEvent { event, .. } => match event {
                    ev => _camera.look_at(&ev)
                },
                event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => action = Action::Stop,
                    ev => _camera.input(&ev),
                },
                _ => (),
            }
        }
        action
    })
}