use glium::{glutin::{self, event::{DeviceEvent}}, Display};
use std::f32::consts::PI as pi;
use crate::{utils::*};
#[derive(Clone, Copy)]
pub struct CameraMat{ 
    pub view_mat: [[f32;4];4],
    pub pers_mat: [[f32;4];4],
}
#[derive(Clone, Copy)]
pub struct Camera {
    aspect_ratio: f32,
    translation_sensitivity: f32,
    rotation_sensitivity: f32,

    position: (f32, f32, f32),
    direction: (f32, f32, f32),
    up: (f32,f32,f32),

    m_position: (f32, f32, f32),
    yaw: f32,
    pitch:f32,

    moving_up: bool,
    moving_left: bool,
    moving_down: bool,
    moving_right: bool,
    moving_forward: bool,
    moving_backward: bool,
    
    first_mouse: bool
}
impl Camera {
    pub fn new(screen: &Display) -> Camera {

        let m_position = (screen.get_framebuffer_dimensions().0 as f32 /2., screen.get_framebuffer_dimensions().1 as f32 /2.);
        
        Camera {
            aspect_ratio: 1024.0 / 768.0,
            translation_sensitivity: 0.05,
            rotation_sensitivity: 0.0025,

            position: (0.1, 0.1, 1.1),
            direction: (0.0, 0.0, 1.0),
            up: (0.0, 1.0, 0.0),
            
            m_position: (m_position.0, m_position.1, 0.,),
            yaw: -pi/2.,
            pitch: 0.0,

            moving_up: false,
            moving_left: false,
            moving_down: false,
            moving_right: false,
            moving_forward: false,
            moving_backward: false,

            first_mouse: true,
        }
    }

    pub fn get_perspective(&mut self) -> [[f32;4];4] {
        let fov: f32 = pi / 3.0;
        let zfar = 1024.0;
        let znear = 0.1;
        let f = 1.0 / (fov / 2.0).tan();
        [
            [f / self.aspect_ratio  , 0.0 , 0.0 , 0.0],
            [ 0.0  ,  f  ,  0.0  ,  0.0],
            [ 0.0  , 0.0 , (zfar+znear)/(zfar-znear) ,     1.0],
            [ 0.0  , 0.0 , -(2.0*zfar*znear)/(zfar-znear) , 0.0],
        ]
    }
    
        //  x  y  z   
        // --------
        // 1  0  0  | x' =  x | although the x axis aligns
        // 0  0  1  | y' =  z | the y and z axis dont so we
        // 0 -1  0  | z' = -y | re-align them

    pub fn view_matrix(&mut self) -> [[f32;4];4] {
        let f = normalize(self.direction); // X axis
        let s = cross(f, self.up); 
        let s_norm = normalize(s); // Z axis
        let u = cross(f,s_norm); // y axis relative to camera

        let p = (
            -self.position.0 * s.0 - self.position.1 * s.1 - self.position.2 * s.2,
            -self.position.0 * u.0 - self.position.1 * u.1 - self.position.2 * u.2,
            -self.position.0 * f.0 - self.position.1 * f.1 - self.position.2 * f.2);

        return [
            [s_norm.0, u.0, f.0, 0.0],
            [s_norm.1, u.1, f.1, 0.0],
            [s_norm.2, u.2, f.2, 0.0],
            [p.0, p.1,  p.2, 1.0],
        ]
    }
    
    pub fn look_at(&mut self, event: &glutin::event::DeviceEvent) {
        let _mouse_callback = match *event {
            DeviceEvent::MouseMotion { delta } => {
                if self.first_mouse
                {
                    self.first_mouse = false;
                }

                self.yaw += delta.0 as f32 * self.rotation_sensitivity;
                self.pitch += delta.1 as f32* self.rotation_sensitivity;

                let z = 1.55334;
                if self.pitch > z {
                    self.pitch = z };
                if self.pitch < -z {
                    self.pitch = -z };

                self.direction.0 = self.yaw.cos() * self.pitch.cos();
                self.direction.1 = self.pitch.sin();
                self.direction.2 = self.yaw.sin() * self.pitch.cos();
            },
            _ => return,
        };
    }
// 
//  User Input
// 
    pub fn update(&mut self) {
        let f = normalize(self.direction);

        let up = (0.0, 1.0, 0.0);

        let mut s = cross(f, up);

        s = normalize(s);

        let u = cross(s, f);

        if self.moving_up {
            self.position.0 += u.0 * self.translation_sensitivity;
            self.position.1 += u.1 * self.translation_sensitivity;
            self.position.2 += u.2 * self.translation_sensitivity;
        }

        if self.moving_left {
            self.position.0 -= s.0 * self.translation_sensitivity;
            self.position.1 -= s.1 * self.translation_sensitivity;
            self.position.2 -= s.2 * self.translation_sensitivity;
        }

        if self.moving_down {
            self.position.0 -= u.0 * self.translation_sensitivity;
            self.position.1 -= u.1 * self.translation_sensitivity;
            self.position.2 -= u.2 * self.translation_sensitivity;
        }

        if self.moving_right {
            self.position.0 += s.0 * self.translation_sensitivity;
            self.position.1 += s.1 * self.translation_sensitivity;
            self.position.2 += s.2 * self.translation_sensitivity;
        }

        if self.moving_forward {
            self.position.0 += f.0 * self.translation_sensitivity;
            self.position.1 += f.1 * self.translation_sensitivity;
            self.position.2 += f.2 * self.translation_sensitivity;
        }

        if self.moving_backward {
            self.position.0 -= f.0 * self.translation_sensitivity;
            self.position.1 -= f.1 * self.translation_sensitivity;
            self.position.2 -= f.2 * self.translation_sensitivity;
        }
    }

    pub fn input(&mut self, event: &glutin::event::WindowEvent<'_>) {
        let input = match *event {
            glutin::event::WindowEvent::KeyboardInput { input, .. } => input,
            _ => return,
        };
        let pressed = input.state == glutin::event::ElementState::Pressed;
        let key = match input.virtual_keycode {
            Some(key) => key,
            None => return,
        };
        match key {
            glutin::event::VirtualKeyCode::Up => self.moving_up = pressed,
            glutin::event::VirtualKeyCode::Down => self.moving_down = pressed,
            glutin::event::VirtualKeyCode::A => self.moving_left = pressed,
            glutin::event::VirtualKeyCode::D => self.moving_right = pressed,
            glutin::event::VirtualKeyCode::W => self.moving_forward = pressed,
            glutin::event::VirtualKeyCode::S => self.moving_backward = pressed,
            _ => (),
    
        };
    }
}