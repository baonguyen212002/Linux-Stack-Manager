mod cli;
mod commands;
mod error;
mod menu;
mod projects;

use clap::Parser;
use cli::{Cli, CliCommand};
use commands::{check_service_status, check_supervisor_status, ngrok_service_action, run_artisan, run_command, run_command_streaming, show_log, show_ngrok_url, supervisor_action, sv_tail};
use projects::load_projects;
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

fn validate_ngrok_service(service: &str) -> error::Result<()> {
    let valid = ["gateway", "portal"];
    if !valid.contains(&service) {
        return Err(AppError::CommandFailed {
            cmd: format!("ngrok {}", service),
            code: None,
        });
    }
    Ok(())
}

fn execute_cli(cmd: CliCommand) -> error::Result<()> {
    match cmd {
        CliCommand::Install => {
            println!("🚀 Đang cài đặt Nginx...");
            run_command_streaming("sudo", &["apt", "install", "nginx", "-y"])?;
        }
        CliCommand::Status => {
            check_service_status("nginx")?;
        }
        CliCommand::Start => {
            println!("🟢 Đang bật Nginx...");
            run_command("sudo", &["systemctl", "start", "nginx"])?;
        }
        CliCommand::Restart => {
            println!("🔄 Đang restart Nginx...");
            run_command("sudo", &["systemctl", "restart", "nginx"])?;
        }
        CliCommand::Reload => {
            println!("🔄 Đang reload Nginx...");
            run_command("sudo", &["systemctl", "reload", "nginx"])?;
        }
        CliCommand::Stop => {
            println!("🔴 Đang tắt Nginx...");
            run_command("sudo", &["systemctl", "stop", "nginx"])?;
        }
        CliCommand::TestConfig => {
            println!("🔧 Đang kiểm tra config Nginx...");
            run_command("sudo", &["nginx", "-t"])?;
        }
        CliCommand::Logs { log_type } => {
            let path = match log_type.as_str() {
                "access" => "/var/log/nginx/access.log",
                "error" => "/var/log/nginx/error.log",
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
            validate_ngrok_service(&service)?;
            let service_name = format!("{}-ngrok", service);
            check_service_status(&service_name)?;
        }
        CliCommand::NgrokStart { service } => {
            validate_ngrok_service(&service)?;
            ngrok_service_action(&service, "start")?;
        }
        CliCommand::NgrokStop { service } => {
            validate_ngrok_service(&service)?;
            ngrok_service_action(&service, "stop")?;
        }
        CliCommand::NgrokRestart { service } => {
            validate_ngrok_service(&service)?;
            ngrok_service_action(&service, "restart")?;
        }
        CliCommand::NgrokUrl { service } => {
            validate_ngrok_service(&service)?;
            show_ngrok_url(&service)?;
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
