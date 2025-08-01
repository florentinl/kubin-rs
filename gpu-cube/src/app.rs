use std::sync::Arc;

use solver::solvers::methods::Methods;
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    window::Window,
};

use crate::state::State;

pub struct App {
    state: Option<State>,
    solver: Option<Methods>,
}

impl App {
    pub fn new(solver: Methods) -> Self {
        App {
            state: None,
            solver: Some(solver),
        }
    }
}

impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes();
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        let solver = self
            .solver
            .take()
            .expect("Solver must be set before resumed");
        self.state = Some(pollster::block_on(State::new(window, solver)).unwrap());
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: State) {
        self.state = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let Some(state) = &mut self.state else {
            return;
        };

        state.input(&event);

        if event == WindowEvent::CloseRequested {
            event_loop.exit();
        }
    }
}
