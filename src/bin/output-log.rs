use log::{info, warn, trace};

fn main() {
    env_logger::init();
    info!("Starting up");
    warn!("oops, nothing implemented!");
    trace!("trace testing!");
}
