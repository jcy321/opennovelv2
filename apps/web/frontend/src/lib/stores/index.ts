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
  thinkingContent?: string;
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
    return $books.find((b) => b.id === $activeBookId) || null;
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
