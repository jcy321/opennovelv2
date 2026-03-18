<script lang="ts">
  import { goto } from '$app/navigation';
  import { bookActions } from '$lib/stores/actions';
  import { activeBookId, books } from '$lib/stores/index';
  import ChatPage from '$lib/components/ChatPage.svelte';

  async function handleCreateBook() {
    const title = prompt('请输入书名：');
    if (!title) return;
    
    const book = await bookActions.createBook({ title });
    bookActions.selectBook(book.id);
  }
</script>

{#if $activeBookId}
  <ChatPage bookId={$activeBookId} />
{:else}
  <div class="empty-state">
    <div class="empty-icon">📚</div>
    <h2>选择或创建一本书</h2>
    <p>从左侧列表选择一本书开始创作，或点击下方按钮创建新书。</p>
    <button class="btn btn-primary" on:click={handleCreateBook}>
      <span>+</span> 创建新书
    </button>
  </div>
{/if}

<style>
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    padding: var(--space-6);
    text-align: center;
    background: var(--color-bg-page);
  }
  
  .empty-icon {
    font-size: 64px;
    margin-bottom: var(--space-4);
  }
  
  .empty-state h2 {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-primary);
    margin-bottom: var(--space-2);
  }
  
  .empty-state p {
    font-size: var(--font-size-base);
    color: var(--color-text-secondary);
    margin-bottom: var(--space-6);
    max-width: 400px;
  }
</style>