// src/lib/stores/actions.ts

import { get } from 'svelte/store';
import {
  books,
  activeBookId,
  messagesByBook,
  agentStatuses,
  providers,
  selectedModel,
  loadingState,
  errors,
} from './index';
import { apiClient } from '$lib/api/client';

// ─────────────────────────────────────────────────────
// 书籍操作
// ─────────────────────────────────────────────────────

export const bookActions = {
  /** 加载书籍列表 */
  async loadBooks() {
    loadingState.update((s) => ({ ...s, booksLoading: true }));
    try {
      const response = await apiClient.get<{ books: Array<{
        id: string;
        title: string;
        author: string;
        stage: string;
        status: string;
        chapter_count: number;
        word_count: number;
        last_active_at: string;
        created_at: string;
      }> }>('/api/books');
      
      // 转换字段名
      const mappedBooks: Book[] = (response.books || []).map(b => ({
        id: b.id,
        title: b.title,
        author: b.author,
        stage: b.stage as Book['stage'],
        status: b.status as Book['status'],
        chapterCount: b.chapter_count,
        wordCount: b.word_count,
        lastActive: new Date(b.last_active_at),
        createdAt: new Date(b.created_at),
      }));
      
      books.set(mappedBooks);
    } catch (error) {
      errors.update((e) => ({ ...e, global: '加载书籍列表失败' }));
      throw error;
    } finally {
      loadingState.update((s) => ({ ...s, booksLoading: false }));
    }
  },

  /** 创建新书 */
  async createBook(data: { title: string; description?: string }) {
    try {
      const book = await apiClient.post<Book>('/api/books', data);
      books.update((list) => [...list, book]);
      return book;
    } catch (error) {
      errors.update((e) => ({ ...e, global: '创建书籍失败' }));
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
      books.update((list) => list.filter((b) => b.id !== bookId));
      if (get(activeBookId) === bookId) {
        activeBookId.set(null);
      }
    } catch (error) {
      errors.update((e) => ({ ...e, global: '删除书籍失败' }));
      throw error;
    }
  },

  /** 更新书籍阶段 */
  async updateStage(bookId: string, stage: Book['stage']) {
    try {
      await apiClient.patch(`/api/books/${bookId}`, { stage });
      books.update((list) =>
        list.map((b) => (b.id === bookId ? { ...b, stage } : b))
      );
    } catch (error) {
      throw error;
    }
  },
};

function mapMessageFromApi(m: {
  id: string;
  book_id: string;
  role: string;
  agent_name?: string | null;
  content: string;
  annotations: any[];
  created_at: string;
  is_streaming?: boolean;
  streaming_content?: string | null;
}): Message {
  return {
    id: m.id,
    bookId: m.book_id,
    role: m.role as Message['role'],
    agentName: m.agent_name || undefined,
    content: m.content,
    annotations: m.annotations || [],
    timestamp: new Date(m.created_at),
    isStreaming: m.is_streaming,
    streamingContent: m.streaming_content || undefined,
  };
}

// ─────────────────────────────────────────────────────
// 消息操作
// ─────────────────────────────────────────────────────

export const messageActions = {
  /** 加载消息列表 */
  async loadMessages(
    bookId: string,
    options?: { limit?: number; before?: string }
  ) {
    loadingState.update((s) => ({ ...s, messagesLoading: true }));
    try {
      const params = new URLSearchParams();
      if (options?.limit) params.set('limit', String(options.limit));
      if (options?.before) params.set('before', options.before);

      const response = await apiClient.get<{ messages: any[] }>(`/api/books/${bookId}/messages?${params}`);
      const mappedMessages = (response.messages || []).map(mapMessageFromApi);

      messagesByBook.update((map) => ({
        ...map,
        [bookId]: mappedMessages,
      }));
    } catch (error) {
      errors.update((e) => ({ ...e, chat: '加载消息失败' }));
      throw error;
    } finally {
      loadingState.update((s) => ({ ...s, messagesLoading: false }));
    }
  },

  /** 发送消息 */
  async sendMessage(bookId: string, content: string) {
    if (!content.trim()) return;

    loadingState.update((s) => ({ ...s, sendingMessage: true }));
    errors.update((e) => ({ ...e, chat: undefined }));

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

    messagesByBook.update((map) => ({
      ...map,
      [bookId]: [...(map[bookId] || []), userMessage],
    }));

    try {
      // 发送到服务器
      const response = await apiClient.post<any>(`/api/books/${bookId}/chat`, {
        content: content.trim(),
      });
      const mappedMessage = mapMessageFromApi(response);

      // 替换临时消息为真实消息
      messagesByBook.update((map) => ({
        ...map,
        [bookId]: (map[bookId] || []).map((m) =>
          m.id === tempId ? mappedMessage : m
        ),
      }));

      return mappedMessage;
    } catch (error) {
      // 移除乐观更新的消息
      messagesByBook.update((map) => ({
        ...map,
        [bookId]: (map[bookId] || []).filter((m) => m.id !== tempId),
      }));
      errors.update((e) => ({ ...e, chat: '发送消息失败' }));
      throw error;
    } finally {
      loadingState.update((s) => ({ ...s, sendingMessage: false }));
    }
  },

  /** 添加流式消息 */
  addStreamingMessage(bookId: string, agentName: string, messageId?: string) {
    const tempId = messageId || `stream-${Date.now()}`;
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
      thinkingContent: '',
    };

    messagesByBook.update((map) => ({
      ...map,
      [bookId]: [...(map[bookId] || []), streamingMessage],
    }));

    return tempId;
  },

  /** 更新思考内容 */
  updateThinkingContent(bookId: string, messageId: string, chunk: string) {
    messagesByBook.update((map) => ({
      ...map,
      [bookId]: (map[bookId] || []).map((m) =>
        m.id === messageId
          ? { ...m, thinkingContent: (m.thinkingContent || '') + chunk }
          : m
      ),
    }));
  },

  /** 更新流式内容 */
  updateStreamingContent(bookId: string, messageId: string, chunk: string) {
    messagesByBook.update((map) => ({
      ...map,
      [bookId]: (map[bookId] || []).map((m) =>
        m.id === messageId
          ? { ...m, streamingContent: (m.streamingContent || '') + chunk }
          : m
      ),
    }));
  },

  /** 完成流式消息 */
  completeStreamingMessage(bookId: string, tempId: string, finalMessage: Message) {
    messagesByBook.update((map) => ({
      ...map,
      [bookId]: (map[bookId] || []).map((m) =>
        m.id === tempId ? { ...finalMessage, isStreaming: false } : m
      ),
    }));
  },

  /** 加载更多历史消息 */
  async loadMoreMessages(bookId: string) {
    const currentMsgs = get(messagesByBook)[bookId] || [];
    if (currentMsgs.length === 0) return;

    const oldestMsg = currentMsgs[0];
    await this.loadMessages(bookId, {
      limit: 50,
      before: oldestMsg.id,
    });
  },
};

// ─────────────────────────────────────────────────────
// Agent 状态操作
// ─────────────────────────────────────────────────────

export const agentActions = {
  /** 更新单个 Agent 状态 */
  updateStatus(agentName: string, status: Partial<AgentStatus>) {
    agentStatuses.update((list) =>
      list.map((a) => (a.name === agentName ? { ...a, ...status } : a))
    );
  },

  /** 设置 Agent 为思考中 */
  setThinking(agentName: string, task?: string) {
    this.updateStatus(agentName, {
      status: 'thinking',
      currentTask: task,
    });
  },

  /** 设置 Agent 为撰写中 */
  setWriting(agentName: string) {
    this.updateStatus(agentName, {
      status: 'writing',
    });
  },

  /** 重置 Agent 为空闲 */
  setIdle(agentName: string) {
    this.updateStatus(agentName, {
      status: 'idle',
      currentTask: undefined,
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
    agentStatuses.update((list) =>
      list.map((a) => ({ ...a, isLocked: stageLocks[a.name] ?? false }))
    );
  },
};

// ─────────────────────────────────────────────────────
// Provider 操作
// ─────────────────────────────────────────────────────

export const providerActions = {
  /** 加载 Provider 列表 */
  async loadProviders() {
    loadingState.update((s) => ({ ...s, providersLoading: true }));
    try {
      const response = await apiClient.get<{ providers: Provider[] }>(
        '/api/providers'
      );
      providers.set(response.providers);
    } catch (error) {
      errors.update((e) => ({ ...e, provider: '加载 Provider 失败' }));
      throw error;
    } finally {
      loadingState.update((s) => ({ ...s, providersLoading: false }));
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
      providers.update((list) => [...list, provider]);
      return provider;
    } catch (error) {
      errors.update((e) => ({ ...e, provider: '创建 Provider 失败' }));
      throw error;
    }
  },

  /** 测试 Provider 连接 */
  async testProvider(providerId: string) {
    // 更新状态为测试中
    providers.update((list) =>
      list.map((p) =>
        p.id === providerId ? { ...p, status: ('testing' as const) } : p
      )
    );

    try {
      const response = await apiClient.post<{
        success: boolean;
        availableModels?: string[];
        error?: string;
      }>(`/api/providers/${providerId}/test`);

      providers.update((list) =>
        list.map((p) =>
          p.id === providerId
            ? {
                ...p,
                status: response.success ? ('connected' as const) : ('error' as const),
                lastTestAt: new Date(),
                errorMessage: response.error,
              }
            : p
        )
      );

      return response;
    } catch (error) {
      providers.update((list) =>
        list.map((p) =>
          p.id === providerId
            ? { ...p, status: ('error' as const), errorMessage: '测试失败' }
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
      providers.update((list) =>
        list.map((p) => (p.id === providerId ? { ...p, ...data } : p))
      );
    } catch (error) {
      throw error;
    }
  },

  /** 删除 Provider */
  async deleteProvider(providerId: string) {
    try {
      await apiClient.delete(`/api/providers/${providerId}`);
      providers.update((list) => list.filter((p) => p.id !== providerId));
    } catch (error) {
      throw error;
    }
  },

  /** 切换 Provider 启用状态 */
  async toggleProvider(providerId: string) {
    const current = get(providers).find((p) => p.id === providerId);
    if (!current) return;

    await this.updateProvider(providerId, { enabled: !current.enabled });
  },
};

// ─────────────────────────────────────────────────────
// 统一导出
// ─────────────────────────────────────────────────────

export * from '$lib/api/stream';
