use axlog::info;
use axtask::init_scheduler;

pub fn init_loader_lib() {
    info!("[Loader_lib]: Init");

    init_scheduler();

    info!("[Loader_lib]: Fini");
}
