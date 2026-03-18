# Phase 8: Web UI 集成

**生成日期**: 2026-03-17
**预计工期**: 2-3 周
**前置依赖**: Phase 0-7 完成
**状态**: 规划中

---

## 一、Phase 8 概述

### 1.1 目标

实现 OpenNovel 的 Web 用户界面，包括：
- 群聊界面（核心功能）
- Provider 配置界面
- Agent 状态展示
- 流式输出支持
- AI-Reader-V2 可视化集成

### 1.2 当前状态

| 模块 | 状态 | 说明 |
|------|------|------|
| Provider API | ✅ 完成 | `routes.rs` 中已实现 |
| LLM API | ✅ 完成 | resolve, generate 端点 |
| 配置热重载 | ✅ 完成 | WebSocket 推送 |
| 书籍管理 | ⚠️ 存根 | 仅返回空数据 |
| 群聊功能 | ⚠️ 存根 | 仅返回空数据 |
| 前端界面 | ❌ 不存在 | 需要从零实现 |

### 1.3 技术选型

| 组件 | 选择 | 理由 |
|------|------|------|
| **后端框架** | Axum (已有) | Phase 6-7 已实现 |
| **前端框架** | SvelteKit | 轻量、响应式、SSR 支持 |
| **UI 组件** | shadcn-svelte | 现代化、可定制 |
| **样式** | Tailwind CSS | 与设计系统一致 |
| **状态管理** | Svelte Stores | 内置、简单 |
| **实时通信** | WebSocket + SSE | 流式输出、热重载 |

---

## 二、架构设计

### 2.1 设计文档

📄 **[前端界面设计规范](./02-ui-design.md)** - 完整的 UI/UX 设计方案，包含：
- 设计系统（色彩、字体、间距）
- 核心组件设计（Agent 头像、消息气泡、书籍列表、状态面板）
- 页面布局设计（群聊界面、Provider 配置）
- 响应式适配和动画效果

📄 **[前端交互逻辑设计](./03-interaction-logic.md)** - 详细的操作逻辑规范，包含：
- 全局状态管理（Store 结构、操作方法）
- SSE 流式连接管理
- 页面交互逻辑（主页面、群聊页面、消息输入）
- 关键交互流程（发送消息、@提及、流式输出、Provider 配置）
- 错误处理策略
- 性能优化策略
- 无障碍支持

### 2.2 整体架构

```
┌─────────────────────────────────────────────────────────────────────┐
│                           Frontend (SvelteKit)                       │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│   ┌─────────────┐  ┌─────────────┐  ┌─────────────┐               │
│   │  群聊界面   │  │  配置界面   │  │  状态界面   │               │
│   └─────────────┘  └─────────────┘  └─────────────┘               │
│          │                │                │                        │
│          └────────────────┼────────────────┘                        │
│                           │                                         │
│                    HTTP + WebSocket                                 │
│                           │                                         │
├───────────────────────────┼─────────────────────────────────────────┤
│                           ▼                                         │
│   ┌─────────────────────────────────────────────────────────────┐  │
│   │                    Backend (Axum)                             │  │
│   │                                                              │  │
│   │   ┌───────────┐  ┌───────────┐  ┌───────────────────────┐   │  │
│   │   │ Book API  │  │ Chat API  │  │ Provider/Config API   │   │  │
│   │   └───────────┘  └───────────┘  └───────────────────────┘   │  │
│   │          │              │                    │               │  │
│   │          └──────────────┼────────────────────┘               │  │
│   │                         │                                    │  │
│   │   ┌─────────────────────┴─────────────────────────────────┐ │  │
│   │   │              Agent System (Phase 7)                   │ │  │
│   │   │                                                       │ │  │
│   │   │   TianDao │ Writer │ WorldGuardian │ LiuHeping │ ... │ │  │
│   │   └───────────────────────────────────────────────────────┘ │  │
│   │                         │                                    │  │
│   │   ┌─────────────────────┴─────────────────────────────────┐ │  │
│   │   │              LLM Integration (Phase 6)                │ │  │
│   │   │                                                       │ │  │
│   │   │   ProviderRegistry │ ModelResolver │ HotReload        │ │  │
│   │   └───────────────────────────────────────────────────────┘ │  │
│   └─────────────────────────────────────────────────────────────┘  │
│                                                                      │
│   ┌─────────────────────────────────────────────────────────────┐  │
│   │              AI-Reader-V2 (MCP Integration)                  │  │
│   │                                                              │  │
│   │   人物关系图 │ 世界地图 │ 时间线 │ 实体档案                  │  │
│   └─────────────────────────────────────────────────────────────┘  │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### 2.2 目录结构

```
/opennovel-core/apps/web/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── routes.rs           # API 路由
│   ├── state.rs            # AppState
│   └── handlers/           # 新增：处理器模块
│       ├── mod.rs
│       ├── book.rs         # 书籍管理
│       ├── chat.rs         # 群聊功能
│       └── visualization.rs # AI-Reader 可视化
│
└── frontend/               # 新增：SvelteKit 前端
    ├── package.json
    ├── svelte.config.js
    ├── vite.config.ts
    ├── tailwind.config.js
    ├── src/
    │   ├── routes/
    │   │   ├── +layout.svelte
    │   │   ├── +page.svelte           # 首页
    │   │   ├── books/
    │   │   │   ├── +page.svelte       # 书籍列表
    │   │   │   └── [id]/
    │   │   │       ├── +page.svelte   # 书籍详情
    │   │   │       └── chat/
    │   │   │           └── +page.svelte  # 群聊界面
    │   │   ├── settings/
    │   │   │   └── providers/
    │   │   │       └── +page.svelte   # Provider 配置
    │   │   └── visualization/
    │   │       ├── graph/
    │   │       │   └── +page.svelte   # 人物关系图
    │   │       └── map/
    │   │           └── +page.svelte   # 世界地图
    │   ├── lib/
    │   │   ├── api/
    │   │   │   ├── client.ts          # API 客户端
    │   │   │   ├── websocket.ts       # WebSocket 管理
    │   │   │   └── types.ts           # 类型定义
    │   │   ├── stores/
    │   │   │   ├── books.ts           # 书籍状态
    │   │   │   ├── chat.ts            # 聊天状态
    │   │   │   └── providers.ts       # Provider 状态
    │   │   └── components/
    │   │       ├── chat/
    │   │       │   ├── MessageList.svelte
    │   │       │   ├── MessageInput.svelte
    │   │       │   ├── AgentBadge.svelte
    │   │       │   └── StreamingMessage.svelte
    │   │       ├── visualization/
    │   │       │   ├── GraphView.svelte
    │   │       │   └── MapView.svelte
    │   │       └── settings/
    │   │           ├── ProviderCard.svelte
    │   │           └── ModelSelector.svelte
    │   └── app.css
    └── static/
```

---

## 三、核心功能实现

### 3.1 群聊界面（Task 8.1）

#### 3.1.1 功能需求

- 展示群聊消息列表
- 支持流式输出（SSE）
- Agent 身份标识
- @提及功能
- 消息发送

#### 3.1.2 后端实现

```rust
// src/handlers/chat.rs

use axum::{
    extract::{Path, State, WebSocketUpgrade},
    response::{sse::Event, Sse},
    Json,
};
use futures::stream::{self, Stream};
use std::convert::Infallible;

/// 获取书籍的群聊消息列表
pub async fn list_messages(
    Path(book_id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    match state.chat_manager.get_messages(&book_id).await {
        Ok(messages) => Json(messages),
        Err(e) => Json(vec![]),
    }
}

/// 发送消息到群聊
pub async fn send_message(
    Path(book_id): Path<String>,
    State(state): State<AppState>,
    Json(request): Json<SendMessageRequest>,
) -> impl IntoResponse {
    // 1. 保存用户消息
    let user_msg = state.chat_manager
        .save_user_message(&book_id, &request.content)
        .await;
    
    // 2. 触发 Agent 处理
    let response = state.agent_orchestrator
        .process(&book_id, &request.content)
        .await;
    
    // 3. 返回响应
    Json(response)
}

/// SSE 流式输出
pub async fn stream_chat(
    Path(book_id): Path<String>,
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let receiver = state.chat_manager.subscribe(&book_id).await;
    
    let stream = async_stream::stream! {
        while let Ok(msg) = receiver.recv().await {
            let json = serde_json::to_string(&msg).unwrap();
            yield Ok(Event::default().data(json));
        }
    };
    
    Sse::new(stream)
}
```

#### 3.1.3 前端实现

```svelte
<!-- src/routes/books/[id]/chat/+page.svelte -->

<script lang="ts">
  import { onMount } from 'svelte';
  import MessageList from '$lib/components/chat/MessageList.svelte';
  import MessageInput from '$lib/components/chat/MessageInput.svelte';
  import { chatStore } from '$lib/stores/chat';
  import { connectStream } from '$lib/api/websocket';

  export let data;

  let messages = [];
  let streamingContent = '';

  onMount(() => {
    // 连接 SSE 流
    const eventSource = connectStream(`/api/books/${data.bookId}/stream`);
    
    eventSource.onmessage = (event) => {
      const msg = JSON.parse(event.data);
      if (msg.type === 'streaming') {
        streamingContent = msg.content;
      } else if (msg.type === 'complete') {
        messages = [...messages, msg];
        streamingContent = '';
      }
    };
  });

  async function sendMessage(content: string) {
    await fetch(`/api/books/${data.bookId}/chat`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ content })
    });
  }
</script>

<div class="chat-container">
  <MessageList {messages} {streamingContent} />
  <MessageInput on:send={(e) => sendMessage(e.detail)} />
</div>

<style>
  .chat-container {
    display: flex;
    flex-direction: column;
    height: 100vh;
    max-width: 800px;
    margin: 0 auto;
  }
</style>
```

---

### 3.2 Provider 配置界面（Task 8.2）

#### 3.2.1 功能需求

- Provider 列表展示
- 添加/编辑/删除 Provider
- 测试 Provider 连接
- 模型选择

#### 3.2.2 前端实现

```svelte
<!-- src/routes/settings/providers/+page.svelte -->

<script lang="ts">
  import { onMount } from 'svelte';
  import ProviderCard from '$lib/components/settings/ProviderCard.svelte';
  import { providersStore } from '$lib/stores/providers';

  let providers = [];
  let showAddModal = false;
  let editingProvider = null;

  onMount(async () => {
    providers = await fetchProviders();
  });

  async function testProvider(id: string) {
    const res = await fetch(`/api/providers/${id}/test`, { method: 'POST' });
    const data = await res.json();
    // 更新状态
  }

  async function saveProvider(provider) {
    if (provider.id) {
      await fetch(`/api/providers/${provider.id}`, {
        method: 'PATCH',
        body: JSON.stringify(provider)
      });
    } else {
      await fetch('/api/providers', {
        method: 'POST',
        body: JSON.stringify(provider)
      });
    }
    showAddModal = false;
    providers = await fetchProviders();
  }
</script>

<div class="providers-page">
  <header>
    <h1>LLM Provider 配置</h1>
    <button on:click={() => showAddModal = true}>添加 Provider</button>
  </header>

  <div class="provider-grid">
    {#each providers as provider}
      <ProviderCard
        {provider}
        on:edit={() => editingProvider = provider}
        on:test={() => testProvider(provider.id)}
        on:delete={() => deleteProvider(provider.id)}
      />
    {/each}
  </div>
</div>
```

---

### 3.3 Agent 状态展示（Task 8.3）

#### 3.3.1 功能需求

- 显示当前活跃的 Agent
- 显示 Agent 处理状态
- 显示 Agent 执行的工具调用
- 阶段锁定状态

#### 3.3.2 状态数据结构

```typescript
// src/lib/api/types.ts

export interface AgentStatus {
  id: string;
  name: string;
  role: AgentRole;
  status: 'idle' | 'thinking' | 'writing' | 'waiting';
  currentTask?: string;
  lastActivity?: Date;
  lockedUntil?: CreationStage;
}

export type AgentRole = 
  | 'tian-dao'      // 天道
  | 'writer'        // 执笔
  | 'world-guardian' // 世界观守护者
  | 'liuheping'     // 刘和平
  | 'planner'       // 规划者
  | 'reviewer'      // 审阅
  | 'observer'      // 观察者
  | 'researcher';   // 调研者

export type CreationStage = 
  | 'conception'    // 阶段一：构思
  | 'knowledge'     // 阶段二：知识库建立
  | 'writing';      // 阶段三：撰写
```

#### 3.3.3 前端组件

```svelte
<!-- src/lib/components/AgentStatusPanel.svelte -->

<script lang="ts">
  import type { AgentStatus } from '$lib/api/types';

  export let agents: AgentStatus[];
  export let currentStage: string;

  function getStatusColor(status: string) {
    return {
      idle: 'gray',
      thinking: 'blue',
      writing: 'green',
      waiting: 'yellow'
    }[status] || 'gray';
  }

  function isLocked(agent: AgentStatus) {
    return agent.lockedUntil && agent.lockedUntil !== currentStage;
  }
</script>

<div class="agent-panel">
  <h3>Agent 状态</h3>
  
  <div class="agent-list">
    {#each agents as agent}
      <div class="agent-item" class:locked={isLocked(agent)}>
        <div class="agent-icon" style="background: {getStatusColor(agent.status)}">
          {agent.name[0]}
        </div>
        <div class="agent-info">
          <span class="name">{agent.name}</span>
          <span class="status">{agent.status}</span>
          {#if agent.currentTask}
            <span class="task">{agent.currentTask}</span>
          {/if}
        </div>
        {#if isLocked(agent)}
          <span class="lock-badge">🔒 锁定</span>
        {/if}
      </div>
    {/each}
  </div>
</div>
```

---

### 3.4 AI-Reader-V2 可视化集成（Task 8.4）

#### 3.4.1 功能需求

- 人物关系图展示
- 世界地图展示（多层级）
- 时间线展示
- 与群聊界面联动

#### 3.4.2 嵌入方式

```svelte
<!-- src/lib/components/visualization/GraphView.svelte -->

<script lang="ts">
  import { onMount } from 'svelte';
  
  export let bookId: string;
  export let chapterRange?: { start: number; end: number };

  let graphData = null;
  let loading = true;

  onMount(async () => {
    // 调用 AI-Reader-V2 API
    const res = await fetch(`/api/visualization/${bookId}/graph`, {
      headers: { 'X-AI-Reader-URL': 'http://localhost:8000' }
    });
    graphData = await res.json();
    loading = false;
  });
</script>

<div class="graph-container">
  {#if loading}
    <div class="loading">加载中...</div>
  {:else}
    <!-- 使用 D3.js 或 vis-network 渲染 -->
    <div id="graph-canvas" />
  {/if}
</div>
```

#### 3.4.3 后端代理

```rust
// src/handlers/visualization.rs

/// 代理 AI-Reader-V2 API
pub async fn proxy_graph(
    Path(book_id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let ai_reader_url = &state.config.ai_reader_url;
    
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/api/novels/{}/graph", ai_reader_url, book_id))
        .send()
        .await;
    
    match response {
        Ok(res) => {
            let body = res.text().await.unwrap_or_default();
            (StatusCode::OK, body)
        }
        Err(e) => {
            (StatusCode::BAD_GATEWAY, format!("AI-Reader error: {}", e))
        }
    }
}
```

---

## 四、任务分解

### Task 8.1: 群聊界面实现（5 天）

| 子任务 | 工时 | 说明 |
|--------|------|------|
| 8.1.1 后端 Chat API | 1 天 | 实现 list_messages, send_message, stream_chat |
| 8.1.2 前端消息列表 | 1 天 | MessageList 组件，Agent 身份标识 |
| 8.1.3 流式输出 | 1 天 | SSE 连接，StreamingMessage 组件 |
| 8.1.4 消息输入 | 1 天 | @提及，消息发送 |
| 8.1.5 测试 | 1 天 | 集成测试 |

### Task 8.2: Provider 配置界面（3 天）

| 子任务 | 工时 | 说明 |
|--------|------|------|
| 8.2.1 Provider 列表 | 0.5 天 | 列表展示，状态标识 |
| 8.2.2 添加/编辑表单 | 1 天 | 表单验证，模型配置 |
| 8.2.3 测试连接 | 0.5 天 | 测试按钮，结果显示 |
| 8.2.4 热重载集成 | 1 天 | WebSocket 更新推送 |

### Task 8.3: Agent 状态展示（2 天）

| 子任务 | 工时 | 说明 |
|--------|------|------|
| 8.3.1 状态 API | 0.5 天 | 后端状态端点 |
| 8.3.2 状态面板 | 1 天 | AgentStatusPanel 组件 |
| 8.3.3 阶段锁定显示 | 0.5 天 | 锁定状态视觉反馈 |

### Task 8.4: AI-Reader 可视化（3 天）

| 子任务 | 工时 | 说明 |
|--------|------|------|
| 8.4.1 API 代理 | 0.5 天 | 后端代理 AI-Reader API |
| 8.4.2 人物关系图 | 1 天 | GraphView 组件 |
| 8.4.3 世界地图 | 1 天 | MapView 组件，多层级切换 |
| 8.4.4 群聊联动 | 0.5 天 | 从群聊跳转到可视化 |

### Task 8.5: 整体集成与测试（2 天）

| 子任务 | 工时 | 说明 |
|--------|------|------|
| 8.5.1 路由集成 | 0.5 天 | 页面路由，导航 |
| 8.5.2 状态管理 | 0.5 天 | 全局状态，持久化 |
| 8.5.3 E2E 测试 | 1 天 | Playwright 测试 |

---

## 五、验收标准

### 5.1 功能验收

- [ ] 群聊界面可正常发送/接收消息
- [ ] 流式输出正常工作
- [ ] Provider 可添加/编辑/删除/测试
- [ ] Agent 状态正确显示
- [ ] AI-Reader 可视化正常展示

### 5.2 性能验收

- [ ] 消息发送延迟 < 500ms
- [ ] 首屏加载时间 < 3s
- [ ] SSE 连接稳定，无频繁断开

### 5.3 兼容性验收

- [ ] Chrome/Firefox/Safari 支持
- [ ] 移动端响应式适配

---

## 六、风险与应对

| 风险 | 可能性 | 影响 | 应对措施 |
|------|--------|------|---------|
| SSE 连接不稳定 | 中 | 高 | 实现自动重连 + 心跳检测 |
| AI-Reader 服务不可用 | 中 | 中 | 实现降级策略，显示占位符 |
| 前端状态管理复杂 | 低 | 中 | 使用 Svelte Stores，保持简洁 |

---

## 七、后续优化

- **PWA 支持**：离线访问，桌面快捷方式
- **暗色主题**：护眼模式
- **快捷键**：提升操作效率
- **消息搜索**：全文搜索历史消息