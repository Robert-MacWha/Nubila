use std::collections::HashMap;

use log::info;
use winit::{event::WindowEvent, platform::modifier_supplement::KeyEventExtModifierSupplement};

/// WindowInputs is an InputManager that uses winit events to get input.
pub struct Input {
    keys: HashMap<winit::keyboard::Key, bool>,
    prev_keys: HashMap<winit::keyboard::Key, bool>,

    mouse_position: (f64, f64),
    prev_mouse_position: (f64, f64),
}

impl Input {
    pub fn new() -> Self {
        return Self {
            keys: HashMap::new(),
            prev_keys: HashMap::new(),
            mouse_position: (0.0, 0.0),
            prev_mouse_position: (0.0, 0.0),
        };
    }

    pub fn get_key(&self, key: &winit::keyboard::Key) -> bool {
        return *self.keys.get(&key).unwrap_or(&false);
    }

    pub fn get_key_down(&self, key: &winit::keyboard::Key) -> bool {
        return *self.keys.get(&key).unwrap_or(&false)
            && !*self.prev_keys.get(&key).unwrap_or(&false);
    }

    pub fn get_key_up(&self, key: &winit::keyboard::Key) -> bool {
        return !*self.keys.get(&key).unwrap_or(&false)
            && *self.prev_keys.get(&key).unwrap_or(&false);
    }

    pub fn get_mouse_position(&self) -> (f64, f64) {
        return self.mouse_position;
    }

    pub fn get_mouse_delta(&self) -> (f64, f64) {
        return (
            self.mouse_position.0 - self.prev_mouse_position.0,
            self.mouse_position.1 - self.prev_mouse_position.1,
        );
    }

    // Returns true if the event was handled.
    pub fn update(&mut self, event: &WindowEvent) -> bool {
        self.prev_mouse_position = self.mouse_position.clone();
        self.prev_keys = self.keys.clone();

        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                let key = event.key_without_modifiers();

                info!(
                    "key {0} {1}",
                    key.to_text().unwrap_or("unknown"),
                    event.state == winit::event::ElementState::Pressed
                );

                self.keys
                    .insert(key, event.state == winit::event::ElementState::Pressed);
            }
            WindowEvent::CursorMoved { position, .. } => {
                info!("mouse {0} {1}", position.x, position.y);

                self.mouse_position = (position.x, position.y);
            }
            _ => {
                return false;
            }
        }

        return true;
    }
}
