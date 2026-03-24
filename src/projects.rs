use std::collections::BTreeMap;
use std::sync::Once;

pub struct Project {
    pub name: String,
    pub user: Option<String>,
    pub path: String,
}

pub struct NgrokConfig {
    pub name: String,
    pub port: String,
    pub service: String,
}

pub struct NginxConfig {
    pub service: String,
    pub access_log: String,
    pub error_log: String,
}

static INIT_ENV: Once = Once::new();

/// Load .env theo thứ tự ưu tiên:
/// 1. ~/.config/lstack/.env
/// 2. Thư mục hiện tại (.env)
fn init_env() {
    INIT_ENV.call_once(|| {
        if let Some(home) = std::env::var_os("HOME") {
            let config_env = std::path::PathBuf::from(home).join(".config/lstack/.env");
            if config_env.exists() {
                let _ = dotenvy::from_path(&config_env);
                return;
            }
        }
        let _ = dotenvy::dotenv();
    });
}

pub fn load_nginx_config() -> NginxConfig {
    init_env();

    NginxConfig {
        service: std::env::var("NGINX_SERVICE").unwrap_or_else(|_| "nginx".to_string()),
        access_log: std::env::var("NGINX_ACCESS_LOG").unwrap_or_else(|_| "/var/log/nginx/access.log".to_string()),
        error_log: std::env::var("NGINX_ERROR_LOG").unwrap_or_else(|_| "/var/log/nginx/error.log".to_string()),
    }
}

pub fn load_ngrok_configs() -> Vec<NgrokConfig> {
    init_env();

    let mut configs = BTreeMap::new();
    for (key, value) in std::env::vars() {
        if let Some(name) = key.strip_prefix("NGROK_") {
            if let Some((port, service)) = value.split_once(':') {
                configs.insert(
                    name.to_lowercase(),
                    NgrokConfig {
                        name: name.to_lowercase(),
                        port: port.to_string(),
                        service: service.to_string(),
                    },
                );
            }
        }
    }

    configs.into_values().collect()
}

pub fn load_projects() -> Vec<Project> {
    init_env();

    let mut projects = BTreeMap::new();
    for (key, value) in std::env::vars() {
        if let Some(name) = key.strip_prefix("PROJECT_") {
            let (user, path) = if let Some((u, p)) = value.split_once(':') {
                if u.contains('/') || u.contains('\\') {
                    (None, value.as_str())
                } else {
                    (Some(u.to_string()), p)
                }
            } else {
                (None, value.as_str())
            };

            projects.insert(
                name.to_lowercase(),
                Project {
                    name: name.to_lowercase(),
                    user,
                    path: path.to_string(),
                },
            );
        }
    }

    projects.into_values().collect()
}
