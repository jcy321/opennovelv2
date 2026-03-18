# Phase 8: 前端交互逻辑设计

**生成日期**: 2026-03-17
**文档类型**: 详细操作逻辑规范

---

## 一、全局状态管理

### 1.1 Store 结构

```typescript
// src/lib/stores/index.ts

import { writable, derived, get } from 'svelte/store';

// ─────────────────────────────────────────────────────
// 类型定义
// ─────────────────────────────────────────────────────

export interface Book {
  id: string;
  title: string;
  author: string;
  stage: 'conception' | 'knowledge' | 'writing';
  status: 'draft' | 'writing' | 'paused' | 'completed';
  chapterCount: number;
  wordCount: number;
  lastActive: Date;
  createdAt: Date;
}

export interface Message {
  id: string;
  bookId: string;
  role: 'user' | 'assistant' | 'system';
  agentName?: string;
  content: string;
  annotations: Annotation[];
  timestamp: Date;
  isStreaming?: boolean;
  streamingContent?: string;
}

export interface Annotation {
  id: string;
  agentName: string;
  type: 'suggestion' | 'warning' | 'correction' | 'info';
  content: string;
  position?: { start: number; end: number };
}

export interface AgentStatus {
  name: string;
  role: string;
  status: 'idle' | 'thinking' | 'writing' | 'waiting' | 'error';
  currentTask?: string;
  lastActivity?: Date;
  isLocked: boolean;
}

export interface Provider {
  id: string;
  name: string;
  type: 'openai' | 'anthropic' | 'openai-compatible' | 'custom';
  baseUrl: string;
  enabled: boolean;
  models: Model[];
  status: 'connected' | 'disconnected' | 'testing' | 'error';
  lastTestAt?: Date;
  errorMessage?: string;
}

export interface Model {
  id: string;
  name: string;
  modelId: string;
  enabled: boolean;
  supportsThinking: boolean;
  maxContextLength: number;
}

// ─────────────────────────────────────────────────────
// 核心状态
// ─────────────────────────────────────────────────────

// 当前用户
export const currentUser = writable<{
  id: string;
  name: string;
  avatar?: string;
} | null>(null);

// 书籍列表
export const books = writable<Book[]>([]);

// 当前选中的书籍 ID
export const activeBookId = writable<string | null>(null);

// 当前书籍详情
export const activeBook = derived(
  [books, activeBookId],
  ([$books, $activeBookId]) => {
    return $books.find(b => b.id === $activeBookId) || null;
  }
);

// 消息列表（按书籍分组）
export const messagesByBook = writable<Record<string, Message[]>>({});

// 当前书籍的消息
export const currentMessages = derived(
  [messagesByBook, activeBookId],
  ([$messagesByBook, $activeBookId]) => {
    if (!$activeBookId) return [];
    return $messagesByBook[$activeBookId] || [];
  }
);

// Agent 状态列表
export const agentStatuses = writable<AgentStatus[]>([
  { name: '天道', role: '剧情编排', status: 'idle', isLocked: false },
  { name: '执笔', role: '内容撰写', status: 'idle', isLocked: false },
  { name: '世界观守护者', role: '规则检查', status: 'idle', isLocked: false },
  { name: '刘和平', role: '人物塑造', status: 'idle', isLocked: false },
  { name: '规划者', role: '新书规划', status: 'idle', isLocked: true },
  { name: '审阅', role: '质量评估', status: 'idle', isLocked: false },
  { name: '观察者', role: '知识库管理', status: 'idle', isLocked: false },
  { name: '调研者', role: '爆点分析', status: 'idle', isLocked: true },
]);

// Provider 列表
export const providers = writable<Provider[]>([]);

// 当前选中的模型
export const selectedModel = writable<{
  providerId: string;
  modelId: string;
} | null>(null);

// UI 状态
export const uiState = writable({
  sidebarCollapsed: false,
  rightPanelOpen: false,
  rightPanelTab: 'agents' as 'agents' | 'knowledge' | 'settings',
  theme: 'light' as 'light' | 'dark',
  isMobile: false,
});

// 加载状态
export const loadingState = writable({
  booksLoading: false,
  messagesLoading: false,
  sendingMessage: false,
  providersLoading: false,
});

// 错误状态
export const errors = writable<{
  global?: string;
  chat?: string;
  provider?: string;
}>({});
```

### 1.2 Store 操作方法

```typescript
// src/lib/stores/actions.ts

import { get } from 'svelte/store';
import {
  books, activeBookId, messagesByBook, agentStatuses,
  providers, selectedModel, loadingState, errors
} from './index';
import { apiClient } from '$lib/api/client';

// ─────────────────────────────────────────────────────
// 书籍操作
// ─────────────────────────────────────────────────────

export const bookActions = {
  /** 加载书籍列表 */
  async loadBooks() {
    loadingState.update(s => ({ ...s, booksLoading: true }));
    try {
      const response = await apiClient.get<Book[]>('/api/books');
      books.set(response);
    } catch (error) {
      errors.update(e => ({ ...e, global: '加载书籍列表失败' }));
      throw error;
    } finally {
      loadingState.update(s => ({ ...s, booksLoading: false }));
    }
  },

  /** 创建新书 */
  async createBook(data: { title: string; description?: string }) {
    try {
      const book = await apiClient.post<Book>('/api/books', data);
      books.update(list => [...list, book]);
      return book;
    } catch (error) {
      errors.update(e => ({ ...e, global: '创建书籍失败' }));
      throw error;
    }
  },

  /** 选择书籍 */
  selectBook(bookId: string) {
    activeBookId.set(bookId);
    // 加载该书籍的消息
    messageActions.loadMessages(bookId);
    // 连接 SSE 流
    streamActions.connect(bookId);
  },

  /** 删除书籍 */
  async deleteBook(bookId: string) {
    try {
      await apiClient.delete(`/api/books/${bookId}`);
      books.update(list => list.filter(b => b.id !== bookId));
      if (get(activeBookId) === bookId) {
        activeBookId.set(null);
      }
    } catch (error) {
      errors.update(e => ({ ...e, global: '删除书籍失败' }));
      throw error;
    }
  },

  /** 更新书籍阶段 */
  async updateStage(bookId: string, stage: Book['stage']) {
    try {
      await apiClient.patch(`/api/books/${bookId}`, { stage });
      books.update(list => 
        list.map(b => b.id === bookId ? { ...b, stage } : b)
      );
    } catch (error) {
      throw error;
    }
  }
};

// ─────────────────────────────────────────────────────
// 消息操作
// ─────────────────────────────────────────────────────

export const messageActions = {
  /** 加载消息列表 */
  async loadMessages(bookId: string, options?: { limit?: number; before?: string }) {
    loadingState.update(s => ({ ...s, messagesLoading: true }));
    try {
      const params = new URLSearchParams();
      if (options?.limit) params.set('limit', String(options.limit));
      if (options?.before) params.set('before', options.before);
      
      const response = await apiClient.get<Message[]>(
        `/api/books/${bookId}/messages?${params}`
      );
      
      messagesByBook.update(map => ({
        ...map,
        [bookId]: response
      }));
    } catch (error) {
      errors.update(e => ({ ...e, chat: '加载消息失败' }));
      throw error;
    } finally {
      loadingState.update(s => ({ ...s, messagesLoading: false }));
    }
  },

  /** 发送消息 */
  async sendMessage(bookId: string, content: string) {
    if (!content.trim()) return;
    
    loadingState.update(s => ({ ...s, sendingMessage: true }));
    errors.update(e => ({ ...e, chat: undefined }));
    
    // 乐观更新：立即显示用户消息
    const tempId = `temp-${Date.now()}`;
    const userMessage: Message = {
      id: tempId,
      bookId,
      role: 'user',
      content: content.trim(),
      annotations: [],
      timestamp: new Date(),
    };
    
    messagesByBook.update(map => ({
      ...map,
      [bookId]: [...(map[bookId] || []), userMessage]
    }));
    
    try {
      // 发送到服务器
      const response = await apiClient.post<Message>(
        `/api/books/${bookId}/chat`,
        { content: content.trim() }
      );
      
      // 替换临时消息为真实消息
      messagesByBook.update(map => ({
        ...map,
        [bookId]: (map[bookId] || []).map(m => 
          m.id === tempId ? response : m
        )
      }));
      
      return response;
    } catch (error) {
      // 移除乐观更新的消息
      messagesByBook.update(map => ({
        ...map,
        [bookId]: (map[bookId] || []).filter(m => m.id !== tempId)
      }));
      errors.update(e => ({ ...e, chat: '发送消息失败' }));
      throw error;
    } finally {
      loadingState.update(s => ({ ...s, sendingMessage: false }));
    }
  },

  /** 添加流式消息 */
  addStreamingMessage(bookId: string, agentName: string) {
    const tempId = `stream-${Date.now()}`;
    const streamingMessage: Message = {
      id: tempId,
      bookId,
      role: 'assistant',
      agentName,
      content: '',
      annotations: [],
      timestamp: new Date(),
      isStreaming: true,
      streamingContent: '',
    };
    
    messagesByBook.update(map => ({
      ...map,
      [bookId]: [...(map[bookId] || []), streamingMessage]
    }));
    
    return tempId;
  },

  /** 更新流式内容 */
  updateStreamingContent(bookId: string, messageId: string, chunk: string) {
    messagesByBook.update(map => ({
      ...map,
      [bookId]: (map[bookId] || []).map(m => 
        m.id === messageId 
          ? { ...m, streamingContent: (m.streamingContent || '') + chunk }
          : m
      )
    }));
  },

  /** 完成流式消息 */
  completeStreamingMessage(bookId: string, tempId: string, finalMessage: Message) {
    messagesByBook.update(map => ({
      ...map,
      [bookId]: (map[bookId] || []).map(m => 
        m.id === tempId ? { ...finalMessage, isStreaming: false } : m
      )
    }));
  },

  /** 加载更多历史消息 */
  async loadMoreMessages(bookId: string) {
    const currentMsgs = get(messagesByBook)[bookId] || [];
    if (currentMsgs.length === 0) return;
    
    const oldestMsg = currentMsgs[0];
    await this.loadMessages(bookId, { 
      limit: 50, 
      before: oldestMsg.id 
    });
  }
};

// ─────────────────────────────────────────────────────
// Agent 状态操作
// ─────────────────────────────────────────────────────

export const agentActions = {
  /** 更新单个 Agent 状态 */
  updateStatus(agentName: string, status: Partial<AgentStatus>) {
    agentStatuses.update(list => 
      list.map(a => a.name === agentName ? { ...a, ...status } : a)
    );
  },

  /** 设置 Agent 为思考中 */
  setThinking(agentName: string, task?: string) {
    this.updateStatus(agentName, { 
      status: 'thinking', 
      currentTask: task 
    });
  },

  /** 设置 Agent 为撰写中 */
  setWriting(agentName: string) {
    this.updateStatus(agentName, { 
      status: 'writing' 
    });
  },

  /** 重置 Agent 为空闲 */
  setIdle(agentName: string) {
    this.updateStatus(agentName, { 
      status: 'idle', 
      currentTask: undefined 
    });
  },

  /** 根据阶段锁定/解锁 Agent */
  updateLocksByStage(stage: Book['stage']) {
    const locks: Record<string, boolean> = {
      conception: {
        '规划者': false,
        '调研者': true,
        '天道': true,
        '执笔': true,
        '世界观守护者': true,
        '刘和平': true,
        '审阅': true,
        '观察者': true,
      },
      knowledge: {
        '规划者': false,
        '调研者': false,
        '天道': true,
        '执笔': true,
        '世界观守护者': true,
        '刘和平': true,
        '审阅': true,
        '观察者': false,
      },
      writing: {
        '规划者': true,
        '调研者': true,
        '天道': false,
        '执笔': false,
        '世界观守护者': false,
        '刘和平': false,
        '审阅': false,
        '观察者': false,
      },
    };
    
    const stageLocks = locks[stage] || locks.writing;
    agentStatuses.update(list => 
      list.map(a => ({ ...a, isLocked: stageLocks[a.name] ?? false }))
    );
  }
};

// ─────────────────────────────────────────────────────
// Provider 操作
// ─────────────────────────────────────────────────────

export const providerActions = {
  /** 加载 Provider 列表 */
  async loadProviders() {
    loadingState.update(s => ({ ...s, providersLoading: true }));
    try {
      const response = await apiClient.get<{ providers: Provider[] }>('/api/providers');
      providers.set(response.providers);
    } catch (error) {
      errors.update(e => ({ ...e, provider: '加载 Provider 失败' }));
      throw error;
    } finally {
      loadingState.update(s => ({ ...s, providersLoading: false }));
    }
  },

  /** 创建 Provider */
  async createProvider(data: {
    name: string;
    type: Provider['type'];
    baseUrl: string;
    apiKey: string;
    models: { name: string; modelId: string }[];
  }) {
    try {
      const provider = await apiClient.post<Provider>('/api/providers', data);
      providers.update(list => [...list, provider]);
      return provider;
    } catch (error) {
      errors.update(e => ({ ...e, provider: '创建 Provider 失败' }));
      throw error;
    }
  },

  /** 测试 Provider 连接 */
  async testProvider(providerId: string) {
    // 更新状态为测试中
    providers.update(list => 
      list.map(p => p.id === providerId 
        ? { ...p, status: 'testing' as const } 
        : p
      )
    );
    
    try {
      const response = await apiClient.post<{
        success: boolean;
        availableModels?: string[];
        error?: string;
      }>(`/api/providers/${providerId}/test`);
      
      providers.update(list => 
        list.map(p => p.id === providerId 
          ? { 
              ...p, 
              status: response.success ? 'connected' : 'error',
              lastTestAt: new Date(),
              errorMessage: response.error
            } 
          : p
        )
      );
      
      return response;
    } catch (error) {
      providers.update(list => 
        list.map(p => p.id === providerId 
          ? { ...p, status: 'error' as const, errorMessage: '测试失败' } 
          : p
        )
      );
      throw error;
    }
  },

  /** 更新 Provider */
  async updateProvider(providerId: string, data: Partial<Provider>) {
    try {
      await apiClient.patch(`/api/providers/${providerId}`, data);
      providers.update(list => 
        list.map(p => p.id === providerId ? { ...p, ...data } : p)
      );
    } catch (error) {
      throw error;
    }
  },

  /** 删除 Provider */
  async deleteProvider(providerId: string) {
    try {
      await apiClient.delete(`/api/providers/${providerId}`);
      providers.update(list => list.filter(p => p.id !== providerId));
    } catch (error) {
      throw error;
    }
  },

  /** 切换 Provider 启用状态 */
  async toggleProvider(providerId: string) {
    const current = get(providers).find(p => p.id === providerId);
    if (!current) return;
    
    await this.updateProvider(providerId, { enabled: !current.enabled });
  }
};
```

---

## 二、SSE 流式连接

### 2.1 流管理器

```typescript
// src/lib/api/stream.ts

import { get } from 'svelte/store';
import { messageActions, agentActions } from '$lib/stores/actions';
import { activeBookId } from '$lib/stores/index';

interface StreamConnection {
  bookId: string;
  eventSource: EventSource;
  reconnectAttempts: number;
  lastEventId?: string;
}

class StreamManager {
  private connections: Map<string, StreamConnection> = new Map();
  private maxReconnectAttempts = 5;
  private reconnectDelay = 1000;

  /** 连接到书籍的 SSE 流 */
  connect(bookId: string) {
    // 如果已有连接，先断开
    this.disconnect(bookId);
    
    const eventSource = new EventSource(
      `/api/books/${bookId}/stream`,
      { withCredentials: true }
    );
    
    const connection: StreamConnection = {
      bookId,
      eventSource,
      reconnectAttempts: 0,
    };
    
    // 连接成功
    eventSource.addEventListener('open', () => {
      console.log(`[SSE] Connected to book ${bookId}`);
      connection.reconnectAttempts = 0;
    });
    
    // 接收消息
    eventSource.addEventListener('message', (event) => {
      this.handleMessage(bookId, event);
    });
    
    // Agent 状态变更
    eventSource.addEventListener('agent_status', (event) => {
      this.handleAgentStatus(event);
    });
    
    // 错误处理
    eventSource.addEventListener('error', (event) => {
      this.handleError(bookId, event);
    });
    
    this.connections.set(bookId, connection);
  }

  /** 断开连接 */
  disconnect(bookId: string) {
    const connection = this.connections.get(bookId);
    if (connection) {
      connection.eventSource.close();
      this.connections.delete(bookId);
    }
  }

  /** 断开所有连接 */
  disconnectAll() {
    for (const [bookId] of this.connections) {
      this.disconnect(bookId);
    }
  }

  /** 处理消息事件 */
  private handleMessage(bookId: string, event: MessageEvent) {
    try {
      const data = JSON.parse(event.data);
      
      switch (data.type) {
        // 流式输出开始
        case 'stream_start': {
          const { agentName, messageId } = data;
          messageActions.addStreamingMessage(bookId, agentName);
          agentActions.setThinking(agentName, '正在生成内容');
          break;
        }
        
        // 流式输出内容块
        case 'stream_chunk': {
          const { messageId, content } = data;
          messageActions.updateStreamingContent(bookId, messageId, content);
          break;
        }
        
        // 流式输出完成
        case 'stream_complete': {
          const { messageId, message } = data;
          messageActions.completeStreamingMessage(bookId, messageId, message);
          if (message.agentName) {
            agentActions.setIdle(message.agentName);
          }
          break;
        }
        
        // 完整消息（非流式）
        case 'message': {
          messageActions.loadMessages(bookId);
          break;
        }
        
        // 批注
        case 'annotation': {
          // 处理批注...
          break;
        }
      }
    } catch (error) {
      console.error('[SSE] Failed to parse message:', error);
    }
  }

  /** 处理 Agent 状态变更 */
  private handleAgentStatus(event: MessageEvent) {
    try {
      const { agentName, status, task } = JSON.parse(event.data);
      
      switch (status) {
        case 'thinking':
          agentActions.setThinking(agentName, task);
          break;
        case 'writing':
          agentActions.setWriting(agentName);
          break;
        case 'idle':
          agentActions.setIdle(agentName);
          break;
      }
    } catch (error) {
      console.error('[SSE] Failed to parse agent status:', error);
    }
  }

  /** 处理错误 */
  private handleError(bookId: string, event: Event) {
    const connection = this.connections.get(bookId);
    if (!connection) return;
    
    console.error(`[SSE] Error for book ${bookId}:`, event);
    
    // 检查是否需要重连
    if (connection.reconnectAttempts < this.maxReconnectAttempts) {
      connection.reconnectAttempts++;
      const delay = this.reconnectDelay * connection.reconnectAttempts;
      
      console.log(`[SSE] Reconnecting in ${delay}ms (attempt ${connection.reconnectAttempts})`);
      
      setTimeout(() => {
        // 检查当前书籍是否仍然活跃
        if (get(activeBookId) === bookId) {
          this.connect(bookId);
        }
      }, delay);
    } else {
      console.error(`[SSE] Max reconnect attempts reached for book ${bookId}`);
      this.disconnect(bookId);
    }
  }

  /** 发送心跳 */
  sendHeartbeat(bookId: string) {
    const connection = this.connections.get(bookId);
    if (connection && connection.eventSource.readyState === EventSource.OPEN) {
      // SSE 是单向的，心跳由服务器发送
    }
  }
}

export const streamManager = new StreamManager();

// 导出便捷方法
export const streamActions = {
  connect: (bookId: string) => streamManager.connect(bookId),
  disconnect: (bookId: string) => streamManager.disconnect(bookId),
  disconnectAll: () => streamManager.disconnectAll(),
};
```

---

## 三、页面交互逻辑

### 3.1 主页面逻辑

```svelte
<!-- src/routes/+page.svelte -->

<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { get } from 'svelte/store';
  import BookList from '$lib/components/BookList.svelte';
  import ChatPage from '$lib/components/ChatPage.svelte';
  import { bookActions, messageActions, providerActions } from '$lib/stores/actions';
  import { books, activeBookId, loadingState, uiState } from '$lib/stores/index';
  
  let sidebarOpen = true;

  onMount(async () => {
    // 检测移动端
    checkMobile();
    window.addEventListener('resize', checkMobile);
    
    // 加载初始数据
    await Promise.all([
      bookActions.loadBooks(),
      providerActions.loadProviders(),
    ]);
  });

  onDestroy(() => {
    window.removeEventListener('resize', checkMobile);
  });

  function checkMobile() {
    uiState.update(s => ({
      ...s,
      isMobile: window.innerWidth < 768
    }));
  }

  async function handleCreateBook() {
    const title = prompt('请输入书名：');
    if (!title) return;
    
    const book = await bookActions.createBook({ title });
    bookActions.selectBook(book.id);
  }

  function handleSelectBook(bookId: string) {
    bookActions.selectBook(bookId);
    // 移动端自动收起侧边栏
    if (get(uiState).isMobile) {
      sidebarOpen = false;
    }
  }
</script>

<div class="app-layout" class:sidebar-collapsed={!sidebarOpen}>
  <!-- 顶部栏 -->
  <header class="top-bar">
    <div class="left">
      <button 
        class="menu-toggle" 
        on:click={() => sidebarOpen = !sidebarOpen}
        aria-label="切换侧边栏"
      >
        <svg viewBox="0 0 24 24" width="24" height="24" fill="currentColor">
          <path d="M3 18h18v-2H3v2zm0-5h18v-2H3v2zm0-7v2h18V6H3z"/>
        </svg>
      </button>
      <div class="logo">
        <span class="logo-icon">📖</span>
        <span class="logo-text">OpenNovel</span>
      </div>
    </div>
    
    <div class="center">
      <div class="search-box">
        <svg viewBox="0 0 24 24" width="18" height="18" fill="currentColor">
          <path d="M15.5 14h-.79l-.28-.27a6.5 6.5 0 0 0 1.48-5.34c-.47-2.78-2.79-5-5.59-5.34a6.505 6.505 0 0 0-7.27 7.27c.34 2.8 2.56 5.12 5.34 5.59a6.5 6.5 0 0 0 5.34-1.48l.27.28v.79l4.25 4.25c.41.41 1.08.41 1.49 0 .41-.41.41-1.08 0-1.49L15.5 14zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z"/>
        </svg>
        <input type="text" placeholder="搜索书籍或内容..." />
      </div>
    </div>
    
    <div class="right">
      <button class="icon-btn" title="通知">
        <span>🔔</span>
      </button>
      <button class="icon-btn" title="设置">
        <span>⚙️</span>
      </button>
      <div class="user-avatar">
        <span>我</span>
      </div>
    </div>
  </header>

  <!-- 侧边栏 -->
  <aside class="sidebar" class:open={sidebarOpen}>
    <BookList 
      books={$books}
      activeBookId={$activeBookId}
      loading={$loadingState.booksLoading}
      on:select={(e) => handleSelectBook(e.detail.bookId)}
      on:create={handleCreateBook}
    />
  </aside>

  <!-- 主内容区 -->
  <main class="main-content">
    {#if $activeBookId}
      <ChatPage bookId={$activeBookId} />
    {:else}
      <div class="empty-state">
        <div class="empty-icon">📚</div>
        <h2>选择或创建一本书</h2>
        <p>从左侧列表选择一本书开始创作，或点击"新建"创建新书。</p>
        <button class="primary-btn" on:click={handleCreateBook}>
          创建新书
        </button>
      </div>
    {/if}
  </main>
</div>

<style>
  .app-layout {
    display: grid;
    grid-template-rows: 56px 1fr;
    grid-template-columns: 260px 1fr;
    height: 100vh;
    background: var(--color-bg-page);
    transition: grid-template-columns 0.3s ease;
  }
  
  .app-layout.sidebar-collapsed {
    grid-template-columns: 0 1fr;
  }
  
  .top-bar {
    grid-column: 1 / -1;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 20px;
    background: var(--color-white);
    border-bottom: 1px solid var(--color-border);
    box-shadow: var(--shadow-sm);
    z-index: 100;
  }
  
  .left, .right {
    display: flex;
    align-items: center;
    gap: 12px;
  }
  
  .menu-toggle {
    width: 40px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: transparent;
    border-radius: var(--radius-lg);
    cursor: pointer;
    color: var(--color-text-secondary);
    transition: var(--transition-fast);
  }
  
  .menu-toggle:hover {
    background: var(--color-bg-hover);
    color: var(--color-primary);
  }
  
  .logo {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  
  .logo-icon {
    font-size: 24px;
  }
  
  .logo-text {
    font-size: 18px;
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-primary);
  }
  
  .center {
    flex: 1;
    display: flex;
    justify-content: center;
  }
  
  .search-box {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 400px;
    max-width: 100%;
    padding: 8px 16px;
    background: var(--color-bg-page);
    border-radius: var(--radius-full);
    border: 1px solid var(--color-border);
    transition: var(--transition-fast);
  }
  
  .search-box:focus-within {
    border-color: var(--color-primary);
    box-shadow: 0 0 0 3px var(--color-primary-bg);
  }
  
  .search-box input {
    flex: 1;
    border: none;
    background: transparent;
    font-size: var(--font-size-sm);
    color: var(--color-text-primary);
    outline: none;
  }
  
  .search-box svg {
    color: var(--color-text-placeholder);
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
  
  .user-avatar {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--color-primary);
    color: white;
    border-radius: var(--radius-full);
    font-size: var(--font-size-sm);
    cursor: pointer;
  }
  
  .sidebar {
    background: var(--color-white);
    border-right: 1px solid var(--color-border);
    overflow: hidden;
    transition: width 0.3s ease;
  }
  
  .app-layout.sidebar-collapsed .sidebar {
    width: 0;
  }
  
  .main-content {
    overflow: hidden;
  }
  
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    padding: 40px;
    text-align: center;
  }
  
  .empty-icon {
    font-size: 64px;
    margin-bottom: 20px;
  }
  
  .empty-state h2 {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-primary);
    margin-bottom: 8px;
  }
  
  .empty-state p {
    font-size: var(--font-size-base);
    color: var(--color-text-secondary);
    margin-bottom: 24px;
    max-width: 400px;
  }
  
  .primary-btn {
    padding: 12px 32px;
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
  
  /* 移动端适配 */
  @media (max-width: 768px) {
    .app-layout {
      grid-template-columns: 1fr;
    }
    
    .sidebar {
      position: fixed;
      left: 0;
      top: 56px;
      bottom: 0;
      width: 280px;
      z-index: 50;
      transform: translateX(-100%);
      transition: transform 0.3s ease;
    }
    
    .sidebar.open {
      transform: translateX(0);
    }
    
    .app-layout.sidebar-collapsed .sidebar {
      width: 280px;
    }
    
    .search-box {
      width: 200px;
    }
    
    .logo-text {
      display: none;
    }
  }
</style>
```

### 3.2 群聊页面交互逻辑

```svelte
<!-- src/lib/components/ChatPage.svelte -->

<script lang="ts">
  import { onMount, onDestroy, tick } from 'svelte';
  import { get } from 'svelte/store';
  import MessageList from './MessageList.svelte';
  import MessageInput from './MessageInput.svelte';
  import AgentStatusPanel from './AgentStatusPanel.svelte';
  import KnowledgePanel from './KnowledgePanel.svelte';
  import { 
    currentMessages, 
    activeBook, 
    agentStatuses, 
    loadingState, 
    errors,
    uiState 
  } from '$lib/stores/index';
  import { messageActions, agentActions } from '$lib/stores/actions';

  export let bookId: string;

  let messagesContainer: HTMLElement;
  let showRightPanel = false;
  let rightPanelTab: 'agents' | 'knowledge' | 'settings' = 'agents';
  
  // @提及下拉框状态
  let mentionDropdownOpen = false;
  let mentionQuery = '';
  let mentionPosition = { start: 0, end: 0 };

  // 滚动到最新消息
  async function scrollToBottom(smooth = false) {
    await tick();
    if (messagesContainer) {
      messagesContainer.scrollTo({
        top: messagesContainer.scrollHeight,
        behavior: smooth ? 'smooth' : 'auto'
      });
    }
  }

  // 监听消息变化，自动滚动
  $: if ($currentMessages.length > 0) {
    scrollToBottom(true);
  }

  onMount(() => {
    // 根据书籍阶段更新 Agent 锁定状态
    if ($activeBook) {
      agentActions.updateLocksByStage($activeBook.stage);
    }
  });

  // 发送消息
  async function handleSendMessage(event: CustomEvent<string>) {
    const content = event.detail;
    if (!content.trim()) return;
    
    await messageActions.sendMessage(bookId, content);
    scrollToBottom();
  }

  // 处理 @提及
  function handleMention(event: CustomEvent<{ query: string; position: { start: number; end: number } }>) {
    mentionQuery = event.detail.query;
    mentionPosition = event.detail.position;
    mentionDropdownOpen = true;
  }

  // 选择提及的 Agent
  function selectMentionAgent(agentName: string) {
    mentionDropdownOpen = false;
    // 通知 MessageInput 组件插入提及
    // 通过事件或 store 传递
  }

  // 切换右侧面板
  function toggleRightPanel(tab?: typeof rightPanelTab) {
    if (tab && rightPanelTab === tab && showRightPanel) {
      showRightPanel = false;
    } else {
      rightPanelTab = tab || rightPanelTab;
      showRightPanel = true;
    }
  }

  // 加载更多历史消息
  async function handleLoadMore() {
    await messageActions.loadMoreMessages(bookId);
  }

  // 重试发送
  async function handleRetry() {
    errors.update(e => ({ ...e, chat: undefined }));
  }
</script>

<div class="chat-page">
  <!-- 右侧面板 -->
  {#if showRightPanel}
    <aside class="right-panel">
      <header class="panel-header">
        <div class="tabs">
          <button 
            class:active={rightPanelTab === 'agents'}
            on:click={() => rightPanelTab = 'agents'}
          >
            Agent 状态
          </button>
          <button 
            class:active={rightPanelTab === 'knowledge'}
            on:click={() => rightPanelTab = 'knowledge'}
          >
            知识库
          </button>
        </div>
        <button class="close-btn" on:click={() => showRightPanel = false}>✕</button>
      </header>
      
      <div class="panel-content">
        {#if rightPanelTab === 'agents'}
          <AgentStatusPanel agents={$agentStatuses} />
        {:else if rightPanelTab === 'knowledge'}
          <KnowledgePanel bookId={bookId} />
        {/if}
      </div>
    </aside>
  {/if}

  <!-- 主聊天区 -->
  <main class="chat-main">
    <!-- 头部 -->
    <header class="chat-header">
      <div class="book-info">
        {#if $activeBook}
          <h1>{$activeBook.title}</h1>
          <div class="meta">
            <span class="stage-badge stage-{$activeBook.stage}">
              {#if $activeBook.stage === 'conception'}构思中
              {:else if $activeBook.stage === 'knowledge'}知识库建立
              {:else}撰写中{/if}
            </span>
            <span class="word-count">{$activeBook.wordCount.toLocaleString()} 字</span>
            <span class="chapter-count">{$activeBook.chapterCount} 章</span>
          </div>
        {/if}
      </div>
      
      <div class="header-actions">
        <button class="action-btn" on:click={() => toggleRightPanel('agents')} class:active={showRightPanel && rightPanelTab === 'agents'}>
          <span>🤖</span>
          <span class="label">Agent</span>
        </button>
        <button class="action-btn" on:click={() => toggleRightPanel('knowledge')} class:active={showRightPanel && rightPanelTab === 'knowledge'}>
          <span>📚</span>
          <span class="label">知识库</span>
        </button>
        <button class="action-btn" title="导出">
          <span>📤</span>
        </button>
        <button class="action-btn" title="设置">
          <span>⚙️</span>
        </button>
      </div>
    </header>

    <!-- 消息列表 -->
    <div class="messages-container" bind:this={messagesContainer}>
      {#if $loadingState.messagesLoading}
        <div class="loading-indicator">
          <div class="spinner"></div>
          <span>加载中...</span>
        </div>
      {:else}
        <button class="load-more-btn" on:click={handleLoadMore}>
          加载更多历史消息
        </button>
        
        <MessageList 
          messages={$currentMessages}
          on:retry={handleRetry}
        />
        
        {#if $errors.chat}
          <div class="error-banner">
            <span class="error-text">{$errors.chat}</span>
            <button class="retry-btn" on:click={handleRetry}>重试</button>
          </div>
        {/if}
      {/if}
    </div>

    <!-- 输入区域 -->
    <footer class="input-area">
      <MessageInput 
        disabled={$loadingState.sendingMessage}
        on:send={handleSendMessage}
        on:mention={handleMention}
      />
      
      <!-- @提及下拉框 -->
      {#if mentionDropdownOpen}
        <div class="mention-dropdown">
          <div class="mention-header">选择 Agent</div>
          {#each $agentStatuses.filter(a => !a.isLocked && a.name.includes(mentionQuery)) as agent}
            <button 
              class="mention-option" 
              on:click={() => selectMentionAgent(agent.name)}
            >
              <span class="agent-color" style="background: var(--agent-{agent.name.toLowerCase()})"></span>
              <span class="agent-name">{agent.name}</span>
              <span class="agent-role">{agent.role}</span>
            </button>
          {/each}
        </div>
      {/if}
    </footer>
  </main>
</div>

<style>
  .chat-page {
    display: flex;
    height: 100%;
    background: var(--color-bg-page);
  }
  
  .right-panel {
    width: 320px;
    background: var(--color-white);
    border-left: 1px solid var(--color-border);
    display: flex;
    flex-direction: column;
  }
  
  .panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
    border-bottom: 1px solid var(--color-border-light);
  }
  
  .tabs {
    display: flex;
    gap: 4px;
  }
  
  .tabs button {
    padding: 6px 12px;
    border: none;
    background: transparent;
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
    border-radius: var(--radius-base);
    cursor: pointer;
    transition: var(--transition-fast);
  }
  
  .tabs button:hover {
    background: var(--color-bg-hover);
  }
  
  .tabs button.active {
    background: var(--color-primary-bg);
    color: var(--color-primary);
    font-weight: var(--font-weight-medium);
  }
  
  .close-btn {
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: transparent;
    font-size: 16px;
    color: var(--color-text-secondary);
    border-radius: var(--radius-base);
    cursor: pointer;
  }
  
  .close-btn:hover {
    background: var(--color-bg-hover);
  }
  
  .panel-content {
    flex: 1;
    overflow-y: auto;
    padding: 16px;
  }
  
  .chat-main {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-width: 0;
  }
  
  .chat-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 24px;
    background: var(--color-white);
    border-bottom: 1px solid var(--color-border-light);
  }
  
  .book-info h1 {
    font-size: var(--font-size-xl);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-primary);
    margin-bottom: 4px;
  }
  
  .meta {
    display: flex;
    align-items: center;
    gap: 12px;
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
  }
  
  .stage-badge {
    padding: 2px 8px;
    border-radius: var(--radius-sm);
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-medium);
  }
  
  .stage-badge.stage-conception {
    background: #fff7e6;
    color: #fa8c16;
  }
  
  .stage-badge.stage-knowledge {
    background: #e6f7ff;
    color: #1890ff;
  }
  
  .stage-badge.stage-writing {
    background: #f6ffed;
    color: #52c41a;
  }
  
  .header-actions {
    display: flex;
    gap: 8px;
  }
  
  .action-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 8px 12px;
    border: 1px solid var(--color-border);
    background: var(--color-white);
    border-radius: var(--radius-lg);
    font-size: var(--font-size-sm);
    color: var(--color-text-primary);
    cursor: pointer;
    transition: var(--transition-fast);
  }
  
  .action-btn:hover {
    background: var(--color-bg-hover);
    border-color: var(--color-primary);
  }
  
  .action-btn.active {
    background: var(--color-primary-bg);
    border-color: var(--color-primary);
    color: var(--color-primary);
  }
  
  .action-btn .label {
    display: none;
  }
  
  @media (min-width: 1024px) {
    .action-btn .label {
      display: inline;
    }
  }
  
  .messages-container {
    flex: 1;
    overflow-y: auto;
    padding: 20px 24px;
    display: flex;
    flex-direction: column;
  }
  
  .load-more-btn {
    align-self: center;
    padding: 8px 24px;
    margin-bottom: 16px;
    border: 1px solid var(--color-border);
    background: var(--color-white);
    border-radius: var(--radius-full);
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
    cursor: pointer;
    transition: var(--transition-fast);
  }
  
  .load-more-btn:hover {
    background: var(--color-bg-hover);
    color: var(--color-primary);
    border-color: var(--color-primary);
  }
  
  .loading-indicator {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 200px;
    gap: 12px;
    color: var(--color-text-secondary);
  }
  
  .spinner {
    width: 32px;
    height: 32px;
    border: 3px solid var(--color-border);
    border-top-color: var(--color-primary);
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }
  
  @keyframes spin {
    to { transform: rotate(360deg); }
  }
  
  .error-banner {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 12px;
    padding: 12px;
    margin-top: 16px;
    background: #fff2f0;
    border: 1px solid #ffccc7;
    border-radius: var(--radius-lg);
  }
  
  .error-text {
    color: var(--color-error);
    font-size: var(--font-size-sm);
  }
  
  .retry-btn {
    padding: 4px 12px;
    border: 1px solid var(--color-error);
    background: var(--color-white);
    color: var(--color-error);
    font-size: var(--font-size-sm);
    border-radius: var(--radius-base);
    cursor: pointer;
    transition: var(--transition-fast);
  }
  
  .retry-btn:hover {
    background: var(--color-error);
    color: white;
  }
  
  .input-area {
    position: relative;
    padding: 16px 24px;
    background: var(--color-white);
    border-top: 1px solid var(--color-border-light);
  }
  
  .mention-dropdown {
    position: absolute;
    bottom: 100%;
    left: 24px;
    width: 240px;
    max-height: 300px;
    overflow-y: auto;
    background: var(--color-white);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-lg);
    z-index: 100;
  }
  
  .mention-header {
    padding: 8px 12px;
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-medium);
    color: var(--color-text-secondary);
    border-bottom: 1px solid var(--color-border-light);
  }
  
  .mention-option {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 8px 12px;
    border: none;
    background: transparent;
    text-align: left;
    cursor: pointer;
    transition: var(--transition-fast);
  }
  
  .mention-option:hover {
    background: var(--color-bg-hover);
  }
  
  .agent-color {
    width: 8px;
    height: 8px;
    border-radius: 50%;
  }
  
  .agent-name {
    flex: 1;
    font-size: var(--font-size-sm);
    color: var(--color-text-primary);
  }
  
  .agent-role {
    font-size: var(--font-size-xs);
    color: var(--color-text-secondary);
  }
</style>
```

### 3.3 消息输入组件交互逻辑

```svelte
<!-- src/lib/components/MessageInput.svelte -->

<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { get } from 'svelte/store';
  import { agentStatuses } from '$lib/stores/index';

  export let disabled = false;
  export let placeholder = '输入消息，@提及 Agent...';

  const dispatch = createEventDispatcher<{
    send: string;
    mention: { query: string; position: { start: number; end: number } };
  }>();

  let inputElement: HTMLTextAreaElement;
  let value = '';
  let isFocused = false;

  // 检测 @提及
  function checkMention() {
    const cursorPos = inputElement.selectionStart;
    const textBeforeCursor = value.substring(0, cursorPos);
    
    // 查找最后一个 @ 符号
    const lastAtIndex = textBeforeCursor.lastIndexOf('@');
    if (lastAtIndex === -1) return;
    
    // 检查 @ 后面是否有空格（如果有，则不是有效的提及）
    const textAfterAt = textBeforeCursor.substring(lastAtIndex + 1);
    if (textAfterAt.includes(' ')) return;
    
    // 触发提及事件
    dispatch('mention', {
      query: textAfterAt,
      position: { start: lastAtIndex, end: cursorPos }
    });
  }

  // 插入 @提及
  export function insertMention(agentName: string, position: { start: number; end: number }) {
    const before = value.substring(0, position.start);
    const after = value.substring(position.end);
    value = `${before}@${agentName} ${after}`;
    
    // 将光标移动到提及后面
    setTimeout(() => {
      const newPos = position.start + agentName.length + 2;
      inputElement.setSelectionRange(newPos, newPos);
      inputElement.focus();
    }, 0);
  }

  // 发送消息
  function send() {
    const content = value.trim();
    if (!content || disabled) return;
    
    dispatch('send', content);
    value = '';
    
    // 重置输入框高度
    inputElement.style.height = 'auto';
  }

  // 键盘事件处理
  function handleKeydown(event: KeyboardEvent) {
    // Enter 发送（不带 Shift）
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault();
      send();
    }
    
    // Tab 自动补全
    if (event.key === 'Tab') {
      event.preventDefault();
      // TODO: 实现自动补全
    }
  }

  // 输入事件处理
  function handleInput() {
    // 自动调整高度
    inputElement.style.height = 'auto';
    inputElement.style.height = Math.min(inputElement.scrollHeight, 200) + 'px';
    
    // 检测 @提及
    checkMention();
  }

  // 粘贴处理
  async function handlePaste(event: ClipboardEvent) {
    const items = event.clipboardData?.items;
    if (!items) return;
    
    // 检查是否有图片
    for (const item of items) {
      if (item.type.startsWith('image/')) {
        event.preventDefault();
        // TODO: 处理图片粘贴
        console.log('Image pasted:', item.getAsFile());
        return;
      }
    }
    
    // 检查是否有文本
    const text = event.clipboardData?.getData('text');
    if (text && text.length > 1000) {
      // 长文本警告
      event.preventDefault();
      const confirmed = confirm('粘贴的内容较长，是否继续？');
      if (confirmed) {
        document.execCommand('insertText', false, text);
      }
    }
  }

  // 聚焦输入框
  export function focus() {
    inputElement?.focus();
  }
</script>

<div class="message-input" class:disabled class:focused={isFocused}>
  <div class="input-wrapper">
    <!-- 工具栏 -->
    <div class="toolbar">
      <button class="tool-btn" title="上传文件" disabled>
        <svg viewBox="0 0 24 24" width="18" height="18" fill="currentColor">
          <path d="M16.5 6v11.5c0 2.21-1.79 4-4 4s-4-1.79-4-4V5a2.5 2.5 0 0 1 5 0v10.5c0 .55-.45 1-1 1s-1-.45-1-1V6H10v9.5a2.5 2.5 0 0 0 5 0V5c0-2.21-1.79-4-4-4S7 2.79 7 5v12.5c0 3.04 2.46 5.5 5.5 5.5s5.5-2.46 5.5-5.5V6h-1.5z"/>
        </svg>
      </button>
    </div>
    
    <!-- 输入框 -->
    <textarea
      bind:this={inputElement}
      bind:value
      {placeholder}
      {disabled}
      rows="1"
      on:keydown={handleKeydown}
      on:input={handleInput}
      on:paste={handlePaste}
      on:focus={() => isFocused = true}
      on:blur={() => isFocused = false}
    />
    
    <!-- 发送按钮 -->
    <button 
      class="send-btn" 
      class:active={value.trim().length > 0 && !disabled}
      on:click={send}
      disabled={disabled || !value.trim()}
    >
      <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor">
        <path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"/>
      </svg>
    </button>
  </div>
  
  <!-- 提示信息 -->
  <div class="hints">
    <span class="hint">Shift + Enter 换行</span>
    <span class="hint">@ 提及 Agent</span>
    <span class="char-count">{value.length} / 10000</span>
  </div>
</div>

<style>
  .message-input {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  
  .input-wrapper {
    display: flex;
    align-items: flex-end;
    gap: 8px;
    padding: 12px 16px;
    background: var(--color-bg-page);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-xl);
    transition: var(--transition-fast);
  }
  
  .message-input.focused .input-wrapper {
    border-color: var(--color-primary);
    box-shadow: 0 0 0 3px var(--color-primary-bg);
  }
  
  .message-input.disabled .input-wrapper {
    opacity: 0.6;
    cursor: not-allowed;
  }
  
  .toolbar {
    display: flex;
    gap: 4px;
    padding-bottom: 4px;
  }
  
  .tool-btn {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: transparent;
    color: var(--color-text-secondary);
    border-radius: var(--radius-base);
    cursor: pointer;
    transition: var(--transition-fast);
  }
  
  .tool-btn:not(:disabled):hover {
    background: var(--color-bg-hover);
    color: var(--color-primary);
  }
  
  .tool-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }
  
  textarea {
    flex: 1;
    min-height: 24px;
    max-height: 200px;
    padding: 0;
    border: none;
    background: transparent;
    font-size: var(--font-size-base);
    line-height: 1.5;
    color: var(--color-text-primary);
    resize: none;
    outline: none;
    font-family: inherit;
  }
  
  textarea::placeholder {
    color: var(--color-text-placeholder);
  }
  
  textarea:disabled {
    cursor: not-allowed;
  }
  
  .send-btn {
    width: 36px;
    height: 36px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: var(--color-text-disabled);
    color: white;
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
  
  .hints {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 0 16px;
    font-size: var(--font-size-xs);
    color: var(--color-text-placeholder);
  }
  
  .hint::before {
    content: '⌘';
    margin-right: 4px;
    opacity: 0.6;
  }
  
  .char-count {
    margin-left: auto;
  }
</style>
```

---

## 四、关键交互流程

### 4.1 发送消息完整流程

```
用户输入消息
    │
    ├─ 输入验证
    │   ├─ 非空检查
    │   ├─ 长度限制（10000 字符）
    │   └─ 特殊字符处理
    │
    ├─ 乐观更新 UI
    │   └─ 立即显示用户消息（临时 ID）
    │
    ├─ 发送到服务器
    │   │
    │   ├─ 成功
    │   │   ├─ 替换临时消息为真实消息
    │   │   ├─ SSE 接收 Agent 响应
    │   │   │   ├─ stream_start: 创建流式消息占位
    │   │   │   ├─ stream_chunk: 更新内容
    │   │   │   └─ stream_complete: 完成消息
    │   │   └─ 滚动到最新消息
    │   │
    │   └─ 失败
    │       ├─ 移除乐观更新的消息
    │       ├─ 显示错误提示
    │       └─ 提供重试按钮
    │
    └─ 清空输入框
```

### 4.2 @提及交互流程

```
用户输入 @
    │
    ├─ 检测 @ 符号位置
    │
    ├─ 弹出 Agent 选择下拉框
    │   ├─ 过滤已锁定的 Agent
    │   ├─ 根据输入内容过滤
    │   └─ 显示 Agent 状态
    │
    ├─ 用户选择 Agent
    │   └─ 插入 @{AgentName} 到输入框
    │
    └─ 发送时解析提及
        └─ 将 @{AgentName} 转换为通知对象
```

### 4.3 SSE 流式输出流程

```
SSE 连接建立
    │
    ├─ 连接成功
    │   ├─ 重置重连计数
    │   └─ 开始监听事件
    │
    ├─ 接收事件
    │   ├─ message: 普通消息
    │   ├─ stream_start: 开始流式输出
    │   │   └─ 创建占位消息，显示加载动画
    │   ├─ stream_chunk: 内容块
    │   │   └─ 追加内容，实时渲染
    │   ├─ stream_complete: 完成
    │   │   └─ 替换为最终消息
    │   └─ agent_status: Agent 状态变更
    │       └─ 更新 Agent 状态面板
    │
    ├─ 连接错误
    │   ├─ 检查重连次数
    │   ├─ 指数退避重连
    │   └─ 超过最大次数则断开
    │
    └─ 主动断开
        └─ 切换书籍或关闭页面时
```

### 4.4 Provider 配置流程

```
用户点击"添加 Provider"
    │
    ├─ 打开配置表单
    │   ├─ 名称（必填）
    │   ├─ 类型（下拉选择）
    │   ├─ Base URL（必填）
    │   ├─ API Key（必填）
    │   └─ 模型列表
    │
    ├─ 表单验证
    │   ├─ URL 格式验证
    │   ├─ API Key 非空
    │   └─ 至少一个模型
    │
    ├─ 保存配置
    │   └─ POST /api/providers
    │
    ├─ 测试连接
    │   ├─ POST /api/providers/{id}/test
    │   ├─ 显示测试中状态
    │   ├─ 成功：显示可用模型列表
    │   └─ 失败：显示错误信息
    │
    └─ 启用/禁用
        └─ PATCH /api/providers/{id}
```

---

## 五、错误处理策略

### 5.1 网络错误处理

| 场景 | 处理方式 |
|------|---------|
| API 请求超时 | 显示超时提示，提供重试按钮 |
| 服务器 500 错误 | 显示服务器错误提示，建议稍后重试 |
| 认证失效 (401) | 跳转到登录页面 |
| 权限不足 (403) | 显示权限错误提示 |
| 资源不存在 (404) | 显示资源不存在提示，返回列表页 |

### 5.2 业务错误处理

| 场景 | 处理方式 |
|------|---------|
| 发送消息失败 | 保留输入内容，显示错误，提供重试 |
| Agent 响应超时 | 显示超时提示，允许继续等待或取消 |
| 流式输出中断 | 自动重连，从中断点继续 |
| 阶段切换冲突 | 显示确认对话框，说明影响 |

### 5.3 用户输入错误

| 场景 | 处理方式 |
|------|---------|
| 空消息 | 禁用发送按钮 |
| 超长消息 | 显示字符计数，超过限制时警告 |
| 无效 @提及 | 忽略无效的提及标记 |
| 敏感词检测 | 替换为 *** 或拦截提示 |

---

## 六、性能优化策略

### 6.1 消息列表虚拟化

```typescript
// 当消息超过 100 条时，启用虚拟滚动
const VIRTUAL_SCROLL_THRESHOLD = 100;

// 只渲染可见区域的消息
function getVisibleMessages(messages: Message[], viewport: DOMRect) {
  // 计算可见区域
  // 返回需要渲染的消息范围
}
```

### 6.2 输入防抖

```typescript
// @提及搜索防抖
const debouncedMentionSearch = debounce((query: string) => {
  // 搜索 Agent
}, 200);
```

### 6.3 消息分页加载

```typescript
// 初始加载最近 50 条
// 滚动到顶部时加载更多
const PAGE_SIZE = 50;
```

---

## 七、无障碍支持

### 7.1 键盘导航

| 快捷键 | 功能 |
|--------|------|
| `Tab` | 切换焦点元素 |
| `Enter` | 发送消息 |
| `Shift + Enter` | 换行 |
| `Escape` | 关闭弹窗/下拉框 |
| `↑` `↓` | 在 Agent 列表中导航 |

### 7.2 屏幕阅读器支持

```html
<!-- 消息区域 -->
<div role="log" aria-label="聊天消息">
  <div role="article" aria-label="用户消息">
    ...
  </div>
  <div role="article" aria-label="天道回复">
    ...
  </div>
</div>

<!-- 输入区域 -->
<label for="message-input">输入消息</label>
<textarea id="message-input" aria-describedby="input-hint" />
<span id="input-hint">按 Enter 发送，Shift+Enter 换行</span>
```

### 7.3 焦点管理

```typescript
// 发送消息后，保持焦点在输入框
function sendMessage() {
  // ...发送逻辑
  inputElement.focus();
}

// 打开弹窗时，焦点移动到弹窗
// 关闭弹窗时，焦点返回触发元素
```

---

## 八、测试检查清单

### 8.1 功能测试

- [ ] 发送文本消息成功
- [ ] 发送空消息被阻止
- [ ] 发送超长消息有警告
- [ ] @提及 Agent 功能正常
- [ ] 流式输出正常显示
- [ ] 消息滚动自动到底部
- [ ] 加载历史消息正常
- [ ] Provider 配置保存成功
- [ ] Provider 测试连接正常
- [ ] 切换书籍消息正确加载

### 8.2 边界测试

- [ ] 网络断开时正确提示
- [ ] 服务器错误时正确处理
- [ ] 并发消息发送处理正确
- [ ] 长时间无活动 SSE 保持连接
- [ ] 大量消息渲染性能正常

### 8.3 交互测试

- [ ] 鼠标悬停状态正确
- [ ] 聚焦状态视觉反馈清晰
- [ ] 禁用状态不可操作
- [ ] 加载状态显示正确
- [ ] 错误状态可恢复