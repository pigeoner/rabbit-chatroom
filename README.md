# rabbit-chatroom

一个没什么用的聊天室

## 技术栈

- **客户端**

  - [electron-vite](https://github.com/alex8088/electron-vite)

- **后端**

  - 待定（大概是 Rust）

## 项目启动

### 1. 客户端
#### 安装依赖

```bash
$ npm install
```

#### 开发环境启动

```bash
$ npm run dev

# For macOS
$ npm run dev:mac
```

#### 构建

```bash
# For windows
$ npm run build:win

# For macOS
$ npm run build:mac

# For Linux
$ npm run build:linux
```

### 2. 后端

- [ ] TODO
#### 启动
确保已安装rust环境与cargo
```
cd server
cargo run --release
```
#### 配置
后端配置文件位于`server/config.toml`，默认配置如下
```toml