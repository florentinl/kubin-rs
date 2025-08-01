mod app;
mod cam;
mod consts;
mod cube;
mod cubie;
mod depth_texture;
mod state;
mod vertex;
use solver::solvers::methods::from_method_name;
use winit::event_loop::EventLoop;

use crate::app::App;

pub fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        println!("Usage: {} <solver>", args[0]);
        return Err(anyhow::anyhow!("Solver argument is required"));
    }

    let solver = from_method_name(&args[1]);
    match solver {
        Ok(solver) => {
            env_logger::init();

            let event_loop = EventLoop::with_user_event().build()?;
            let mut app = App::new(solver);
            event_loop.run_app(&mut app)?;

            Ok(())
        }
        Err(msg) => Err(anyhow::anyhow!(msg)),
    }
}
