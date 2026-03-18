# AI-Reader-V2 MCP 集成方案

**生成日期**: 2026-03-17
**状态**: 设计方案

---

## 一、MCP (Model Context Protocol) 概述

### 1.1 什么是 MCP？

MCP (Model Context Protocol) 是 Anthropic 提出的标准化协议，用于让 LLM 应用与外部工具/数据源交互。

```
┌──────────────────┐     MCP Protocol      ┌──────────────────┐
│   MCP Client     │ ◄───────────────────► │   MCP Server     │
│  (Claude/LLM)    │     JSON-RPC 2.0      │  (Tool Provider) │
└──────────────────┘                       └──────────────────┘
```

### 1.2 MCP 核心概念

| 概念 | 说明 | AI-Reader 对应 |
|------|------|---------------|
| **Tool** | 可调用的函数 | `get_character_graph`, `get_world_map` |
| **Resource** | 可读取的数据源 | 小说数据、分析结果 |
| **Prompt** | 预定义的提示模板 | 分析模板、可视化模板 |

### 1.3 为什么选择 MCP？

| 方案 | 优点 | 缺点 |
|------|------|------|
| **直接 HTTP API** | 简单直接 | 需要在 OpenNovel 中硬编码 API 调用 |
| **MCP Server** | 标准化协议、可被多种客户端复用 | 需要额外实现 MCP 层 |
| **两者结合** | ✅ 灵活 + 标准化 | 工作量稍大 |

**推荐**：实现 MCP Server 封装 AI-Reader-V2 API，同时保留直接 HTTP 调用能力。

---

## 二、MCP Server 设计

### 2.1 架构图

```
┌─────────────────────────────────────────────────────────────────────┐
│                         OpenNovel System                             │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌─────────────────────────────────────────────────────────────────┐│
│  │                    MCP Client (Rust)                            ││
│  │                                                                 ││
│  │   使用 mcp-rust-sdk 或自实现 JSON-RPC 2.0 客户端                ││
│  │                                                                 ││
│  └─────────────────────────────────────────────────────────────────┘│
│                              │                                       │
│                              │ MCP Protocol (JSON-RPC 2.0)          │
│                              ▼                                       │
│  ┌─────────────────────────────────────────────────────────────────┐│
│  │              AI-Reader MCP Server (Python)                      ││
│  │                                                                 ││
│  │   基于 FastMCP 或 mcp-python-sdk 实现                           ││
│  │                                                                 ││
│  │   Tools:                                                        ││
│  │   - get_character_graph(novel_id, chapter_range)               ││
│  │   - get_world_map(novel_id, chapter_range, layer_id)           ││
│  │   - get_entity_profile(novel_id, entity_name, type)            ││
│  │   - get_timeline(novel_id, chapter_range)                      ││
│  │   - analyze_novel(novel_id, force)                             ││
│  │   - get_analysis_status(novel_id)                              ││
│  │                                                                 ││
│  │   Resources:                                                    ││
│  │   - novels://list                                               ││
│  │   - novels://{novel_id}/chapters                               ││
│  │   - novels://{novel_id}/entities                               ││
│  │                                                                 ││
│  └─────────────────────────────────────────────────────────────────┘│
│                              │                                       │
│                              │ HTTP API                             │
│                              ▼                                       │
│  ┌─────────────────────────────────────────────────────────────────┐│
│  │                  AI-Reader-V2 Backend                           ││
│  │                    (Python + FastAPI)                           ││
│  └─────────────────────────────────────────────────────────────────┘│
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### 2.2 MCP Server 实现 (Python)

```python
# ai_reader_mcp_server.py
"""
AI-Reader-V2 MCP Server
提供小说可视化分析工具给 OpenNovel Agents 使用
"""

import json
from typing import Any, Optional
from mcp.server import Server
from mcp.server.stdio import stdio_server
from mcp.types import Tool, TextContent

import httpx

# AI-Reader-V2 Backend URL
AI_READER_URL = "http://localhost:8000"

app = Server("ai-reader-v2-mcp")


# ─────────────────────────────────────────────────────────────────────
# Tools
# ─────────────────────────────────────────────────────────────────────

@app.list_tools()
async def list_tools() -> list[Tool]:
    """返回所有可用工具"""
    return [
        Tool(
            name="get_character_graph",
            description="""获取小说的人物关系图数据。

返回：
- nodes: 人物节点列表（包含名称、出现章节数、组织、别名）
- edges: 关系边列表（包含来源、目标、关系类型、分类）
- category_counts: 各分类的关系数量

用于天道 Agent 展示人物关系网络。""",
            inputSchema={
                "type": "object",
                "properties": {
                    "novel_id": {
                        "type": "string",
                        "description": "小说 ID"
                    },
                    "chapter_start": {
                        "type": "integer",
                        "description": "起始章节（可选）"
                    },
                    "chapter_end": {
                        "type": "integer",
                        "description": "结束章节（可选）"
                    }
                },
                "required": ["novel_id"]
            }
        ),
        Tool(
            name="get_world_map",
            description="""获取小说的世界地图数据。

返回：
- locations: 地点列表（包含名称、类型、层级、父节点、子节点）
- layout: 布局坐标
- portals: 传送门信息
- layers: 空间层级（人间、天界、冥界等）
- trajectories: 人物轨迹

用于天道、规划者 Agent 展示世界地图和人物轨迹。""",
            inputSchema={
                "type": "object",
                "properties": {
                    "novel_id": {
                        "type": "string",
                        "description": "小说 ID"
                    },
                    "chapter_start": {
                        "type": "integer",
                        "description": "起始章节（可选）"
                    },
                    "chapter_end": {
                        "type": "integer",
                        "description": "结束章节（可选）"
                    },
                    "layer_id": {
                        "type": "string",
                        "description": "空间层级 ID（可选）：overworld/sky/underground/sea"
                    }
                },
                "required": ["novel_id"]
            }
        ),
        Tool(
            name="get_entity_profile",
            description="""获取实体的详细档案。

支持类型：person（人物）、location（地点）、item（物品）、org（组织）

返回：
- 人物：姓名、别名、首次/最后出现章节、关系链、特征
- 地点：名称、类型、层级、父节点、子节点、空间关系

用于刘和平 Agent 查询人物信息，天道 Agent 查询地点详情。""",
            inputSchema={
                "type": "object",
                "properties": {
                    "novel_id": {
                        "type": "string",
                        "description": "小说 ID"
                    },
                    "entity_name": {
                        "type": "string",
                        "description": "实体名称（支持别名）"
                    },
                    "entity_type": {
                        "type": "string",
                        "enum": ["person", "location", "item", "org"],
                        "description": "实体类型（可选，不提供则自动检测）"
                    }
                },
                "required": ["novel_id", "entity_name"]
            }
        ),
        Tool(
            name="get_timeline",
            description="""获取小说的时间线数据。

返回：
- events: 事件列表（按时间排序）
- 包含：角色登场、物品流转、关系变迁、组织变动等

用于审阅 Agent 分析剧情节奏。""",
            inputSchema={
                "type": "object",
                "properties": {
                    "novel_id": {
                        "type": "string",
                        "description": "小说 ID"
                    },
                    "chapter_start": {
                        "type": "integer",
                        "description": "起始章节（可选）"
                    },
                    "chapter_end": {
                        "type": "integer",
                        "description": "结束章节（可选）"
                    }
                },
                "required": ["novel_id"]
            }
        ),
        Tool(
            name="analyze_novel",
            description="""启动小说分析任务。

AI-Reader-V2 会使用 LLM 分析小说内容，提取：
- 人物信息
- 关系网络
- 地点层级
- 事件时间线

分析是异步执行的，可通过 get_analysis_status 查询进度。""",
            inputSchema={
                "type": "object",
                "properties": {
                    "novel_id": {
                        "type": "string",
                        "description": "小说 ID"
                    },
                    "force": {
                        "type": "boolean",
                        "description": "是否强制重新分析（默认 false）"
                    }
                },
                "required": ["novel_id"]
            }
        ),
        Tool(
            name="get_analysis_status",
            description="""获取小说分析状态。

返回：
- status: pending/running/paused/completed/failed
- progress: 分析进度（已完成/总数）
- stats: 已提取的实体、关系、事件数量""",
            inputSchema={
                "type": "object",
                "properties": {
                    "novel_id": {
                        "type": "string",
                        "description": "小说 ID"
                    }
                },
                "required": ["novel_id"]
            }
        ),
    ]


@app.call_tool()
async def call_tool(name: str, arguments: Any) -> list[TextContent]:
    """执行工具调用"""
    async with httpx.AsyncClient(timeout=60.0) as client:
        
        if name == "get_character_graph":
            novel_id = arguments["novel_id"]
            params = {}
            if "chapter_start" in arguments:
                params["chapter_start"] = arguments["chapter_start"]
            if "chapter_end" in arguments:
                params["chapter_end"] = arguments["chapter_end"]
            
            resp = await client.get(
                f"{AI_READER_URL}/api/novels/{novel_id}/graph",
                params=params
            )
            data = resp.json()
            
            return [TextContent(
                type="text",
                text=json.dumps(data, ensure_ascii=False, indent=2)
            )]
        
        elif name == "get_world_map":
            novel_id = arguments["novel_id"]
            params = {}
            if "chapter_start" in arguments:
                params["chapter_start"] = arguments["chapter_start"]
            if "chapter_end" in arguments:
                params["chapter_end"] = arguments["chapter_end"]
            if "layer_id" in arguments:
                params["layer_id"] = arguments["layer_id"]
            
            resp = await client.get(
                f"{AI_READER_URL}/api/novels/{novel_id}/map",
                params=params
            )
            data = resp.json()
            
            return [TextContent(
                type="text",
                text=json.dumps(data, ensure_ascii=False, indent=2)
            )]
        
        elif name == "get_entity_profile":
            novel_id = arguments["novel_id"]
            entity_name = arguments["entity_name"]
            params = {}
            if "entity_type" in arguments:
                params["type"] = arguments["entity_type"]
            
            resp = await client.get(
                f"{AI_READER_URL}/api/novels/{novel_id}/entities/{entity_name}",
                params=params
            )
            data = resp.json()
            
            return [TextContent(
                type="text",
                text=json.dumps(data, ensure_ascii=False, indent=2)
            )]
        
        elif name == "get_timeline":
            novel_id = arguments["novel_id"]
            params = {}
            if "chapter_start" in arguments:
                params["chapter_start"] = arguments["chapter_start"]
            if "chapter_end" in arguments:
                params["chapter_end"] = arguments["chapter_end"]
            
            resp = await client.get(
                f"{AI_READER_URL}/api/novels/{novel_id}/timeline",
                params=params
            )
            data = resp.json()
            
            return [TextContent(
                type="text",
                text=json.dumps(data, ensure_ascii=False, indent=2)
            )]
        
        elif name == "analyze_novel":
            novel_id = arguments["novel_id"]
            force = arguments.get("force", False)
            
            # Get novel info to find chapter range
            novel_resp = await client.get(f"{AI_READER_URL}/api/novels/{novel_id}")
            novel_data = novel_resp.json()
            chapter_count = novel_data.get("chapter_count", 0)
            
            resp = await client.post(
                f"{AI_READER_URL}/api/novels/{novel_id}/analysis/start",
                json={
                    "chapter_start": 1,
                    "chapter_end": chapter_count,
                    "force": force
                }
            )
            data = resp.json()
            
            return [TextContent(
                type="text",
                text=json.dumps({
                    "task_id": data.get("task_id"),
                    "status": "started",
                    "message": f"分析任务已启动，共 {chapter_count} 章"
                }, ensure_ascii=False, indent=2)
            )]
        
        elif name == "get_analysis_status":
            novel_id = arguments["novel_id"]
            
            resp = await client.get(
                f"{AI_READER_URL}/api/novels/{novel_id}/analysis/status"
            )
            data = resp.json()
            
            return [TextContent(
                type="text",
                text=json.dumps(data, ensure_ascii=False, indent=2)
            )]
        
        else:
            return [TextContent(
                type="text",
                text=f"Unknown tool: {name}"
            )]


# ─────────────────────────────────────────────────────────────────────
# Resources
# ─────────────────────────────────────────────────────────────────────

@app.list_resources()
async def list_resources() -> list[dict]:
    """返回所有可用资源"""
    async with httpx.AsyncClient(timeout=10.0) as client:
        resp = await client.get(f"{AI_READER_URL}/api/novels")
        novels = resp.json().get("novels", [])
        
        resources = []
        for novel in novels:
            resources.append({
                "uri": f"novels://{novel['id']}",
                "name": novel['title'],
                "mimeType": "application/json"
            })
        
        return resources


@app.read_resource()
async def read_resource(uri: str) -> str:
    """读取资源内容"""
    if uri.startswith("novels://"):
        novel_id = uri[9:]
        
        async with httpx.AsyncClient(timeout=10.0) as client:
            resp = await client.get(f"{AI_READER_URL}/api/novels/{novel_id}")
            data = resp.json()
            return json.dumps(data, ensure_ascii=False, indent=2)
    
    raise ValueError(f"Unknown resource: {uri}")


# ─────────────────────────────────────────────────────────────────────
# Main
# ─────────────────────────────────────────────────────────────────────

async def main():
    async with stdio_server() as (read_stream, write_stream):
        await app.run(
            read_stream,
            write_stream,
            app.create_initialization_options()
        )


if __name__ == "__main__":
    import asyncio
    asyncio.run(main())
```

### 2.3 MCP Client 集成 (Rust)

在 OpenNovel 中集成 MCP Client：

```rust
// src/tools/mcp_client.rs
use serde::{Deserialize, Serialize};
use tokio::process::Command;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// MCP Client for AI-Reader-V2
pub struct AiReaderMcpClient {
    process: Option<tokio::process::Child>,
}

impl AiReaderMcpClient {
    pub fn new() -> Self {
        Self { process: None }
    }

    /// 启动 MCP Server 进程
    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut child = Command::new("python")
            .arg("ai_reader_mcp_server.py")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()?;
        
        self.process = Some(child);
        Ok(())
    }

    /// 调用 MCP Tool
    pub async fn call_tool<T: Serialize, R: for<'de> Deserialize<'de>>(
        &mut self,
        tool_name: &str,
        arguments: T,
    ) -> Result<R, Box<dyn std::error::Error>> {
        let request = jsonrpc::Request {
            jsonrpc: "2.0".to_string(),
            method: "tools/call".to_string(),
            params: Some(serde_json::json!({
                "name": tool_name,
                "arguments": arguments
            })),
            id: Some(1),
        };

        let response = self.send_request(&request).await?;
        let result: R = serde_json::from_value(response.result.unwrap())?;
        Ok(result)
    }

    /// 获取人物关系图
    pub async fn get_character_graph(
        &mut self,
        novel_id: &str,
        chapter_start: Option<u32>,
        chapter_end: Option<u32>,
    ) -> Result<GraphData, Box<dyn std::error::Error>> {
        self.call_tool("get_character_graph", serde_json::json!({
            "novel_id": novel_id,
            "chapter_start": chapter_start,
            "chapter_end": chapter_end
        })).await
    }

    /// 获取世界地图
    pub async fn get_world_map(
        &mut self,
        novel_id: &str,
        chapter_start: Option<u32>,
        chapter_end: Option<u32>,
        layer_id: Option<&str>,
    ) -> Result<MapData, Box<dyn std::error::Error>> {
        self.call_tool("get_world_map", serde_json::json!({
            "novel_id": novel_id,
            "chapter_start": chapter_start,
            "chapter_end": chapter_end,
            "layer_id": layer_id
        })).await
    }

    /// 获取实体档案
    pub async fn get_entity_profile(
        &mut self,
        novel_id: &str,
        entity_name: &str,
        entity_type: Option<&str>,
    ) -> Result<EntityProfile, Box<dyn std::error::Error>> {
        self.call_tool("get_entity_profile", serde_json::json!({
            "novel_id": novel_id,
            "entity_name": entity_name,
            "entity_type": entity_type
        })).await
    }
}
```

---

## 三、资源需求评估

### 3.1 AI-Reader-V2 资源消耗分析

基于代码分析（`context_budget.py`, `analysis_service.py`）：

#### 3.1.1 每章 Token 消耗

| 模式 | 每章 Input Tokens | 每章 Output Tokens | 说明 |
|------|-------------------|-------------------|------|
| **本地 Ollama** | 8K-16K | 2K-4K | 受 _OLLAMA_CTX_CAP=16384 限制 |
| **云端 API** | 16K-50K | 4K-8K | 可处理更长章节 |

**每章 Token 计算公式**：
```
Input Tokens ≈ 
  章节文本（截断到 max_chapter_len）
  + 上下文摘要（context_max_chars）
  + 实体词典（如有预扫描）
  + 系统提示词

Output Tokens ≈ 
  ChapterFact JSON（人物、关系、地点、事件）
```

#### 3.1.2 内存占用

| 组件 | 内存占用 | 说明 |
|------|---------|------|
| **FastAPI 进程** | 100-200 MB | Python 运行时 |
| **SQLite** | ~10 MB/小说 | 章节内容 + 分析结果 |
| **ChromaDB** | 100-500 MB | 向量索引（取决于模型） |
| **sentence-transformers** | 200-400 MB | BAAI/bge-base-zh-v1.5 模型 |
| **LLM 调用缓冲** | 50-100 MB | 临时缓冲 |

**峰值内存**：~800 MB - 1.2 GB

#### 3.1.3 CPU 消耗

| 阶段 | CPU 消耗 | 持续时间 |
|------|---------|---------|
| **章节切分** | 低 | 几秒 |
| **实体预扫描** | 中（jieba 分词） | 几十秒 |
| **LLM 提取** | 低（等待 API 响应） | 每章 10-60 秒 |
| **向量嵌入** | 高（本地模型） | 每章 2-5 秒 |
| **可视化计算** | 中 | 几秒 |

### 3.2 2核2G 服务器容量评估

#### 3.2.1 瓶颈分析

| 瓶颈 | 影响 | 解决方案 |
|------|------|---------|
| **内存（2GB）** | ⚠️ 主要瓶颈 | 禁用本地嵌入、使用云端 API |
| **CPU（2核）** | 可接受 | 分析速度慢但可运行 |
| **存储** | 充足 | SQLite 文件很小 |

#### 3.2.2 容量估算

**场景 A：使用云端 LLM + 禁用本地嵌入**

```
配置：
- LLM: 云端 API（DeepSeek/Claude）
- 嵌入: 禁用 ChromaDB，或使用云端向量服务

内存分配：
- FastAPI: 100-150 MB
- SQLite: ~50 MB
- 缓冲: 100-200 MB
- 系统: ~500 MB
- 预留: ~500 MB
---------------------------------
总计: ~800-1000 MB（2GB 内存在安全范围内）

可支撑规模：
- 小说字数：100-300 万字
- 章节数：100-500 章
- 分析时间：约 1-2 秒/章（受网络延迟影响）
```

**场景 B：使用本地 Ollama + 本地嵌入**

```
配置：
- LLM: 本地 Ollama（需要额外内存）
- 嵌入: 本地 ChromaDB

⚠️ 不推荐在 2GB 服务器上运行

原因：
- Ollama + 模型需要 4-8 GB 内存
- sentence-transformers 需要 200-400 MB
- 总内存需求 5-10 GB
```

#### 3.2.3 推荐配置

| 配置 | 内存 | CPU | 适用场景 |
|------|------|-----|---------|
| **最低配置** | 2 GB | 2 核 | 云端 LLM、禁用嵌入、< 100 章小说 |
| **推荐配置** | 4 GB | 4 核 | 云端 LLM、启用嵌入、< 500 章小说 |
| **理想配置** | 8 GB | 4 核 | 本地 Ollama + 嵌入、任意规模 |

### 3.3 优化建议（2核2G 场景）

```python
# .env 配置优化
LLM_PROVIDER=openai              # 使用云端 LLM
LLM_API_KEY=your_api_key
LLM_BASE_URL=https://api.deepseek.com/v1
LLM_MODEL=deepseek-chat

# 禁用本地嵌入（减少 200-400 MB 内存）
ENABLE_EMBEDDINGS=false

# 限制并发
MAX_CONCURRENT_REQUESTS=1

# 减少缓冲区大小
BUFFER_SIZE_MB=50
```

---

## 四、与 OpenNovel Agent 的集成

### 4.1 天道 Agent 集成

```rust
impl TianDaoAgent {
    /// 展示人物关系图（通过 MCP）
    pub async fn show_character_graph_mcp(
        &mut self,
        novel_id: &str,
    ) -> Result<GraphVisualization, Error> {
        // 通过 MCP Client 调用
        let graph = self.mcp_client
            .get_character_graph(novel_id, None, None)
            .await?;
        
        // 生成可视化链接
        let viz = GraphVisualization {
            nodes: graph.nodes.len(),
            edges: graph.edges.len(),
            url: format!("/novels/{}/graph", novel_id),
            data: graph,
        };
        
        // 在群聊中发送
        self.send_message(&format!(
            "📊 人物关系图已生成\n\
             - 人物数量: {}\n\
             - 关系数量: {}\n\
             - 查看链接: {}",
            viz.nodes, viz.edges, viz.url
        )).await?;
        
        Ok(viz)
    }

    /// 展示世界地图（通过 MCP）
    pub async fn show_world_map_mcp(
        &mut self,
        novel_id: &str,
        layer: Option<&str>,
    ) -> Result<MapVisualization, Error> {
        let map = self.mcp_client
            .get_world_map(novel_id, None, None, layer)
            .await?;
        
        // 分析层级结构
        let hierarchy = self.analyze_map_hierarchy(&map);
        
        self.send_message(&format!(
            "🗺️ 世界地图已生成\n\
             - 地点数量: {}\n\
             - 空间层级: {}\n\
             - 传送门: {}\n\
             - 查看链接: /novels/{}/map",
            map.locations.len(),
            map.layers.iter().map(|l| &l.name).join(", "),
            map.portals.len(),
            novel_id
        )).await?;
        
        Ok(MapVisualization {
            locations: map.locations.len(),
            layers: map.layers,
            url: format!("/novels/{}/map", novel_id),
            data: map,
        })
    }
}
```

### 4.2 刘和平 Agent 集成

```rust
impl LiuHepingAgent {
    /// 查询人物档案（通过 MCP）
    pub async fn query_character_mcp(
        &mut self,
        novel_id: &str,
        character_name: &str,
    ) -> Result<CharacterProfile, Error> {
        let profile = self.mcp_client
            .get_entity_profile(novel_id, character_name, Some("person"))
            .await?;
        
        // 更新本地人物知识库
        self.update_character_db(&profile).await?;
        
        // 在群聊中展示摘要
        self.send_message(&format!(
            "🎭 人物档案：{}\n\
             - 别名: {}\n\
             - 出现章节: {} - {}\n\
             - 出现次数: {} 章\n\
             - 关系数量: {}",
            profile.name,
            profile.aliases.join(", "),
            profile.first_appearance,
            profile.last_appearance,
            profile.chapter_count,
            profile.relations.len()
        )).await?;
        
        Ok(profile)
    }
}
```

---

## 五、部署架构

### 5.1 开发环境

```yaml
# docker-compose.dev.yml
version: '3.8'

services:
  ai-reader:
    build:
      context: ./AI-Reader-V2/backend
      dockerfile: Dockerfile
    ports:
      - "8000:8000"
    environment:
      - LLM_PROVIDER=openai
      - LLM_API_KEY=${LLM_API_KEY}
      - LLM_BASE_URL=${LLM_BASE_URL:-https://api.deepseek.com/v1}
      - LLM_MODEL=${LLM_MODEL:-deepseek-chat}
    volumes:
      - ai-reader-data:/root/.ai-reader-v2
    deploy:
      resources:
        limits:
          memory: 2G
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8000/api/health"]
      interval: 30s
      timeout: 10s
      retries: 3

  opennovel:
    build:
      context: ./opennovel-core
      dockerfile: Dockerfile
    ports:
      - "3000:3000"
    environment:
      - AI_READER_MCP_URL=stdio:///app/ai_reader_mcp_server.py
    depends_on:
      - ai-reader

volumes:
  ai-reader-data:
```

### 5.2 生产环境

```
┌─────────────────────────────────────────────────────────────────────┐
│                          Load Balancer                               │
├─────────────────────────────────────────────────────────────────────┤
│                              │                                       │
│              ┌───────────────┼───────────────┐                      │
│              ▼               ▼               ▼                      │
│      ┌──────────────┐ ┌──────────────┐ ┌──────────────┐            │
│      │  OpenNovel   │ │  OpenNovel   │ │  OpenNovel   │            │
│      │  Instance 1  │ │  Instance 2  │ │  Instance 3  │            │
│      │  (4核8G)     │ │  (4核8G)     │ │  (4核8G)     │            │
│      └──────────────┘ └──────────────┘ └──────────────┘            │
│              │               │               │                      │
│              └───────────────┼───────────────┘                      │
│                              │                                       │
│                              ▼                                       │
│      ┌──────────────────────────────────────────────────┐          │
│      │              AI-Reader-V2 Cluster                 │          │
│      │                                                  │          │
│      │   ┌────────────┐  ┌────────────┐  ┌────────────┐ │          │
│      │   │  Reader 1  │  │  Reader 2  │  │  Reader 3  │ │          │
│      │   │  (4核8G)   │  │  (4核8G)   │  │  (4核8G)   │ │          │
│      │   └────────────┘  └────────────┘  └────────────┘ │          │
│      │                                                  │          │
│      └──────────────────────────────────────────────────┘          │
│                              │                                       │
│                              ▼                                       │
│      ┌──────────────────────────────────────────────────┐          │
│      │              共享存储 (NFS / S3)                  │          │
│      │                                                  │          │
│      │   - 小说数据 (SQLite)                           │          │
│      │   - 分析结果 (JSON)                             │          │
│      │   - 向量索引 (ChromaDB)                         │          │
│      └──────────────────────────────────────────────────┘          │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 六、总结

### 6.1 方案对比

| 方案 | 实现复杂度 | AGPL 合规 | 性能 | 推荐度 |
|------|-----------|----------|------|-------|
| **直接 HTTP API** | 低 | ✅ | 高 | ⭐⭐⭐⭐ |
| **MCP Server** | 中 | ✅ | 高 | ⭐⭐⭐⭐⭐ |
| **两者结合** | 中 | ✅ | 高 | ⭐⭐⭐⭐⭐ |

### 6.2 资源需求总结

| 配置 | 内存 | 适用场景 | 可支撑规模 |
|------|------|---------|-----------|
| **最低** | 2 GB | 云端 LLM、禁用嵌入 | < 100 章、< 100 万字 |
| **推荐** | 4 GB | 云端 LLM、启用嵌入 | < 500 章、< 300 万字 |
| **理想** | 8 GB | 本地 LLM + 嵌入 | 无限制 |

### 6.3 下一步

1. 实现 MCP Server (`ai_reader_mcp_server.py`)
2. 在 OpenNovel 中集成 MCP Client
3. 编写 Phase 8-10 详细实施计划