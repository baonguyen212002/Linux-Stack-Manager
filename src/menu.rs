use inquire::Select;

use crate::commands::MenuAction;
use crate::error::{AppError, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
enum MainMenu {
    Nginx,
    Ngrok,
    Supervisor,
    Laravel,
    Quit,
}

impl std::fmt::Display for MainMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nginx => write!(f, "Quản lý Nginx"),
            Self::Ngrok => write!(f, "Quản lý Ngrok"),
            Self::Supervisor => write!(f, "Quản lý Supervisor"),
            Self::Laravel => write!(f, "Quản lý Laravel Cache"),
            Self::Quit => write!(f, "Thoát"),
        }
    }
}

pub fn interactive_menu() -> Result<()> {
    loop {
        let options = vec![MainMenu::Nginx, MainMenu::Ngrok, MainMenu::Supervisor, MainMenu::Laravel, MainMenu::Quit];
        let choice = Select::new("Bạn muốn làm gì?", options)
            .prompt()
            .map_err(|_| AppError::MenuCancelled)?;

        match choice {
            MainMenu::Nginx => nginx_submenu()?,
            MainMenu::Ngrok => ngrok_submenu()?,
            MainMenu::Supervisor => supervisor_submenu()?,
            MainMenu::Laravel => laravel_submenu()?,
            MainMenu::Quit => {
                println!("Tạm biệt!");
                break;
            }
        }

        println!("\n---------------------------------\n");
    }
    Ok(())
}

fn nginx_submenu() -> Result<()> {
    let options = MenuAction::nginx_actions();
    run_submenu("── Quản lý Nginx ──", options)
}

fn supervisor_submenu() -> Result<()> {
    let options = MenuAction::supervisor_actions();
    run_submenu("── Quản lý Supervisor ──", options)
}

fn laravel_submenu() -> Result<()> {
    let options = MenuAction::laravel_actions();
    run_submenu("── Quản lý Laravel Cache ──", options)
}

fn ngrok_submenu() -> Result<()> {
    let options = MenuAction::ngrok_actions();
    run_submenu("── Quản lý Ngrok ──", options)
}

fn run_submenu(title: &str, mut options: Vec<MenuAction>) -> Result<()> {
    options.push(MenuAction::Back);
    loop {
        let choice = Select::new(title, options.clone())
            .prompt()
            .map_err(|_| AppError::MenuCancelled)?;

        if choice == MenuAction::Back {
            return Ok(());
        }

        if let Err(e) = choice.execute() {
            eprintln!("❌ {}", e);
        }

        println!();
    }
}
