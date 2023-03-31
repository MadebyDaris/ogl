use crate::utils::matrix::ModelMat;
use super::super::utils::app::*;
use super::super::render::*;
use crate::{utils::{app::App as app}, object::*, client::*};
use glium::glutin::{self, event};

pub fn environment() {
    let app = App::new();
    
    let mut model_matrix = ModelMat::new();
    let mut model_matrix_2 = ModelMat::new();
    model_matrix.scale(8.0, 8.0, 8.0);
    model_matrix_2.scale(8.0, 8.0, 2.0);

    
    let mut translation = [1., 4., 3.];
    let mut t = [1., 1., 1.];
    let u_li = (0.1, 0.1, 0.1);
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    
    // SPHERE
    let mut sphere_data = load_obj_file("./data/obj/sphere.obj");
    // let mut sphere_shaders = shader_data { tex_filename :  "./data/tex/tex.jpg".to_string(), vertex_shader : "data/glsl/vertex_shader.glsl".to_string(), fragment_shader : "data/glsl/fragment_shader.glsl".to_string()};
    
    // TERRAIN OBJECT
    let mut terrain_data = load_obj_file("./data/obj/ground.obj");
    // let mut terrain_shader = shader_data { tex_filename :  "./data/tex/tex.jpg".to_string(), vertex_shader : "data/glsl/vertex_shader.glsl".to_string(), fragment_shader : "data/glsl/fragment_shader.glsl".to_string()};
    let mut sphere_shaders = shader_data { tex_filename :  "./data/tex/tex.jpg".to_string(), vertex_shader : "data/glsl/vertex_shader.glsl".to_string(), fragment_shader : "data/glsl/fragment_shader.glsl".to_string()};
    let mut terrain_shader = shader_data { tex_filename :  "./data/tex/tex.jpg".to_string(), vertex_shader : "data/glsl/vertex_shader.glsl".to_string(), fragment_shader : "data/glsl/fragment_shader.glsl".to_string()};

    let mut sphere = RigidBody::new(model_matrix , translation , &app.screen , &sphere_data.verts , sphere_shaders);
    let mut terrain = RigidBody::new(model_matrix_2 , t , &app.screen , &terrain_data.verts , terrain_shader);

    
    let mut _camera = Camera::new(&app.screen);
    let mut children = vec![sphere, terrain];
    let mut w = world::World::new(children, _camera, u_li);

    app::update(app.event_loop, move |events| {

        let camera_mat = CameraMat{ view_mat: w.camera.view_matrix(), pers_mat: w.camera.get_perspective() };
        w.camera.update();
        
        let gravity = 0.98;
        
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
                        model_matrix.translate(0.0, 2.5, 0.0);
                        a.y.0 += 2.5; a.y.1 += 2.5;
                        println!("intersecting");
                    }}
                }}
            }}
        }
                        
        model_matrix.translate(0.0, 0.0, -gravity);
                        
        w.render(&app.screen, &camera_mat, w.u_light);
        let mut action = Action::Continue;
        for event in events {
            match event {
                event::Event::DeviceEvent { event, .. } => match event {
                    ev => w.camera.look_at(&ev)
                },
                event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => action = Action::Stop,
                    ev => w.camera.input(&ev),
                },
                _ => (),
            }
        }
        action
    })
}