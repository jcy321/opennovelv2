# AI-Reader-V2 集成可行性分析报告

**生成日期**: 2026-03-17
**分析对象**: https://github.com/mouseart2025/AI-Reader-V2
**分析目的**: 评估作为 MCP/Tool 集成到 OpenNovel 的可行性

---

## 一、项目概述

### 1.1 AI-Reader-V2 简介

AI-Reader-V2 是一个**本地部署的智能小说阅读理解系统**，利用 LLM 将小说文本转化为结构化知识图谱，提供关系图、多层级世界地图、时间线等多维可视化。

### 1.2 核心功能

| 功能 | 描述 | OpenNovel 需求匹配 |
|------|------|-------------------|
| 🕸️ **智能知识图谱** | 力导向人物关系图，70+ 关系类型，6 大分类着色 | ✅ 天道、刘和平需要展示人物关系 |
| 🗺️ **多层级世界地图** | 宏观区域划分、天界/冥界/洞府多空间层、传送门连接 | ✅ 天道、规划者需要展示地图层级 |
| ⏳ **多泳道时间线** | 多源事件聚合，智能降噪过滤 | ✅ 审阅需要时间线分析 |
| 📖 **百科全书** | 五类实体分类浏览 | ✅ 刘和平需要人物信息管理 |

### 1.3 技术栈

| 层 | 技术 | 与 OpenNovel 兼容性 |
|----|------|-------------------|
| 后端 | Python 3.9+ + FastAPI + SQLite + ChromaDB | ✅ 可通过 HTTP API 调用 |
| 前端 | React 19 + TypeScript + Vite + Tailwind CSS | ⚠️ 需要嵌入或重写 |
| 桌面 | Tauri 2 (Rust) + Python sidecar | ✅ 与 OpenNovel (Rust) 兼容 |
| 可视化 | D3.js + react-force-graph-2d + react-leaflet | ✅ 可复用组件 |

---

## 二、许可证合规性分析

### 2.1 AGPL-3.0 关键条款

AI-Reader-V2 使用 **GNU Affero General Public License v3.0 (AGPL-3.0)** 许可证。

**关键条款解读**：

| 条款 | 内容 | 影响 |
|------|------|------|
| **Section 5** | 修改后必须以相同许可证发布 | 修改代码会触发传染 |
| **Section 6** | 网络服务必须提供源代码 | 通过网络提供服务需开源 |
| **Section 13** | 远程网络交互需提供源代码 | SaaS 模式需开源 |

### 2.2 传染性分析

AGPL-3.0 的传染性触发条件：

```
触发传染的条件：
1. 修改 AGPL 软件的源代码
2. 将 AGPL 软件的代码链接/合并到自己的项目中
3. 创建 AGPL 软件的衍生作品

不触发传染的情况：
1. 仅通过网络 API 调用 AGPL 软件（不修改源代码）
2. AGPL 软件作为独立进程运行，通过 IPC/HTTP 通信
3. 使用 AGPL 软件生成数据，而非软件本身
```

### 2.3 AGPL-3.0 Section 0 关键定义

> "Mere interaction with a user through a computer network, with no transfer of a copy, is not conveying."

**翻译**：仅通过计算机网络与用户交互，不转移副本，**不构成"传送"**。

这意味着：
- 通过 HTTP API 调用 AI-Reader-V2 **不触发许可证传染**
- OpenNovel 可以作为独立服务调用 AI-Reader-V2
- **前提条件**：不修改 AI-Reader-V2 的源代码

### 2.4 商业许可证选项

AI-Reader-V2 提供商业许可证选项：

| 计划 | 用途 | 价格 |
|------|------|------|
| Indie | 独立开发者、小团队 (< 5 人) | 即将公布 |
| Business | 公司、SaaS 部署 | 即将公布 |
| Enterprise | 定制条款、优先支持 | 联系咨询 |

---

## 三、集成方案设计

### 3.1 方案对比

| 方案 | 描述 | AGPL 合规 | 实现复杂度 | 推荐度 |
|------|------|----------|-----------|-------|
| **A: 独立服务 + API 调用** | AI-Reader-V2 作为独立服务运行，OpenNovel 通过 HTTP API 调用 | ✅ 完全合规 | 低 | ⭐⭐⭐⭐⭐ |
| **B: MCP Server 封装** | 创建 MCP Server 封装 AI-Reader-V2 API | ✅ 完全合规 | 中 | ⭐⭐⭐⭐ |
| **C: 前端组件嵌入** | 直接嵌入 AI-Reader-V2 前端组件 | ⚠️ 需评估 | 高 | ⭐⭐ |
| **D: 代码合并** | 将 AI-Reader-V2 代码合并到 OpenNovel | ❌ 触发 AGPL 传染 | - | ❌ 不推荐 |

### 3.2 推荐方案：独立服务 + API 调用

#### 3.2.1 架构图

```
┌─────────────────────────────────────────────────────────────────────┐
│                         OpenNovel System                             │
│                    (Rust + Axum + Agents)                            │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌─────────────────────────────────────────────────────────────────┐│
│  │                      Agent System                               ││
│  │                                                                 ││
│  │   天道 ────────────────┐                                       ││
│  │   刘和平 ──────────────┼──► Visualization Tool ◄──┐            ││
│  │   规划者 ──────────────┘                          │            ││
│  └─────────────────────────────────────────────────────────────────┘│
│                                      │                              │
│                                      ▼                              │
│  ┌─────────────────────────────────────────────────────────────────┐│
│  │                 Visualization Tool (Rust)                       ││
│  │                                                                 ││
│  │   Tool 定义：                                                   ││
│  │   - get_character_graph(novel_id, chapter_range)               ││
│  │   - get_world_map(novel_id, chapter_range, layer_id)           ││
│  │   - get_entity_profile(novel_id, entity_name, type)            ││
│  │   - get_character_trajectory(novel_id, character_name)         ││
│  └─────────────────────────────────────────────────────────────────┘│
│                                      │                              │
│                                      │ HTTP API                     │
│                                      ▼                              │
└─────────────────────────────────────────────────────────────────────┘
                                       │
                                       │ localhost:8000
                                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│                      AI-Reader-V2 Backend                           │
│                    (Python + FastAPI)                               │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  REST API 端点：                                                    │
│  - GET  /api/novels/{id}/graph                                     │
│  - GET  /api/novels/{id}/map                                       │
│  - GET  /api/novels/{id}/entities                                  │
│  - GET  /api/novels/{id}/entities/{name}                           │
│  - GET  /api/novels/{id}/timeline                                  │
│                                                                     │
│  数据存储：                                                         │
│  - SQLite: novels, chapters, chapter_facts, entities               │
│  - ChromaDB: 向量嵌入                                              │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

#### 3.2.2 数据流

```
OpenNovel 知识库                    AI-Reader-V2
       │                                  │
       │  1. 导出小说文本                 │
       │  ─────────────────────────────►  │
       │                                  │
       │  2. AI-Reader 分析               │
       │  (LLM 提取 ChapterFacts)         │
       │                                  │
       │  3. 请求可视化数据               │
       │  ◄─────────────────────────────  │
       │                                  │
       │  4. 返回 nodes/edges/locations  │
       │  ─────────────────────────────►  │
       │                                  │
       │  5. 渲染到 OpenNovel Web UI     │
       │                                  │
```

### 3.3 API 接口映射

| OpenNovel 需求 | AI-Reader-V2 API | 说明 |
|----------------|------------------|------|
| 天道展示人物关系 | `GET /api/novels/{id}/graph` | 返回 nodes + edges |
| 规划者展示地图层级 | `GET /api/novels/{id}/map` | 返回 locations + layout + layers |
| 刘和平查询人物信息 | `GET /api/novels/{id}/entities/{name}` | 返回 PersonProfile |
| 审阅展示时间线 | `GET /api/novels/{id}/timeline` | 返回 events |

#### 3.3.1 Graph API 响应结构

```json
{
  "nodes": [
    {
      "id": "林风",
      "name": "林风",
      "type": "person",
      "chapter_count": 50,
      "org": "青云宗",
      "aliases": ["少主", "风哥"]
    }
  ],
  "edges": [
    {
      "source": "林风",
      "target": "苏瑶",
      "relation_type": "青梅竹马",
      "category": "intimate",
      "chapters": [1, 5, 12, 30],
      "weight": 4
    }
  ],
  "analyzed_range": [1, 50],
  "category_counts": {
    "family": 5,
    "intimate": 12,
    "hierarchical": 8,
    "social": 25,
    "hostile": 10,
    "other": 3
  }
}
```

#### 3.3.2 Map API 响应结构

```json
{
  "locations": [
    {
      "name": "青云宗",
      "type": "宗门",
      "tier": "region",
      "parent": "东洲",
      "children": ["外门", "内门", "后山"],
      "mention_count": 120,
      "role": "setting",
      "icon": "temple"
    }
  ],
  "layout": [
    {
      "name": "青云宗",
      "x": 450.5,
      "y": 320.2,
      "is_portal": false
    }
  ],
  "layout_mode": "hierarchy",
  "portals": [
    {
      "name": "传送阵",
      "source_layer": "overworld",
      "target_layer": "sky",
      "position": [500, 200]
    }
  ],
  "layers": [
    {
      "id": "overworld",
      "name": "人间",
      "type": "overworld"
    },
    {
      "id": "sky",
      "name": "天界",
      "type": "sky"
    }
  ],
  "trajectories": {
    "林风": [
      {"chapter": 1, "location": "林家"},
      {"chapter": 5, "location": "青云宗"},
      {"chapter": 20, "location": "天界"}
    ]
  },
  "terrain_url": "/api/novels/{id}/map/terrain",
  "analyzed_range": [1, 50]
}
```

### 3.4 Tool 定义（Rust 实现）

```rust
use serde::{Deserialize, Serialize};
use reqwest::Client;

/// AI-Reader-V2 可视化工具
pub struct VisualizationTool {
    client: Client,
    base_url: String,
}

impl VisualizationTool {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    /// 获取人物关系图
    pub async fn get_character_graph(
        &self,
        novel_id: &str,
        chapter_start: Option<u32>,
        chapter_end: Option<u32>,
    ) -> Result<GraphData, Box<dyn std::error::Error>> {
        let mut url = format!("{}/api/novels/{}/graph", self.base_url, novel_id);
        
        if let (Some(start), Some(end)) = (chapter_start, chapter_end) {
            url = format!("{}?chapter_start={}&chapter_end={}", url, start, end);
        }
        
        let response = self.client.get(&url).send().await?;
        let data = response.json::<GraphData>().await?;
        Ok(data)
    }

    /// 获取世界地图
    pub async fn get_world_map(
        &self,
        novel_id: &str,
        chapter_start: Option<u32>,
        chapter_end: Option<u32>,
        layer_id: Option<&str>,
    ) -> Result<MapData, Box<dyn std::error::Error>> {
        let mut params = Vec::new();
        
        if let (Some(start), Some(end)) = (chapter_start, chapter_end) {
            params.push(format!("chapter_start={}", start));
            params.push(format!("chapter_end={}", end));
        }
        
        if let Some(layer) = layer_id {
            params.push(format!("layer_id={}", layer));
        }
        
        let url = if params.is_empty() {
            format!("{}/api/novels/{}/map", self.base_url, novel_id)
        } else {
            format!("{}/api/novels/{}/map?{}", self.base_url, novel_id, params.join("&"))
        };
        
        let response = self.client.get(&url).send().await?;
        let data = response.json::<MapData>().await?;
        Ok(data)
    }

    /// 获取实体详情
    pub async fn get_entity_profile(
        &self,
        novel_id: &str,
        entity_name: &str,
        entity_type: Option<&str>,
    ) -> Result<EntityProfile, Box<dyn std::error::Error>> {
        let mut url = format!("{}/api/novels/{}/entities/{}", self.base_url, novel_id, entity_name);
        
        if let Some(t) = entity_type {
            url = format!("{}?type={}", url, t);
        }
        
        let response = self.client.get(&url).send().await?;
        let data = response.json::<EntityProfile>().await?;
        Ok(data)
    }

    /// 获取人物轨迹
    pub async fn get_character_trajectory(
        &self,
        novel_id: &str,
        character_name: &str,
    ) -> Result<Vec<TrajectoryPoint>, Box<dyn std::error::Error>> {
        let map_data = self.get_world_map(novel_id, None, None, None).await?;
        
        let trajectory = map_data.trajectories
            .get(character_name)
            .cloned()
            .unwrap_or_default();
        
        Ok(trajectory)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphData {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub analyzed_range: [u32; 2],
    pub category_counts: HashMap<String, u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub node_type: String,
    pub chapter_count: u32,
    pub org: Option<String>,
    pub aliases: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Edge {
    pub source: String,
    pub target: String,
    pub relation_type: String,
    pub category: String,
    pub chapters: Vec<u32>,
    pub weight: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapData {
    pub locations: Vec<Location>,
    pub layout: Vec<LayoutItem>,
    pub layout_mode: String,
    pub portals: Vec<Portal>,
    pub layers: Vec<Layer>,
    pub trajectories: HashMap<String, Vec<TrajectoryPoint>>,
    pub terrain_url: Option<String>,
    pub analyzed_range: [u32; 2],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    #[serde(rename = "type")]
    pub location_type: String,
    pub tier: String,
    pub parent: Option<String>,
    pub children: Vec<String>,
    pub mention_count: u32,
    pub role: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrajectoryPoint {
    pub chapter: u32,
    pub location: String,
}
```

---

## 四、与 OpenNovel Agent 的集成方式

### 4.1 天道 (Tian Dao) 集成

**使用场景**：天道在编排剧情时，需要可视化展示人物关系和地图层级。

**集成方式**：

```rust
impl TianDaoAgent {
    /// 展示人物关系图
    async fn show_character_graph(&self, novel_id: &str) -> Result<(), Error> {
        let graph = self.viz_tool.get_character_graph(novel_id, None, None).await?;
        
        // 在群聊中发送可视化链接
        self.send_message(&format!(
            "📊 人物关系图已生成，共 {} 个人物，{} 条关系。\n\
             查看链接: /novels/{}/graph",
            graph.nodes.len(),
            graph.edges.len(),
            novel_id
        )).await?;
        
        Ok(())
    }

    /// 展示地图层级
    async fn show_world_map(&self, novel_id: &str, layer: Option<&str>) -> Result<(), Error> {
        let map = self.viz_tool.get_world_map(novel_id, None, None, layer).await?;
        
        let layer_info = if let Some(l) = layer {
            format!("当前层级: {}", l)
        } else {
            format!("可用层级: {}", map.layers.iter().map(|l| &l.name).join(", "))
        };
        
        self.send_message(&format!(
            "🗺️ 世界地图已生成。\n\
             地点数量: {}\n\
             {}\n\
             查看链接: /novels/{}/map",
            map.locations.len(),
            layer_info,
            novel_id
        )).await?;
        
        Ok(())
    }
}
```

### 4.2 刘和平 (Liu Heping) 集成

**使用场景**：刘和平在维护人物知识库时，需要查询人物信息和关系。

**集成方式**：

```rust
impl LiuHepingAgent {
    /// 查询人物档案
    async fn query_character(&self, novel_id: &str, name: &str) -> Result<PersonProfile, Error> {
        let profile = self.viz_tool.get_entity_profile(novel_id, name, Some("person")).await?;
        
        // 更新本地人物知识库
        self.update_character_db(&profile).await?;
        
        Ok(profile)
    }

    /// 分析人物关系网络
    async fn analyze_relationships(&self, novel_id: &str, character: &str) -> Result<(), Error> {
        let graph = self.viz_tool.get_character_graph(novel_id, None, None).await?;
        
        // 筛选与目标人物相关的关系
        let related_edges: Vec<_> = graph.edges
            .iter()
            .filter(|e| e.source == character || e.target == character)
            .collect();
        
        self.send_message(&format!(
            "🎭 {} 的关系网络分析：\n\
             直接关系数量: {}\n\
             关系类型分布: {:?}",
            character,
            related_edges.len(),
            self.categorize_relationships(&related_edges)
        )).await?;
        
        Ok(())
    }
}
```

### 4.3 规划者 (Planner) 集成

**使用场景**：规划者在构思阶段，需要展示世界地图和势力分布。

**集成方式**：

```rust
impl PlannerAgent {
    /// 展示世界架构
    async fn show_world_structure(&self, novel_id: &str) -> Result<(), Error> {
        let map = self.viz_tool.get_world_map(novel_id, None, None, None).await?;
        
        // 分析层级结构
        let hierarchy = self.build_hierarchy(&map.locations);
        
        self.send_message(&format!(
            "🌍 世界架构概览：\n\
             顶层区域: {}\n\
             层级数量: {}\n\
             传送门数量: {}\n\
             \n\
             层级结构:\n{}",
            hierarchy.top_level_count,
            hierarchy.max_depth,
            map.portals.len(),
            self.format_hierarchy(&hierarchy)
        )).await?;
        
        Ok(())
    }
}
```

---

## 五、部署方案

### 5.1 开发环境

```yaml
# docker-compose.yml
version: '3.8'

services:
  ai-reader:
    image: ai-reader-v2:latest
    build:
      context: ./AI-Reader-V2
      dockerfile: Dockerfile
    ports:
      - "8000:8000"
    volumes:
      - ai-reader-data:/root/.ai-reader-v2
    environment:
      - LLM_PROVIDER=openai
      - LLM_API_KEY=${LLM_API_KEY}
      - LLM_BASE_URL=${LLM_BASE_URL}
      - LLM_MODEL=${LLM_MODEL}

  opennovel:
    image: opennovel:latest
    build:
      context: ./opennovel-core
      dockerfile: Dockerfile
    ports:
      - "3000:3000"
    environment:
      - AI_READER_URL=http://ai-reader:8000
    depends_on:
      - ai-reader

volumes:
  ai-reader-data:
```

### 5.2 生产环境

```
┌─────────────────────────────────────────────────────────────────────┐
│                         Load Balancer (Nginx)                        │
├─────────────────────────────────────────────────────────────────────┤
│                              │                                       │
│              ┌───────────────┼───────────────┐                      │
│              ▼               ▼               ▼                      │
│      ┌──────────────┐ ┌──────────────┐ ┌──────────────┐            │
│      │  OpenNovel   │ │  OpenNovel   │ │  OpenNovel   │            │
│      │  Instance 1  │ │  Instance 2  │ │  Instance 3  │            │
│      └──────────────┘ └──────────────┘ └──────────────┘            │
│              │               │               │                      │
│              └───────────────┼───────────────┘                      │
│                              ▼                                       │
│      ┌──────────────────────────────────────────────────┐          │
│      │              AI-Reader-V2 Cluster                 │          │
│      │                                                  │          │
│      │   ┌────────────┐  ┌────────────┐  ┌────────────┐ │          │
│      │   │  Reader 1  │  │  Reader 2  │  │  Reader 3  │ │          │
│      │   └────────────┘  └────────────┘  └────────────┘ │          │
│      │                                                  │          │
│      │   共享存储: NFS / S3                             │          │
│      └──────────────────────────────────────────────────┘          │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 六、前端集成方案

### 6.1 方案对比

| 方案 | 描述 | 优点 | 缺点 |
|------|------|------|------|
| **A: iframe 嵌入** | 通过 iframe 嵌入 AI-Reader 前端 | 实现简单 | 样式不统一、交互受限 |
| **B: 组件复用** | 复用 AI-Reader 的 React 组件 | 灵活可控 | 需要改造、许可证风险 |
| **C: API + 自建前端** | 使用 API 数据，自建可视化组件 | 完全可控 | 开发量大 |
| **D: 生成静态图片** | 后端生成图片，前端展示 | 简单可靠 | 无交互 |

### 6.2 推荐方案：API + 自建轻量级可视化

**理由**：
1. OpenNovel 已有 Web UI（Phase 8）
2. 可视化需求相对简单（关系图、地图）
3. 避免许可证风险
4. 保持 UI 风格统一

**技术选型**：

| 可视化类型 | 库 | 说明 |
|-----------|-----|------|
| 关系图 | D3.js force-directed | 复用 AI-Reader 的布局逻辑 |
| 地图 | Leaflet + 自定义 SVG | 支持多层级、传送门 |
| 时间线 | vis-timeline 或自建 | 简单时间线即可 |

---

## 七、结论与建议

### 7.1 可行性结论

| 评估维度 | 结论 | 说明 |
|----------|------|------|
| **许可证合规** | ✅ 可行 | 通过 API 调用不触发 AGPL 传染 |
| **技术兼容** | ✅ 可行 | Python 后端 + HTTP API 兼容 |
| **功能匹配** | ✅ 高度匹配 | 人物关系图、多层级地图正是所需 |
| **开发成本** | ✅ 可控 | 需实现 Tool 封装 + 前端组件 |
| **维护成本** | ⚠️ 中等 | 依赖外部服务，需版本同步 |

### 7.2 实施建议

#### 短期（1-2 周）

1. **搭建 AI-Reader-V2 开发环境**
   - 本地运行 AI-Reader-V2 后端
   - 测试 API 接口可用性

2. **实现 VisualizationTool**
   - 封装 Graph API
   - 封装 Map API
   - 封装 Entity API

3. **集成到天道 Agent**
   - 实现 `show_character_graph` 命令
   - 实现 `show_world_map` 命令

#### 中期（2-4 周）

1. **前端可视化组件**
   - 实现关系图组件
   - 实现地图组件
   - 集成到 OpenNovel Web UI

2. **数据同步机制**
   - OpenNovel 知识库 → AI-Reader 数据格式
   - 自动导入小说文本

#### 长期（持续）

1. **版本同步**
   - 跟踪 AI-Reader-V2 更新
   - 适配 API 变化

2. **性能优化**
   - 缓存可视化数据
   - 增量更新机制

### 7.3 风险与缓解

| 风险 | 影响 | 缓解措施 |
|------|------|---------|
| AI-Reader-V2 API 变更 | 功能中断 | 版本锁定、接口适配层 |
| 服务不可用 | 功能降级 | 本地缓存、降级方案 |
| 性能瓶颈 | 响应慢 | 缓存、预计算、异步加载 |
| 商业许可证变更 | 法律风险 | 监控许可证变化、准备备选方案 |

---

## 八、附录

### 8.1 AI-Reader-V2 核心数据模型

```python
# ChapterFact - 核心数据模型
class ChapterFact(BaseModel):
    chapter_id: int
    novel_id: str
    characters: List[CharacterFact]
    relationships: List[RelationshipFact]
    locations: List[LocationFact]
    item_events: List[ItemEvent]
    org_events: List[OrgEvent]
    events: List[Event]
    new_concepts: List[Concept]

# PersonProfile - 聚合后的人物档案
class PersonProfile(BaseModel):
    name: str
    aliases: List[str]
    first_appearance: int
    last_appearance: int
    chapter_count: int
    relations: List[RelationChain]
    locations: List[str]
    traits: List[str]

# LocationProfile - 聚合后的地点档案
class LocationProfile(BaseModel):
    name: str
    type: str
    tier: str
    parent: Optional[str]
    children: List[str]
    mention_count: int
    role: Optional[str]  # setting/referenced/boundary
    spatial_relationships: List[SpatialRelationship]
```

### 8.2 参考链接

- AI-Reader-V2 GitHub: https://github.com/mouseart2025/AI-Reader-V2
- AI-Reader-V2 Demo: https://ai-reader.cc/demo/honglou/graph?v=3
- AGPL-3.0 许可证: https://www.gnu.org/licenses/agpl-3.0
- FastAPI 文档: https://fastapi.tiangolo.com/

---

**报告完成日期**: 2026-03-17
**分析者**: Sisyphus (OpenNovel Team)