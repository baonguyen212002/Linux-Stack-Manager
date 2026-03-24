use std::io::{self, Write};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal;

use crate::commands::MenuAction;
use crate::error::{AppError, Result};

const VERSION: &str = env!("CARGO_PKG_VERSION");

const RESET: &str = "\x1B[0m";
const BOLD: &str = "\x1B[1m";
const DIM: &str = "\x1B[2m";
const CYAN: &str = "\x1B[36m";
const GREEN: &str = "\x1B[32m";
const YELLOW: &str = "\x1B[33m";
const RED: &str = "\x1B[31m";
const MAGENTA: &str = "\x1B[35m";

#[derive(Debug, Clone, Copy, PartialEq)]
enum MainMenu {
    Nginx,
    Ngrok,
    Supervisor,
    Laravel,
}

impl MainMenu {
    fn icon(&self) -> &str {
        match self {
            Self::Nginx => "🌐",
            Self::Ngrok => "🔗",
            Self::Supervisor => "⚙️ ",
            Self::Laravel => "🧹",
        }
    }
}

impl std::fmt::Display for MainMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nginx => write!(f, "Quan ly Nginx"),
            Self::Ngrok => write!(f, "Quan ly Ngrok"),
            Self::Supervisor => write!(f, "Quan ly Supervisor"),
            Self::Laravel => write!(f, "Quan ly Laravel Cache"),
        }
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().ok();
}

fn enter_alt_screen() {
    print!("\x1B[?1049h\x1B[2J\x1B[1;1H");
    io::stdout().flush().ok();
}

fn leave_alt_screen() {
    print!("\x1B[?1049l");
    io::stdout().flush().ok();
}

fn show_header() {
    println!();
    println!("  {BOLD}{CYAN}██╗     ███████╗████████╗ █████╗  ██████╗██╗  ██╗{RESET}");
    println!("  {BOLD}{CYAN}██║     ██╔════╝╚══██╔══╝██╔══██╗██╔════╝██║ ██╔╝{RESET}");
    println!("  {BOLD}{CYAN}██║     ███████╗   ██║   ███████║██║     █████╔╝ {RESET}");
    println!("  {BOLD}{CYAN}██║     ╚════██║   ██║   ██╔══██║██║     ██╔═██╗ {RESET}");
    println!("  {BOLD}{CYAN}███████╗███████║   ██║   ██║  ██║╚██████╗██║  ██╗{RESET}");
    println!("  {BOLD}{CYAN}╚══════╝╚══════╝   ╚═╝   ╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝{RESET}");
    println!();
    println!("  {CYAN}{BOLD}Linux Stack Manager{RESET} {DIM}v{VERSION}{RESET}");
    println!("  {DIM}Nginx · Ngrok · Supervisor · Laravel{RESET}");
    println!();
    println!("  {DIM}╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌{RESET}");
    println!();
}

/// Kết quả đọc input
enum InputResult {
    Number(usize),
    Escape,
    Invalid,
}

/// Đọc input từ user, hỗ trợ Esc để thoát
fn read_input(prompt: &str) -> InputResult {
    print!("{prompt}");
    io::stdout().flush().ok();

    // Bật raw mode để bắt Esc ngay lập tức
    let _ = terminal::enable_raw_mode();

    let mut buf = String::new();
    loop {
        if let Ok(Event::Key(KeyEvent { code, modifiers, .. })) = event::read() {
            match code {
                KeyCode::Esc => {
                    let _ = terminal::disable_raw_mode();
                    println!();
                    return InputResult::Escape;
                }
                KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
                    let _ = terminal::disable_raw_mode();
                    println!();
                    return InputResult::Escape;
                }
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    buf.push(c);
                    print!("{c}");
                    io::stdout().flush().ok();
                }
                KeyCode::Backspace => {
                    if !buf.is_empty() {
                        buf.pop();
                        print!("\x08 \x08");
                        io::stdout().flush().ok();
                    }
                }
                KeyCode::Enter => {
                    let _ = terminal::disable_raw_mode();
                    println!();
                    return match buf.parse::<usize>() {
                        Ok(n) => InputResult::Number(n),
                        Err(_) => InputResult::Invalid,
                    };
                }
                _ => {}
            }
        }
    }
}

fn prompt_menu(options: &[MainMenu]) -> Result<Option<usize>> {
    for (i, opt) in options.iter().enumerate() {
        let num = i + 1;
        println!("  {GREEN}{BOLD}{num}{RESET}{DIM}.{RESET} {} {}", opt.icon(), opt);
    }
    println!();
    println!("  {DIM}╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌{RESET}");
    println!();

    match read_input(&format!("  {YELLOW}➜{RESET} Nhap lua chon {DIM}[0=Thoat, Esc=Thoat]{RESET}: ")) {
        InputResult::Escape => Ok(None),
        InputResult::Number(0) => Ok(None),
        InputResult::Number(n) if n <= options.len() => Ok(Some(n - 1)),
        _ => {
            println!("  {RED}✗ Lua chon khong hop le!{RESET}");
            Err(AppError::MenuCancelled)
        }
    }
}

fn prompt_submenu<T: std::fmt::Display>(title: &str, options: &[T]) -> Result<Option<usize>> {
    println!("  {MAGENTA}{BOLD}▸ {title}{RESET}");
    println!("  {MAGENTA}{}{RESET}", "─".repeat(title.len() + 2));
    println!();
    for (i, opt) in options.iter().enumerate() {
        let num = i + 1;
        println!("    {GREEN}{BOLD}{num}{RESET}{DIM}.{RESET} {opt}");
    }
    println!();

    match read_input(&format!("  {YELLOW}➜{RESET} Nhap lua chon {DIM}[0=Quay lai, Esc=Thoat]{RESET}: ")) {
        InputResult::Escape => Err(AppError::MenuCancelled), // Esc trong submenu = thoát hẳn
        InputResult::Number(0) => Ok(None),
        InputResult::Number(n) if n <= options.len() => Ok(Some(n - 1)),
        _ => {
            println!("  {RED}✗ Lua chon khong hop le!{RESET}");
            Err(AppError::MenuCancelled)
        }
    }
}

fn pause() {
    print!("\n  {DIM}Nhan Enter de tiep tuc (Esc = Thoat)...{RESET}");
    io::stdout().flush().ok();

    let _ = terminal::enable_raw_mode();
    loop {
        if let Ok(Event::Key(KeyEvent { code, modifiers, .. })) = event::read() {
            match code {
                KeyCode::Enter => {
                    let _ = terminal::disable_raw_mode();
                    println!();
                    return;
                }
                KeyCode::Esc => {
                    let _ = terminal::disable_raw_mode();
                    leave_alt_screen();
                    println!("{GREEN}✔ Tam biet!{RESET}");
                    std::process::exit(0);
                }
                KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
                    let _ = terminal::disable_raw_mode();
                    leave_alt_screen();
                    println!("{GREEN}✔ Tam biet!{RESET}");
                    std::process::exit(0);
                }
                _ => {}
            }
        }
    }
}

pub fn interactive_menu() -> Result<()> {
    enter_alt_screen();

    let options = vec![
        MainMenu::Nginx,
        MainMenu::Ngrok,
        MainMenu::Supervisor,
        MainMenu::Laravel,
    ];

    loop {
        clear_screen();
        show_header();

        let idx = match prompt_menu(&options) {
            Ok(Some(i)) => i,
            Ok(None) => {
                leave_alt_screen();
                println!("{GREEN}✔ Tam biet!{RESET}");
                break;
            }
            Err(_) => continue,
        };

        clear_screen();
        show_header();

        let result = match options[idx] {
            MainMenu::Nginx => nginx_submenu(),
            MainMenu::Ngrok => ngrok_submenu(),
            MainMenu::Supervisor => supervisor_submenu(),
            MainMenu::Laravel => laravel_submenu(),
        };

        // Nếu submenu trả Err (Esc) → thoát hẳn
        if let Err(AppError::MenuCancelled) = &result {
            leave_alt_screen();
            println!("{GREEN}✔ Tam biet!{RESET}");
            break;
        }
        result?;
    }
    Ok(())
}


fn nginx_submenu() -> Result<()> {
    let options = MenuAction::nginx_actions();
    run_submenu("Quan ly Nginx", options)
}

fn supervisor_submenu() -> Result<()> {
    let options = MenuAction::supervisor_actions();
    run_submenu("Quan ly Supervisor", options)
}

fn laravel_submenu() -> Result<()> {
    let options = MenuAction::laravel_actions();
    run_submenu("Quan ly Laravel Cache", options)
}

fn ngrok_submenu() -> Result<()> {
    let options = MenuAction::ngrok_actions();
    run_submenu("Quan ly Ngrok", options)
}

fn run_submenu(title: &str, options: Vec<MenuAction>) -> Result<()> {
    loop {
        let idx = match prompt_submenu(title, &options) {
            Ok(Some(i)) => i,
            Ok(None) => return Ok(()),  // 0 = quay lại menu chính
            Err(e) => return Err(e),     // Esc = thoát hẳn (bubble up)
        };

        let choice = options[idx];
        println!();
        if let Err(e) = choice.execute() {
            eprintln!("  {RED}✗ {e}{RESET}");
        }

        pause();
        clear_screen();
        show_header();
    }
}
