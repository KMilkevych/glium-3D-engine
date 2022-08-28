use glium::glutin;

pub struct Camera {
    position: [f32; 3],
    direction: [f32; 3],
    orientation: [f32; 3],
    moving_up: bool,
    moving_down: bool,
    moving_forward: bool,
    moving_backwards: bool,
    moving_left: bool,
    moving_right: bool,
    move_speed: f32,
    mouse_speed: f32,
    pitch: f32,
    yaw: f32,
}

impl Camera {


    pub fn new(position: [f32; 3], orientation: [f32; 3], pitch: f32, yaw: f32, move_speed: f32, mouse_speed: f32) -> Camera {
        Camera {
            position: position,
            direction: [0.0, 0.0, 1.0],
            orientation: orientation,
            moving_up: false,
            moving_down: false,
            moving_forward: false,
            moving_backwards: false,
            moving_left: false,
            moving_right: false,
            move_speed: move_speed,
            mouse_speed: mouse_speed,
            pitch: pitch,
            yaw: yaw,
        }
    }


    pub fn set_moving_up(&mut self, value: bool) {
        self.moving_up = value;
    }

    pub fn set_moving_down(&mut self, value: bool) {
        self.moving_down = value;
    }

    pub fn set_moving_right(&mut self, value: bool) {
        self.moving_right = value;
    }

    pub fn set_moving_left(&mut self, value: bool) {
        self.moving_left = value;
    }

    pub fn set_moving_forward(&mut self, value: bool) {
        self.moving_forward = value;
    }

    pub fn set_moving_backwards(&mut self, value: bool) {
        self.moving_backwards = value;
    }

    pub fn get_position(&self) -> [f32; 3] {
        return self.position;
    }

    pub fn get_direction(&self) -> [f32; 3] {
        return self.direction;
    }

    pub fn get_orientation(&self) -> [f32; 3] {
        return self.orientation;
    }

    pub fn update_position(&mut self) {
        let f = {
            let f = self.direction;
            let len = (f[0].powi(2) + f[1].powi(2) + f[2].powi(2)).sqrt();
            (f[0] / len, f[1] / len, f[2] / len)
        };

        let up = (self.orientation[0], self.orientation[1], self.orientation[2]);

        let s = (f.1 * up.2 - f.2 * up.1,
                 f.2 * up.0 - f.0 * up.2,
                 f.0 * up.1 - f.1 * up.0);

        let s = {
            let len = s.0 * s.0 + s.1 * s.1 + s.2 * s.2;
            let len = len.sqrt();
            (s.0 / len, s.1 / len, s.2 / len)
        };

        let u = (s.1 * f.2 - s.2 * f.1,
                 s.2 * f.0 - s.0 * f.2,
                 s.0 * f.1 - s.1 * f.0);

        let mut movement: [f32; 3] = [0.0; 3];

        if self.moving_up {
            movement[0] += u.0 * 1.0;
            movement[1] += u.1 * 1.0;
            movement[2] += u.2 * 1.0;
        }

        if self.moving_left {
            movement[0] += s.0 * 1.0;
            movement[1] += s.1 * 1.0;
            movement[2] += s.2 * 1.0;
        }

        if self.moving_down {
            movement[0] -= u.0 * 1.0;
            movement[1] -= u.1 * 1.0;
            movement[2] -= u.2 * 1.0;
        }

        if self.moving_right {
            movement[0] -= s.0 * 1.0;
            movement[1] -= s.1 * 1.0;
            movement[2] -= s.2 * 1.0;
        }

        if self.moving_forward {
            movement[0] += f.0 * 1.0;
            movement[1] += f.1 * 1.0;
            movement[2] += f.2 * 1.0;
        }

        if self.moving_backwards {
            movement[0] -= f.0 * 1.0;
            movement[1] -= f.1 * 1.0;
            movement[2] -= f.2 * 1.0;
        }

        // Normalize movement vector
        
        let movement = {
            let len = (movement[0].powi(2) + movement[1].powi(2) + movement[2].powi(2)).sqrt();
            if len == 0.0 {
                movement
            } else {
                [movement[0]/len, movement[1]/len, movement[2]/len]
            }
            
        };
        

        self.position[0] += movement[0] * self.move_speed;
        self.position[1] += movement[1] * self.move_speed;
        self.position[2] += movement[2] * self.move_speed;

    }

    pub fn update_direction(&mut self) {
        let new_x = self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
        let new_y = self.pitch.to_radians().sin();
        let new_z = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();

        self.direction = [new_x, new_y, new_z];
    }

    pub fn process_input(&mut self, event: &glutin::event::WindowEvent<'_>) {
        let input = match *event {
            glutin::event::WindowEvent::KeyboardInput { input, .. } => input,
            _ => return,
        };
        let state = input.state == glutin::event::ElementState::Pressed;
        let key = match input.virtual_keycode {
            Some(key) => key,
            None => return,
        };
        match key {
            glutin::event::VirtualKeyCode::A => self.moving_left = state,
            glutin::event::VirtualKeyCode::D => self.moving_right = state,
            glutin::event::VirtualKeyCode::W => self.moving_forward = state,
            glutin::event::VirtualKeyCode::S => self.moving_backwards = state,
            glutin::event::VirtualKeyCode::LShift => self.moving_up = state,
            glutin::event::VirtualKeyCode::LControl => self.moving_down = state,
            _ => (),
        };
    }

    pub fn process_mouse_input(&mut self, event: &glutin::event::DeviceEvent) {
        match *event {
            glutin::event::DeviceEvent::MouseMotion { delta } => {
                let (delta_x, delta_y) = (delta.0 as f32, delta.1 as f32);
                self.yaw -= delta_x*self.mouse_speed;
                self.pitch -= delta_y*self.mouse_speed;

                if self.pitch > 89.0f32 {
                    self.pitch = 89.0f32;
                }
                if self.pitch < -89.0f32 {
                    self.pitch = -89.0f32;
                }
            },
            _ => return,
        };
    }

    pub fn get_view_matrix(&self) -> [[f32; 4]; 4] {
        
        let position = &self.position;
        let direction = &self.direction;
        let up = &self.orientation;

        let f = {
            let f = direction;
            let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
            let len = len.sqrt();
            [f[0] / len, f[1] / len, f[2] / len]
        };
    
        let s = [up[1] * f[2] - up[2] * f[1],
                 up[2] * f[0] - up[0] * f[2],
                 up[0] * f[1] - up[1] * f[0]];
    
        let s_norm = {
            let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
            let len = len.sqrt();
            [s[0] / len, s[1] / len, s[2] / len]
        };
    
        let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
                 f[2] * s_norm[0] - f[0] * s_norm[2],
                 f[0] * s_norm[1] - f[1] * s_norm[0]];
    
        let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
                 -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
                 -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];
    
        return [
            [s_norm[0], u[0], f[0], 0.0],
            [s_norm[1], u[1], f[1], 0.0],
            [s_norm[2], u[2], f[2], 0.0],
            [p[0], p[1], p[2], 1.0],
        ]
    }
}