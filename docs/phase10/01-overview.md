# Phase 10: 文档与部署

**生成日期**: 2026-03-17
**预计工期**: 1-2 周
**前置依赖**: Phase 9 完成
**状态**: 规划中

---

## 一、Phase 10 概述

### 1.1 目标

完成 OpenNovel 系统的文档和部署准备：
- API 文档（OpenAPI/Swagger）
- 用户指南
- 开发者文档
- Docker 镜像
- 部署脚本
- 监控与日志

### 1.2 文档结构

```
/docs/
├── api/                    # API 文档
│   ├── openapi.yaml       # OpenAPI 规范
│   └── swagger-ui/        # Swagger UI
│
├── user-guide/            # 用户指南
│   ├── getting-started.md
│   ├── create-novel.md
│   ├── chat-with-agents.md
│   └── visualization.md
│
├── developer/              # 开发者文档
│   ├── architecture.md
│   ├── contributing.md
│   ├── agent-development.md
│   └── plugin-development.md
│
└── deployment/             # 部署文档
    ├── docker.md
    ├── kubernetes.md
    └── configuration.md
```

---

## 二、API 文档

### 2.1 OpenAPI 规范

```yaml
# docs/api/openapi.yaml

openapi: 3.1.0
info:
  title: OpenNovel API
  description: |
    OpenNovel 是一个多智能体小说创作系统，提供以下核心功能：
    
    - **群聊协作**：与 8 个专业 Agent 协作创作小说
    - **知识库管理**：人物、世界观、伏笔等知识管理
    - **LLM 集成**：支持多种 LLM Provider
    - **可视化**：人物关系图、世界地图
    
  version: 2.0.0
  contact:
    name: OpenNovel Team
    url: https://github.com/your-org/opennovelv2

servers:
  - url: http://localhost:3000
    description: 本地开发服务器
  - url: https://api.opennovel.example.com
    description: 生产服务器

tags:
  - name: Books
    description: 书籍管理
  - name: Chat
    description: 群聊功能
  - name: Providers
    description: LLM Provider 管理
  - name: Visualization
    description: 可视化功能

paths:
  # ─────────────────────────────────────────────────────
  # Books
  # ─────────────────────────────────────────────────────
  
  /api/books:
    get:
      tags: [Books]
      summary: 获取书籍列表
      responses:
        '200':
          description: 成功
          content:
            application/json:
              schema:
                type: object
                properties:
                  books:
                    type: array
                    items:
                      $ref: '#/components/schemas/Book'
    
    post:
      tags: [Books]
      summary: 创建新书籍
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/CreateBookRequest'
      responses:
        '201':
          description: 创建成功
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/Book'

  /api/books/{bookId}:
    get:
      tags: [Books]
      summary: 获取书籍详情
      parameters:
        - name: bookId
          in: path
          required: true
          schema:
            type: string
      responses:
        '200':
          description: 成功
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/BookDetail'
        '404':
          description: 书籍不存在

  # ─────────────────────────────────────────────────────
  # Chat
  # ─────────────────────────────────────────────────────
  
  /api/books/{bookId}/chat:
    post:
      tags: [Chat]
      summary: 发送群聊消息
      description: |
        发送消息到书籍群聊，触发 Agent 处理。
        
        **支持 @提及**：在消息中使用 `@Agent名称` 可以直接调用特定 Agent。
        
        示例：
        ```
        @天道 设计第二章的剧情
        ```
      parameters:
        - name: bookId
          in: path
          required: true
          schema:
            type: string
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/SendMessageRequest'
      responses:
        '200':
          description: 消息已发送
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/Message'

  /api/books/{bookId}/messages:
    get:
      tags: [Chat]
      summary: 获取群聊消息列表
      parameters:
        - name: bookId
          in: path
          required: true
          schema:
            type: string
        - name: limit
          in: query
          schema:
            type: integer
            default: 50
        - name: before
          in: query
          description: 获取此消息 ID 之前的消息
          schema:
            type: string
      responses:
        '200':
          description: 成功
          content:
            application/json:
              schema:
                type: object
                properties:
                  messages:
                    type: array
                    items:
                      $ref: '#/components/schemas/Message'

  /api/books/{bookId}/stream:
    get:
      tags: [Chat]
      summary: SSE 流式输出
      description: |
        连接 SSE 端点，接收实时消息流。
        
        **事件类型**：
        - `message`: 新消息
        - `streaming`: 流式输出中
        - `agent_status`: Agent 状态变更
        
      parameters:
        - name: bookId
          in: path
          required: true
          schema:
            type: string
      responses:
        '200':
          description: SSE 流
          content:
            text/event-stream:
              schema:
                type: string

  # ─────────────────────────────────────────────────────
  # Providers
  # ─────────────────────────────────────────────────────
  
  /api/providers:
    get:
      tags: [Providers]
      summary: 获取 Provider 列表
      responses:
        '200':
          description: 成功
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/ProviderList'
    
    post:
      tags: [Providers]
      summary: 创建 Provider
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/CreateProviderRequest'
      responses:
        '201':
          description: 创建成功

  /api/providers/{providerId}/test:
    post:
      tags: [Providers]
      summary: 测试 Provider 连接
      parameters:
        - name: providerId
          in: path
          required: true
          schema:
            type: string
      responses:
        '200':
          description: 测试结果
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/TestProviderResponse'

  # ─────────────────────────────────────────────────────
  # Visualization
  # ─────────────────────────────────────────────────────
  
  /api/visualization/{bookId}/graph:
    get:
      tags: [Visualization]
      summary: 获取人物关系图
      parameters:
        - name: bookId
          in: path
          required: true
          schema:
            type: string
        - name: chapterStart
          in: query
          schema:
            type: integer
        - name: chapterEnd
          in: query
          schema:
            type: integer
      responses:
        '200':
          description: 人物关系图数据
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/CharacterGraph'

  /api/visualization/{bookId}/map:
    get:
      tags: [Visualization]
      summary: 获取世界地图
      parameters:
        - name: bookId
          in: path
          required: true
          schema:
            type: string
        - name: layerId
          in: query
          description: 空间层级（overworld/sky/underground/sea）
          schema:
            type: string
      responses:
        '200':
          description: 世界地图数据
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/WorldMap'

components:
  schemas:
    Book:
      type: object
      properties:
        id:
          type: string
        title:
          type: string
        author:
          type: string
        status:
          $ref: '#/components/schemas/BookStatus'
        stage:
          $ref: '#/components/schemas/CreationStage'
        chapter_count:
          type: integer
        word_count:
          type: integer
        created_at:
          type: string
          format: date-time

    BookStatus:
      type: string
      enum: [draft, writing, paused, completed]

    CreationStage:
      type: string
      enum: [conception, knowledge, writing]
      description: |
        创作阶段：
        - `conception`: 构思阶段，规划者与用户私密对话
        - `knowledge`: 知识库建立阶段
        - `writing`: 撰写阶段，天道编排剧情

    Message:
      type: object
      properties:
        id:
          type: string
        role:
          $ref: '#/components/schemas/MessageRole'
        agent_name:
          type: string
          nullable: true
        content:
          type: string
        annotations:
          type: array
          items:
            $ref: '#/components/schemas/Annotation'
        created_at:
          type: string
          format: date-time

    MessageRole:
      type: string
      enum: [user, assistant, system]

    Annotation:
      type: object
      properties:
        id:
          type: string
        agent_name:
          type: string
        type:
          $ref: '#/components/schemas/AnnotationType'
        content:
          type: string
        position:
          $ref: '#/components/schemas/AnnotationPosition'

    AnnotationType:
      type: string
      enum: [suggestion, warning, correction, info]

    Provider:
      type: object
      properties:
        id:
          type: string
        name:
          type: string
        provider_type:
          type: string
          enum: [openai, anthropic, openai-compatible, custom]
        base_url:
          type: string
        enabled:
          type: boolean
        models:
          type: array
          items:
            $ref: '#/components/schemas/Model'

    CharacterGraph:
      type: object
      properties:
        nodes:
          type: array
          items:
            $ref: '#/components/schemas/CharacterNode'
        edges:
          type: array
          items:
            $ref: '#/components/schemas/CharacterEdge'

    CharacterNode:
      type: object
      properties:
        id:
          type: string
        name:
          type: string
        aliases:
          type: array
          items:
            type: string
        chapter_count:
          type: integer
        organization:
          type: string
          nullable: true

    CharacterEdge:
      type: object
      properties:
        source:
          type: string
        target:
          type: string
        relation_type:
          type: string
        category:
          type: string
```

### 2.2 Swagger UI 集成

```rust
// apps/web/src/main.rs

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        // 引入所有 API 路径
    ),
    components(
        // 引入所有 Schema
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui")
            .url("/api-docs/openapi.json", ApiDoc::openapi()))
        // ... 其他路由
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

---

## 三、用户指南

### 3.1 快速开始

```markdown
# docs/user-guide/getting-started.md

# 快速开始

## 系统要求

- Docker 20.10+
- Docker Compose 2.0+
- 4GB+ 内存
- 现代浏览器（Chrome/Firefox/Safari）

## 快速启动

### 1. 克隆项目

\`\`\`bash
git clone https://github.com/your-org/opennovelv2.git
cd opennovelv2
\`\`\`

### 2. 配置环境变量

\`\`\`bash
cp .env.example .env
# 编辑 .env，填入你的 LLM API Key
\`\`\`

### 3. 启动服务

\`\`\`bash
docker-compose up -d
\`\`\`

### 4. 访问界面

打开浏览器访问 `http://localhost:3000`

## 配置 LLM Provider

### 添加 OpenAI Provider

1. 进入「设置」→「Provider 管理」
2. 点击「添加 Provider」
3. 填写以下信息：
   - 名称：`OpenAI`
   - 类型：`OpenAI`
   - Base URL：`https://api.openai.com/v1`
   - API Key：你的 OpenAI API Key
4. 点击「测试连接」验证配置
5. 保存

### 添加 DeepSeek Provider（推荐）

DeepSeek 提供高性价比的中文模型：

1. 类型：`OpenAI Compatible`
2. Base URL：`https://api.deepseek.com/v1`
3. API Key：你的 DeepSeek API Key

## 创建第一本小说

1. 点击「新建书籍」
2. 填写书名和简介
3. 进入构思阶段，与规划者讨论故事设定
4. 完成构思后，系统自动进入撰写阶段

## 与 Agent 协作

OpenNovel 有 8 个专业 Agent：

| Agent | 职责 | 触发方式 |
|-------|------|---------|
| 天道 | 剧情编排 | 自动/`@天道` |
| 执笔 | 撰写内容 | 自动 |
| 世界观守护者 | 规则检查 | 自动 |
| 刘和平 | 人物塑造 | `@刘和平` |
| 规划者 | 新书规划 | 阶段一 |
| 审阅 | 质量评估 | 自动 |
| 观察者 | 知识库管理 | 自动 |
| 调研者 | 爆点分析 | 阶段二 |

### 发送消息

在群聊界面输入消息，点击发送或按回车。

### @提及 Agent

使用 `@` 符号可以直接调用特定 Agent：

\`\`\`
@天道 分析一下当前的剧情走向
@刘和平 帮我设计一个反派人物
\`\`\`
```

### 3.2 创作流程指南

```markdown
# docs/user-guide/create-novel.md

# 创作流程指南

## 三阶段创作模型

OpenNovel 采用三阶段创作模型，每个阶段有不同的 Agent 可用。

### 阶段一：构思阶段

**目标**：确定故事的核心框架

**可用 Agent**：规划者（与用户私密对话）

**流程**：

1. 用户提出创作想法
2. 规划者引导完善：
   - 世界观设定
   - 主要人物
   - 核心冲突
   - 故事走向
3. 用户确认后，规划者整理成大纲

**注意**：
- 此阶段其他 Agent 被锁定，避免干扰构思
- 可以随时修改，直到满意为止
- 一旦确认进入下一阶段，规划者将永久锁定

### 阶段二：知识库建立

**目标**：建立小说的知识体系

**可用 Agent**：规划者、调研者、观察者

**流程**：

1. 观察者创建知识库
2. 规划者填充初始设定
3. 调研者分析同类型爆款，提取成功要素

**知识库包括**：
- 世界观知识库
- 人物知识库
- 阵营派系知识库
- 地图知识库
- 伏笔知识库

### 阶段三：撰写阶段

**目标**：章节内容创作

**可用 Agent**：天道、执笔、世界观守护者、刘和平、审阅、观察者

**锁定 Agent**：规划者、调研者（防止方向漂移）

**流程**：

1. 天道通读知识库，推演剧情走向
2. 天道设计本章关键事件
3. 执笔撰写章节内容
4. 世界观守护者检查一致性
5. 刘和平检查人物声音
6. 审阅评估读者体验
7. 观察者更新知识库

## 第一章的特殊处理

第一章由规划者和用户在构思阶段共同设计，天道不参与。

从第二章开始，天道成为剧情编排的核心。

## 伏笔管理

天道维护一个"伏笔压力表"：

- 每章不暗示：压力 +5
- 每章不触发：压力 +10
- 超过预期触发章节：压力 +15

当压力超过阈值时，天道会优先考虑触发该伏笔。
```

---

## 四、开发者文档

### 4.1 架构文档

```markdown
# docs/developer/architecture.md

# 架构概览

## 整体架构

\`\`\`
┌─────────────────────────────────────────────────────────────────────┐
│                         Application Layer                            │
│                    (Axum Web Server + SvelteKit)                     │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌─────────────────────────────────────────────────────────────────┐│
│  │                      Agent System (Phase 7)                     ││
│  │                                                                 ││
│  │   ┌─────────────┐  ┌─────────────┐  ┌─────────────┐           ││
│  │   │   Hooks     │  │   Tools     │  │   Skills    │           ││
│  │   │   Registry  │  │   Registry  │  │   Loader    │           ││
│  │   └─────────────┘  └─────────────┘  └─────────────┘           ││
│  └─────────────────────────────────────────────────────────────────┘│
│                                  │                                   │
│  ┌───────────────────────────────┴─────────────────────────────────┐│
│  │                    Delegation Protocol                          ││
│  │                                                                 ││
│  │   IntentGate (4阶段)  +  SessionContinuityManager              ││
│  └─────────────────────────────────────────────────────────────────┘│
│                                  │                                   │
│  ┌───────────────────────────────┴─────────────────────────────────┐│
│  │                      Agent Core (Phase 0)                       ││
│  │                                                                 ││
│  │   Agent Trait  +  AgentRegistry  +  PermissionMatrix           ││
│  └─────────────────────────────────────────────────────────────────┘│
│                                  │                                   │
│  ┌───────────────────────────────┴─────────────────────────────────┐│
│  │                   LLM Integration Layer (Phase 6)               ││
│  │                                                                 ││
│  │   ProviderRegistry  +  ModelResolver  +  FallbackChain         ││
│  └─────────────────────────────────────────────────────────────────┘│
│                                  │                                   │
│  ┌───────────────────────────────┴─────────────────────────────────┐│
│  │                  Knowledge & Tools Layer (Phase 1-5)           ││
│  │                                                                 ││
│  │   7 Knowledge Bases  +  CharacterDB  +  ForeshadowingPool      ││
│  └─────────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────────┘
\`\`\`

## 核心模块

### Phase 0: SDK 核心

- **Agent Trait**：所有 Agent 的基础接口
- **IntentGate**：四阶段意图门控
- **PermissionMatrix**：权限控制矩阵

### Phase 1-5: 基础设施

- **Knowledge System**：七大知识库
- **Text Tools**：文本处理工具链
- **Collaboration**：协作系统
- **Analysis**：分析系统
- **Sync**：同步系统

### Phase 6: LLM 集成层

- **ProviderRegistry**：Provider 注册与管理
- **ModelResolver**：模型解析与 Fallback
- **HotReloadManager**：配置热重载

### Phase 7: Agent 系统

- **8 个专业 Agent**：天道、执笔、世界观守护者、刘和平、规划者、审阅、观察者、调研者
- **Hooks Registry**：钩子系统
- **Tools Registry**：工具系统
- **Skills Loader**：技能加载器

## 数据流

\`\`\`
用户输入 → IntentGate → 选择 Agent → 加载上下文
    ↓
Agent 处理 → 调用 LLM → 生成响应
    ↓
Hooks 后处理 → 返回用户 → 更新知识库
\`\`\`
```

### 4.2 贡献指南

```markdown
# docs/developer/contributing.md

# 贡献指南

## 开发环境设置

### 前置要求

- Rust 1.70+
- Node.js 18+
- Docker & Docker Compose

### 设置步骤

\`\`\`bash
# 1. Fork 并克隆项目
git clone https://github.com/YOUR_USERNAME/opennovelv2.git
cd opennovelv2

# 2. 安装 Rust 依赖
cargo build

# 3. 安装前端依赖
cd apps/web/frontend
npm install

# 4. 启动开发环境
docker-compose -f docker-compose.dev.yml up -d
\`\`\`

## 代码风格

### Rust

- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 遵循 Rust API 设计指南

### TypeScript/Svelte

- 使用 ESLint + Prettier
- 组件使用 PascalCase 命名
- 文件使用 kebab-case 命名

## 提交规范

使用 Conventional Commits：

\`\`\`
feat: 添加新功能
fix: 修复 Bug
docs: 文档更新
test: 测试相关
refactor: 代码重构
chore: 杂项
\`\`\`

## PR 流程

1. 创建功能分支
2. 编写代码和测试
3. 运行所有测试
4. 提交 PR
5. 等待 Review
```

---

## 五、Docker 部署

### 5.1 Dockerfile

```dockerfile
# apps/web/Dockerfile

# ─────────────────────────────────────────────────────
# Build Stage
# ─────────────────────────────────────────────────────
FROM rust:1.75 AS builder

WORKDIR /app

# 复制 Cargo 文件
COPY Cargo.toml Cargo.lock ./
COPY packages ./packages
COPY apps/web ./apps/web

# 构建
RUN cargo build --release -p opennovel-web

# ─────────────────────────────────────────────────────
# Runtime Stage
# ─────────────────────────────────────────────────────
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 复制构建产物
COPY --from=builder /app/target/release/opennovel-web /app/

# 复制前端静态文件
COPY apps/web/frontend/build /app/static

ENV PORT=3000
ENV RUST_LOG=info

EXPOSE 3000

CMD ["./opennovel-web"]
```

### 5.2 Docker Compose

```yaml
# docker-compose.yml

version: '3.8'

services:
  # ─────────────────────────────────────────────────────
  # OpenNovel 主服务
  # ─────────────────────────────────────────────────────
  opennovel:
    build:
      context: .
      dockerfile: apps/web/Dockerfile
    ports:
      - "3000:3000"
    environment:
      - RUST_LOG=info
      - DATABASE_URL=postgres://opennovel:password@postgres:5432/opennovel
      - REDIS_URL=redis://redis:6379
      - AI_READER_URL=http://ai-reader:8000
    depends_on:
      - postgres
      - redis
      - ai-reader
    volumes:
      - opennovel-data:/app/data
    restart: unless-stopped
    deploy:
      resources:
        limits:
          memory: 1G
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:3000/health"]
      interval: 30s
      timeout: 10s
      retries: 3

  # ─────────────────────────────────────────────────────
  # PostgreSQL 数据库
  # ─────────────────────────────────────────────────────
  postgres:
    image: postgres:15-alpine
    environment:
      - POSTGRES_USER=opennovel
      - POSTGRES_PASSWORD=password
      - POSTGRES_DB=opennovel
    volumes:
      - postgres-data:/var/lib/postgresql/data
    restart: unless-stopped
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U opennovel"]
      interval: 10s
      timeout: 5s
      retries: 5

  # ─────────────────────────────────────────────────────
  # Redis 缓存
  # ─────────────────────────────────────────────────────
  redis:
    image: redis:7-alpine
    volumes:
      - redis-data:/data
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 10s
      timeout: 5s
      retries: 5

  # ─────────────────────────────────────────────────────
  # AI-Reader-V2 可视化服务
  # ─────────────────────────────────────────────────────
  ai-reader:
    image: ghcr.io/mouseart2025/ai-reader-v2:latest
    ports:
      - "8000:8000"
    environment:
      - LLM_PROVIDER=openai
      - LLM_API_KEY=${LLM_API_KEY}
      - LLM_BASE_URL=${LLM_BASE_URL:-https://api.deepseek.com/v1}
      - LLM_MODEL=${LLM_MODEL:-deepseek-chat}
    volumes:
      - ai-reader-data:/root/.ai-reader-v2
    restart: unless-stopped
    deploy:
      resources:
        limits:
          memory: 2G

volumes:
  opennovel-data:
  postgres-data:
  redis-data:
  ai-reader-data:
```

### 5.3 环境变量配置

```bash
# .env.example

# ─────────────────────────────────────────────────────
# LLM 配置
# ─────────────────────────────────────────────────────
LLM_API_KEY=your_api_key_here
LLM_BASE_URL=https://api.deepseek.com/v1
LLM_MODEL=deepseek-chat

# ─────────────────────────────────────────────────────
# 数据库配置
# ─────────────────────────────────────────────────────
DATABASE_URL=postgres://opennovel:password@localhost:5432/opennovel
REDIS_URL=redis://localhost:6379

# ─────────────────────────────────────────────────────
# AI-Reader 配置
# ─────────────────────────────────────────────────────
AI_READER_URL=http://localhost:8000

# ─────────────────────────────────────────────────────
# 服务配置
# ─────────────────────────────────────────────────────
PORT=3000
RUST_LOG=info

# ─────────────────────────────────────────────────────
# 安全配置
# ─────────────────────────────────────────────────────
JWT_SECRET=your_jwt_secret_here
CORS_ORIGINS=http://localhost:3000,https://your-domain.com
```

---

## 六、Kubernetes 部署

### 6.1 Deployment 配置

```yaml
# k8s/deployment.yaml

apiVersion: apps/v1
kind: Deployment
metadata:
  name: opennovel
  labels:
    app: opennovel
spec:
  replicas: 3
  selector:
    matchLabels:
      app: opennovel
  template:
    metadata:
      labels:
        app: opennovel
    spec:
      containers:
        - name: opennovel
          image: opennovel:latest
          ports:
            - containerPort: 3000
          env:
            - name: DATABASE_URL
              valueFrom:
                secretKeyRef:
                  name: opennovel-secrets
                  key: database-url
            - name: REDIS_URL
              valueFrom:
                secretKeyRef:
                  name: opennovel-secrets
                  key: redis-url
            - name: LLM_API_KEY
              valueFrom:
                secretKeyRef:
                  name: opennovel-secrets
                  key: llm-api-key
          resources:
            requests:
              memory: "512Mi"
              cpu: "250m"
            limits:
              memory: "1Gi"
              cpu: "500m"
          livenessProbe:
            httpGet:
              path: /health
              port: 3000
            initialDelaySeconds: 10
            periodSeconds: 30
          readinessProbe:
            httpGet:
              path: /health
              port: 3000
            initialDelaySeconds: 5
            periodSeconds: 10
---
apiVersion: v1
kind: Service
metadata:
  name: opennovel
spec:
  selector:
    app: opennovel
  ports:
    - port: 80
      targetPort: 3000
  type: LoadBalancer
---
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: opennovel-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: opennovel
  minReplicas: 2
  maxReplicas: 10
  metrics:
    - type: Resource
      resource:
        name: cpu
        target:
          type: Utilization
          averageUtilization: 70
```

---

## 七、监控与日志

### 7.1 Prometheus 配置

```yaml
# prometheus.yml

global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'opennovel'
    static_configs:
      - targets: ['opennovel:3000']
```

### 7.2 Grafana Dashboard

```json
{
  "dashboard": {
    "title": "OpenNovel Dashboard",
    "panels": [
      {
        "title": "Request Rate",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(http_requests_total[5m])"
          }
        ]
      },
      {
        "title": "Response Time",
        "type": "graph",
        "targets": [
          {
            "expr": "histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m]))"
          }
        ]
      },
      {
        "title": "Error Rate",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(http_requests_total{status=~\"5..\"}[5m])"
          }
        ]
      }
    ]
  }
}
```

### 7.3 日志配置

```rust
// 使用 tracing 进行结构化日志

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn init_logging() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}

// 使用示例
tracing::info!(
    book_id = %book_id,
    agent = %agent_name,
    "Processing message"
);
```

---

## 八、任务分解

### Task 10.1: API 文档（2 天）

| 子任务 | 工时 | 说明 |
|--------|------|------|
| 10.1.1 OpenAPI 规范 | 1 天 | 完整 API 定义 |
| 10.1.2 Swagger UI 集成 | 0.5 天 | 文档界面 |
| 10.1.3 示例代码 | 0.5 天 | curl/Python 示例 |

### Task 10.2: 用户指南（2 天）

| 子任务 | 工时 | 说明 |
|--------|------|------|
| 10.2.1 快速开始 | 0.5 天 | 安装、配置、第一本小说 |
| 10.2.2 创作流程 | 1 天 | 三阶段、Agent 使用 |
| 10.2.3 可视化功能 | 0.5 天 | 人物关系图、世界地图 |

### Task 10.3: 开发者文档（1 天）

| 子任务 | 工时 | 说明 |
|--------|------|------|
| 10.3.1 架构文档 | 0.5 天 | 模块说明、数据流 |
| 10.3.2 贡献指南 | 0.5 天 | 开发环境、代码风格 |

### Task 10.4: Docker 部署（2 天）

| 子任务 | 工时 | 说明 |
|--------|------|------|
| 10.4.1 Dockerfile | 0.5 天 | 多阶段构建 |
| 10.4.2 Docker Compose | 0.5 天 | 本地部署配置 |
| 10.4.3 环境变量 | 0.5 天 | 配置管理 |
| 10.4.4 测试部署 | 0.5 天 | 验证部署流程 |

### Task 10.5: 监控与日志（1 天）

| 子任务 | 工时 | 说明 |
|--------|------|------|
| 10.5.1 Prometheus 指标 | 0.5 天 | 关键指标暴露 |
| 10.5.2 日志配置 | 0.5 天 | 结构化日志 |

---

## 九、验收标准

### 9.1 文档

- [ ] OpenAPI 规范完整
- [ ] Swagger UI 可访问
- [ ] 用户指南覆盖主要功能
- [ ] 开发者文档完整

### 9.2 部署

- [ ] Docker 镜像构建成功
- [ ] Docker Compose 一键启动
- [ ] 生产环境部署成功

### 9.3 监控

- [ ] Prometheus 指标正常采集
- [ ] Grafana Dashboard 可用
- [ ] 日志正常输出