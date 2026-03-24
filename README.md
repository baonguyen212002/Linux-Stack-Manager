# lstack - Linux Stack Manager

CLI tool quản lý server Linux: **Nginx**, **Ngrok**, **Supervisor**, **Laravel Cache**.

Hỗ trợ 2 chế độ:
- **Interactive menu** — chạy `lstack` không cần nhớ lệnh
- **Direct CLI** — chạy nhanh `lstack <command>` cho scripting/automation

## Yêu cầu

- Linux (Ubuntu/Debian)
- Quyền sudo
- PHP (cho Laravel cache commands)

## Cài đặt

### Bước 0: Cài các công cụ cần thiết

**0.1.** Cài build tools (gcc, cc linker) — **bắt buộc** để compile Rust:

```bash
sudo apt update && sudo apt install -y build-essential
```

> Nếu hỏi password, gõ mật khẩu rồi nhấn **Enter** (password sẽ không hiện ra trên màn hình, cứ gõ bình thường).

### Bước 1: Cài Rust (nếu chưa có)

> Nếu đã có Rust, bỏ qua bước này. Kiểm tra bằng lệnh `cargo --version`.

**1.1.** Mở terminal, copy và paste **toàn bộ** lệnh dưới đây, rồi nhấn **Enter**:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**1.2.** Đợi vài giây, màn hình sẽ hiện ra nhiều dòng chữ. Kéo xuống cuối sẽ thấy:

```
1) Proceed with standard installation (default - just press enter)
2) Customize installation
3) Cancel installation
>
```

**1.3.** Nhấn **Enter** (không cần gõ gì). Hệ thống sẽ bắt đầu tải và cài Rust.

**1.4.** Đợi khoảng 1-2 phút. Khi cài xong sẽ hiện:

```
Rust is installed now. Great!

To get started you may need to restart your current shell.
```

**1.5.** Copy và paste lệnh sau, nhấn **Enter**:

```bash
source "$HOME/.cargo/env"
```

> Lệnh này giúp terminal hiện tại nhận diện được Rust. Không có output gì là bình thường.

**1.6.** Kiểm tra cài thành công. Gõ lệnh:

```bash
cargo --version
```

Nếu hiện ra kiểu `cargo 1.85.0` (số phiên bản có thể khác) là **thành công**.

Nếu hiện `command not found` → đóng terminal, mở lại terminal mới, rồi thử lại `cargo --version`.

---

### Bước 2: Tải project về

**2.1.** Copy và paste **toàn bộ 2 dòng** lệnh dưới đây, nhấn **Enter**:

```bash
git clone https://github.com/baonguyen212002/Linux-Stack-Manager.git
cd Linux-Stack-Manager
```

> Dòng 1 tải project về máy. Dòng 2 di chuyển vào thư mục project.

Nếu thấy `fatal: destination path 'Linux-Stack-Manager' already exists` → project đã tải trước đó rồi. Chỉ cần chạy `cd Linux-Stack-Manager`.

---

### Bước 3: Cấu hình Laravel projects

**3.1.** Tạo file `.env` từ file mẫu:

```bash
cp .env.example .env
```

> Lệnh này copy file `.env.example` thành file `.env`. Không có output gì là bình thường.

**3.2.** Mở file `.env` để chỉnh sửa:

```bash
nano .env
```

> `nano` là trình soạn thảo trong terminal. Nếu chưa có, cài bằng `sudo apt install nano`.

**3.3.** Thêm các Laravel projects vào file. Mỗi project 1 dòng, format:

```env
PROJECT_<TÊN>=<USER>:<ĐƯỜNG DẪN>
```

Ví dụ:

```env
PROJECT_GATEWAY=gatewayjcEz:/home/gatewayjcEz/local-gpay-gateway.glodival-sandbox.local/public_html
PROJECT_PORTAL=localgp39nV:/home/localgp39nV/portal-api.gpay.local/public_html
```

> **Cách xác định USER:**
>
> Vào thư mục project, chạy:
> ```bash
> ls -la storage/framework/cache/data/
> ```
>
> Nhìn cột thứ 3 (owner). Ví dụ:
> ```
> drwxr-sr-x 3 localgp39nV devs 4096 Mar 23 13:18 00
>                ^^^^^^^^^^^
>                Đây là USER
> ```
>
> - Nếu owner **khác** tên đăng nhập của bạn → thêm `USER:` trước path
>   ```
>   PROJECT_PORTAL=localgp39nV:/home/localgp39nV/.../public_html
>   ```
> - Nếu owner **là** tên đăng nhập của bạn → không cần USER, chỉ cần path
>   ```
>   PROJECT_MYAPP=/home/shino/my-project
>   ```
>
> Không biết tên đăng nhập? Gõ `whoami` trong terminal.

**3.4.** Lưu file và thoát nano:
- Nhấn **Ctrl + O** (chữ O, không phải số 0) → nhấn **Enter** để lưu
- Nhấn **Ctrl + X** để thoát nano

---

### Bước 4: Build và cài đặt

**4.1.** Chạy lệnh sau để build và cài:

```bash
cargo install --path .
```

> Lần đầu sẽ tải dependencies và compile, mất khoảng 1-3 phút. Khi xong sẽ hiện:
> ```
> Installing /home/<user>/.cargo/bin/lstack
> Installed package `lstack v0.1.0` (executable `lstack`)
> ```

**4.2.** Kiểm tra cài thành công:

```bash
lstack --help
```

Nếu hiện ra danh sách commands là **thành công**.

Nếu hiện `command not found` → chạy `source "$HOME/.cargo/env"` rồi thử lại.

---

### Bước 5 (Optional): Thêm alias build nhanh

Khi có bản cập nhật mới, bạn cần pull code và build lại. Alias giúp làm điều này bằng 1 lệnh.

**5.1.** Chạy lệnh sau (**copy toàn bộ 2 dòng**):

```bash
echo 'alias lstack-build="cd ~/Linux-Stack-Manager && git pull && cargo clean -p lstack && cargo install --path ."' >> ~/.bashrc
source ~/.bashrc
```

**5.2.** Từ giờ khi cần cập nhật, chỉ cần gõ:

```bash
lstack-build
```

---

## Sử dụng

### Interactive menu

Gõ `lstack` rồi nhấn **Enter**:

```bash
lstack
```

Dùng **phím mũi tên lên/xuống** để di chuyển, nhấn **Enter** để chọn:

```
> Bạn muốn làm gì?
  Quản lý Nginx
  Quản lý Ngrok
  Quản lý Supervisor
  Quản lý Laravel Cache
  Thoát
```

Chọn vào từng mục sẽ hiện sub-menu với các chức năng chi tiết.

Nhấn **Esc** hoặc **Ctrl + C** để thoát bất cứ lúc nào.

### CLI commands (chạy trực tiếp)

> Dùng khi muốn chạy nhanh mà không cần vào menu.

#### Nginx

```bash
lstack install          # Cài đặt Nginx
lstack status           # Kiểm tra trạng thái (đang chạy hay không)
lstack start            # Khởi động
lstack restart          # Restart
lstack reload           # Reload config (không downtime)
lstack stop             # Dừng
lstack test-config      # Test config (nginx -t)
lstack logs access      # Xem 50 dòng cuối access log
lstack logs error       # Xem 50 dòng cuối error log
```

#### Ngrok

```bash
lstack ngrok gateway          # Check status
lstack ngrok-start gateway    # Start service
lstack ngrok-stop gateway     # Stop service
lstack ngrok-restart gateway  # Restart service
lstack ngrok-url gateway      # Xem public URL
```

> Thay `gateway` bằng `portal` cho portal service.

#### Supervisor

```bash
lstack sv-status                # Check tất cả process
lstack sv-start <process|all>   # Start process
lstack sv-stop <process|all>    # Stop process
lstack sv-restart <process|all> # Restart process
lstack sv-reload                # Reload config (reread & update)
lstack sv-logs <process>        # Xem logs process
```

> `<process>` là tên process từ `supervisorctl status`, ví dụ: `gateway-worker:gateway-worker_00`. Gõ `all` để thao tác tất cả.

#### Laravel Cache

```bash
lstack laravel-cache-clear <project|all>   # Xóa cache
lstack laravel-config-clear <project|all>  # Xóa config cache
lstack laravel-route-clear <project|all>   # Xóa route cache
lstack laravel-view-clear <project|all>    # Xóa view cache
lstack laravel-clear-all <project|all>     # Xóa tất cả cache
lstack laravel-optimize <project|all>      # Build cache (optimize)
```

> `<project>` là tên project trong `.env` (viết thường), ví dụ: `gateway`, `portal`.
> Gõ `all` để chạy cho tất cả projects.
