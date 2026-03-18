<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { get } from 'svelte/store';
  import { browser } from '$app/environment';
  import { goto } from '$app/navigation';
  import BookList from '$lib/components/BookList.svelte';
  import { bookActions, providerActions } from '$lib/stores/actions';
  import { books, activeBookId, loadingState, uiState } from '$lib/stores/index';
  
  let sidebarOpen = true;
  let mounted = false;

  onMount(async () => {
    mounted = true;
    
    // 检测移动端（仅在浏览器环境）
    if (browser) {
      checkMobile();
      window.addEventListener('resize', checkMobile);
    }
    
    // 加载初始数据
    await Promise.all([
      bookActions.loadBooks(),
      providerActions.loadProviders(),
    ]);
  });

  onDestroy(() => {
    if (browser && mounted) {
      window.removeEventListener('resize', checkMobile);
    }
  });

  function checkMobile() {
    if (browser) {
      uiState.update((s) => ({
        ...s,
        isMobile: window.innerWidth < 768,
      }));
    }
  }
  
  // 处理选择书籍
  function handleSelectBook(event: CustomEvent<string>) {
    const bookId = event.detail;
    bookActions.selectBook(bookId);
  }
  
  // 处理创建书籍
  async function handleCreateBook() {
    const title = prompt('请输入书名：');
    if (!title) return;
    
    const book = await bookActions.createBook({ title });
    bookActions.selectBook(book.id);
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
  <aside class="side-nav" class:open={sidebarOpen}>
    <BookList 
      books={$books}
      activeBookId={$activeBookId}
      loading={$loadingState.booksLoading}
      on:select={handleSelectBook}
      on:create={handleCreateBook}
    />
  </aside>

  <!-- 主内容区 - 使用 slot 渲染子页面 -->
  <main class="main-content">
    <slot />
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
  
  .app-layout.sidebar-collapsed .side-nav {
    display: none;
  }
  
  .top-bar {
    grid-column: 1 / -1;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 var(--space-4);
    background: var(--color-white);
    border-bottom: 1px solid var(--color-border);
    box-shadow: var(--shadow-sm);
    z-index: 100;
  }
  
  .left, .right {
    display: flex;
    align-items: center;
    gap: var(--space-3);
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
    gap: var(--space-2);
  }
  
  .logo-icon {
    font-size: 24px;
  }
  
  .logo-text {
    font-size: var(--font-size-lg);
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
    gap: var(--space-2);
    width: 400px;
    max-width: 100%;
    padding: var(--space-2) var(--space-4);
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
    color: var(--color-white);
    border-radius: var(--radius-full);
    font-size: var(--font-size-sm);
    cursor: pointer;
  }
  
  .side-nav {
    background: var(--color-white);
    border-right: 1px solid var(--color-border);
    overflow: hidden;
    transition: width 0.3s ease;
    height: calc(100vh - 56px);
  }
  
  .main-content {
    overflow: hidden;
    height: calc(100vh - 56px);
  }
  
  /* 移动端适配 */
  @media (max-width: 768px) {
    .app-layout {
      grid-template-columns: 1fr;
    }
    
    .search-box {
      width: 200px;
    }
    
    .logo-text {
      display: none;
    }
  }
</style>