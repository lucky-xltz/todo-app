# 待办清单 (Todo App)

一个使用 Tauri + Vue 3 + TypeScript + SQLite 构建的跨平台待办事项应用。

## 功能特性

- ✅ 添加待办事项（标题、描述、优先级、截止日期）
- ✅ 完成/取消完成待办
- ✅ 编辑待办事项
- ✅ 删除待办事项
- ✅ 搜索待办
- ✅ 筛选（全部、待办、完成）
- ✅ 排序（最新、最早、优先级、截止日期）
- ✅ 清除已完成
- ✅ 统计（总计、待办、完成、完成率进度环）
- ✅ 数据持久化（SQLite 数据库）
- ✅ 响应式设计
- ✅ 跨平台支持（macOS、Windows）

## 技术栈

### 前端
- Vue 3
- TypeScript
- Vite
- Pinia 状态管理
- @vueuse/core 工具库

### 后端
- Tauri 2
- Rust
- SQLite (rusqlite)
- chrono 时间处理
- uuid 生成

## 项目结构

```
todo-app/
├── src/                    # 前端源码
│   ├── components/        # Vue 组件
│   │   ├── TodoHeader.vue
│   │   ├── TodoInput.vue
│   │   ├── TodoItem.vue
│   │   ├── TodoFilters.vue
│   │   └── TodoList.vue
│   ├── stores/            # Pinia 状态管理
│   │   └── todo.ts
│   ├── types/             # TypeScript 类型
│   │   └── todo.ts
│   ├── App.vue            # 主应用组件
│   └── main.ts            # 入口文件
├── src-tauri/             # Tauri 后端
│   ├── src/
│   │   ├── database.rs    # SQLite 数据库操作
│   │   ├── lib.rs         # Tauri 命令
│   │   └── main.rs        # 入口文件
│   ├── icons/             # 应用图标
│   ├── Cargo.toml         # Rust 依赖
│   └── tauri.conf.json    # Tauri 配置
├── package.json           # 前端依赖
└── README.md              # 项目说明
```

## 开发

### 前置要求

- Node.js 18+
- Rust 1.70+
- Xcode (macOS) 或 Visual Studio (Windows)

### 安装依赖

```bash
# 前端依赖
npm install

# Rust 依赖会自动安装
```

### 开发模式

```bash
# 启动 Tauri 开发模式
npm run tauri:dev
```

### 构建

```bash
# 构建生产版本
npm run tauri:build
```

构建完成后，可执行文件会生成在 `src-tauri/target/release/` 目录下。

## 样式设计

应用采用新拟态风格（Neumorphism），特点：

- 柔和的阴影效果
- 紫色主题强调色 (#6c5ce7)
- 优先级颜色标识：
  - 高优先级：红色 (#e17055)
  - 中优先级：黄色 (#fdcb6e)
  - 低优先级：绿色 (#00b894)
- 流畅的动画效果
- 现代化的 UI 设计

## 数据库

使用 SQLite 作为本地数据库，数据文件保存在应用数据目录中。

### 数据库表结构

```sql
CREATE TABLE todos (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    completed BOOLEAN NOT NULL DEFAULT 0,
    priority TEXT NOT NULL DEFAULT 'medium',
    category TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    due_date TEXT
);
```

## 跨平台支持

- **macOS**: 支持 macOS 10.15+
- **Windows**: 支持 Windows 10+

## 许可证

MIT License