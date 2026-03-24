# lstack - Linux Stack Manager

CLI tool quản lý server Linux: **Nginx**, **Ngrok**, **Supervisor**, **Laravel Cache**.

Hỗ trợ 2 chế độ:
- **Interactive menu** — chạy `lstack` không cần nhớ lệnh
- **Direct CLI** — chạy nhanh `lstack <command>` cho scripting/automation

## Yêu cầu

- Rust 1.85+ (edition 2024)
- Linux (systemctl, supervisorctl)
- PHP (cho Laravel cache commands)

## Cài đặt

```bash
git clone https://github.com/baonguyen212002/Linux-Stack-Manager.git
cd Linux-Stack-Manager
```

### 1. Cấu hình Laravel projects

```bash
cp .env.example .env
```

Mở `.env`, thêm Laravel projects:

```env
# Project tạo từ HostVN (user riêng):
PROJECT_GATEWAY=gatewayjcEz:/home/gatewayjcEz/local-gpay-gateway.glodival-sandbox.local/public_html

# Project setup thủ công (cache owner khác user hiện tại):
PROJECT_PORTAL=localgp39nV:/home/localgp39nV/portal-api.gpay.local/public_html

# Project mà bạn là owner (không cần user):
PROJECT_MYAPP=/home/shino/my-project
```

**Cách xác định có cần thêm USER hay không:**

```bash
cd /path/to/project
ls -la storage/framework/cache/data/
```

- Cache files owner **khác** user hiện tại → thêm `USER:` trước path
- Cache files owner **là** user hiện tại → chỉ cần path

Xem chi tiết trong `.env.example`.

### 2. Build và cài đặt

```bash
cargo install --path .
```

Binary sẽ được cài vào `~/.cargo/bin/lstack`.

### 3. (Optional) Thêm alias build nhanh

```bash
echo 'alias lstack-build="cd ~/Linux-Stack-Manager && cargo clean -p lstack && cargo install --path ."' >> ~/.bashrc
source ~/.bashrc
```

Sau này chỉ cần chạy `lstack-build` để cập nhật.

## Sử dụng

### Interactive menu

```bash
lstack
```

```
> Bạn muốn làm gì?
  Quản lý Nginx
  Quản lý Ngrok
  Quản lý Supervisor
  Quản lý Laravel Cache
  Thoát
```

### CLI commands

#### Nginx

```bash
lstack install          # Cài đặt Nginx
lstack status           # Kiểm tra trạng thái
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

Thay `gateway` bằng `portal` cho portal service.

#### Supervisor

```bash
lstack sv-status                # Check tất cả process
lstack sv-start <process|all>   # Start process
lstack sv-stop <process|all>    # Stop process
lstack sv-restart <process|all> # Restart process
lstack sv-reload                # Reload config (reread & update)
lstack sv-logs <process>        # Xem logs process
```

#### Laravel Cache

```bash
lstack laravel-cache-clear <project|all>   # Xóa cache
lstack laravel-config-clear <project|all>  # Xóa config cache
lstack laravel-route-clear <project|all>   # Xóa route cache
lstack laravel-view-clear <project|all>    # Xóa view cache
lstack laravel-clear-all <project|all>     # Xóa tất cả cache
lstack laravel-optimize <project|all>      # Build cache (optimize)
```

`<project>` là tên project trong `.env` (viết thường), ví dụ: `gateway`, `portal`.
