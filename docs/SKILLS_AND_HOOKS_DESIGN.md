# OpenNovel v2 Agent Skills & Hooks 设计

> 本文档详细定义八大Agent的Skills和Hooks配置

---

## 一、Skills系统设计

### 1.1 Skill结构定义

```markdown
---
name: skill-name
description: Skill描述
trigger_keywords: [关键词1, 关键词2]
tools: [Tool1, Tool2]
mcp:
  - name: mcp-name
    type: stdio
    command: npx
    args: [-y, mcp-server]
---
Skill指令内容...
```

### 1.2 八大Agent Skills配置

#### 规划者 Skills

| Skill名称 | 描述 | 触发关键词 |
|----------|------|-----------|
| `worldview-design` | 世界观设计 | 世界观、设定、背景 |
| `character-creation` | 人物创建 | 创建角色、人物设定 |
| `faction-design` | 势力阵营设计 | 势力、阵营、派系 |
| `plot-outline` | 大纲规划 | 大纲、规划、篇幅 |
| `foreshadowing-setup` | 伏笔预设 | 伏笔、悬念 |

#### 天道 Skills

| Skill名称 | 描述 | 触发关键词 |
|----------|------|-----------|
| `plot-evolution` | 剧情演化 | 剧情、发展、推进 |
| `character-arc` | 角色弧线 | 弧线、成长、变化 |
| `conflict-design` | 冲突设计 | 冲突、矛盾、对抗 |
| `foreshadowing-trigger` | 伏笔触发 | 触发伏笔、揭晓 |
| `accident-factor` | 意外因素 | 意外、随机、转折 |
| `pressure-check` | 剧情压力检查 | 压力、节奏 |

#### 世界观守护者 Skills

| Skill名称 | 描述 | 触发关键词 |
|----------|------|-----------|
| `consistency-check` | 一致性检查 | 一致性、检查、冲突 |
| `setting-validate` | 设定验证 | 设定、验证、合理 |
| `timeline-check` | 时间线检查 | 时间线、时间顺序 |
| `worldview-update` | 世界观更新 | 更新设定、扩展 |

#### 刘和平 Skills

| Skill名称 | 描述 | 触发关键词 |
|----------|------|-----------|
| `dialogue-writing` | 对话写作 | 对话、台词 |
| `character-behavior` | 人物行为分析 | 行为、反应、性格 |
| `interaction-design` | 互动设计 | 互动、关系、冲突 |
| `voice-consistency` | 声音一致性 | 口吻、风格、语气 |

#### 执笔 Skills

| Skill名称 | 描述 | 触发关键词 |
|----------|------|-----------|
| `prose-writing` | 散文写作 | 写、描写、叙述 |
| `scene-description` | 场景描写 | 场景、环境、氛围 |
| `action-writing` | 动作描写 | 动作、打斗、战斗 |
| `emotion-writing` | 情感描写 | 情感、心理、感受 |
| `style-adjustment` | 风格调整 | 风格、文风、调整 |

#### 审阅 Skills

| Skill名称 | 描述 | 触发关键词 |
|----------|------|-----------|
| `quality-review` | 质量审核 | 审核、检查、评价 |
| `readability-check` | 可读性检查 | 可读性、流畅 |
| `literary-evaluation` | 文学性评估 | 文学性、艺术性 |
| `explosive-point` | 爆点评估 | 爆点、传播、热点 |

#### 观察者 Skills

| Skill名称 | 描述 | 触发关键词 |
|----------|------|-----------|
| `knowledge-sync` | 知识库同步 | 同步、更新知识库 |
| `agent-coordinate` | Agent协调 | 协调、调度 |
| `session-manage` | 会话管理 | 会话、状态 |

#### 调研者 Skills

| Skill名称 | 描述 | 触发关键词 |
|----------|------|-----------|
| `market-analysis` | 市场分析 | 市场、分析、趋势 |
| `explosive-reference` | 爆款参考 | 爆款、参考、分析 |
| `reader-portrait` | 读者画像 | 读者、受众、画像 |

---

## 二、Hooks系统设计

### 2.1 Hook类型定义

| 类型 | 触发时机 | 用途 |
|------|---------|------|
| `pre_process` | 消息处理前 | 意图分析、权限检查 |
| `post_process` | 消息处理后 | 结果验证、状态更新 |
| `pre_write` | 章节写入前 | 一致性预检查 |
| `post_write` | 章节写入后 | 批注收集、知识库更新 |
| `on_conflict` | 冲突检测时 | 冲突通知、仲裁请求 |
| `on_stage_change` | 阶段切换时 | Agent锁定/解锁 |
| `on_idle` | 空闲时 | 继续提示、主动介入 |

### 2.2 八大Agent Hooks配置

#### 规划者 Hooks

```json
{
  "hooks": {
    "on_stage_change": {
      "stage_2_start": "锁定自身，禁止后续接入",
      "stage_3_start": "清理会话状态"
    }
  }
}
```

#### 天道 Hooks

```json
{
  "hooks": {
    "post_write": {
      "trigger": "chapter_written",
      "action": "检查伏笔池，更新剧情压力表"
    },
    "pre_process": {
      "trigger": "user_message",
      "action": "检查是否需要主动介入（基于压力表）"
    }
  }
}
```

#### 世界观守护者 Hooks

```json
{
  "hooks": {
    "pre_write": {
      "trigger": "before_chapter_write",
      "action": "世界观一致性预检查"
    },
    "post_write": {
      "trigger": "after_chapter_write",
      "action": "生成世界观批注"
    },
    "on_conflict": {
      "trigger": "worldview_conflict",
      "action": "在群内@用户说明冲突"
    }
  }
}
```

#### 刘和平 Hooks

```json
{
  "hooks": {
    "post_write": {
      "trigger": "after_chapter_write",
      "action": "人物一致性批注生成"
    },
    "on_conflict": {
      "trigger": "character_conflict",
      "action": "人物行为冲突通知"
    }
  }
}
```

#### 执笔 Hooks

```json
{
  "hooks": {
    "pre_write": {
      "trigger": "before_chapter_write",
      "action": "加载本章知识库、人物信息"
    },
    "post_write": {
      "trigger": "after_chapter_write",
      "action": "通知观察者收集批注"
    },
    "on_conflict": {
      "trigger": "annotation_conflict",
      "action": "在群内@用户请求裁决"
    }
  }
}
```

#### 审阅 Hooks

```json
{
  "hooks": {
    "post_write": {
      "trigger": "after_chapter_write",
      "action": "生成质量评估报告"
    }
  }
}
```

#### 观察者 Hooks

```json
{
  "hooks": {
    "post_write": {
      "trigger": "after_chapter_write",
      "action": [
        "更新历史情节知识库",
        "归档本章知识库",
        "触发WebDAV同步"
      ]
    },
    "on_stage_change": {
      "stage_1_to_2": "建立知识库目录结构",
      "stage_2_to_3": "锁定规划者和调研者"
    }
  }
}
```

#### 调研者 Hooks

```json
{
  "hooks": {
    "on_stage_change": {
      "stage_2_start": "开始爆点分析",
      "stage_3_start": "锁定自身，禁止后续接入"
    }
  }
}
```

### 2.3 全局Hooks

```json
{
  "global_hooks": {
    "intent_gate": {
      "trigger": "user_message",
      "action": "分析用户意图，决定调用哪个Agent"
    },
    "proactive_intervention": {
      "trigger": "agent_message",
      "action": "检查是否需要其他Agent主动介入"
    },
    "stage_monitor": {
      "trigger": "session_idle",
      "action": "检查是否满足阶段切换条件"
    },
    "todo_continuation": {
      "trigger": "session_idle",
      "interval": "30s",
      "action": "检查未完成任务，注入继续提示"
    }
  }
}
```

---

## 三、工具系统设计

### 3.1 知识库工具

| 工具名称 | 描述 | 使用Agent |
|---------|------|----------|
| `knowledge_search` | 向量搜索知识库 | 所有Agent |
| `character_query` | 查询人物信息 | 刘和平、执笔 |
| `worldview_query` | 查询世界观设定 | 世界观守护者、天道 |
| `foreshadowing_query` | 查询伏笔池 | 天道 |
| `map_query` | 查询地图位置信息 | 天道、执笔 |

### 3.2 写作工具

| 工具名称 | 描述 | 使用Agent |
|---------|------|----------|
| `write_chapter` | 撰写章节 | 执笔 |
| `add_annotation` | 添加批注 | 除执笔外所有Agent |
| `update_knowledge` | 更新知识库 | 天道、观察者 |

### 3.3 协作工具

| 工具名称 | 描述 | 使用Agent |
|---------|------|----------|
| `invoke_agent` | 调用其他Agent | 观察者 |
| `broadcast_message` | 广播消息到群聊 | 所有Agent |
| `request_arbitration` | 请求仲裁 | 执笔 |

---

## 四、MCP服务器配置

### 4.1 内置MCP

```json
{
  "mcp_servers": {
    "knowledge_mcp": {
      "type": "stdio",
      "command": "node",
      "args": ["mcp/knowledge-server.js"],
      "tools": ["knowledge_search", "knowledge_update"]
    },
    "worldview_mcp": {
      "type": "stdio",
      "command": "node",
      "args": ["mcp/worldview-server.js"],
      "tools": ["consistency_check", "worldview_validate"]
    }
  }
}
```

### 4.2 Skill-Embedded MCP

部分Skills可以携带自己的MCP服务器：

```markdown
---
name: reference-analysis
mcp:
  - name: websearch
    type: remote
    url: https://mcp.exa.ai/mcp
---
```

---

## 五、Agent完整配置示例

### 天道完整配置

```json
{
  "tian_dao": {
    "model": "bailian-coding-plan/glm-5",
    "temperature": 0.3,
    "thinking": {
      "type": "enabled",
      "budgetTokens": 8192
    },
    "reasoningEffort": "xhigh",
    "maxTokens": 16384,
    "skills": [
      "plot-evolution",
      "character-arc",
      "conflict-design",
      "foreshadowing-trigger",
      "accident-factor"
    ],
    "hooks": {
      "post_write": "check_foreshadowing_pool",
      "pre_process": "check_pressure_table"
    },
    "tools": {
      "knowledge_search": true,
      "foreshadowing_query": true,
      "update_knowledge": true
    },
    "permissions": {
      "edit_worldview": "allow",
      "edit_plot": "allow",
      "edit_foreshadowing": "allow",
      "write_chapter": "deny",
      "annotate": "allow"
    },
    "mcp_servers": ["knowledge_mcp"]
  }
}
```

### 执笔完整配置

```json
{
  "writer": {
    "model": "bailian-coding-plan/glm-5",
    "temperature": 0.7,
    "thinking": {
      "type": "enabled",
      "budgetTokens": 8192
    },
    "maxTokens": 32768,
    "skills": [
      "prose-writing",
      "scene-description",
      "action-writing",
      "emotion-writing",
      "style-adjustment"
    ],
    "hooks": {
      "pre_write": "load_chapter_context",
      "post_write": "notify_observer",
      "on_conflict": "request_arbitration"
    },
    "tools": {
      "knowledge_search": true,
      "character_query": true,
      "write_chapter": true
    },
    "permissions": {
      "write_chapter": "allow",
      "edit_chapter": "ask",
      "annotate": "deny"
    }
  }
}
```

---

**文档版本**: v2.3
**更新时间**: 2026-03-15