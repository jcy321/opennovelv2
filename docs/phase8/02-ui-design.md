# Phase 8: 前端界面设计规范

**生成日期**: 2026-03-17
**设计风格**: 专业商务风（参考钉钉）
**主色调**: 钉钉蓝 #1296db

---

## 一、设计理念

### 1.1 设计定位

**关键词**: 专业、高效、清晰、可信赖

OpenNovel 是一个专业的小说创作工具，用户是认真的创作者。界面应该：
- 传递专业感和可信赖感
- 减少视觉噪音，聚焦内容
- 支持长时间使用不疲劳
- 清晰的状态反馈和操作引导

### 1.2 与钉钉的相似性

| 维度 | 钉钉特征 | OpenNovel 应用 |
|------|---------|---------------|
| **色彩** | 蓝色主调 #1296db | 同样的蓝色系 |
| **布局** | 左侧导航 + 右侧内容 | 左侧项目列表 + 右侧主内容 |
| **卡片** | 白色卡片 + 浅灰背景 | 同样的卡片层次 |
| **状态** | 清晰的在线/离线标识 | Agent 状态指示 |
| **聊天** | 气泡式对话 | 群聊气泡式设计 |
| **间距** | 16px 基础间距 | 同样的间距系统 |

### 1.3 独特设计

为避免完全复制钉钉，我们加入小说创作特色：

| 元素 | 设计 |
|------|------|
| **Agent 头像** | 使用中文名首字 + 专属颜色 |
| **章节进度** | 可视化章节时间线 |
| **知识图谱** | 内嵌可视化面板 |
| **创作阶段** | 阶段指示条 |

---

## 二、设计系统

### 2.1 色彩系统

```css
/* 主色调 - 钉钉蓝系 */
--color-primary: #1296db;           /* 主色 */
--color-primary-light: #4fb0e8;     /* 浅色 */
--color-primary-dark: #0d7ab3;      /* 深色 */
--color-primary-bg: #e8f4fc;        /* 背景色 */

/* 中性色 */
--color-white: #ffffff;
--color-bg-page: #f5f6f7;           /* 页面背景 */
--color-bg-card: #ffffff;           /* 卡片背景 */
--color-bg-hover: #f0f2f5;          /* 悬停背景 */
--color-bg-active: #e8f4fc;         /* 激活背景 */

/* 文字色 */
--color-text-primary: #1f2329;      /* 主文字 */
--color-text-secondary: #646a73;    /* 次要文字 */
--color-text-placeholder: #8f959e;  /* 占位文字 */
--color-text-disabled: #bbbfc4;     /* 禁用文字 */

/* 边框色 */
--color-border: #dee0e3;
--color-border-light: #e5e6eb;

/* 状态色 */
--color-success: #34c724;
--color-warning: #ff9800;
--color-error: #f54a45;
--color-info: #1296db;

/* Agent 专属色 */
--agent-tiandao: #1890ff;           /* 天道 - 蓝 */
--agent-writer: #52c41a;            /* 执笔 - 绿 */
--agent-worldguardian: #fa8c16;     /* 世界观守护者 - 橙 */
--agent-liuheping: #eb2f96;         /* 刘和平 - 粉 */
--agent-planner: #722ed1;           /* 规划者 - 紫 */
--agent-reviewer: #13c2c2;          /* 审阅 - 青 */
--agent-observer: #faad14;          /* 观察者 - 金 */
--agent-researcher: #595959;        /* 调研者 - 灰 */

/* 阴影 */
--shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.05);
--shadow-md: 0 4px 12px rgba(0, 0, 0, 0.08);
--shadow-lg: 0 8px 24px rgba(0, 0, 0, 0.12);
```

### 2.2 字体系统

```css
/* 中文字体栈 - 参考钉钉 */
--font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", "PingFang SC", 
               "Hiragino Sans GB", "Microsoft YaHei", "Helvetica Neue", 
               Helvetica, Arial, sans-serif;

/* 代码字体 */
--font-family-mono: "SF Mono", "Fira Code", "JetBrains Mono", 
                    Consolas, "Liberation Mono", Menlo, monospace;

/* 字号 */
--font-size-xs: 12px;
--font-size-sm: 13px;
--font-size-base: 14px;
--font-size-lg: 16px;
--font-size-xl: 18px;
--font-size-2xl: 20px;
--font-size-3xl: 24px;
--font-size-4xl: 30px;

/* 行高 */
--line-height-tight: 1.25;
--line-height-base: 1.5;
--line-height-relaxed: 1.75;

/* 字重 */
--font-weight-normal: 400;
--font-weight-medium: 500;
--font-weight-semibold: 600;
--font-weight-bold: 700;
```

### 2.3 间距系统

```css
/* 基础单位 4px */
--space-1: 4px;
--space-2: 8px;
--space-3: 12px;
--space-4: 16px;
--space-5: 20px;
--space-6: 24px;
--space-8: 32px;
--space-10: 40px;
--space-12: 48px;
--space-16: 64px;
```

### 2.4 圆角系统

```css
--radius-sm: 4px;
--radius-base: 6px;
--radius-lg: 8px;
--radius-xl: 12px;
--radius-2xl: 16px;
--radius-full: 9999px;
```

### 2.5 过渡动画

```css
--transition-fast: 150ms ease;
--transition-base: 200ms ease;
--transition-slow: 300ms ease;
--transition-bounce: 300ms cubic-bezier(0.34, 1.56, 0.64, 1);
```

---

## 三、布局设计

### 3.1 整体布局

```
┌─────────────────────────────────────────────────────────────────────┐
│                           Top Bar (56px)                             │
│  [Logo] [搜索框]                    [通知] [设置] [用户头像]          │
├────────────┬────────────────────────────────────────────────────────┤
│            │                                                         │
│   Side     │                      Main Content                       │
│   Nav      │                                                         │
│   (240px)  │                    (flex: 1, min-width: 800px)          │
│            │                                                         │
│   ┌──────┐ │   ┌─────────────────────────────────────────────────┐  │
│   │书籍1 │ │   │                                                 │  │
│   ├──────┤ │   │                                                 │  │
│   │书籍2 │ │   │                                                 │  │
│   ├──────┤ │   │                                                 │  │
│   │ + 新建│ │   │                                                 │  │
│   └──────┘ │   └─────────────────────────────────────────────────┘  │
│            │                                                         │
└────────────┴────────────────────────────────────────────────────────┘
```

### 3.2 CSS Grid 实现

```css
/* 主布局 */
.app-layout {
  display: grid;
  grid-template-rows: 56px 1fr;
  grid-template-columns: 240px 1fr;
  height: 100vh;
  background: var(--color-bg-page);
}

/* 顶部栏 */
.top-bar {
  grid-column: 1 / -1;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 24px;
  background: var(--color-white);
  border-bottom: 1px solid var(--color-border);
  box-shadow: var(--shadow-sm);
}

/* 侧边导航 */
.side-nav {
  display: flex;
  flex-direction: column;
  padding: 16px 12px;
  background: var(--color-white);
  border-right: 1px solid var(--color-border);
  overflow-y: auto;
}

/* 主内容区 */
.main-content {
  padding: 24px;
  overflow-y: auto;
}
```

---

## 四、核心组件设计

### 4.1 Agent 头像组件

```svelte
<!-- AgentAvatar.svelte -->

<script lang="ts">
  export let name: string;        // Agent 名称
  export let status: 'idle' | 'thinking' | 'writing' | 'waiting' = 'idle';
  export let size: 'sm' | 'md' | 'lg' = 'md';
  
  const agentColors: Record<string, string> = {
    '天道': 'var(--agent-tiandao)',
    '执笔': 'var(--agent-writer)',
    '世界观守护者': 'var(--agent-worldguardian)',
    '刘和平': 'var(--agent-liuheping)',
    '规划者': 'var(--agent-planner)',
    '审阅': 'var(--agent-reviewer)',
    '观察者': 'var(--agent-observer)',
    '调研者': 'var(--agent-researcher)',
  };
  
  const statusColors: Record<string, string> = {
    'idle': 'var(--color-text-disabled)',
    'thinking': 'var(--color-primary)',
    'writing': 'var(--color-success)',
    'waiting': 'var(--color-warning)',
  };
  
  $: color = agentColors[name] || 'var(--color-primary)';
  $: statusColor = statusColors[status];
  $: initial = name.charAt(0);
  $: sizeClass = `avatar-${size}`;
</script>

<div class="agent-avatar {sizeClass}" style="--agent-color: {color}">
  <div class="avatar-inner">
    <span class="avatar-initial">{initial}</span>
  </div>
  {#if status !== 'idle'}
    <div class="status-dot" style="--status-color: {statusColor}"></div>
  {/if}
</div>

<style>
  .agent-avatar {
    position: relative;
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }
  
  .avatar-inner {
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--agent-color);
    border-radius: var(--radius-full);
    color: white;
    font-weight: var(--font-weight-semibold);
  }
  
  .avatar-sm .avatar-inner {
    width: 28px;
    height: 28px;
    font-size: var(--font-size-xs);
  }
  
  .avatar-md .avatar-inner {
    width: 36px;
    height: 36px;
    font-size: var(--font-size-sm);
  }
  
  .avatar-lg .avatar-inner {
    width: 48px;
    height: 48px;
    font(--font-size-lg);
  }
  
  .status-dot {
    position: absolute;
    bottom: 0;
    right: 0;
    width: 10px;
    height: 10px;
    background: var(--status-color);
    border: 2px solid var(--color-white);
    border-radius: var(--radius-full);
    animation: pulse 2s infinite;
  }
  
  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.5; }
  }
</style>
```

### 4.2 消息气泡组件

```svelte
<!-- MessageBubble.svelte -->

<script lang="ts">
  import AgentAvatar from './AgentAvatar.svelte';
  
  export let message: {
    id: string;
    role: 'user' | 'assistant' | 'system';
    agentName?: string;
    content: string;
    timestamp: Date;
    isStreaming?: boolean;
  };
  
  $: isUser = message.role === 'user';
  $: isSystem = message.role === 'system';
</script>

<div class="message-wrapper" class:user={isUser} class:system={isSystem}>
  {#if !isUser}
    <AgentAvatar 
      name={message.agentName || '系统'} 
      status={message.isStreaming ? 'thinking' : 'idle'}
      size="md"
    />
  {/if}
  
  <div class="message-content">
    {#if !isUser && message.agentName}
      <div class="agent-name">{message.agentName}</div>
    {/if}
    
    <div class="bubble" class:streaming={message.isStreaming}>
      <div class="text">{message.content}</div>
      {#if message.isStreaming}
        <span class="cursor">▊</span>
      {/if}
    </div>
    
    <div class="timestamp">
      {new Date(message.timestamp).toLocaleTimeString('zh-CN', { 
        hour: '2-digit', 
        minute: '2-digit' 
      })}
    </div>
  </div>
  
  {#if isUser}
    <div class="user-avatar">
      <span>我</span>
    </div>
  {/if}
</div>

<style>
  .message-wrapper {
    display: flex;
    gap: var(--space-3);
    margin-bottom: var(--space-5);
    animation: fadeIn 0.2s ease;
  }
  
  .message-wrapper.user {
    flex-direction: row-reverse;
  }
  
  .message-content {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    max-width: 70%;
  }
  
  .agent-name {
    font-size: var(--font-size-xs);
    color: var(--color-text-secondary);
    margin-left: var(--space-2);
  }
  
  .bubble {
    padding: var(--space-3) var(--space-4);
    border-radius: var(--radius-lg);
    background: var(--color-white);
    box-shadow: var(--shadow-sm);
    line-height: var(--line-height-relaxed);
    word-break: break-word;
  }
  
  .message-wrapper:not(.user) .bubble {
    border-top-left-radius: var(--radius-sm);
  }
  
  .message-wrapper.user .bubble {
    background: var(--color-primary);
    color: white;
    border-top-right-radius: var(--radius-sm);
  }
  
  .bubble.streaming {
    box-shadow: var(--shadow-sm), 0 0 0 2px var(--color-primary-bg);
  }
  
  .cursor {
    display: inline-block;
    animation: blink 1s infinite;
    color: var(--color-primary);
  }
  
  @keyframes blink {
    0%, 50% { opacity: 1; }
    51%, 100% { opacity: 0; }
  }
  
  .timestamp {
    font-size: var(--font-size-xs);
    color: var(--color-text-placeholder);
    padding: 0 var(--space-2);
  }
  
  .message-wrapper.user .timestamp {
    text-align: right;
  }
  
  .user-avatar {
    width: 36px;
    height: 36px;
    border-radius: var(--radius-full);
    background: var(--color-primary);
    color: white;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: var(--font-size-sm);
    flex-shrink: 0;
  }
  
  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(8px); }
    to { opacity: 1; transform: translateY(0); }
  }
</style>
```

### 4.3 侧边栏书籍列表

```svelte
<!-- BookList.svelte -->

<script lang="ts">
  export let books: Array<{
    id: string;
    title: string;
    stage: 'conception' | 'knowledge' | 'writing';
    chapterCount: number;
    lastActive: Date;
  }>;
  
  export let activeBookId: string | null = null;
  
  const stageLabels: Record<string, string> = {
    conception: '构思',
    knowledge: '知识库',
    writing: '撰写',
  };
  
  const stageColors: Record<string, string> = {
    conception: 'var(--color-warning)',
    knowledge: 'var(--color-info)',
    writing: 'var(--color-success)',
  };
</script>

<div class="book-list">
  <div class="list-header">
    <span>我的小说</span>
    <button class="add-btn" title="新建小说">
      <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
        <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/>
      </svg>
    </button>
  </div>
  
  <div class="books">
    {#each books as book (book.id)}
      <button 
        class="book-item" 
        class:active={book.id === activeBookId}
        onclick={() => activeBookId = book.id}
      >
        <div class="book-icon">📖</div>
        <div class="book-info">
          <div class="book-title">{book.title}</div>
          <div class="book-meta">
            <span 
              class="stage-badge" 
              style="--stage-color: {stageColors[book.stage]}"
            >
              {stageLabels[book.stage]}
            </span>
            <span class="chapter-count">{book.chapterCount} 章</span>
          </div>
        </div>
      </button>
    {/each}
  </div>
</div>

<style>
  .book-list {
    display: flex;
    flex-direction: column;
    height: 100%;
  }
  
  .list-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-2) var(--space-3);
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: var(--color-text-secondary);
  }
  
  .add-btn {
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: transparent;
    color: var(--color-text-secondary);
    border-radius: var(--radius-sm);
    cursor: pointer;
    transition: var(--transition-fast);
  }
  
  .add-btn:hover {
    background: var(--color-bg-hover);
    color: var(--color-primary);
  }
  
  .books {
    flex: 1;
    overflow-y: auto;
  }
  
  .book-item {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    width: 100%;
    padding: var(--space-3);
    border: none;
    background: transparent;
    text-align: left;
    cursor: pointer;
    border-radius: var(--radius-lg);
    transition: var(--transition-fast);
    margin-bottom: var(--space-1);
  }
  
  .book-item:hover {
    background: var(--color-bg-hover);
  }
  
  .book-item.active {
    background: var(--color-bg-active);
  }
  
  .book-icon {
    width: 40px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--color-primary-bg);
    border-radius: var(--radius-lg);
    font-size: 20px;
  }
  
  .book-info {
    flex: 1;
    min-width: 0;
  }
  
  .book-title {
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-medium);
    color: var(--color-text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  
  .book-meta {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    margin-top: var(--space-1);
    font-size: var(--font-size-xs);
    color: var(--color-text-secondary);
  }
  
  .stage-badge {
    padding: 2px 6px;
    background: color-mix(in srgb, var(--stage-color) 15%, white);
    color: var(--stage-color);
    border-radius: var(--radius-sm);
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-medium);
  }
</style>
```

### 4.4 Agent 状态面板

```svelte
<!-- AgentStatusPanel.svelte -->

<script lang="ts">
  import AgentAvatar from './AgentAvatar.svelte';
  
  const agents = [
    { name: '天道', role: '剧情编排', status: 'thinking' },
    { name: '执笔', role: '内容撰写', status: 'writing' },
    { name: '世界观守护者', role: '规则检查', status: 'idle' },
    { name: '刘和平', role: '人物塑造', status: 'idle' },
    { name: '规划者', role: '新书规划', status: 'idle', locked: true },
    { name: '审阅', role: '质量评估', status: 'idle' },
    { name: '观察者', role: '知识库管理', status: 'idle' },
    { name: '调研者', role: '爆点分析', status: 'idle', locked: true },
  ];
  
  const statusLabels: Record<string, string> = {
    idle: '空闲',
    thinking: '思考中',
    writing: '撰写中',
    waiting: '等待中',
  };
</script>

<div class="agent-panel">
  <div class="panel-header">
    <span class="title">Agent 状态</span>
    <span class="stage-badge">撰写阶段</span>
  </div>
  
  <div class="agent-list">
    {#each agents as agent}
      <div class="agent-item" class:locked={agent.locked}>
        <AgentAvatar name={agent.name} status={agent.status} size="sm" />
        
        <div class="agent-info">
          <span class="name">{agent.name}</span>
          <span class="role">{agent.role}</span>
        </div>
        
        {#if agent.locked}
          <span class="lock-icon" title="当前阶段锁定">🔒</span>
        {:else}
          <span class="status-label" class:active={agent.status !== 'idle'}>
            {statusLabels[agent.status]}
          </span>
        {/if}
      </div>
    {/each}
  </div>
</div>

<style>
  .agent-panel {
    background: var(--color-white);
    border-radius: var(--radius-xl);
    padding: var(--space-4);
    box-shadow: var(--shadow-sm);
  }
  
  .panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: var(--space-4);
  }
  
  .title {
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-primary);
  }
  
  .stage-badge {
    padding: var(--space-1) var(--space-2);
    background: var(--color-success);
    color: white;
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-medium);
    border-radius: var(--radius-sm);
  }
  
  .agent-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }
  
  .agent-item {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-2);
    border-radius: var(--radius-lg);
    transition: var(--transition-fast);
  }
  
  .agent-item:not(.locked):hover {
    background: var(--color-bg-hover);
  }
  
  .agent-item.locked {
    opacity: 0.5;
  }
  
  .agent-info {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-width: 0;
  }
  
  .name {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: var(--color-text-primary);
  }
  
  .role {
    font-size: var(--font-size-xs);
    color: var(--color-text-secondary);
  }
  
  .lock-icon {
    font-size: 14px;
  }
  
  .status-label {
    font-size: var(--font-size-xs);
    color: var(--color-text-placeholder);
    padding: 2px 8px;
    background: var(--color-bg-page);
    border-radius: var(--radius-sm);
  }
  
  .status-label.active {
    color: var(--color-primary);
    background: var(--color-primary-bg);
  }
</style>
```

---

## 五、页面设计

### 5.1 群聊主界面

```svelte
<!-- ChatPage.svelte -->

<script lang="ts">
  import MessageBubble from '$lib/components/chat/MessageBubble.svelte';
  import AgentStatusPanel from '$lib/components/chat/AgentStatusPanel.svelte';
  
  let messages = [
    { id: '1', role: 'user', content: '开始写第二章', timestamp: new Date() },
    { id: '2', role: 'assistant', agentName: '天道', content: '正在编排第二章剧情...', timestamp: new Date() },
    { id: '3', role: 'assistant', agentName: '执笔', content: '根据天道的设计，第二章将...', timestamp: new Date(), isStreaming: true },
  ];
  
  let inputText = '';
</script>

<div class="chat-page">
  <!-- 左侧：Agent 状态面板 -->
  <aside class="side-panel">
    <AgentStatusPanel />
    
    <div class="quick-actions">
      <button class="action-btn">
        <span>📊</span> 人物关系图
      </button>
      <button class="action-btn">
        <span>🗺️</span> 世界地图
      </button>
      <button class="action-btn">
        <span>📝</span> 知识库
      </button>
    </div>
  </aside>
  
  <!-- 中间：聊天区域 -->
  <main class="chat-area">
    <header class="chat-header">
      <h1>《仙途逆行》</h1>
      <div class="header-actions">
        <button class="icon-btn" title="章节列表">📑</button>
        <button class="icon-btn" title="导出">📤</button>
        <button class="icon-btn" title="设置">⚙️</button>
      </div>
    </header>
    
    <div class="messages-container">
      {#each messages as msg (msg.id)}
        <MessageBubble {message} />
      {/each}
    </div>
    
    <footer class="input-area">
      <div class="input-wrapper">
        <button class="attach-btn" title="附件">📎</button>
        <input 
          type="text" 
          bind:value={inputText}
          placeholder="输入消息，@提及 Agent..."
        />
        <button class="send-btn" class:active={inputText.length > 0}>
          发送
        </button>
      </div>
    </footer>
  </main>
</div>

<style>
  .chat-page {
    display: grid;
    grid-template-columns: 280px 1fr;
    gap: var(--space-4);
    height: calc(100vh - 56px);
    padding: var(--space-4);
    background: var(--color-bg-page);
  }
  
  .side-panel {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }
  
  .quick-actions {
    background: var(--color-white);
    border-radius: var(--radius-xl);
    padding: var(--space-3);
    box-shadow: var(--shadow-sm);
  }
  
  .action-btn {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    width: 100%;
    padding: var(--space-3);
    border: none;
    background: transparent;
    border-radius: var(--radius-lg);
    font-size: var(--font-size-sm);
    color: var(--color-text-primary);
    cursor: pointer;
    transition: var(--transition-fast);
  }
  
  .action-btn:hover {
    background: var(--color-bg-hover);
  }
  
  .chat-area {
    display: flex;
    flex-direction: column;
    background: var(--color-white);
    border-radius: var(--radius-xl);
    box-shadow: var(--shadow-sm);
    overflow: hidden;
  }
  
  .chat-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-4) var(--space-6);
    border-bottom: 1px solid var(--color-border-light);
  }
  
  .chat-header h1 {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-primary);
  }
  
  .header-actions {
    display: flex;
    gap: var(--space-2);
  }
  
  .icon-btn {
    width: 36px;
    height: 36px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: transparent;
    border-radius: var(--radius-lg);
    cursor: pointer;
    font-size: 18px;
    transition: var(--transition-fast);
  }
  
  .icon-btn:hover {
    background: var(--color-bg-hover);
  }
  
  .messages-container {
    flex: 1;
    overflow-y: auto;
    padding: var(--space-6);
  }
  
  .input-area {
    padding: var(--space-4) var(--space-6);
    border-top: 1px solid var(--color-border-light);
  }
  
  .input-wrapper {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-3) var(--space-4);
    background: var(--color-bg-page);
    border-radius: var(--radius-xl);
    border: 1px solid var(--color-border);
    transition: var(--transition-fast);
  }
  
  .input-wrapper:focus-within {
    border-color: var(--color-primary);
    box-shadow: 0 0 0 3px var(--color-primary-bg);
  }
  
  .attach-btn {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: transparent;
    font-size: 18px;
    cursor: pointer;
    opacity: 0.6;
    transition: var(--transition-fast);
  }
  
  .attach-btn:hover {
    opacity: 1;
  }
  
  input {
    flex: 1;
    border: none;
    background: transparent;
    font-size: var(--font-size-base);
    color: var(--color-text-primary);
    outline: none;
  }
  
  input::placeholder {
    color: var(--color-text-placeholder);
  }
  
  .send-btn {
    padding: var(--space-2) var(--space-4);
    border: none;
    background: var(--color-text-disabled);
    color: white;
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    border-radius: var(--radius-lg);
    cursor: not-allowed;
    transition: var(--transition-fast);
  }
  
  .send-btn.active {
    background: var(--color-primary);
    cursor: pointer;
  }
  
  .send-btn.active:hover {
    background: var(--color-primary-dark);
  }
</style>
```

### 5.2 Provider 配置页面

```svelte
<!-- ProviderSettingsPage.svelte -->

<script lang="ts">
  let providers = [
    { 
      id: '1', 
      name: 'DeepSeek', 
      type: 'openai-compatible',
      baseUrl: 'https://api.deepseek.com/v1',
      enabled: true,
      status: 'connected',
      models: ['deepseek-chat', 'deepseek-coder']
    },
    { 
      id: '2', 
      name: 'OpenAI', 
      type: 'openai',
      baseUrl: 'https://api.openai.com/v1',
      enabled: false,
      status: 'disconnected',
      models: ['gpt-4', 'gpt-3.5-turbo']
    },
  ];
</script>

<div class="settings-page">
  <header class="page-header">
    <h1>LLM Provider 配置</h1>
    <button class="primary-btn">
      <span>+</span> 添加 Provider
    </button>
  </header>
  
  <div class="provider-grid">
    {#each providers as provider}
      <div class="provider-card">
        <div class="card-header">
          <div class="provider-icon" class:active={provider.enabled}>
            {provider.name.charAt(0)}
          </div>
          <div class="provider-info">
            <h3>{provider.name}</h3>
            <span class="type-badge">{provider.type}</span>
          </div>
          <div class="status-indicator" class:connected={provider.status === 'connected'}>
            {provider.status === 'connected' ? '已连接' : '未连接'}
          </div>
        </div>
        
        <div class="card-body">
          <div class="info-row">
            <span class="label">Base URL</span>
            <span class="value">{provider.baseUrl}</span>
          </div>
          <div class="info-row">
            <span class="label">模型</span>
            <span class="value">{provider.models.join(', ')}</span>
          </div>
        </div>
        
        <div class="card-footer">
          <button class="test-btn">测试连接</button>
          <button class="edit-btn">编辑</button>
          <button class="toggle-btn" class:active={provider.enabled}>
            {provider.enabled ? '禁用' : '启用'}
          </button>
        </div>
      </div>
    {/each}
  </div>
</div>

<style>
  .settings-page {
    padding: var(--space-6);
    max-width: 1200px;
    margin: 0 auto;
  }
  
  .page-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: var(--space-6);
  }
  
  .page-header h1 {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-primary);
  }
  
  .primary-btn {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    padding: var(--space-3) var(--space-5);
    border: none;
    background: var(--color-primary);
    color: white;
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-medium);
    border-radius: var(--radius-lg);
    cursor: pointer;
    transition: var(--transition-fast);
  }
  
  .primary-btn:hover {
    background: var(--color-primary-dark);
  }
  
  .provider-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(380px, 1fr));
    gap: var(--space-5);
  }
  
  .provider-card {
    background: var(--color-white);
    border-radius: var(--radius-xl);
    box-shadow: var(--shadow-sm);
    overflow: hidden;
    transition: var(--transition-base);
  }
  
  .provider-card:hover {
    box-shadow: var(--shadow-md);
  }
  
  .card-header {
    display: flex;
    align-items: center;
    gap: var(--space-4);
    padding: var(--space-5);
    border-bottom: 1px solid var(--color-border-light);
  }
  
  .provider-icon {
    width: 48px;
    height: 48px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--color-bg-page);
    border-radius: var(--radius-lg);
    font-size: var(--font-size-xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text-secondary);
  }
  
  .provider-icon.active {
    background: var(--color-primary-bg);
    color: var(--color-primary);
  }
  
  .provider-info {
    flex: 1;
  }
  
  .provider-info h3 {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-primary);
    margin-bottom: var(--space-1);
  }
  
  .type-badge {
    font-size: var(--font-size-xs);
    color: var(--color-text-secondary);
    padding: 2px 8px;
    background: var(--color-bg-page);
    border-radius: var(--radius-sm);
  }
  
  .status-indicator {
    font-size: var(--font-size-xs);
    padding: var(--space-1) var(--space-2);
    background: var(--color-error);
    color: white;
    border-radius: var(--radius-sm);
  }
  
  .status-indicator.connected {
    background: var(--color-success);
  }
  
  .card-body {
    padding: var(--space-4) var(--space-5);
  }
  
  .info-row {
    display: flex;
    justify-content: space-between;
    padding: var(--space-2) 0;
    font-size: var(--font-size-sm);
  }
  
  .info-row .label {
    color: var(--color-text-secondary);
  }
  
  .info-row .value {
    color: var(--color-text-primary);
    font-weight: var(--font-weight-medium);
  }
  
  .card-footer {
    display: flex;
    gap: var(--space-2);
    padding: var(--space-4) var(--space-5);
    border-top: 1px solid var(--color-border-light);
  }
  
  .card-footer button {
    flex: 1;
    padding: var(--space-2) var(--space-3);
    border: 1px solid var(--color-border);
    background: var(--color-white);
    font-size: var(--font-size-sm);
    border-radius: var(--radius-lg);
    cursor: pointer;
    transition: var(--transition-fast);
  }
  
  .card-footer button:hover {
    background: var(--color-bg-hover);
  }
  
  .test-btn {
    color: var(--color-primary);
    border-color: var(--color-primary);
  }
  
  .toggle-btn.active {
    background: var(--color-error);
    border-color: var(--color-error);
    color: white;
  }
</style>
```

---

## 六、响应式设计

### 6.1 断点定义

```css
/* 响应式断点 */
--breakpoint-sm: 640px;
--breakpoint-md: 768px;
--breakpoint-lg: 1024px;
--breakpoint-xl: 1280px;
--breakpoint-2xl: 1536px;
```

### 6.2 移动端适配

```css
/* 移动端隐藏侧边栏 */
@media (max-width: 768px) {
  .app-layout {
    grid-template-columns: 1fr;
  }
  
  .side-nav {
    position: fixed;
    left: -280px;
    top: 56px;
    bottom: 0;
    width: 280px;
    z-index: 100;
    transition: var(--transition-base);
  }
  
  .side-nav.open {
    left: 0;
  }
  
  .chat-page {
    grid-template-columns: 1fr;
  }
  
  .side-panel {
    display: none;
  }
}
```

---

## 七、暗色主题（可选）

```css
/* 暗色主题变量 */
[data-theme="dark"] {
  --color-bg-page: #1a1a1a;
  --color-bg-card: #2d2d2d;
  --color-bg-hover: #3d3d3d;
  --color-bg-active: #1e3a5f;
  
  --color-text-primary: #ffffff;
  --color-text-secondary: #a0a0a0;
  --color-text-placeholder: #707070;
  
  --color-border: #404040;
  --color-border-light: #353535;
  
  --shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.3);
  --shadow-md: 0 4px 12px rgba(0, 0, 0, 0.4);
  --shadow-lg: 0 8px 24px rgba(0, 0, 0, 0.5);
}
```

---

## 八、动画效果

### 8.1 页面加载动画

```css
/* 页面入场动画 */
@keyframes pageEnter {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.page-enter {
  animation: pageEnter 0.4s ease-out;
}

/* 列表项交错入场 */
.list-item {
  animation: fadeInUp 0.3s ease-out;
  animation-delay: calc(var(--index) * 50ms);
}

@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
```

### 8.2 微交互

```css
/* 按钮悬停效果 */
.btn-hover-lift {
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}

.btn-hover-lift:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-md);
}

/* 卡片悬停效果 */
.card-hover-scale {
  transition: transform 0.2s ease;
}

.card-hover-scale:hover {
  transform: scale(1.02);
}
```

---

## 九、图标系统

使用 emoji + SVG 混合方案：

| 类型 | 图标 | 说明 |
|------|------|------|
| 书籍 | 📖 | 书籍列表项 |
| 发送 | ➤ | 发送按钮 |
| 设置 | ⚙️ | 设置入口 |
| 通知 | 🔔 | 通知图标 |
| 锁定 | 🔒 | Agent 锁定状态 |
| 图表 | 📊 | 人物关系图 |
| 地图 | 🗺️ | 世界地图 |
| 文档 | 📝 | 知识库 |

---

## 十、设计检查清单

- [x] 主色调为钉钉蓝 #1296db，无紫色背景
- [x] 使用系统字体栈，兼容中英文
- [x] 清晰的视觉层次：页面 → 卡片 → 元素
- [x] 一致的间距和圆角
- [x] Agent 有专属颜色标识
- [x] 状态反馈清晰（在线/思考/撰写）
- [x] 流式输出有打字机效果
- [x] 响应式设计支持移动端
- [x] 无障碍支持（语义化标签）
- [x] 动画适度，不影响性能