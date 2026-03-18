<script lang="ts">
  import { onMount, createEventDispatcher } from 'svelte';
  
  export let provider: {
    id: string;
    name: string;
    type: 'openai' | 'anthropic' | 'openai-compatible' | 'custom';
    baseUrl: string;
    enabled: boolean;
    models: Array<{ name: string; modelId: string }>;
    status: 'connected' | 'disconnected' | 'testing' | 'error';
    lastTestAt?: Date;
    errorMessage?: string;
  };
  
  const dispatch = createEventDispatcher<{
    test: string;
    edit: string;
    toggle: string;
    delete: string;
  }>();
  
  let mounted = false;
  
  onMount(() => {
    mounted = true;
  });
  
  const typeLabels: Record<string, string> = {
    openai: 'OpenAI',
    anthropic: 'Anthropic',
    'openai-compatible': 'OpenAI 兼容',
    custom: '自定义',
  };
</script>

{#if mounted}
  <div class="provider-card">
    <div class="card-header">
      <div class="provider-icon" class:active={provider.enabled}>
        {provider.name.charAt(0)}
      </div>
      <div class="provider-info">
        <h3>{provider.name}</h3>
        <span class="type-badge">{typeLabels[provider.type] || provider.type}</span>
      </div>
      <div class="status-indicator" class:connected={provider.status === 'connected'} class:testing={provider.status === 'testing'}>
        {#if provider.status === 'testing'}
          测试中...
        {:else if provider.status === 'connected'}
          已连接
        {:else if provider.status === 'error'}
          错误
        {:else}
          未连接
        {/if}
      </div>
    </div>
    
    <div class="card-body">
      <div class="info-row">
        <span class="label">Base URL</span>
        <span class="value">{provider.baseUrl}</span>
      </div>
      <div class="info-row">
        <span class="label">模型</span>
        <span class="value">{provider.models.length > 0 ? provider.models.map(m => m.name).join(', ') : '未配置'}</span>
      </div>
      {#if provider.errorMessage}
        <div class="error-row">
          <span class="error-text">⚠️ {provider.errorMessage}</span>
        </div>
      {/if}
    </div>
    
    <div class="card-footer">
      <button class="test-btn" on:click={() => dispatch('test', provider.id)}>
        {provider.status === 'testing' ? '测试中...' : '测试连接'}
      </button>
      <button class="edit-btn" on:click={() => dispatch('edit', provider.id)}>编辑</button>
      <button class="toggle-btn" class:active={provider.enabled} on:click={() => dispatch('toggle', provider.id)}>
        {provider.enabled ? '禁用' : '启用'}
      </button>
    </div>
  </div>
{/if}

<style>
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
    padding: var(--space-1) var(--space-2);
    background: var(--color-bg-page);
    border-radius: var(--radius-sm);
  }
  
  .status-indicator {
    font-size: var(--font-size-xs);
    padding: var(--space-1) var(--space-2);
    background: var(--color-error);
    color: var(--color-white);
    border-radius: var(--radius-sm);
  }
  
  .status-indicator.connected {
    background: var(--color-success);
  }
  
  .status-indicator.testing {
    background: var(--color-warning);
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
  
  .error-row {
    padding: var(--space-2) 0;
  }
  
  .error-text {
    font-size: var(--font-size-sm);
    color: var(--color-error);
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
    color: var(--color-white);
  }
</style>
