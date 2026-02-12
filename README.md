# Teammate - TODO CLI 工具

一个强大的代码 TODO 管理工具，帮助你追踪、管理和完成项目中的待办事项。

## 功能特性

- **多语言支持** - 自动扫描 Rust、Python、JavaScript/TypeScript 代码中的 TODO
- **灵活的标签系统** - 使用标签组织和分类 TODO
- **优先级管理** - 支持低/中/高优先级
- **状态追踪** - 跟踪 TODO 的状态变化（Open / In Progress / Resolved）
- **Git 集成** - 查看 TODO 的 Git blame 信息
- **分支管理** - 比较不同分支之间的 TODO 差异
- **Git Hooks** - 自动扫描和提醒待处理的 TODO
- **多种输出格式** - 支持 Table、JSON、Compact、CSV 格式
- **本地存储** - 使用 SQLite 数据库存储，数据安全可控

## 安装

### 方式一：使用安装脚本

```bash
# 克隆项目
git clone https://github.com/illli-studio/teammate.git
cd teammate

# 运行安装脚本
./install.sh

# 添加到 PATH（如果需要）
export PATH="$HOME/.local/bin:$PATH"
```

### 方式二：手动安装

```bash
# 编译项目
cargo build --release

# 复制二进制文件
cp target/release/teammate /usr/local/bin/

# 或添加到 PATH
export PATH="/path/to/teammate:$PATH"
```

### 方式三：下载预编译版本

访问 [Releases](https://github.com/illli-studio/teammate/releases) 页面下载预编译的二进制文件。

## 使用方法

### 基本命令

```bash
# 查看帮助
teammate --help

# 扫描代码库中的 TODO
teammate scan

# 扫描指定路径
teammate scan ./src

# 列出所有 TODO
teammate list

# 过滤显示
teammate list --open          # 只显示未解决的 TODO
teammate list --tag bug      # 按标签过滤
teammate list --priority high # 按优先级过滤
teammate list --author "John" # 按作者过滤
```

### 添加 TODO

```bash
# 添加新 TODO
teammate add "修复登录bug"

# 添加带标签和优先级
teammate add "优化性能" --tag performance --priority high

# 关联文件和行号
teammate add "重构这个函数" --file src/main.rs --line 42
```

### 更新 TODO

```bash
# 更新状态
teammate status <id> in-progress

# 更新内容
teammate update <id> --content "新的描述"

# 更新优先级
teammate update <id> --priority low

# 删除 TODO
teammate remove <id>
```

### 标签管理

```bash
# 为 TODO 添加标签
teammate tag <id> --add bug

# 移除标签
teammate tag <id> --remove docs

# 列出所有标签
teammate tag --list
```

### 查看统计

```bash
# 基本统计
teammate stats

# 按标签分组
teammate stats --by-tag

# 按优先级分组
teammate stats --by-priority

# 按文件分组
teammate stats --by-file
```

### Git 集成

```bash
# 查看 TODO 的 Git blame
teammate blame <id>

# 分支 TODO 管理
teammate branch --list       # 列出分支 TODO
teammate branch --compare feature # 比较分支

# 安装 Git hooks
teammate hooks --install     # 自动扫描 TODO
teammate hooks --list       # 查看已安装的 hooks
```

## 命令速查

| 命令 | 别名 | 描述 |
|------|------|------|
| `teammate scan` | `s` | 扫描代码库中的 TODO |
| `teammate list` | `ls` | 列出 TODO |
| `teammate add` | `a` | 添加新 TODO |
| `teammate remove` | `rm` | 删除 TODO |
| `teammate update` | `u` | 更新 TODO |
| `teammate status` | - | 更新 TODO 状态 |
| `teammate tag` | - | 管理标签 |
| `teammate config` | - | 管理配置 |
| `teammate init` | - | 初始化项目 |
| `teammate stats` | - | 显示统计信息 |
| `teammate blame` | - | 查看 TODO 的 Git blame |
| `teammate branch` | - | 分支 TODO 管理 |
| `teammate sync` | - | 同步数据 |
| `teammate hooks` | - | 管理 Git hooks |

## 输出格式

支持多种输出格式，使用 `--format` 选项指定：

```bash
# 表格格式（默认）
teammate list

# JSON 格式
teammate list --format json

# 简洁格式
teammate list --format compact

# CSV 格式
teammate list --format csv
```

## 配置文件

默认配置文件位于 `~/.config/teammate/config.yaml`

```yaml
# 项目路径
project: "."

# 扫描选项
scan:
  exclude:
    - "target"
    - "node_modules"
  languages:
    - "rust"
    - "python"
    - "javascript"

# 输出格式
format: "table"

# 颜色输出
no_color: false
```

## 开发

```bash
# 克隆项目
git clone https://github.com/illli-studio/teammate.git
cd teammate

# 开发模式运行
cargo run -- scan

# 运行测试
cargo test

# 构建 Release 版本
cargo build --release
```

## 技术栈

- **Rust** - 主要开发语言
- **SQLite** - 数据存储
- **Clap** - 命令行参数解析
- **Rayon** - 并行扫描

## 贡献

欢迎提交 Issue 和 Pull Request！

## 许可证

MIT License

## 作者

[illli-studio](https://github.com/illli-studio)
