use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Lệnh thất bại: {cmd} (exit code: {code:?})")]
    CommandFailed { cmd: String, code: Option<i32> },

    #[error("Không thể gọi lệnh: {cmd}")]
    CommandNotFound {
        cmd: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Đã hủy chọn")]
    MenuCancelled,
}

pub type Result<T> = std::result::Result<T, AppError>;
