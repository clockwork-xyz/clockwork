use {
    crate::{execute_pending_tasks, monitor_blocktime},
    std::thread,
};

pub fn process_tasks() {
    let blocktime_receiver = monitor_blocktime();
    for blocktime in blocktime_receiver {
        println!("⏳ Blocktime: {}", blocktime);
        thread::spawn(move || execute_pending_tasks(blocktime));
    }
    process_tasks()
}
