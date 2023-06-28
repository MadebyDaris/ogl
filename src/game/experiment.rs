use std::vec;

use crate::utils::matrix::ModelMat;
use super::super::utils::app::*;
use super::super::render::*;
use crate::{game::world, utils::{app::App as app}, object::*};
use glium::{glutin::{self, event}};

pub fn environment() {
    let app = App::new();

// TRANSFORM MATRIXES
    let model_matrix = ModelMat::new();
    let model_matrix_2 = ModelMat::new();
    model_matrix.scale(8.0, 8.0, 8.0);
    model_matrix.translate(0.0, 0.0, -4.4);
    model_matrix_2.scale(8.0, 0.1, 2.0);

    let translation = [1., 1., 1.];
    let t = [1., 1., 1.];
    let u_li = (0.1, 0.1, 0.1);
    
// SPHERE
    let sphere_data = load_gltf("./data/gltf/player.gltf");
    let sphere_shaders = ShaderData { transform_data: model_matrix, tex_filename : "./data/tex/tex1.jpg".to_string(), vertex_shader : "data/glsl/vertex_shader.glsl".to_string(), fragment_shader : "data/glsl/fragment_shader.glsl".to_string()};
    let sphere = Mesh::new(translation , &app.screen , &sphere_data.verts , sphere_shaders);
    
// TERRAIN OBJECT
    let terrain_data = load_gltf("./data/gltf/terrain.gltf");
    let terrain_shader = ShaderData { transform_data: model_matrix_2, tex_filename : "./data/tex/tex1.jpg".to_string(), vertex_shader : "data/glsl/vertex_shader.glsl".to_string(), fragment_shader : "data/glsl/fragment_shader.glsl".to_string()};
    let terrain = Mesh::new(t , &app.screen , &terrain_data.verts , terrain_shader);

    let mut _camera = Camera::new(&app.screen);
    _camera.update();

    let children:Vec<Mesh> = vec![sphere, terrain];

    let mut w = world::World::new(children, _camera, u_li);

    app::update(app.event_loop, move |events| {

        let mut dt = 0.015;
        let t_old = std::time::Instant::now();
        let mut object_vel = [0.,1.,0.];

        let g = [0., 1000., 0.];

        let camera_mat = CameraMat{ view_mat: _camera.view_matrix(), pers_mat: _camera.get_perspective() };
        _camera.update();
        
// COLISION DETECTION
        {
            let mut a = box_collision_object(&sphere_data.verts);

            let b = box_collision_object(&terrain_data.verts);
            
            if a.x.0 < b.x.1 { if a.x.1 > b.x.0{
                if a.y.0 < b.y.1 { if a.y.1 > b.y.0{
                    if a.z.0 < b.z.1 { if a.z.1 > b.z.0{
                        object_vel[1] -= object_vel[1]*dt;
                    }}
                }}
            }}
            a.x.0 += object_vel[0]; a.x.1 += object_vel[0];
            a.y.0 += object_vel[1]; a.y.1 += object_vel[1];
            a.z.0 += object_vel[2]; a.z.1 += object_vel[2];
        }

    // UPDATE
        object_vel[1] += dt*(-g[1]);

        model_matrix.translate(object_vel[0], object_vel[1], object_vel[2]);      
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
        let t_new = std::time::Instant::now();
        dt = (t_new - t_old).as_secs_f32();
        action
    })
}