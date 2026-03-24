use std::collections::BTreeMap;

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

/// Đọc cấu hình Nginx từ .env
/// NGINX_SERVICE=nginx
/// NGINX_ACCESS_LOG=/var/log/nginx/access.log
/// NGINX_ERROR_LOG=/var/log/nginx/error.log
pub fn load_nginx_config() -> NginxConfig {
    let _ = dotenvy::dotenv();

    NginxConfig {
        service: std::env::var("NGINX_SERVICE").unwrap_or_else(|_| "nginx".to_string()),
        access_log: std::env::var("NGINX_ACCESS_LOG").unwrap_or_else(|_| "/var/log/nginx/access.log".to_string()),
        error_log: std::env::var("NGINX_ERROR_LOG").unwrap_or_else(|_| "/var/log/nginx/error.log".to_string()),
    }
}

/// Đọc danh sách ngrok services từ .env (biến có prefix NGROK_)
/// Format: NGROK_<TÊN>=<PORT>:<SERVICE_NAME>
/// Ví dụ: NGROK_GATEWAY=4040:gateway-ngrok
pub fn load_ngrok_configs() -> Vec<NgrokConfig> {
    let _ = dotenvy::dotenv();

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

/// Đọc danh sách projects từ .env (biến có prefix PROJECT_)
/// Format: PROJECT_<TÊN>=<PATH> hoặc PROJECT_<TÊN>=<USER>:<PATH>
pub fn load_projects() -> Vec<Project> {
    let _ = dotenvy::dotenv();

    let mut projects = BTreeMap::new();
    for (key, value) in std::env::vars() {
        if let Some(name) = key.strip_prefix("PROJECT_") {
            let (user, path) = if let Some((u, p)) = value.split_once(':') {
                // Kiểm tra xem phần đầu có phải user hay là path (vd: C:\... hoặc /...)
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
