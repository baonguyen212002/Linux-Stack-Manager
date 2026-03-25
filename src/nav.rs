use std::collections::VecDeque;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

static NAV_QUEUE: Mutex<VecDeque<usize>> = Mutex::new(VecDeque::new());
static NAV_MODE: AtomicBool = AtomicBool::new(false);

pub fn set_nav_path(path: Vec<usize>) {
    NAV_MODE.store(true, Ordering::Relaxed);
    *NAV_QUEUE.lock().unwrap() = VecDeque::from(path);
}

/// Lấy số tiếp theo từ nav queue, None nếu hết
pub fn next_nav() -> Option<usize> {
    NAV_QUEUE.lock().unwrap().pop_front()
}

/// Đang ở chế độ quick nav (không hiển thị TUI)
pub fn is_nav_mode() -> bool {
    NAV_MODE.load(Ordering::Relaxed)
}
