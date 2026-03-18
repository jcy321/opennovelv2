<!-- 这个文件是 ChatPage.svelte 的副本 -->
<!-- 用于 books/[id] 页面 -->
<script lang="ts">
  import { onMount, onDestroy, tick } from 'svelte';
  import { get } from 'svelte/store';
  import MessageList from '$lib/components/MessageList.svelte';
  import MessageInput from '$lib/components/MessageInput.svelte';
  import AgentStatusPanel from '$lib/components/AgentStatusPanel.svelte';
  import { currentMessages, activeBook, loadingState, errors, uiState } from '$lib/stores/index';
  import { messageActions, agentActions } from '$lib/stores/actions';

  export let id: string;

  let messagesContainer: HTMLElement;
  let showRightPanel = false;
  let rightPanelTab: 'agents' | 'knowledge' | 'settings' = 'agents';
  
  // 滚动到最新消息
  async function scrollToBottom(smooth = false) {
    await tick();
    if (messagesContainer) {
      messagesContainer.scrollTo({
        top: messagesContainer.scrollHeight,
        behavior: smooth ? 'smooth' : 'auto',
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
    
    await messageActions.sendMessage(id, content);
    scrollToBottom();
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
    await messageActions.loadMoreMessages(id);
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
          <AgentStatusPanel />
        {:else if rightPanelTab === 'knowledge'}
          <div class="placeholder">知识库内容</div>
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
        />
        
        {#if $errors.chat}
          <div class="error-banner">
            <span class="error-text">{$errors.chat}</span>
          </div>
        {/if}
      {/if}
    </div>

    <!-- 输入区域 -->
    <footer class="input-area">
      <MessageInput 
        disabled={$loadingState.sendingMessage}
        on:send={handleSendMessage}
      />
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
    padding: var(--space-3) var(--space-4);
    border-bottom: 1px solid var(--color-border-light);
  }
  
  .tabs {
    display: flex;
    gap: var(--space-1);
  }
  
  .tabs button {
    padding: var(--space-2) var(--space-3);
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
    padding: var(--space-4);
  }
  
  .placeholder {
    color: var(--color-text-secondary);
    text-align: center;
    padding: var(--space-6);
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
    padding: var(--space-4) var(--space-6);
    background: var(--color-white);
    border-bottom: 1px solid var(--color-border-light);
  }
  
  .book-info h1 {
    font-size: var(--font-size-xl);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-primary);
    margin-bottom: var(--space-1);
  }
  
  .meta {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
  }
  
  .stage-badge {
    padding: var(--space-1) var(--space-2);
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
    gap: var(--space-2);
  }
  
  .action-btn {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    padding: var(--space-2) var(--space-3);
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
    padding: var(--space-5) var(--space-6);
    display: flex;
    flex-direction: column;
  }
  
  .load-more-btn {
    align-self: center;
    padding: var(--space-2) var(--space-6);
    margin-bottom: var(--space-4);
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
    gap: var(--space-3);
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
    gap: var(--space-3);
    padding: var(--space-3);
    margin-top: var(--space-4);
    background: #fff2f0;
    border: 1px solid #ffccc7;
    border-radius: var(--radius-lg);
  }
  
  .error-text {
    color: var(--color-error);
    font-size: var(--font-size-sm);
  }
  
  .input-area {
    position: relative;
    padding: var(--space-4) var(--space-6);
    background: var(--color-white);
    border-top: 1px solid var(--color-border-light);
  }
</style>
