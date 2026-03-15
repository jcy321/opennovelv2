# Oh-My-OpenCode 架构研究报告

> 本报告基于对 Oh-My-OpenCode 的深入调查，为 OpenNovel v2 的设计提供参考

---

## 一、核心组件清单

| 组件 | 职责 | 依赖关系 |
|------|------|----------|
| **Plugin Core** | 插件入口与初始化 | TypeScript, Zod v4 |
| **Agent System** | 11+ 专家 agent 编排 | Model requirements, category mapping |
| **Skill System** | 46+ skills 加载与MCP嵌入 | SkillMCPManager |
| **Hooks System** | 45+ 生命周期钩子 | Plugin context |
| **Tools Registry** | 26 个 agent 工具注册 | LSP, AST-Grep, session tools |
| **MCP System** | 3 内置 + 技能嵌入 MCP | Exa, Context7, Grep.app |
| **Background Tasks** | 并发后台任务管理 | Tmux, session |
| **Tmux Integration** | 交互式终端支持 | tmux, bash |
| **Config System** | 多级 JSONC 配置 | Zod v4 validation |

---

## 二、Agent 协作机制

### 2.1 Agent 职责划分

```
┌─────────────────────────────────────────────────────────────┐
│                      Sisyphus (主编排器)                      │
│           职责: 任务分解 → 委派 → 验证（不生成代码）           │
└─────────────────────────────────────────────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        ▼                     ▼                     ▼
┌───────────────┐   ┌─────────────────┐   ┌──────────────────┐
│   Prometheus  │   │    Hephaestus   │   │     Atlas        │
│  (战略规划器)  │   │ (代码生成专家)  │   │ (执行编排器)     │
│  → 访谈模式    │   │ → 目标导向      │   │ → Todo 协调      │
│  → 生成计划    │   │ → 自主执行      │   │ → 并行验证       │
└───────────────┘   └─────────────────┘   └──────────────────┘
        │                     │                     │
        ▼                     ▼                     ▼
┌───────────────┐   ┌─────────────────┐   ┌──────────────────┐
│    Metis      │   │     Oracle      │   │    Librarian     │
│ (计划顾问)    │   │ (架构顾问)      │   │ (资料专家)       │
│ → Gap 分析    │   │ → 架构设计      │   │ → OSS 考证       │
│ → 失败模式识别 │   │ → 代码审查      │   │ → 文档检索       │
└───────────────┘   └─────────────────┘   └──────────────────┘
                              │
                              ▼
                    ┌──────────────────┐
                    │    Explore       │
                    │ (代码探索专家)   │
                    │ → 快速模式匹配   │
                    │ → 类 grep 行为   │
                    └──────────────────┘
```

### 2.2 Agent 工作流模式

| 模式 | 触发条件 | 执行流程 | 适用场景 |
|------|----------|----------|----------|
| **/start-work** | Prometheus 计划完成 | Atlas编排 → Hephaestus执行 → Oracle审查 | 复杂任务 |
| **/init-deep** | 用户命令 | 自动扫描项目 → 生成 AGENTS.md 层级 | 初始化 |
| **ultrawork/ulw** | 简单命令 | Sisyphus → Hephaestus → Oracle → Done | 自动工作流 |
| **/ralph-loop** | 用户命令 | 自我引用循环 → 直到DONE | 持续任务 |

### 2.3 Category → Model 映射

| Category | 模型 | 温度 | Agent | 用途 |
|----------|------|------|-------|------|
| `visual-engineering` | `qwen3-coder-next` | 0.3 | Hephaestus | UI/UX/前端设计 |
| `ultrabrain` | `qwen3-coder-next` | 0.3 | Hephaestus | 复杂逻辑/架构决策 |
| `deep` | `qwen3-coder-next` | 0.3 | Hephaestus | 深度研究+执行 |
| `artistry` | `qwen3-coder-next` | 0.4 | Hephaestus | 创意方案生成 |
| `quick` | `qwen3-coder-next` | 0.3 | Hephaestus | 单文件修改/错字 |

### 2.4 意图门 (IntentGate) 工作流程

```
用户请求 → Intent Gate (意图分析)
           ↓
      分类真实意图 (不是字面意思)
           ↓
    分配到正确的 Category
           ↓
    选择对应的 Model
           ↓
    分配给正确的 Agent
```

---

## 三、工作流设计

### 3.1 Sisyphus → Prometheus → Atlas 工作流

```
┌──────────────────────────────────────────────────────────────────────┐
│ 1. 用户请求 (e.g., "在 React 应用中添加 Auth")                        │
└──────────────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌──────────────────────────────────────────────────────────────────────┐
│ 2. Sisyphus (主编排器)                                                 │
│    - 意图分类 (IntentGate)                                             │
│    - 任务分解 (if complex)                                             │
│    - 决策: 是否需要详细计划?                                           │
└──────────────────────────────────────────────────────────────────────┘
                            │
            ┌───────────────┴───────────────┐
            ▼                               ▼
    简单任务                           复杂任务
            │                               │
            ▼                               ▼
┌─────────────────────┐         ┌────────────────────────┐
│ 3a. Hephaestus      │         │ 3b. Prometheus         │
│     - 直接生成代码    │         │    - "访谈模式"        │
│     - 简单验证        │         │    - 澄清需求         │
│     - 完成            │         │    - 生成详细计划      │
└─────────────────────┘         └────────────────────────┘
                                        │
                                        ▼
                             ┌────────────────────────┐
                             │ 4. Metis (计划顾问)     │
                             │    - Gap 分析           │
                             │    - 失败模式识别       │
                             └────────────────────────┘
                                        │
                                        ▼
                             ┌────────────────────────┐
                             │ 5. Momus (质量评审)     │
                             │    - 计划清晰度检查     │
                             │    - 可验证性验证       │
                             └────────────────────────┘
                                        │
                                        ▼
                             ┌────────────────────────┐
                             │ 6. /start-work Trigger │
                             │    - Atlas 编排          │
                             └────────────────────────┘
                                        │
                    ┌───────────────────┼───────────────────┐
                    ▼                   ▼                   ▼
           ┌───────────────┐  ┌───────────────┐  ┌───────────────┐
           │ 7. Hephaestus │  │  Oracle      │  │  Librarian    │
           │  - 代码生成    │  │  - 架构审查   │  │  - OSS 考证   │
           └───────────────┘  └───────────────┘  └───────────────┘
                    │                   │                   │
                    └───────────────────┴───────────────────┘
                                        ▼
                             ┌────────────────────────┐
                             │ 8. Atlas 校验完成       │
                             │    - 所有子任务完成     │
                             │    - 质量保证           │
                             └────────────────────────┘
```

### 3.2 Session 传递机制

```
Session 链式传递:
  主 Session (Sisyphus)
    ↓ (创建子会话)
  计划 Session (Prometheus)
    ↓ (委派任务)
  执行 Session (Hephaestus)
    ↓ (并行执行)
  审查 Session (Oracle)
```

---

## 四、Hooks 触发时机

| Tier | Hooks | 触发时机 |
|------|-------|----------|
| **Tier 1: Session (23)** | context-window-monitor, preemptive-compaction | Session 创建/更新 |
| **Tier 2: Tool Guard (10)** | comment-checker, hashline-read-enhancer | 工具调用前 |
| **Tier 3: Transform (4)** | keyword-detector, context-injector | 消息转换 |
| **Tier 4: Continuation (7)** | todo-continuation-enforcer (Boulder) | Session Idle |
| **Tier 5: Skill (2)** | category-skill-reminder | Skill 加载 |

**关键 Hooks**:
- `todo-continuation-enforcer` (Boulder): 30s + 指数退避
- `hashline-read-enhancer`: 每行添加 `LINE#ID` 哈希
- `comment-checker`: 检查并清理 AI 味评论
- `ralph-loop`: 自我引用循环控制

---

## 五、MCP 集成机制

### 5.1 三层架构

| Level | 来源 | 机制 | 例子 |
|-------|------|------|------|
| **Tier 1** | Built-in | HTTP Remote | websearch, context7, grep_app |
| **Tier 2** | Claude Code | .mcp.json | 外部 MCP 服务器 |
| **Tier 3** | Skill-embedded | SkillMcpManager | Skill 带自己的 MCP |

### 5.2 内置 MCP servers

```json
{
  "websearch": {
    "url": "https://mcp.exa.ai/mcp",
    "type": "remote"
  },
  "context7": {
    "url": "https://mcp.context7.com/mcp",
    "type": "remote"
  },
  "grep_app": {
    "url": "https://mcp.grep.app",
    "type": "remote"
  }
}
```

---

## 六、工具系统 (26 个工具)

| 工具类别 | 工具 | 用途 |
|----------|------|------|
| **任务委派** | `task` | 主工具，调用子 agent |
| **LSP Refactoring** | `lsp_rename`, `lsp_goto_definition` | IDE 级重构 |
| **AST 搜索** | `ast_grep_search`, `ast_grep_replace` | 模式匹配重写 |
| **会话管理** | `session_list`, `session_read`, `session_search` | Session 历史 |
| **交互终端** | `interactive_bash` | Tmux 集成 |
| **媒体分析** | `look_at` | PDF/图片分析 |
| **编辑工具** | `hashline_edit` | 哈希验证编辑 |

---

## 七、可复用的设计决策

### 7.1 意图优先 (Intent-First)

> "不要相信字面意思，先分析真实意图"

- IntentGate 在所有任务前进行意图分类
- Category → Model → Agent 自动映射
- 用户无需知道具体模型，只需描述任务类型

### 7.2 模块化 Agent System

> "分工明确的专家团队，而非通用智能体"

- 11+ 个 Agent，每个专精于特定领域
- Sisyphus 编排 + Hephaestus 执行 + Oracle 审查
- 模型与职责精确匹配

### 7.3 Skill-Embedded MCP

> "按需加载 MCP，避免上下文膨胀"

- Skills 携带自己的 MCP servers
- 任务开始时启动，任务完成即销毁
- 上下文窗口保持清洁

### 7.4 Hash-Anchored Edits

> "哈希验证，杜绝马具问题 (Harness Problem)"

- 每行添加 `LINE#ID` 哈希
- 编辑前哈希验证，不匹配则拒绝
- Grok Code Fast 1: 6.7% → 68.3% 成功率

### 7.5 连续工作模式

> "Auto-resume, no manual intervention"

- Ralph Loop / Boulder 机制
- Session Idle → 注入延续提示
- 自我引用直到 `DONE`

### 7.6 多级配置合并

> "优先级清晰，灵活可定制"

```
Project (.opencode/) > User (~/.config/opencode/) > Defaults
```

---

## 八、对 OpenNovel 的启示

### 8.1 可复用的模式

| OpenCode模式 | OpenNovel映射 |
|-------------|---------------|
| Sisyphus (编排器) | 观察者 (协作枢纽) |
| Hephaestus (执行专家) | 执笔 (唯一写入者) |
| Oracle (架构顾问) | 天道 (剧情设计) |
| Librarian (资料专家) | 调研者 (爆点分析) |
| Explore (代码探索) | 知识库检索工具 |
| Metis (计划顾问) | 规划者 (新书规划) |
| Momus (质量评审) | 审阅 (阅读体验评估) |
| IntentGate | 群聊意图解析 |

### 8.2 可借鉴的机制

1. **意图门机制** - 用户消息先经过意图分析，决定调用哪个Agent
2. **Session链式传递** - Agent之间的协作通过Session ID传递上下文
3. **Skill-Embedded MCP** - 每个Agent可以有自己的MCP服务器
4. **Hook分层** - 不同层级的Hook在不同时机触发
5. **并发控制** - 每个Provider/Model独立限流
6. **Fallback机制** - 错误时自动切换到备用模型

### 8.3 需要调整的差异

| OpenCode | OpenNovel差异 |
|----------|--------------|
| 代码编辑 | 小说章节撰写 |
| LSP/AST工具 | 知识库向量检索 |
| Git版本管理 | WebDAV云盘同步 |
| 一次性任务 | 长期协作项目 |
| 被动响应Agent | 主动介入Agent |

---

**报告版本**: v1.0
**调查时间**: 2026-03-15
**数据来源**: OpenCode/Oh-My-OpenCode 源码分析 + 配置文件