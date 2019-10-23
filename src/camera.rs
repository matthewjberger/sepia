use nalgebra::{Isometry3, Point3, Vector3};
use nalgebra_glm as glm;

pub enum CameraDirection {
    Forward,
    Backward,
    Left,
    Right,
}

pub struct Camera {
    position: Point3<f32>,
    right: Vector3<f32>,
    front: Vector3<f32>,
    up: Vector3<f32>,
    world_up: Vector3<f32>,
    speed: f32,
    sensitivity: f32,
    yaw: f32,
    pitch: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: Add a way to direct/position the camera
impl Camera {
    pub fn new() -> Self {
        let mut camera = Camera {
            position: Point3::new(0.0, 0.0, 10.0),
            right: Vector3::new(0.0, 0.0, 0.0),
            front: Vector3::new(0.0, 0.0, -1.0),
            up: Vector3::new(0.0, 0.0, 0.0),
            world_up: Vector3::new(0.0, -1.0, 0.0),
            speed: 100.0,
            sensitivity: 0.05,
            yaw: -90.0,
            pitch: 0.0,
        };
        camera.calculate_vectors();
        camera
    }

    pub fn view_matrix(&self) -> glm::Mat4 {
        let target = self.position + self.front;
        glm::convert(Isometry3::look_at_rh(&self.position, &target, &self.up))
    }

    pub fn translate(&mut self, direction: CameraDirection, delta_time: f32) {
        let velocity = self.speed * delta_time;
        match direction {
            CameraDirection::Forward => self.position += self.front * velocity,
            CameraDirection::Backward => self.position -= self.front * velocity,
            CameraDirection::Left => self.position += self.right * velocity,
            CameraDirection::Right => self.position -= self.right * velocity,
        };
    }

    pub fn process_mouse_movement(&mut self, x_offset: f32, y_offset: f32) {
        let (x_offset, y_offset) = (x_offset * self.sensitivity, y_offset * self.sensitivity);

        self.yaw -= x_offset;
        self.pitch += y_offset;

        let pitch_threshold = 89.0;
        if self.pitch > pitch_threshold {
            self.pitch = pitch_threshold
        } else if self.pitch < -pitch_threshold {
            self.pitch = -pitch_threshold
        }

        self.calculate_vectors();
    }

    fn calculate_vectors(&mut self) {
        let pitch_radians = self.pitch.to_radians();
        let yaw_radians = self.yaw.to_radians();
        self.front = Vector3::new(
            pitch_radians.cos() * yaw_radians.cos(),
            pitch_radians.sin(),
            yaw_radians.sin() * pitch_radians.cos(),
        )
        .normalize();
        self.right = self.front.cross(&self.world_up).normalize();
        self.up = self.right.cross(&self.front).normalize();
    }
}
