use crate::simulation::grid;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::{EventPump, Sdl};
use std::collections::vec_deque::{Iter, IterMut};
use std::collections::VecDeque;

pub enum Action {
    GridCellClicked(grid::GridPoint),
    QuitGame,
}

impl Action {
    pub fn from_left_click(x: i32, y: i32) -> Option<Self> {
        todo!()
    }
}

pub struct ActionPump {
    sdl_pump: EventPump,
    actions: VecDeque<Action>,
}

impl ActionPump {
    pub fn new(event_pump: EventPump) -> Self {
        Self {
            sdl_pump: event_pump,
            actions: VecDeque::new(),
        }
    }

    pub fn poll_action(&mut self) -> Option<Action> {
        self.actions.pop_front()
    }

    pub fn poll_iter(&mut self) -> Iter<'_, Action> {
        self.actions.iter()
    }

    pub fn poll_iter_mut(&mut self) -> IterMut<'_, Action> {
        self.actions.iter_mut()
    }

    fn pump_actions(&mut self) {
        for sdl_ev in self.sdl_pump.poll_iter() {
            let action = Self::match_input(sdl_ev);
            if let Some(action) = action {
                self.actions.push_back(action)
            }
        }
    }

    fn match_input(sdl_ev: Event) -> Option<Action> {
        match sdl_ev {
            Event::KeyDown { keycode, .. } if keycode.is_some() => {
                Self::match_key_down(keycode.unwrap())
            }

            Event::MouseButtonDown {
                mouse_btn, x, y, ..
            } => Self::match_button_down(mouse_btn, x, y),

            _ => None,
        }
    }

    fn match_key_down(key: Keycode) -> Option<Action> {
        match key {
            Keycode::Escape => Some(Action::QuitGame),
            _ => None,
        }
    }

    fn match_button_down(button: MouseButton, x: i32, y: i32) -> Option<Action> {
        match button {
            MouseButton::Left => Action::from_left_click(x, y),
            _ => None,
        }
    }
}
