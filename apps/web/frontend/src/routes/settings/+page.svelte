<script lang="ts">
  import { onMount } from 'svelte';
  import { get } from 'svelte/store';
  import ProviderCard from '$lib/components/ProviderCard.svelte';
  import { providerActions } from '$lib/stores/actions';
  import { providers, loadingState } from '$lib/stores/index';
  
  let mounted = false;
  
  onMount(async () => {
    mounted = true;
    await providerActions.loadProviders();
  });
</script>

{#if mounted}
  <div class="settings-page">
    <header class="page-header">
      <h1>LLM Provider 配置</h1>
      <button class="btn btn-primary">
        <span>+</span> 添加 Provider
      </button>
    </header>
    
    <div class="provider-grid">
      {#each $providers as provider}
        <ProviderCard {provider} />
      {/each}
    </div>
  </div>
{/if}

<style>
  .settings-page {
    padding: var(--space-6);
    max-width: 1200px;
    margin: 0 auto;
  }
  
  .page-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: var(--space-6);
  }
  
  .page-header h1 {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-primary);
  }
  
  .provider-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(380px, 1fr));
    gap: var(--space-5);
  }
</style>
