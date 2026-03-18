<script lang="ts">
  import { onMount, createEventDispatcher } from 'svelte';
  
  export let books: Array<{
    id: string;
    title: string;
    stage: 'conception' | 'knowledge' | 'writing';
    chapterCount: number;
    lastActive: Date;
  }>;
  
  export let activeBookId: string | null = null;
  export let loading = false;
  
  const dispatch = createEventDispatcher<{
    select: string;
    create: void;
  }>();
  
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
  
  let mounted = false;
  
  onMount(() => {
    mounted = true;
  });
  
  function handleSelect(bookId: string) {
    dispatch('select', bookId);
  }
  
  function handleCreate() {
    dispatch('create');
  }
</script>

{#if mounted}
  <div class="book-list">
    <div class="list-header">
      <span>我的小说</span>
      <button class="add-btn" title="新建小说" on:click={handleCreate}>
        <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
          <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/>
        </svg>
      </button>
    </div>
    
    {#if loading}
      <div class="loading">
        <div class="spinner"></div>
        <span>加载中...</span>
      </div>
    {:else if books.length === 0}
      <div class="empty">
        <span>暂无书籍</span>
        <button class="create-btn" on:click={handleCreate}>
          创建第一本书
        </button>
      </div>
    {:else}
      <div class="books">
        {#each books as book (book.id)}
          <button 
            class="book-item" 
            class:active={book.id === activeBookId}
            on:click={() => handleSelect(book.id)}
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
    {/if}
  </div>
{/if}

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
  
  .loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: var(--space-6);
    gap: var(--space-2);
    color: var(--color-text-secondary);
  }
  
  .spinner {
    width: 24px;
    height: 24px;
    border: 2px solid var(--color-border);
    border-top-color: var(--color-primary);
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }
  
  @keyframes spin {
    to { transform: rotate(360deg); }
  }
  
  .empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: var(--space-6);
    gap: var(--space-3);
    color: var(--color-text-secondary);
    text-align: center;
  }
  
  .create-btn {
    padding: var(--space-2) var(--space-4);
    background: var(--color-primary);
    color: var(--color-white);
    border: none;
    border-radius: var(--radius-lg);
    font-size: var(--font-size-sm);
    cursor: pointer;
    transition: var(--transition-fast);
  }
  
  .create-btn:hover {
    background: var(--color-primary-dark);
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
