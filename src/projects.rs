use std::collections::BTreeMap;

pub struct Project {
    pub name: String,
    pub user: Option<String>,
    pub path: String,
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
