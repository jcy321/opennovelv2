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

    const eventSource = new EventSource(`/api/books/${bookId}/stream`, {
      withCredentials: true,
    });

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
          messageActions.addStreamingMessage(bookId, agentName, messageId);
          agentActions.setThinking(agentName, '正在生成内容');
          break;
        }

        // 思考过程（AI 推理过程）
        case 'thinking': {
          const { messageId, content } = data;
          messageActions.updateThinkingContent(bookId, messageId, content);
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

      console.log(
        `[SSE] Reconnecting in ${delay}ms (attempt ${connection.reconnectAttempts})`
      );

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
