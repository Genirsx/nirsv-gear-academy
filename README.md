# Wordle 游戏项目

这是一个基于 Gear Protocol 开发的 Wordle 游戏项目，包含两个主要组件：游戏会话管理器和 Wordle 游戏程序。

## 项目结构

```
.
├── game_session/          # 游戏会话管理器
│   ├── src/              # 源代码目录
│   ├── io/               # IO 接口定义
│   ├── tests/            # 测试文件
│   └── Cargo.toml        # 项目配置文件
│
└── wordle_program/       # Wordle 游戏程序
    ├── src/              # 源代码目录
    ├── io/               # IO 接口定义
    └── Cargo.toml        # 项目配置文件
```

## 技术栈

- Rust 2021 Edition
- Gear Protocol v1.7.0
- gstd 库
- gear-wasm-builder

## 组件说明

### 游戏会话管理器 (game_session)

负责管理游戏会话，处理玩家交互和游戏状态。主要功能包括：
- 会话创建和管理
- 玩家状态跟踪
- 游戏进度记录

### Wordle 游戏程序 (wordle_program)

实现 Wordle 游戏的核心逻辑，包括：
- 单词验证
- 游戏规则实现
- 游戏状态管理

## 开发环境要求

- Rust 工具链
- Gear Protocol 开发环境
- WASM 支持

## 构建说明

1. 构建游戏会话管理器：
```bash
cd game_session
cargo build
```

2. 构建 Wordle 游戏程序：
```bash
cd wordle_program
cargo build
```

## 测试

运行测试：
```bash
cd game_session
cargo test
```

## 许可证

[待定]

## 贡献指南

[待定] 
