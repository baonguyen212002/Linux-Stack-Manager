use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "lstack", about = "Linux Stack Manager — Quản lý Nginx, Ngrok, Supervisor, Laravel")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<CliCommand>,
}

#[derive(Subcommand)]
pub enum CliCommand {
    /// Cài đặt Nginx
    Install,
    /// Kiểm tra trạng thái Nginx
    Status,
    /// Khởi động Nginx
    Start,
    /// Restart Nginx
    Restart,
    /// Reload Nginx (không downtime)
    Reload,
    /// Dừng Nginx
    Stop,
    /// Test config Nginx (nginx -t)
    TestConfig,
    /// Xem logs Nginx
    Logs {
        /// Loại log: access hoặc error
        log_type: String,
    },
    /// Kiểm tra service ngrok
    Ngrok {
        /// Tên service: gateway hoặc portal
        service: String,
    },
    /// Start service ngrok
    NgrokStart {
        /// Tên service: gateway hoặc portal
        service: String,
    },
    /// Stop service ngrok
    NgrokStop {
        /// Tên service: gateway hoặc portal
        service: String,
    },
    /// Restart service ngrok
    NgrokRestart {
        /// Tên service: gateway hoặc portal
        service: String,
    },
    /// Xem ngrok public URL
    NgrokUrl {
        /// Tên service: gateway hoặc portal
        service: String,
    },
    /// Kiểm tra status tất cả process trong Supervisor
    SvStatus,
    /// Start process trong Supervisor
    SvStart {
        /// Tên process (hoặc "all")
        process: String,
    },
    /// Stop process trong Supervisor
    SvStop {
        /// Tên process (hoặc "all")
        process: String,
    },
    /// Restart process trong Supervisor
    SvRestart {
        /// Tên process (hoặc "all")
        process: String,
    },
    /// Reload Supervisor config (reread & update)
    SvReload,
    /// Xem logs process trong Supervisor
    SvLogs {
        /// Tên process
        process: String,
    },
    /// Xóa Laravel cache (cache:clear)
    LaravelCacheClear {
        /// Tên project (từ .env) hoặc "all"
        project: String,
    },
    /// Xóa Laravel config cache
    LaravelConfigClear {
        /// Tên project hoặc "all"
        project: String,
    },
    /// Xóa Laravel route cache
    LaravelRouteClear {
        /// Tên project hoặc "all"
        project: String,
    },
    /// Xóa Laravel view cache
    LaravelViewClear {
        /// Tên project hoặc "all"
        project: String,
    },
    /// Xóa tất cả Laravel cache (optimize:clear)
    LaravelClearAll {
        /// Tên project hoặc "all"
        project: String,
    },
    /// Build Laravel cache (optimize)
    LaravelOptimize {
        /// Tên project hoặc "all"
        project: String,
    },
}
