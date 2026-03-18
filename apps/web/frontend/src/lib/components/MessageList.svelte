<script lang="ts">
  import { onMount } from 'svelte';
  
  export let messages: Array<{
    id: string;
    role: 'user' | 'assistant' | 'system';
    agentName?: string;
    content: string;
    timestamp: Date;
    isStreaming?: boolean;
    streamingContent?: string;
    thinkingContent?: string;
  }>;
  
  let mounted = false;
  
  onMount(() => {
    mounted = true;
  });
  
  function formatTime(timestamp: Date): string {
    return new Date(timestamp).toLocaleTimeString('zh-CN', {
      hour: '2-digit',
      minute: '2-digit',
    });
  }
</script>

{#if mounted}
  <div class="message-list">
    {#each messages as message (message.id)}
      <div class="message-wrapper" class:user={message.role === 'user'} class:system={message.role === 'system'}>
        {#if message.role !== 'user'}
          <div class="agent-avatar" style="background: var(--agent-tiandao)">
            <span class="avatar-initial">
              {message.agentName ? message.agentName.charAt(0) : '系'}
            </span>
          </div>
        {/if}
        
        <div class="message-content">
          {#if message.role !== 'user' && message.agentName}
            <div class="agent-name">{message.agentName}</div>
          {/if}
          
          {#if message.thinkingContent && (message.isStreaming || message.thinkingContent.length > 0)}
            <div class="thinking-bubble">
              <div class="thinking-header">
                <span class="thinking-icon">💭</span>
                <span class="thinking-label">思考过程</span>
              </div>
              <div class="thinking-text">{message.thinkingContent}</div>
              {#if message.isStreaming && !message.streamingContent}
                <span class="cursor">▊</span>
              {/if}
            </div>
          {/if}
          
          <div class="bubble" class:streaming={message.isStreaming}>
            <div class="text">{message.isStreaming ? message.streamingContent : message.content}</div>
            {#if message.isStreaming && message.streamingContent}
              <span class="cursor">▊</span>
            {/if}
          </div>
          
          <div class="timestamp">
            {formatTime(message.timestamp)}
          </div>
        </div>
        
        {#if message.role === 'user'}
          <div class="user-avatar">
            <span>我</span>
          </div>
        {/if}
      </div>
    {/each}
  </div>
{/if}

<style>
  .message-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }
  
  .message-wrapper {
    display: flex;
    gap: var(--space-3);
    margin-bottom: var(--space-1);
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
  
  .thinking-bubble {
    padding: var(--space-3) var(--space-4);
    border-radius: var(--radius-lg);
    background: linear-gradient(135deg, #f0f7ff 0%, #e8f4f8 100%);
    border: 1px solid #d0e8f5;
    margin-bottom: var(--space-2);
    border-top-left-radius: var(--radius-sm);
  }
  
  .thinking-header {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    margin-bottom: var(--space-2);
    font-size: var(--font-size-xs);
    color: #6b9cc4;
    font-weight: var(--font-weight-medium);
  }
  
  .thinking-icon {
    font-size: 14px;
  }
  
  .thinking-text {
    font-size: var(--font-size-sm);
    color: #5a7a99;
    line-height: var(--line-height-relaxed);
    white-space: pre-wrap;
    word-break: break-word;
    font-style: italic;
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
    color: var(--color-white);
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
    color: var(--color-white);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: var(--font-size-sm);
    flex-shrink: 0;
  }
  
  .agent-avatar {
    width: 36px;
    height: 36px;
    border-radius: var(--radius-full);
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--color-white);
    font-weight: var(--font-weight-semibold);
    font-size: var(--font-size-sm);
    flex-shrink: 0;
  }
  
  .avatar-initial {
    font-size: 14px;
  }
  
  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(8px); }
    to { opacity: 1; transform: translateY(0); }
  }
  
  @keyframes blink {
    0%, 50% { opacity: 1; }
    51%, 100% { opacity: 0; }
  }
</style>
