mod cli;
mod commands;
mod error;
mod menu;
mod projects;

use clap::Parser;
use cli::{Cli, CliCommand};
use commands::{check_service_status, check_supervisor_status, ngrok_service_action, run_artisan, run_command, run_command_streaming, show_log, show_ngrok_url, supervisor_action, sv_tail, sv_tail_realtime};
use projects::{load_nginx_config, load_ngrok_configs, load_projects};
use error::AppError;

fn main() {
    let args = Cli::parse();

    let result = match args.command {
        Some(cmd) => execute_cli(cmd),
        None => menu::interactive_menu(),
    };

    if let Err(e) = result {
        eprintln!("❌ {}", e);
        std::process::exit(1);
    }
}

fn find_ngrok_config(name: &str) -> error::Result<(String, String)> {
    let configs = load_ngrok_configs();
    let config = configs
        .into_iter()
        .find(|c| c.name == name)
        .ok_or_else(|| AppError::CommandFailed {
            cmd: format!("Không tìm thấy ngrok config: {}. Thêm NGROK_{}=<PORT>:<SERVICE> vào .env", name, name.to_uppercase()),
            code: None,
        })?;
    Ok((config.service, config.port))
}

fn execute_cli(cmd: CliCommand) -> error::Result<()> {
    let nginx = load_nginx_config();
    match cmd {
        CliCommand::Install => {
            println!("🚀 Đang cài đặt Nginx...");
            run_command_streaming("sudo", &["apt", "install", &nginx.service, "-y"])?;
        }
        CliCommand::Status => {
            check_service_status(&nginx.service)?;
        }
        CliCommand::Start => {
            println!("🟢 Đang bật Nginx...");
            run_command("sudo", &["systemctl", "start", &nginx.service])?;
        }
        CliCommand::Restart => {
            println!("🔄 Đang restart Nginx...");
            run_command("sudo", &["systemctl", "restart", &nginx.service])?;
        }
        CliCommand::Reload => {
            println!("🔄 Đang reload Nginx...");
            run_command("sudo", &["systemctl", "reload", &nginx.service])?;
        }
        CliCommand::Stop => {
            println!("🔴 Đang tắt Nginx...");
            run_command("sudo", &["systemctl", "stop", &nginx.service])?;
        }
        CliCommand::TestConfig => {
            println!("🔧 Đang kiểm tra config Nginx...");
            run_command("sudo", &[&nginx.service, "-t"])?;
        }
        CliCommand::Logs { log_type } => {
            let path = match log_type.as_str() {
                "access" => nginx.access_log.as_str(),
                "error" => nginx.error_log.as_str(),
                _ => {
                    return Err(AppError::CommandFailed {
                        cmd: format!("logs {}", log_type),
                        code: None,
                    });
                }
            };
            show_log(path)?;
        }
        CliCommand::Ngrok { service } => {
            let (service_name, _) = find_ngrok_config(&service)?;
            check_service_status(&service_name)?;
        }
        CliCommand::NgrokStart { service } => {
            let (service_name, _) = find_ngrok_config(&service)?;
            ngrok_service_action(&service_name, "start")?;
        }
        CliCommand::NgrokStop { service } => {
            let (service_name, _) = find_ngrok_config(&service)?;
            ngrok_service_action(&service_name, "stop")?;
        }
        CliCommand::NgrokRestart { service } => {
            let (service_name, _) = find_ngrok_config(&service)?;
            ngrok_service_action(&service_name, "restart")?;
        }
        CliCommand::NgrokUrl { service } => {
            let (_, port) = find_ngrok_config(&service)?;
            show_ngrok_url(&service, &port)?;
        }
        CliCommand::SvStatus => {
            check_supervisor_status()?;
        }
        CliCommand::SvStart { process } => {
            supervisor_action(&process, "start")?;
        }
        CliCommand::SvStop { process } => {
            supervisor_action(&process, "stop")?;
        }
        CliCommand::SvRestart { process } => {
            supervisor_action(&process, "restart")?;
        }
        CliCommand::SvReload => {
            println!("🔄 Đang reload Supervisor config...");
            run_command("sudo", &["supervisorctl", "reread"])?;
            run_command("sudo", &["supervisorctl", "update"])?;
        }
        CliCommand::SvLogs { process } => {
            sv_tail(&process)?;
        }
        CliCommand::SvLogsRealtime { process } => {
            sv_tail_realtime(&process)?;
        }
        CliCommand::LaravelCacheClear { project } => {
            run_artisan_cli(&project, "cache:clear")?;
        }
        CliCommand::LaravelConfigClear { project } => {
            run_artisan_cli(&project, "config:clear")?;
        }
        CliCommand::LaravelRouteClear { project } => {
            run_artisan_cli(&project, "route:clear")?;
        }
        CliCommand::LaravelViewClear { project } => {
            run_artisan_cli(&project, "view:clear")?;
        }
        CliCommand::LaravelClearAll { project } => {
            run_artisan_cli(&project, "optimize:clear")?;
        }
        CliCommand::LaravelOptimize { project } => {
            run_artisan_cli(&project, "optimize")?;
        }
    }
    Ok(())
}

fn run_artisan_cli(project_name: &str, artisan_cmd: &str) -> error::Result<()> {
    let projects = load_projects();
    if projects.is_empty() {
        return Err(AppError::CommandFailed {
            cmd: "Chưa cấu hình project nào trong .env".to_string(),
            code: None,
        });
    }

    if project_name == "all" {
        for project in &projects {
            println!("\n📦 [{}]", project.name);
            run_artisan(&project.path, project.user.as_deref(), artisan_cmd)?;
        }
    } else {
        let project = projects
            .iter()
            .find(|p| p.name == project_name)
            .ok_or_else(|| AppError::CommandFailed {
                cmd: format!("Không tìm thấy project: {}", project_name),
                code: None,
            })?;
        run_artisan(&project.path, project.user.as_deref(), artisan_cmd)?;
    }
    Ok(())
}
