<script lang="ts">
  import { onMount } from 'svelte';
  import { agentStatuses, activeBook } from '$lib/stores/index';
  
  const statusLabels: Record<string, string> = {
    idle: '空闲',
    thinking: '思考中',
    writing: '撰写中',
    waiting: '等待中',
    error: '错误',
  };
  
  const stageLabels: Record<string, string> = {
    conception: '构思阶段',
    knowledge: '知识库阶段',
    writing: '撰写阶段',
  };
  
  const agentColors: Record<string, string> = {
    '天道': 'var(--agent-tiandao)',
    '执笔': 'var(--agent-writer)',
    '世界观守护者': 'var(--agent-worldguardian)',
    '刘和平': 'var(--agent-liuheping)',
    '规划者': 'var(--agent-planner)',
    '审阅': 'var(--agent-reviewer)',
    '观察者': 'var(--agent-observer)',
    '调研者': 'var(--agent-researcher)',
  };
  
  let mounted = false;
  
  onMount(() => {
    mounted = true;
  });
</script>

{#if mounted}
  <div class="agent-panel">
    <div class="panel-header">
      <span class="title">Agent 状态</span>
      <span class="stage-badge">
        {$activeBook ? stageLabels[$activeBook.stage] : '未选择'}
      </span>
    </div>
    
    <div class="agent-list">
      {#each $agentStatuses as agent}
        <div class="agent-item" class:locked={agent.isLocked}>
           <div 
             class="agent-avatar" 
             style="background: {agentColors[agent.name] || 'var(--color-primary)'}"
          >
            <span class="avatar-initial">{agent.name.charAt(0)}</span>
          </div>
          
          <div class="agent-info">
            <span class="name">{agent.name}</span>
            <span class="role">{agent.role}</span>
          </div>
          
          {#if agent.isLocked}
            <span class="lock-icon" title="当前阶段锁定">🔒</span>
          {:else}
            <span class="status-label" class:active={agent.status !== 'idle'}>
              {statusLabels[agent.status]}
            </span>
          {/if}
        </div>
      {/each}
    </div>
  </div>
{/if}

<style>
  .agent-panel {
    background: var(--color-white);
    border-radius: var(--radius-xl);
    padding: var(--space-4);
    box-shadow: var(--shadow-sm);
  }
  
  .panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: var(--space-4);
  }
  
  .title {
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-primary);
  }
  
  .stage-badge {
    padding: var(--space-1) var(--space-2);
    background: var(--color-success);
    color: var(--color-white);
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-medium);
    border-radius: var(--radius-sm);
  }
  
  .agent-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }
  
  .agent-item {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-2);
    border-radius: var(--radius-lg);
    transition: var(--transition-fast);
  }
  
  .agent-item:not(.locked):hover {
    background: var(--color-bg-hover);
  }
  
  .agent-item.locked {
    opacity: 0.5;
  }
  
  .agent-info {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-width: 0;
  }
  
  .name {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: var(--color-text-primary);
  }
  
  .role {
    font-size: var(--font-size-xs);
    color: var(--color-text-secondary);
  }
  
  .lock-icon {
    font-size: var(--font-size-sm);
  }
  
  .status-label {
    font-size: var(--font-size-xs);
    color: var(--color-text-placeholder);
    padding: var(--space-1) var(--space-2);
    background: var(--color-bg-page);
    border-radius: var(--radius-sm);
  }
  
  .status-label.active {
    color: var(--color-primary);
    background: var(--color-primary-bg);
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
  }
  
  .avatar-initial {
    font-size: 14px;
  }
</style>
