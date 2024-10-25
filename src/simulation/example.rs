use std::f32::consts::PI;

use std::time::Instant;

use crate::mesh::sphere::SphereConstructor;
use crate::mesh::ShaderData;
use super::{super::utils::app::*, world::*};
use super::super::render::*;
use glium::glutin::{self, event::{self}, window::Fullscreen};

pub fn example() {
    // Initialize the basic app and camera settings
    let app = App::new();
    let mut last_frame = Instant::now();

    app.screen.gl_window().window().set_cursor_visible(false);
    app.screen.gl_window().window().set_fullscreen(Some(Fullscreen::Borderless(None)));
    let mut _camera = Camera::new(&app.screen);
    _camera.update();

    // Create a diffuse light source with specified color and direction
    let light = DiffuseLight { u_light_color:(1.,0.1,0.2), u_light_direction:(1.,0.2, 1.)};
    
    // Initialize a sphere constructor with radius, longitude, and latitude for sphere resolution
    let sphere_constructor = SphereConstructor {radius: 2., longitude:32, latitude: 16};

    // Set up shader data, specifying texture and shader file paths
    let sphere_shaders = ShaderData {
        tex_filename: "./data/tex/tex1.jpg".to_string(),
        vertex_shader: "data/glsl/vertex_shader.glsl".to_string(),
        fragment_shader: "data/glsl/fragment_shader.glsl".to_string(),
    };
    
    // Create Meshes for the spheres
    let mut sphere_1 = sphere_constructor.sphere_object(&app.screen, sphere_shaders.clone());
    let mut sphere_2 = sphere_constructor.sphere_object(&app.screen, sphere_shaders.clone());
    let mut sphere_3 = sphere_constructor.sphere_object(&app.screen, sphere_shaders.clone());
    
    sphere_1.translate(0., 0., 0.);
    sphere_2.translate(0., 0., 0.);
    sphere_1.scale(0.1, 1., 0.1);
    sphere_3.translate(2., 3., 0.);


// 
//  RENDERING LOOP
//
    App::update(app.event_loop, move |events| {

        let now = Instant::now();
        let delta_time = now.duration_since(last_frame).as_secs_f32();
        last_frame = now;
        // Get the current screen dimensions
        let (width,height) = app.screen.get_framebuffer_dimensions();

        sphere_1.translate(0.1*delta_time, 0., 0.);


        let mut w: StationnaryWorld<'_> = StationnaryWorld::new(vec![&sphere_1, &sphere_2, &sphere_3], _camera, light);
        
        // Set up the camera matrices for view and perspective, with a 60-degree field of view (PI / 3.0)
        let mut camera_mat: CameraMat = CameraMat{ 
                view_mat: _camera.view_matrix(), 
                pers_mat: _camera.get_perspective(
                    width as f32 / height as f32, 
                    PI/3.0, 
                    1024.,
                    0.1) 
        };
        _camera.update();
        // Render the world with current settings
        w.render(&app.screen, &camera_mat, light, (0.01,0.01,0.01,1.));
    
        // Handle events
        let mut action = Action::Continue;
        for event in events {
            match event {
                // Handle device events (e.g., mouse, keyboard) and update camera view
                event::Event::DeviceEvent { event, .. } => {
                    _camera.look_at(&event); // Handle Device Events
                }
                // Handle window events
                event::Event::WindowEvent { event, .. } => {
                    match event {
                        glutin::event::WindowEvent::Resized(size) => {
                            // Update the projection matrix only
                            camera_mat.pers_mat = _camera.get_perspective(
                                size.width as f32 / size.height as f32,
                                45.0,
                                0.1,
                                100.0);
                        }
                        glutin::event::WindowEvent::CloseRequested => {
                            action = Action::Stop; // Stop the application
                        }
                        glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                            _camera.input(&event); // Pass keyboard input to camera
                            if input.state == glutin::event::ElementState::Pressed 
                               && input.virtual_keycode == Some(glutin::event::VirtualKeyCode::Escape) {
                                action = Action::Stop; // Stop on Escape key press
                            }
                        }
                        _ => {
                            _camera.input(&event); // Handle other window events, if necessary
                        }
                    }
                }
                _ => (),
            }
        }
        action}
    )
}