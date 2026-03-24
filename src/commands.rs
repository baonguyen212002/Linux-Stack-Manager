use std::fmt;
use std::process::{Command, Output};

use crate::error::{AppError, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MenuAction {
    Install,
    Status,
    Start,
    Restart,
    Reload,
    Stop,
    TestConfig,
    Logs,
    NgrokCheck,
    NgrokStart,
    NgrokStop,
    NgrokRestart,
    NgrokUrl,
    SvStatus,
    SvStart,
    SvStop,
    SvRestart,
    SvReload,
    SvLogs,
    LaravelCacheClear,
    LaravelConfigClear,
    LaravelRouteClear,
    LaravelViewClear,
    LaravelOptimizeClear,
    LaravelOptimize,
    Back,
}

impl fmt::Display for MenuAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Install => write!(f, "Cài đặt"),
            Self::Status => write!(f, "Kiểm tra trạng thái"),
            Self::Start => write!(f, "Khởi động"),
            Self::Restart => write!(f, "Restart"),
            Self::Reload => write!(f, "Reload config"),
            Self::Stop => write!(f, "Dừng"),
            Self::TestConfig => write!(f, "Test config"),
            Self::Logs => write!(f, "Xem logs"),
            Self::NgrokCheck => write!(f, "Check service"),
            Self::NgrokStart => write!(f, "Start service"),
            Self::NgrokStop => write!(f, "Stop service"),
            Self::NgrokRestart => write!(f, "Restart service"),
            Self::NgrokUrl => write!(f, "Xem URL"),
            Self::SvStatus => write!(f, "Check status"),
            Self::SvStart => write!(f, "Start process"),
            Self::SvStop => write!(f, "Stop process"),
            Self::SvRestart => write!(f, "Restart process"),
            Self::SvReload => write!(f, "Reload config (reread & update)"),
            Self::SvLogs => write!(f, "Xem logs process"),
            Self::LaravelCacheClear => write!(f, "Xóa cache"),
            Self::LaravelConfigClear => write!(f, "Xóa config cache"),
            Self::LaravelRouteClear => write!(f, "Xóa route cache"),
            Self::LaravelViewClear => write!(f, "Xóa view cache"),
            Self::LaravelOptimizeClear => write!(f, "Xóa tất cả cache"),
            Self::LaravelOptimize => write!(f, "Build cache (optimize)"),
            Self::Back => write!(f, "Quay lại"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NgrokService {
    Gateway,
    Portal,
}

impl fmt::Display for NgrokService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Gateway => write!(f, "gateway"),
            Self::Portal => write!(f, "portal"),
        }
    }
}

impl MenuAction {
    pub fn nginx_actions() -> Vec<Self> {
        vec![
            Self::Install,
            Self::Status,
            Self::Start,
            Self::Restart,
            Self::Reload,
            Self::Stop,
            Self::TestConfig,
            Self::Logs,
        ]
    }

    pub fn supervisor_actions() -> Vec<Self> {
        vec![
            Self::SvStatus,
            Self::SvStart,
            Self::SvStop,
            Self::SvRestart,
            Self::SvReload,
            Self::SvLogs,
        ]
    }

    pub fn laravel_actions() -> Vec<Self> {
        vec![
            Self::LaravelCacheClear,
            Self::LaravelConfigClear,
            Self::LaravelRouteClear,
            Self::LaravelViewClear,
            Self::LaravelOptimizeClear,
            Self::LaravelOptimize,
        ]
    }

    pub fn ngrok_actions() -> Vec<Self> {
        vec![
            Self::NgrokCheck,
            Self::NgrokStart,
            Self::NgrokStop,
            Self::NgrokRestart,
            Self::NgrokUrl,
        ]
    }

    pub fn execute(&self) -> Result<()> {
        match self {
            Self::Install => {
                println!("🚀 Đang cài đặt Nginx...");
                run_command_streaming("sudo", &["apt", "install", "nginx", "-y"])?;
            }
            Self::Status => {
                check_service_status("nginx")?;
            }
            Self::Start => {
                println!("🟢 Đang bật Nginx...");
                run_command("sudo", &["systemctl", "start", "nginx"])?;
            }
            Self::Restart => {
                println!("🔄 Đang restart Nginx...");
                run_command("sudo", &["systemctl", "restart", "nginx"])?;
            }
            Self::Reload => {
                println!("🔄 Đang reload Nginx...");
                run_command("sudo", &["systemctl", "reload", "nginx"])?;
            }
            Self::Stop => {
                println!("🔴 Đang tắt Nginx...");
                run_command("sudo", &["systemctl", "stop", "nginx"])?;
            }
            Self::TestConfig => {
                println!("🔧 Đang kiểm tra config Nginx...");
                run_command("sudo", &["nginx", "-t"])?;
            }
            Self::Logs => {
                logs_submenu()?;
            }
            Self::NgrokCheck => {
                ngrok_submenu("status")?;
            }
            Self::NgrokStart => {
                ngrok_submenu("start")?;
            }
            Self::NgrokStop => {
                ngrok_submenu("stop")?;
            }
            Self::NgrokRestart => {
                ngrok_submenu("restart")?;
            }
            Self::NgrokUrl => {
                ngrok_url_submenu()?;
            }
            Self::SvStatus => {
                check_supervisor_status()?;
            }
            Self::SvStart => {
                supervisor_process_submenu("start")?;
            }
            Self::SvStop => {
                supervisor_process_submenu("stop")?;
            }
            Self::SvRestart => {
                supervisor_process_submenu("restart")?;
            }
            Self::SvReload => {
                println!("🔄 Đang reload Supervisor config...");
                run_command("sudo", &["supervisorctl", "reread"])?;
                run_command("sudo", &["supervisorctl", "update"])?;
            }
            Self::SvLogs => {
                supervisor_logs_submenu()?;
            }
            Self::LaravelCacheClear => {
                run_artisan_on_project("cache:clear")?;
            }
            Self::LaravelConfigClear => {
                run_artisan_on_project("config:clear")?;
            }
            Self::LaravelRouteClear => {
                run_artisan_on_project("route:clear")?;
            }
            Self::LaravelViewClear => {
                run_artisan_on_project("view:clear")?;
            }
            Self::LaravelOptimizeClear => {
                run_artisan_on_project("optimize:clear")?;
            }
            Self::LaravelOptimize => {
                run_artisan_on_project("optimize")?;
            }
            Self::Back => {}
        }
        Ok(())
    }
}

fn ngrok_submenu(action: &str) -> Result<()> {
    use inquire::Select;

    let services = vec![NgrokService::Gateway, NgrokService::Portal];
    let mut options: Vec<String> = services.iter().map(|s| s.to_string()).collect();
    options.push("Quay lại!".to_string());

    let prompt = match action {
        "restart" => "Chọn service Ngrok để restart:",
        "start" => "Chọn service Ngrok để start:",
        "stop" => "Chọn service Ngrok để stop:",
        _ => "Chọn service Ngrok:",
    };

    let choice = Select::new(prompt, options.clone())
        .prompt()
        .map_err(|_| AppError::MenuCancelled)?;

    if choice == "Quay lại!" {
        println!("🔙 Quay về.");
        return Ok(());
    }

    let service_name = format!("{}-ngrok", choice);
    match action {
        "start" => {
            println!("🟢 Đang start service: {}", service_name);
            run_command("sudo", &["systemctl", "start", &service_name])?;
        }
        "stop" => {
            println!("🔴 Đang stop service: {}", service_name);
            run_command("sudo", &["systemctl", "stop", &service_name])?;
        }
        "restart" => {
            println!("🔄 Đang restart service: {}", service_name);
            run_command("sudo", &["systemctl", "restart", &service_name])?;
        }
        _ => {
            check_service_status(&service_name)?;
        }
    }
    Ok(())
}

fn logs_submenu() -> Result<()> {
    use inquire::Select;

    let options = vec!["Access log", "Error log", "Quay lại!"];
    let choice = Select::new("Chọn log để xem:", options)
        .prompt()
        .map_err(|_| AppError::MenuCancelled)?;

    match choice {
        "Access log" => {
            show_log("/var/log/nginx/access.log")?;
        }
        "Error log" => {
            show_log("/var/log/nginx/error.log")?;
        }
        _ => {
            println!("🔙 Quay về.");
        }
    }
    Ok(())
}

pub fn show_log(path: &str) -> Result<()> {
    println!("📄 50 dòng cuối của {}:\n", path);
    run_command("sudo", &["tail", "-n", "50", path])?;
    Ok(())
}

fn ngrok_url_submenu() -> Result<()> {
    use inquire::Select;

    let services = vec![NgrokService::Gateway, NgrokService::Portal];
    let mut options: Vec<String> = services.iter().map(|s| s.to_string()).collect();
    options.push("Quay lại!".to_string());

    let choice = Select::new("Chọn service Ngrok để xem URL:", options.clone())
        .prompt()
        .map_err(|_| AppError::MenuCancelled)?;

    if choice == "Quay lại!" {
        println!("🔙 Quay về.");
        return Ok(());
    }

    show_ngrok_url(&choice)?;
    Ok(())
}

pub fn show_ngrok_url(service: &str) -> Result<()> {
    let port = match service {
        "gateway" => "4040",
        "portal" => "4041",
        _ => "4040",
    };

    let url = format!("http://localhost:{}/api/tunnels", port);
    println!("🔗 Đang lấy URL từ ngrok ({})...\n", service);

    let output = Command::new("curl")
        .args(&["-s", &url])
        .output()
        .map_err(|e| AppError::CommandNotFound {
            cmd: "curl".to_string(),
            source: e,
        })?;

    if !output.status.success() {
        println!("❌ Không thể kết nối tới ngrok API tại {}", url);
        println!("   Đảm bảo ngrok đang chạy cho service: {}", service);
        return Ok(());
    }

    let body = String::from_utf8_lossy(&output.stdout);

    #[derive(serde::Deserialize)]
    struct TunnelsResponse {
        tunnels: Vec<Tunnel>,
    }

    #[derive(serde::Deserialize)]
    struct Tunnel {
        public_url: String,
        proto: String,
        config: TunnelConfig,
    }

    #[derive(serde::Deserialize)]
    struct TunnelConfig {
        addr: String,
    }

    match serde_json::from_str::<TunnelsResponse>(&body) {
        Ok(resp) => {
            if resp.tunnels.is_empty() {
                println!("Không tìm thấy tunnel nào đang active.");
            } else {
                for tunnel in &resp.tunnels {
                    println!(
                        "  {} {} → {}",
                        tunnel.proto, tunnel.public_url, tunnel.config.addr
                    );
                }
            }
        }
        Err(_) => {
            println!("❌ Không thể parse response từ ngrok API.");
            println!("   Raw response: {}", body);
        }
    }
    Ok(())
}

/// Lấy danh sách process names từ `supervisorctl status`
fn get_supervisor_processes() -> Result<Vec<String>> {
    let output = Command::new("sudo")
        .args(&["supervisorctl", "status"])
        .output()
        .map_err(|e| AppError::CommandNotFound {
            cmd: "supervisorctl".to_string(),
            source: e,
        })?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut processes: Vec<String> = stdout
        .lines()
        .filter_map(|line| line.split_whitespace().next())
        .map(|s| s.to_string())
        .collect();

    processes.sort();
    Ok(processes)
}

fn supervisor_process_submenu(action: &str) -> Result<()> {
    use inquire::Select;

    let mut processes = get_supervisor_processes()?;

    if processes.is_empty() {
        println!("Không tìm thấy process nào trong Supervisor.");
        return Ok(());
    }

    processes.insert(0, "all".to_string());
    processes.push("Quay lại!".to_string());

    let prompt = match action {
        "start" => "Chọn process để start:",
        "stop" => "Chọn process để stop:",
        "restart" => "Chọn process để restart:",
        _ => "Chọn process:",
    };

    let choice = Select::new(prompt, processes)
        .prompt()
        .map_err(|_| AppError::MenuCancelled)?;

    if choice == "Quay lại!" {
        println!("🔙 Quay về.");
        return Ok(());
    }

    supervisor_action(&choice, action)?;
    Ok(())
}

fn supervisor_logs_submenu() -> Result<()> {
    use inquire::Select;

    let mut processes = get_supervisor_processes()?;

    if processes.is_empty() {
        println!("Không tìm thấy process nào trong Supervisor.");
        return Ok(());
    }

    processes.push("Quay lại!".to_string());

    let choice = Select::new("Chọn process để xem logs:", processes)
        .prompt()
        .map_err(|_| AppError::MenuCancelled)?;

    if choice == "Quay lại!" {
        println!("🔙 Quay về.");
        return Ok(());
    }

    sv_tail(&choice)?;
    Ok(())
}

pub fn sv_tail(process: &str) -> Result<()> {
    println!("📄 Logs của process: {}\n", process);
    run_command("sudo", &["supervisorctl", "tail", process])?;
    Ok(())
}

pub fn supervisor_action(process: &str, action: &str) -> Result<()> {
    let (icon, verb) = match action {
        "start" => ("🟢", "start"),
        "stop" => ("🔴", "stop"),
        "restart" => ("🔄", "restart"),
        _ => ("🔍", "status"),
    };
    println!("{} Đang {} process: {}", icon, verb, process);
    run_command("sudo", &["supervisorctl", action, process])?;
    Ok(())
}

pub fn ngrok_service_action(service: &str, action: &str) -> Result<()> {
    let service_name = format!("{}-ngrok", service);
    let (icon, verb) = match action {
        "start" => ("🟢", "start"),
        "stop" => ("🔴", "stop"),
        "restart" => ("🔄", "restart"),
        _ => ("🔍", "status"),
    };
    println!("{} Đang {} service: {}", icon, verb, service_name);
    run_command("sudo", &["systemctl", action, &service_name])?;
    Ok(())
}

fn run_artisan_on_project(artisan_cmd: &str) -> Result<()> {
    use inquire::Select;
    use crate::projects::load_projects;

    let projects = load_projects();
    if projects.is_empty() {
        println!("Chưa cấu hình project nào trong .env");
        println!("Thêm PROJECT_<TÊN>=<PATH> vào file .env");
        return Ok(());
    }

    let mut options: Vec<String> = projects.iter().map(|p| p.name.clone()).collect();
    if projects.len() > 1 {
        options.insert(0, "all".to_string());
    }
    options.push("Quay lại!".to_string());

    let choice = Select::new("Chọn project:", options)
        .prompt()
        .map_err(|_| AppError::MenuCancelled)?;

    if choice == "Quay lại!" {
        println!("🔙 Quay về.");
        return Ok(());
    }

    if choice == "all" {
        for project in &projects {
            println!("\n📦 [{}]", project.name);
            run_artisan(&project.path, project.user.as_deref(), artisan_cmd)?;
        }
    } else {
        let project = projects.iter().find(|p| p.name == choice).unwrap();
        run_artisan(&project.path, project.user.as_deref(), artisan_cmd)?;
    }
    Ok(())
}

pub fn run_artisan(path: &str, user: Option<&str>, artisan_cmd: &str) -> Result<()> {
    let artisan_args: Vec<&str> = artisan_cmd.split_whitespace().collect();

    let output = if let Some(u) = user {
        println!("🔧 sudo -u {} php artisan {} ...", u, artisan_cmd);
        let mut cmd_args = vec!["-u", u, "php", "artisan"];
        cmd_args.extend(&artisan_args);
        Command::new("sudo")
            .args(&cmd_args)
            .current_dir(path)
            .output()
    } else {
        println!("🔧 php artisan {} ...", artisan_cmd);
        let mut cmd_args = vec!["artisan"];
        cmd_args.extend(&artisan_args);
        Command::new("php")
            .args(&cmd_args)
            .current_dir(path)
            .output()
    }
    .map_err(|e| AppError::CommandNotFound {
        cmd: "php".to_string(),
        source: e,
    })?;

    if !output.stdout.is_empty() {
        print!("{}", String::from_utf8_lossy(&output.stdout));
    }
    if !output.stderr.is_empty() {
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
    }

    if output.status.success() {
        println!("✅ Thành công!");
    } else {
        println!("❌ Lỗi khi chạy artisan command");
    }
    Ok(())
}

pub fn check_supervisor_status() -> Result<()> {
    let output = Command::new("sudo")
        .args(&["supervisorctl", "status"])
        .output()
        .map_err(|e| AppError::CommandNotFound {
            cmd: "supervisorctl".to_string(),
            source: e,
        })?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.trim().is_empty() {
        println!("Không có process nào trong Supervisor.");
        return Ok(());
    }

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let name = parts[0];
            let status = parts[1];
            let icon = match status {
                "RUNNING" => "🟢",
                "STOPPED" => "⚪",
                "FATAL" => "🔴",
                "STARTING" => "🟡",
                "BACKOFF" => "🟠",
                "EXITED" => "⚫",
                _ => "⚠️",
            };
            let detail = parts[2..].join(" ");
            println!("  {} {} — {} {}", icon, name, status, detail);
        }
    }
    Ok(())
}

pub fn check_service_status(service: &str) -> Result<()> {
    let output = Command::new("systemctl")
        .args(&["is-active", service])
        .output()
        .map_err(|e| AppError::CommandNotFound {
            cmd: "systemctl".to_string(),
            source: e,
        })?;

    let status = String::from_utf8_lossy(&output.stdout).trim().to_string();
    match status.as_str() {
        "active" => println!("🟢 {} đang chạy (active)", service),
        "inactive" => println!("⚪ {} đã dừng (inactive)", service),
        "failed" => println!("🔴 {} bị lỗi (failed)", service),
        other => println!("⚠️  {} trạng thái: {}", service, other),
    }
    Ok(())
}

pub fn run_command(cmd: &str, args: &[&str]) -> Result<Output> {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .map_err(|e| AppError::CommandNotFound {
            cmd: cmd.to_string(),
            source: e,
        })?;

    if !output.stdout.is_empty() {
        print!("{}", String::from_utf8_lossy(&output.stdout));
    }
    if !output.stderr.is_empty() {
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
    }

    if output.status.success() {
        println!("Lệnh chạy thành công!");
        Ok(output)
    } else {
        Err(AppError::CommandFailed {
            cmd: format!("{} {}", cmd, args.join(" ")),
            code: output.status.code(),
        })
    }
}

/// Streaming version cho các lệnh chạy lâu (vd: apt install)
pub fn run_command_streaming(cmd: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(cmd)
        .args(args)
        .status()
        .map_err(|e| AppError::CommandNotFound {
            cmd: cmd.to_string(),
            source: e,
        })?;

    if status.success() {
        println!("Lệnh chạy thành công!");
        Ok(())
    } else {
        Err(AppError::CommandFailed {
            cmd: format!("{} {}", cmd, args.join(" ")),
            code: status.code(),
        })
    }
}
