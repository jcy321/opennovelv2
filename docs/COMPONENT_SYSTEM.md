# OpenNovel v2 完整部件体系设计

> 本文档详细定义OpenNovel v2所需的所有部件，包括运行时组件、工具链、知识系统、协作系统等

---

## 一、部件体系总览

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                            OpenNovel v2 部件架构                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                        核心运行时层 (Core Runtime)                    │   │
│  │  Agent System │ Session Management │ Message Protocol │ Intent Gate │   │
│  │  Category System │ Permission System │ Configuration System         │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                      │                                      │
│  ┌───────────────────────────────────┴───────────────────────────────────┐ │
│  │                          知识与上下文层 (Knowledge Layer)              │   │
│  │  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────────┐ │   │
│  │  │世界观知识库 │ │人物信息知识库│ │历史情节知识库│ │伏笔知识库       │ │   │
│  │  └─────────────┘ └─────────────┘ └─────────────┘ └─────────────────┘ │   │
│  │  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────────┐ │   │
│  │  │本章知识库   │ │阵营势力知识库│ │地图知识库   │ │参考小说库       │ │   │
│  │  └─────────────┘ └─────────────┘ └─────────────┘ └─────────────────┘ │   │
│  │  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐                      │   │
│  │  │CharacterDB  │ │TimelineSystem│ │WorldGraph   │                      │   │
│  │  └─────────────┘ └─────────────┘ └─────────────┘                      │   │
│  └───────────────────────────────────────────────────────────────────────┘ │
│                                      │                                      │
│  ┌───────────────────────────────────┴───────────────────────────────────┐ │
│  │                          文本工具链层 (Text Toolchain)                │   │
│  │  TextEditor │ WordCounter │ StyleChecker │ TextSearch │ SegmentSplit │   │
│  │  DiffMerge │ TextFormatValidator │ ReadabilityChecker                │   │
│  └───────────────────────────────────────────────────────────────────────┘ │
│                                      │                                      │
│  ┌───────────────────────────────────┴───────────────────────────────────┐ │
│  │                          协作系统层 (Collaboration Layer)             │   │
│  │  AnnotationSystem │ ConflictArbitration │ ProactiveIntervention       │   │
│  │  GroupChat │ AgentLock │ ApprovalWorkflow │ NotificationSystem        │   │
│  └───────────────────────────────────────────────────────────────────────┘ │
│                                      │                                      │
│  ┌───────────────────────────────────┴───────────────────────────────────┐ │
│  │                          分析系统层 (Analysis Layer)                  │   │
│  │  StyleAnalyzer │ EmotionAnalyzer │ PacingAnalyzer │ ConsistencyChecker│  │
│  │  CharacterVoiceDetector │ ReaderSimulator │ ExplosivePointPredictor   │   │
│  └───────────────────────────────────────────────────────────────────────┘ │
│                                      │                                      │
│  ┌───────────────────────────────────┴───────────────────────────────────┐ │
│  │                          扩展集成层 (Extension Layer)                 │   │
│  │  MCP Servers │ WebDAV Sync │ Plugin System │ External APIs           │   │
│  └───────────────────────────────────────────────────────────────────────┘ │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 二、核心运行时组件

### 2.1 Agent System

| 组件 | 职责 | 实现要点 |
|------|------|---------|
| AgentRegistry | Agent注册与发现 | 支持8个预定义Agent + 扩展 |
| AgentConfig | Agent配置管理 | model、temperature、thinking、permissions |
| AgentLifecycle | Agent生命周期 | 创建、激活、锁定、销毁 |
| AgentDispatcher | Agent调度 | 意图路由、任务委派 |

### 2.2 Session Management

| 组件 | 职责 | 实现要点 |
|------|------|---------|
| BookSession | 书籍级会话 | 群聊ID、阶段状态、Agent状态 |
| MessageHistory | 消息历史 | 用户消息、Agent响应、批注记录 |
| ContextWindow | 上下文窗口 | token计数、智能截断、关键信息保留 |
| SessionPersistence | 会话持久化 | 数据库存储、恢复机制 |

### 2.3 Message Protocol

| 事件类型 | 描述 | 数据结构 |
|---------|------|---------|
| `thinking_start` | 思考开始 | `{agent, timestamp}` |
| `thinking_delta` | 思考增量 | `{agent, delta}` |
| `thinking_end` | 思考结束 | `{agent, total_length}` |
| `content_start` | 内容开始 | `{agent, timestamp}` |
| `content_delta` | 内容增量 | `{agent, delta}` |
| `content_end` | 内容结束 | `{agent, total_length}` |
| `annotation` | 批注事件 | `{agent, position, content, severity}` |
| `intervention` | 主动介入 | `{agent, reason, message}` |
| `conflict` | 冲突检测 | `{agents, position, descriptions}` |
| `stage_change` | 阶段切换 | `{from_stage, to_stage, locked_agents}` |
| `complete` | 任务完成 | `{session_id, message_id}` |
| `error` | 错误事件 | `{agent, code, message}` |

### 2.4 Intent Gate

```
用户消息 → IntentGate
              │
              ├── 解析@提及 → 目标Agent
              │
              ├── 分析关键词 → Category映射
              │
              ├── 检测意图类型
              │     ├── 撰写请求 → 执笔
              │     ├── 规划请求 → 天道/规划者
              │     ├── 审核请求 → 审阅/世界观守护者
              │     ├── 角色请求 → 刘和平
              │     └── 协调请求 → 观察者
              │
              └── 检查Agent锁定状态 → 允许/拒绝
```

### 2.5 Category System

| Category | 温度 | 用途 | 默认Agent |
|----------|------|------|----------|
| `planning` | 0.3 | 规划类任务 | 规划者、天道 |
| `creation` | 0.7 | 创作类任务 | 执笔、刘和平 |
| `review` | 0.2 | 审核类任务 | 审阅、世界观守护者 |
| `analysis` | 0.3 | 分析类任务 | 调研者、观察者 |
| `coordination` | 0.3 | 协调类任务 | 观察者 |

### 2.6 Permission System

| 权限类型 | 值 | 说明 |
|---------|-----|------|
| `read_worldview` | allow/deny | 读取世界观知识库 |
| `edit_worldview` | allow/deny/ask | 编辑世界观知识库 |
| `read_characters` | allow/deny | 读取人物信息知识库 |
| `edit_characters` | allow/deny/ask | 编辑人物信息知识库 |
| `read_plot` | allow/deny | 读取剧情相关知识库 |
| `edit_plot` | allow/deny/ask | 编辑剧情知识库 |
| `write_chapter` | allow/deny/ask | 撰写章节 |
| `edit_chapter` | allow/deny/ask | 修改章节 |
| `annotate` | allow/deny | 添加批注 |
| `manage_knowledge` | allow/deny | 管理知识库 |
| `orchestrate_agents` | allow/deny | 协调其他Agent |

---

## 三、知识与上下文组件

### 3.1 七大知识库

#### 世界观知识库
```
职责：存储小说世界观设定
内容：
  - 基础设定（时间、地点、历史）
  - 规则系统（魔法体系、科技水平、社会规则）
  - 势力分布（国家、组织、派系）
  - 地理信息（地图、城市、资源）
写入权限：天道、世界观守护者
读取权限：所有Agent
```

#### 人物信息知识库
```
职责：存储所有角色信息
内容：
  - 角色基础属性（姓名、年龄、身份）
  - 性格特征（价值观、恐惧、渴望）
  - 行为模式（说话方式、典型行为）
  - 时间切片状态（随章节变化）
  - 角色关系图谱
写入权限：刘和平
读取权限：所有Agent
```

#### 历史情节知识库
```
职责：存储已完成章节的向量化内容
内容：
  - 章节文本向量
  - 关键事件提取
  - 情感曲线记录
写入权限：观察者（自动）
读取权限：所有Agent
生成方式：章节完成后自动向量化
```

#### 本章知识库
```
职责：存储当前章节的规划信息
内容：
  - 本章目标
  - 情节要点
  - 人物行动
  - 伏笔计划
写入权限：天道
读取权限：所有Agent
生命周期：章节完成后归档到历史情节知识库
```

#### 阵营派系势力知识库
```
职责：存储势力、阵营、派系信息
内容：
  - 势力基础信息
  - 力量对比
  - 利益冲突
  - 时间切片变化
写入权限：天道
读取权限：所有Agent
```

#### 地图知识库
```
职责：存储地理空间信息
内容：
  - 世界地图结构
  - 人物当前位置
  - 重要地点信息
写入权限：天道
读取权限：所有Agent
```

#### 伏笔知识库
```
职责：管理伏笔的生命周期
内容：
  - 伏笔ID和标题
  - 埋设章节和内容
  - 预期触发章节
  - 当前状态（已埋/已暗示/已触发/已放弃）
写入权限：天道
读取权限：天道、规划者
```

### 3.2 CharacterDB（角色数据库）

```rust
pub struct CharacterDB {
    characters: HashMap<String, Character>,
    relationship_graph: RelationshipGraph,
    voice_profiles: HashMap<String, VoiceProfile>,
}

pub struct Character {
    pub id: String,
    pub name: String,
    pub aliases: Vec<String>,
    pub role: CharacterRole,  // 主角/主要/特殊/普通
    
    // 基础属性
    pub attributes: CharacterAttributes,
    
    // 性格特征
    pub personality: Personality,
    
    // 行为模式
    pub behavior_patterns: Vec<BehaviorPattern>,
    
    // 时间切片状态
    pub timeline_states: HashMap<u32, CharacterState>,
    
    // 关系
    pub relationships: HashMap<String, Relationship>,
}

pub struct VoiceProfile {
    pub character_id: String,
    pub speech_patterns: Vec<SpeechPattern>,
    pub vocabulary_preference: HashMap<String, f32>,
    pub sentence_style: SentenceStyle,
}

impl CharacterDB {
    /// 检查角色行为是否符合设定
    pub fn check_behavior_consistency(
        &self,
        character_id: &str,
        action: &str,
        context: &str,
    ) -> ConsistencyResult;
    
    /// 检查对话口吻是否一致
    pub fn check_voice_consistency(
        &self,
        character_id: &str,
        dialogue: &str,
    ) -> VoiceCheckResult;
    
    /// 获取角色在某章的状态
    pub fn get_state_at_chapter(
        &self,
        character_id: &str,
        chapter: u32,
    ) -> Option<&CharacterState>;
}
```

### 3.3 TimelineSystem（时间线系统）

```rust
pub struct TimelineSystem {
    events: Vec<TimelineEvent>,
    chapter_markers: HashMap<u32, DateTime<Utc>>,
    time_scale: TimeScale,  // 小时/天/月/年
}

pub struct TimelineEvent {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub chapter: u32,
    pub description: String,
    pub participants: Vec<String>,
    pub location: String,
    pub event_type: EventType,
}

pub enum TimeScale {
    Hours,   // 几小时内发生的故事
    Days,    // 几天内发生的故事
    Months,  // 几个月内发生的故事
    Years,   // 跨年的故事
}

impl TimelineSystem {
    /// 添加事件到时间线
    pub fn add_event(&mut self, event: TimelineEvent) -> Result<(), TimelineError>;
    
    /// 检查时间冲突
    pub fn check_conflicts(&self, event: &TimelineEvent) -> Vec<TimelineConflict>;
    
    /// 获取某时间点的事件
    pub fn get_events_at(&self, time: DateTime<Utc>) -> Vec<&TimelineEvent>;
    
    /// 生成时间线摘要
    pub fn generate_summary(&self, chapter_range: Range<u32>) -> TimelineSummary;
}
```

### 3.4 WorldGraph（世界观图谱）

```rust
pub struct WorldGraph {
    nodes: HashMap<String, WorldNode>,
    edges: Vec<WorldEdge>,
    rules: Vec<WorldRule>,
}

pub struct WorldNode {
    pub id: String,
    pub name: String,
    pub node_type: WorldNodeType,
    pub attributes: HashMap<String, String>,
}

pub enum WorldNodeType {
    Location,    // 地点
    Organization,// 组织
    Concept,     // 概念（魔法、科技等）
    Item,        // 物品
    Event,       // 历史事件
}

pub struct WorldEdge {
    pub from: String,
    pub to: String,
    pub relation: String,
    pub attributes: HashMap<String, String>,
}

pub struct WorldRule {
    pub id: String,
    pub description: String,
    pub scope: RuleScope,
    pub constraints: Vec<Constraint>,
}

impl WorldGraph {
    /// 检查内容是否违反世界观规则
    pub fn validate_content(&self, content: &str) -> Vec<WorldViolation>;
    
    /// 查询相关设定
    pub fn query_related(&self, node_id: &str, depth: u32) -> Vec<&WorldNode>;
    
    /// 检查设定一致性
    pub fn check_consistency(&self, new_node: &WorldNode) -> Vec<ConsistencyIssue>;
}
```

### 3.5 ForeshadowingPool（伏笔池）

```rust
pub struct ForeshadowingPool {
    foreshadowings: HashMap<String, Foreshadowing>,
    pressure_gauge: PressureGauge,
    trigger_queue: VecDeque<String>,
}

pub struct Foreshadowing {
    pub id: String,
    pub title: String,
    pub description: String,
    pub planted_chapter: u32,
    pub planted_content: String,
    pub expected_trigger: Option<u32>,
    pub actual_trigger: Option<u32>,
    pub state: ForeshadowingState,
    pub importance: Importance,
    pub related_characters: Vec<String>,
}

pub enum ForeshadowingState {
    Planted,    // 已埋下
    Hinted,     // 已暗示
    Triggered,  // 已触发
    Abandoned,  // 已放弃
}

pub struct PressureGauge {
    current_pressure: f32,
    threshold: f32,
    consecutive_no_accident: u32,
    accident_probability: f32,
}

impl ForeshadowingPool {
    /// 添加新伏笔
    pub fn plant(&mut self, foreshadowing: Foreshadowing);
    
    /// 触发伏笔
    pub fn trigger(&mut self, id: &str, chapter: u32) -> Result<(), ForeshadowingError>;
    
    /// 更新压力表
    pub fn update_pressure(&mut self, chapter_has_accident: bool);
    
    /// 检查是否应该触发意外/伏笔
    pub fn should_trigger_something(&self) -> Option<TriggerSuggestion>;
    
    /// 获取待触发的伏笔列表
    pub fn get_pending_foreshadowings(&self) -> Vec<&Foreshadowing>;
}
```

---

## 四、文本工具链组件

### 4.1 TextEditor（文本编辑器）

```rust
pub struct TextEditor {
    current_content: String,
    cursor_position: usize,
    selection: Option<Range<usize>>,
    undo_stack: Vec<EditAction>,
    redo_stack: Vec<EditAction>,
}

pub struct EditAction {
    action_type: EditType,
    position: Range<usize>,
    old_content: String,
    new_content: String,
    timestamp: DateTime<Utc>,
}

impl TextEditor {
    /// 插入文本
    pub fn insert(&mut self, position: usize, text: &str);
    
    /// 删除文本
    pub fn delete(&mut self, range: Range<usize>);
    
    /// 替换文本
    pub fn replace(&mut self, range: Range<usize>, new_text: &str);
    
    /// 撤销
    pub fn undo(&mut self) -> Option<&EditAction>;
    
    /// 重做
    pub fn redo(&mut self) -> Option<&EditAction>;
    
    /// 获取选中内容
    pub fn get_selection(&self) -> Option<&str>;
    
    /// 应用批注修改
    pub fn apply_annotation(&mut self, annotation: &Annotation);
}
```

### 4.2 WordCounter（字数统计）

```rust
pub struct WordCounter {
    total_chars: usize,
    chinese_chars: usize,
    english_words: usize,
    punctuation: usize,
    paragraphs: usize,
}

impl WordCounter {
    /// 统计文本
    pub fn count(text: &str) -> WordCounter;
    
    /// 估算阅读时间（分钟）
    pub fn estimate_reading_time(&self) -> f32;
    
    /// 检查是否达到目标字数
    pub fn check_target(&self, target: usize) -> WordCountStatus;
}
```

### 4.3 StyleChecker（风格检查器）

```rust
pub struct StyleChecker {
    rules: Vec<StyleRule>,
    custom_vocabulary: HashSet<String>,
    forbidden_words: HashSet<String>,
}

pub struct StyleRule {
    pub name: String,
    pub pattern: Regex,
    pub suggestion: String,
    pub severity: Severity,
}

pub struct StyleCheckResult {
    pub issues: Vec<StyleIssue>,
    pub overall_score: f32,
}

impl StyleChecker {
    /// 检查文本风格
    pub fn check(&self, text: &str) -> StyleCheckResult;
    
    /// 检查用词重复
    pub fn check_repetition(&self, text: &str, window_size: usize) -> Vec<RepetitionIssue>;
    
    /// 检查句式单调
    pub fn check_sentence_variety(&self, text: &str) -> SentenceVarietyReport;
    
    /// 检查禁用词
    pub fn check_forbidden_words(&self, text: &str) -> Vec<ForbiddenWordIssue>;
}
```

### 4.4 TextSearch（全文搜索）

```rust
pub struct TextSearch {
    index: InvertedIndex,
    vector_index: Option<VectorIndex>,
}

pub struct SearchResult {
    pub chapter: u32,
    pub position: Range<usize>,
    pub context: String,
    pub score: f32,
}

impl TextSearch {
    /// 关键词搜索
    pub fn search_keyword(&self, keyword: &str) -> Vec<SearchResult>;
    
    /// 语义搜索（向量）
    pub fn search_semantic(&self, query: &str, top_k: usize) -> Vec<SearchResult>;
    
    /// 搜索角色提及
    pub fn search_character(&self, character_name: &str) -> Vec<SearchResult>;
    
    /// 搜索地点提及
    pub fn search_location(&self, location_name: &str) -> Vec<SearchResult>;
}
```

### 4.5 SegmentSplit（章节分割）

```rust
pub struct SegmentSplit;

pub struct ChapterSegment {
    pub title: String,
    pub content: String,
    pub start_position: usize,
    pub end_position: usize,
    pub word_count: usize,
}

impl SegmentSplit {
    /// 智能分割章节
    pub fn split_by_chapter(text: &str) -> Vec<ChapterSegment>;
    
    /// 按场景分割
    pub fn split_by_scene(text: &str) -> Vec<SceneSegment>;
    
    /// 检测章节标题
    pub fn detect_chapter_titles(text: &str) -> Vec<ChapterTitle>;
}
```

---

## 五、协作系统组件

### 5.1 AnnotationSystem（批注系统）

```rust
pub struct AnnotationSystem {
    annotations: HashMap<String, Annotation>,
    conflict_detector: ConflictDetector,
}

pub struct Annotation {
    pub id: String,
    pub chapter_id: String,
    pub agent_id: String,
    pub agent_name: String,
    pub position: TextRange,
    pub selected_text: String,
    pub content: String,
    pub severity: AnnotationSeverity,
    pub status: AnnotationStatus,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}

pub enum AnnotationSeverity {
    Critical,   // 必须修改
    Warning,    // 建议修改
    Suggestion, // 可选修改
    Info,       // 仅供参考
}

pub enum AnnotationStatus {
    Pending,
    Accepted,
    Rejected,
    Modified,
}

impl AnnotationSystem {
    /// 添加批注
    pub fn add(&mut self, annotation: Annotation);
    
    /// 获取章节的所有批注
    pub fn get_chapter_annotations(&self, chapter_id: &str) -> Vec<&Annotation>;
    
    /// 接受批注
    pub fn accept(&mut self, id: &str);
    
    /// 拒绝批注
    pub fn reject(&mut self, id: &str);
    
    /// 检测批注冲突
    pub fn detect_conflicts(&self) -> Vec<AnnotationConflict>;
}
```

### 5.2 ConflictArbitration（冲突仲裁）

```rust
pub struct ConflictArbitration;

pub struct AnnotationConflict {
    pub id: String,
    pub position: TextRange,
    pub conflicting_annotations: Vec<String>, // 批注ID列表
    pub descriptions: Vec<String>,
    pub status: ConflictStatus,
}

pub enum ConflictStatus {
    Pending,    // 等待用户裁决
    Resolved,   // 已解决
}

impl ConflictArbitration {
    /// 生成冲突报告
    pub fn generate_report(&self, conflict: &AnnotationConflict) -> ConflictReport;
    
    /// 用户裁决
    pub fn arbitrate(
        &self,
        conflict_id: &str,
        decision: ArbitrationDecision,
    ) -> ArbitrationResult;
}

pub enum ArbitrationDecision {
    AcceptAnnotation(String),  // 接受某个批注
    CustomSolution(String),    // 自定义解决方案
    Skip,                      // 跳过
}
```

### 5.3 ProactiveIntervention（主动介入）

```rust
pub struct ProactiveIntervention {
    conditions: Vec<InterventionCondition>,
    intervention_history: Vec<InterventionRecord>,
}

pub struct InterventionCondition {
    pub agent_id: String,
    pub trigger_type: InterventionTrigger,
    pub priority: InterventionPriority,
    pub check_fn: Box<dyn Fn(&AgentContext, &str) -> bool + Send + Sync>,
}

pub enum InterventionTrigger {
    CharacterInconsistency,
    WorldviewViolation,
    PlotSuggestion,
    ForeshadowingReminder,
    StyleWarning,
}

pub struct InterventionResponse {
    pub agent: String,
    pub message: String,
    pub priority: InterventionPriority,
    pub related_position: Option<TextRange>,
}

impl ProactiveIntervention {
    /// 检查是否需要介入
    pub async fn check(
        &self,
        context: &AgentContext,
        target_agent: &str,
        message: &str,
    ) -> Vec<InterventionResponse>;
    
    /// 注册介入条件
    pub fn register_condition(&mut self, condition: InterventionCondition);
}
```

### 5.4 GroupChat（群聊管理）

```rust
pub struct GroupChat {
    pub book_id: String,
    pub book_name: String,
    pub stage: BookStage,
    pub messages: Vec<ChatMessage>,
    pub agent_status: HashMap<String, AgentStatus>,
    pub locked_agents: HashSet<String>,
}

pub struct ChatMessage {
    pub id: String,
    pub sender: MessageSender,
    pub content: String,
    pub thinking: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub mentions: Vec<String>,
}

pub enum MessageSender {
    User,
    Agent { id: String, name: String },
    System,
}

pub enum AgentStatus {
    Active,   // 可交互
    Locked,   // 已锁定
    Idle,     // 空闲
    Working,  // 工作中
}

impl GroupChat {
    /// 发送消息
    pub async fn send(&mut self, message: ChatMessage) -> Vec<ChatMessage>;
    
    /// 切换阶段
    pub fn change_stage(&mut self, new_stage: BookStage);
    
    /// 锁定Agent
    pub fn lock_agent(&mut self, agent_id: &str);
    
    /// 解锁Agent
    pub fn unlock_agent(&mut self, agent_id: &str);
    
    /// 获取活跃Agent列表
    pub fn get_active_agents(&self) -> Vec<&String>;
}
```

### 5.5 AgentLock（Agent锁定机制）

```rust
pub struct AgentLockManager {
    locks: HashMap<String, AgentLock>,
    stage: BookStage,
}

pub struct AgentLock {
    pub agent_id: String,
    pub locked_at: Option<DateTime<Utc>>,
    pub lock_reason: String,
    pub permanent: bool,
}

impl AgentLockManager {
    /// 阶段切换时自动锁定/解锁
    pub fn on_stage_change(&mut self, new_stage: BookStage) {
        match new_stage {
            BookStage::Planning => {
                // 只有规划者解锁
                self.lock_all_except(&["planner"]);
            }
            BookStage::KnowledgeInit => {
                // 规划者、调研者、观察者解锁
                self.lock_all_except(&["planner", "researcher", "observer"]);
            }
            BookStage::Writing => {
                // 规划者、调研者永久锁定
                self.lock_permanently(&["planner", "researcher"]);
                // 其他解锁
                self.unlock(&["tian_dao", "world_guardian", "liu_heping", 
                              "writer", "reviewer", "observer"]);
            }
        }
    }
    
    /// 检查Agent是否可用
    pub fn is_available(&self, agent_id: &str) -> bool;
}
```

---

## 六、分析系统组件

### 6.1 StyleAnalyzer（风格分析器）

```rust
pub struct StyleAnalyzer {
    vocabulary_analyzer: VocabularyAnalyzer,
    sentence_analyzer: SentenceAnalyzer,
    rhetoric_detector: RhetoricDetector,
}

pub struct StyleAnalysisResult {
    pub vocabulary_diversity: f32,
    pub sentence_variety: f32,
    pub rhetoric_usage: HashMap<String, u32>,
    pub overall_style_score: f32,
    pub style_signature: StyleSignature,
}

impl StyleAnalyzer {
    /// 分析文本风格
    pub fn analyze(&self, text: &str) -> StyleAnalysisResult;
    
    /// 对比两段文本的风格差异
    pub fn compare(&self, text1: &str, text2: &str) -> StyleDifference;
    
    /// 生成风格指纹
    pub fn generate_signature(&self, text: &str) -> StyleSignature;
}
```

### 6.2 EmotionAnalyzer（情绪分析器）

```rust
pub struct EmotionAnalyzer {
    emotion_model: EmotionModel,
}

pub struct EmotionCurve {
    pub segments: Vec<EmotionSegment>,
    pub overall_trend: EmotionTrend,
    pub peaks: Vec<EmotionPeak>,
    pub valleys: Vec<EmotionValley>,
}

pub struct EmotionSegment {
    pub start: usize,
    pub end: usize,
    pub dominant_emotion: Emotion,
    pub intensity: f32,
}

pub enum Emotion {
    Joy, Sadness, Anger, Fear, Surprise, Disgust, Anticipation, Trust,
}

impl EmotionAnalyzer {
    /// 分析情绪曲线
    pub fn analyze_curve(&self, text: &str) -> EmotionCurve;
    
    /// 预测读者情绪反应
    pub fn predict_reader_emotion(&self, text: &str) -> ReaderEmotionPrediction;
    
    /// 检测情绪转折点
    pub fn detect_turning_points(&self, curve: &EmotionCurve) -> Vec<TurningPoint>;
}
```

### 6.3 PacingAnalyzer（节奏分析器）

```rust
pub struct PacingAnalyzer;

pub struct PacingReport {
    pub overall_pacing: PacingLevel,
    pub chapter_pacing: Vec<ChapterPacing>,
    pub tension_curve: TensionCurve,
    pub suggestions: Vec<PacingSuggestion>,
}

pub enum PacingLevel {
    TooSlow,
    Slow,
    Balanced,
    Fast,
    TooFast,
}

impl PacingAnalyzer {
    /// 分析章节节奏
    pub fn analyze(&self, chapters: &[&str]) -> PacingReport;
    
    /// 计算紧张度曲线
    pub fn calculate_tension(&self, text: &str) -> TensionCurve;
    
    /// 检测节奏问题
    pub fn detect_issues(&self, report: &PacingReport) -> Vec<PacingIssue>;
}
```

### 6.4 ConsistencyChecker（一致性检查器）

```rust
pub struct ConsistencyChecker {
    worldview_checker: WorldviewChecker,
    character_checker: CharacterChecker,
    timeline_checker: TimelineChecker,
}

pub struct ConsistencyReport {
    pub issues: Vec<ConsistencyIssue>,
    pub overall_score: f32,
    pub checked_aspects: Vec<String>,
}

pub enum ConsistencyType {
    TimelineConflict,
    CharacterBehavior,
    CharacterVoice,
    WorldSetting,
    ForeshadowingOrphan,
    GeographicError,
    PowerLevelInconsistent,
}

impl ConsistencyChecker {
    /// 全面一致性检查
    pub fn check_all(&self, chapter: &str, context: &BookContext) -> ConsistencyReport;
    
    /// 检查角色一致性
    pub fn check_character(&self, character_id: &str, content: &str) -> Vec<ConsistencyIssue>;
    
    /// 检查世界观一致性
    pub fn check_worldview(&self, content: &str) -> Vec<ConsistencyIssue>;
    
    /// 检查时间线一致性
    pub fn check_timeline(&self, event: &TimelineEvent) -> Vec<ConsistencyIssue>;
}
```

---

## 七、每个Agent的完整部件配置

### 7.1 规划者 (Planner)

```yaml
agent:
  id: planner
  name: 规划者
  model: bailian-coding-plan/kimi-k2.5
  temperature: 0.3
  thinking:
    enabled: true
    budget_tokens: 8192
  lifecycle:
    active_stages: [planning]
    locked_after: knowledge_init

skills:
  - worldview-design
  - character-creation
  - faction-design
  - plot-outline
  - foreshadowing-setup

tools:
  - knowledge_write
  - character_create
  - faction_create
  - plot_create

hooks:
  on_stage_change:
    stage_2_start: lock_self
    stage_3_start: cleanup_session

knowledge_access:
  read: all
  write: all (stage_1 only)

permissions:
  edit_worldview: allow (stage_1)
  edit_characters: allow (stage_1)
  edit_plot: allow (stage_1)
  write_chapter: deny
  annotate: deny
```

### 7.2 天道 (TianDao)

```yaml
agent:
  id: tian_dao
  name: 天道
  model: bailian-coding-plan/glm-5
  temperature: 0.3
  thinking:
    enabled: true
    budget_tokens: 8192
  reasoning_effort: xhigh

skills:
  - plot-evolution
  - conflict-design
  - accident-factor
  - foreshadowing-trigger
  - character-arc

tools:
  - foreshadowing_manage
  - pressure_check
  - consistency_check
  - knowledge_update
  - timeline_query

hooks:
  pre_process: check_pressure_table
  post_write: update_foreshadowing_pool

knowledge_access:
  read: all
  write:
    - worldview
    - current_chapter
    - foreshadowing
    - factions
    - map

mcp_servers:
  - foreshadowing_mcp
  - plot_mcp

permissions:
  edit_worldview: allow
  edit_plot: allow
  edit_foreshadowing: allow
  write_chapter: deny
  annotate: allow

special_capabilities:
  - update_worldview (with notification)
  - trigger_foreshadowing
  - manage_pressure_gauge
```

### 7.3 世界观守护者 (WorldGuardian)

```yaml
agent:
  id: world_guardian
  name: 世界观守护者
  model: bailian-coding-plan/qwen3-max-2026-01-23
  temperature: 0.2
  thinking:
    enabled: true
    budget_tokens: 4096

skills:
  - consistency-check
  - setting-validate
  - worldview-update
  - timeline-check

tools:
  - consistency_checker
  - world_graph_query
  - rule_validator

hooks:
  post_write: generate_worldview_annotation
  on_conflict: notify_user

knowledge_access:
  read: all
  write:
    - worldview

mcp_servers:
  - worldview_mcp
  - consistency_mcp

permissions:
  edit_worldview: allow
  annotate: allow
  write_chapter: deny
```

### 7.4 刘和平 (LiuHeping)

```yaml
agent:
  id: liu_heping
  name: 刘和平
  model: bailian-coding-plan/qwen3-coder-next
  temperature: 0.4
  thinking:
    enabled: true
    budget_tokens: 4096

skills:
  - dialogue-writing
  - character-behavior
  - interaction-design
  - voice-consistency
  - relationship-design

tools:
  - character_db_query
  - voice_checker
  - relationship_query
  - behavior_analyzer

hooks:
  post_write: generate_character_annotation
  on_conflict: character_conflict_notification

knowledge_access:
  read: all
  write:
    - characters

mcp_servers:
  - character_mcp

permissions:
  edit_characters: allow
  annotate: allow
  write_chapter: deny
```

### 7.5 执笔 (Writer)

```yaml
agent:
  id: writer
  name: 执笔
  model: bailian-coding-plan/glm-5
  temperature: 0.7
  thinking:
    enabled: true
    budget_tokens: 8192
  max_tokens: 32768

skills:
  - prose-writing
  - scene-description
  - action-writing
  - emotion-writing
  - style-adjustment

tools:
  - text_editor
  - word_counter
  - style_checker
  - knowledge_search
  - character_query

hooks:
  pre_write: load_chapter_context
  post_write: notify_annotation_collection
  on_conflict: request_user_arbitration

knowledge_access:
  read: all
  write: []

mcp_servers:
  - text_mcp

permissions:
  write_chapter: allow
  edit_chapter: ask
  annotate: deny

constraints:
  - no_creative_autonomy
  - strict_annotation_compliance
  - conflict_requires_user_decision
```

### 7.6 审阅 (Reviewer)

```yaml
agent:
  id: reviewer
  name: 审阅
  model: bailian-coding-plan/qwen3-max-2026-01-23
  temperature: 0.2
  thinking:
    enabled: true
    budget_tokens: 4096

skills:
  - quality-review
  - readability-check
  - literary-evaluation
  - explosive-point

tools:
  - style_analyzer
  - emotion_analyzer
  - pacing_analyzer
  - readability_checker

hooks:
  post_write: generate_evaluation_report

knowledge_access:
  read:
    - all
    - init_md (explosive_reference)
  write:
    - book_opinion_md

mcp_servers:
  - analysis_mcp

permissions:
  annotate: allow
  edit_evaluation: allow
  write_chapter: deny
```

### 7.7 观察者 (Observer)

```yaml
agent:
  id: observer
  name: 观察者
  model: bailian-coding-plan/qwen3-coder-next
  temperature: 0.3

skills:
  - knowledge-sync
  - agent-coordinate
  - session-manage
  - workflow-orchestrate

tools:
  - all_knowledge_ops
  - agent_invoke
  - annotation_collector
  - session_manager
  - webdav_sync

hooks:
  post_write:
    - update_plot_history
    - sync_to_webdav
  on_stage_change:
    stage_1_to_2: create_knowledge_structure
    stage_2_to_3: lock_planner_researcher

knowledge_access:
  manage: all

mcp_servers:
  - knowledge_mcp
  - webdav_mcp

permissions:
  manage_knowledge: allow
  orchestrate_agents: allow
  write_chapter: deny
  annotate: deny

special_capabilities:
  - create_knowledge_base
  - trigger_annotation_collection
  - coordinate_agent_workflow
```

### 7.8 调研者 (Researcher)

```yaml
agent:
  id: researcher
  name: 调研者
  model: bailian-coding-plan/kimi-k2.5
  temperature: 0.3
  thinking:
    enabled: true
    budget_tokens: 8192
  lifecycle:
    active_stages: [knowledge_init]
    locked_after: writing

skills:
  - market-analysis
  - explosive-reference
  - reader-portrait
  - trend-analysis

tools:
  - reference_search
  - similarity_analyzer
  - pattern_extractor

hooks:
  on_stage_change:
    stage_2_start: activate_and_analyze
    stage_3_start: lock_self

knowledge_access:
  read:
    - reference_library
  write:
    - init_md

permissions:
  read_references: allow
  write_init: allow (once)
  write_chapter: deny
  annotate: deny
```

---

## 八、编程 vs 小说撰写 部件对比总表

| 领域 | OpenCode部件 | OpenNovel部件 | 开发状态 |
|------|-------------|---------------|---------|
| **结构操作** | LSP | TextEditor | 需开发 |
| **结构操作** | AST-Grep | SegmentSplit | 需开发 |
| **版本管理** | Git | WebDAV + User Managed | 需开发 |
| **知识库** | Skills/RAG | 7大知识库 | 需开发 |
| **知识库** | - | CharacterDB | **全新开发** |
| **知识库** | - | TimelineSystem | **全新开发** |
| **知识库** | - | WorldGraph | **全新开发** |
| **知识库** | - | ForeshadowingPool | **全新开发** |
| **一致性** | 类型检查/Lint | ConsistencyChecker | 需开发 |
| **搜索** | 代码搜索 | TextSearch + 知识库检索 | 需开发 |
| **协作** | PR/Review | AnnotationSystem | **全新开发** |
| **协作** | - | ConflictArbitration | **全新开发** |
| **协作** | - | ProactiveIntervention | **全新开发** |
| **协作** | - | GroupChat | **全新开发** |
| **协作** | - | AgentLock | **全新开发** |
| **自动化** | Hooks | Hooks | 可复用模式 |
| **外部集成** | MCP | MCP | 可复用 |
| **分析** | - | StyleAnalyzer | **全新开发** |
| **分析** | - | EmotionAnalyzer | **全新开发** |
| **分析** | - | PacingAnalyzer | **全新开发** |
| **分析** | - | ReaderSimulator | **全新开发** |
| **命令执行** | Terminal/Bash | - | 小说不需要 |
| **诊断** | LSP Diagnostics | StyleChecker | 需开发 |

---

## 九、NovelSDK 技术栈建议

```
NovelSDK/
├── core/                    # Rust + Tokio
│   ├── agent.rs
│   ├── session.rs
│   ├── intent_gate.rs
│   ├── message.rs
│   └── permission.rs
├── knowledge/               # Rust + Qdrant + SQLite
│   ├── worldview.rs
│   ├── character.rs
│   ├── plot.rs
│   ├── foreshadowing.rs
│   └── timeline.rs
├── text/                    # Rust
│   ├── editor.rs
│   ├── analyzer.rs
│   └── style.rs
├── collaboration/           # Rust
│   ├── annotation.rs
│   ├── conflict.rs
│   ├── proactive.rs
│   └── group_chat.rs
├── analysis/                # Rust + ONNX (可选)
│   ├── style_analyzer.rs
│   ├── emotion_analyzer.rs
│   ├── pacing_analyzer.rs
│   └── consistency.rs
└── sync/                    # Rust
    ├── webdav.rs
    └── version.rs
```

---

**文档版本**: v1.0
**创建时间**: 2026-03-15
**状态**: 设计完成，待开发