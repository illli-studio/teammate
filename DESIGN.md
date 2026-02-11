# Teammate - TODO CLI 工具设计文档

## 概述

Teammate 是一个专为开发者设计的 TODO CLI 工具，用于从代码库中自动发现、跟踪和管理 TODO 注释。

## 功能特性

### 核心功能
- 自动扫描代码库中的 TODO 注释
- 支持多语言解析（JavaScript, Python, Rust, Markdown 等）
- 标签系统和优先级管理
- 状态追踪（open → in_progress → resolved）
- Git 集成和历史追溯

### 用户体验
- 直观的 CLI 命令设计
- 交互式 TUI 界面
- 快速过滤和搜索
- 渐进式复杂度（简单使用 → 高级功能）

## 技术架构

### 系统架构

```
┌─────────────────────────────────────────────────────────┐
│                    Teammate CLI                         │
├─────────────────────────────────────────────────────────┤
│  CLI 层: Rust + Clap                                    │
│  解析层: Rust regex + rayon (并行解析)                 │
│  存储层: SQLite + rkyv                                 │
│  UI 层: Ratatui / Bubble Tea                           │
│  集成: git2-rs (Git 集成)                              │
└─────────────────────────────────────────────────────────┘
```

### 数据模型

```typescript
interface Todo {
  id: string;           // SHA-256 hash
  content: string;      // TODO 内容
  file: string;         // 文件路径
  line: number;         // 行号
  language: string;     // 编程语言
  tags: string[];       // 标签数组
  priority: 'low' | 'medium' | 'high';
  author?: string;     // 作者
  created_at: ISO8601;  // 创建时间
  updated_at: ISO8601;  // 更新时间
  status: 'open' | 'in_progress' | 'resolved';
  linked_issue?: string; // 关联 Issue
}
```

### 存储结构

```
~/.teammate/
├── config.yaml         # 用户配置
├── database.db         # SQLite 数据库
└── cache/              # 解析缓存
```

### 支持的 TODO 模式

| 模式 | 示例 | 语言 |
|------|------|------|
| `// TODO` | `// TODO: 修复 bug` | JS/TS/C++/Rust |
| `# TODO` | `# TODO: 添加测试` | Python/Bash |
| `/* TODO */` | `/* TODO: 优化性能 */` | 多语言 |
| `<!-- TODO -->` | `<!-- TODO: 更新文档 -->` | HTML |
| `[ ] TODO` | `[ ] 完成功能设计` | Markdown |
| `TODO:` | `TODO(FIX): 内存泄漏` | 通用 |

## CLI 命令设计

### 基础命令

| 命令 | 描述 |
|------|------|
| `teammate scan` | 扫描代码库中的 TODO |
| `teammate list` | 列出所有 TODO |
| `teammate add` | 添加新 TODO |
| `teammate status` | 更新 TODO 状态 |
| `teammate remove` | 删除 TODO |

### 过滤选项

| 选项 | 描述 |
|------|------|
| `--open` | 仅显示未解决的 TODO |
| `--tag=TAG` | 按标签过滤 |
| `--priority=HIGH` | 按优先级过滤 |
| `--file=PATH` | 按文件过滤 |
| `--author=NAME` | 按作者过滤 |

### 示例

```bash
# 扫描当前目录
teammate scan

# 列出所有高优先级 TODO
teammate list --priority=high --open

# 添加新 TODO
teammate add "优化数据库查询" --tag=performance --priority=medium

# 将 TODO 标记为进行中
teammate status 123 --in_progress
```

## 集成

### Git 集成

```yaml
# .teammate/config.yaml
git:
  enabled: true
  hooks:
    pre-commit: true    # 检查紧急 TODO
    post-commit: true   # 更新 TODO 状态
  integration:
    branch_todos: true   # 按分支显示
    blame: true          # 显示 TODO 作者
```

### 编辑器支持

- **VS Code**: Extension（跳转到 TODO、状态管理）
- **Vim/Neovim**: LSP + Telescope 插件
- **Emacs**: Minor mode

### CI/CD

```yaml
# GitHub Actions
- name: Check TODO status
  run: teammate check --urgent --format=github
```

## 性能

| 操作 | 目标时间 |
|------|---------|
| 全量扫描 (10K 文件) | < 5秒 |
| 单次查询 | < 100ms |
| 增量更新 | < 50ms |

## 跨平台支持

| 平台 | 状态 |
|------|------|
| macOS | ✅ 一级支持 |
| Linux | ✅ 一级支持 |
| Windows | ✅ 一级支持 |
| WSL | ✅ 支持 |

## 实施路线图

### v0.1 (MVP)
- [ ] JSON Lines 存储
- [ ] 基础正则解析
- [ ] CLI 基础命令
- [ ] 简单过滤功能

### v1.0
- [ ] SQLite 存储 ] Git 集成迁移
- [
- [ ] TUI 界面
- [ ] 标签和优先级系统

### v2.0
- [ ] 插件系统 (WASM)
- [ ] 后台守护进程
- [ ] 编辑器扩展
- [ ] 高级统计和报告

## 许可证

MIT License
