<script lang="ts">
  import { onMount, createEventDispatcher } from 'svelte';
  
  export let disabled = false;
  export let placeholder = '输入消息，@提及 Agent...';
  
  const dispatch = createEventDispatcher<{
    send: string;
  }>();
  
  let inputText = '';
  let textareaRef: HTMLTextAreaElement;
  
  onMount(() => {
    if (textareaRef) {
      textareaRef.style.height = 'auto';
    }
  });
  
  function handleInput() {
    if (textareaRef) {
      textareaRef.style.height = 'auto';
      textareaRef.style.height = Math.min(textareaRef.scrollHeight, 200) + 'px';
    }
  }
  
  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault();
      if (inputText.trim() && !disabled) {
        dispatch('send', inputText.trim());
        inputText = '';
        if (textareaRef) {
          textareaRef.style.height = 'auto';
        }
      }
    }
  }
  
  function sendMessage() {
    if (inputText.trim() && !disabled) {
      dispatch('send', inputText.trim());
      inputText = '';
      if (textareaRef) {
        textareaRef.style.height = 'auto';
      }
    }
  }
  
  function focus() {
    textareaRef?.focus();
  }
  
  export { focus };
</script>

<div class="message-input" class:disabled>
  <div class="input-wrapper">
    <!-- 工具栏 -->
    <div class="toolbar">
      <button class="tool-btn" title="上传文件" disabled>
        <svg viewBox="0 0 24 24" width="18" height="18" fill="currentColor">
          <path d="M16.5 6v11.5c0 2.21-1.79 4-4 4s-4-1.79-4-4V5a2.5 2.5 0 0 1 5 0v10.5c0 .55-.45 1-1 1s-1-.45-1-1V6H10v9.5a2.5 2.5 0 0 0 5 0V5c0-2.21-1.79-4-4-4S7 2.79 7 5v12.5c0 3.04 2.46 5.5 5.5 5.5s5.5-2.46 5.5-5.5V6h-1.5z"/>
        </svg>
      </button>
    </div>
    
    <!-- 输入框 -->
    <textarea
      bind:this={textareaRef}
      bind:value={inputText}
      placeholder={placeholder}
      {disabled}
      rows="1"
      on:keydown={handleKeydown}
      on:input={handleInput}
    ></textarea>
    
    <!-- 发送按钮 -->
    <button 
      class="send-btn" 
      class:active={inputText.length > 0}
      on:click={sendMessage}
      disabled={disabled || inputText.length === 0}
      aria-label="发送消息"
    >
      <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor">
        <path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"/>
      </svg>
    </button>
  </div>
  
  <!-- 提示信息 -->
  <div class="hints">
    <span class="hint">Shift + Enter 换行</span>
    <span class="hint">@ 提及 Agent</span>
    <span class="char-count">{inputText.length} / 10000</span>
  </div>
</div>

<style>
  .message-input {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }
  
  .input-wrapper {
    display: flex;
    align-items: flex-end;
    gap: var(--space-3);
    padding: var(--space-3) var(--space-4);
    background: var(--color-bg-page);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-xl);
    transition: var(--transition-fast);
  }
  
  .message-input:focus-within .input-wrapper {
    border-color: var(--color-primary);
    box-shadow: 0 0 0 3px var(--color-primary-bg);
  }
  
  .message-input.disabled .input-wrapper {
    opacity: 0.6;
    cursor: not-allowed;
  }
  
  .toolbar {
    display: flex;
    gap: var(--space-1);
    padding-bottom: var(--space-1);
  }
  
  .tool-btn {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: transparent;
    color: var(--color-text-secondary);
    border-radius: var(--radius-base);
    cursor: not-allowed;
    transition: var(--transition-fast);
  }
  
  .tool-btn:hover {
    opacity: 1;
  }
  
  textarea {
    flex: 1;
    min-height: 24px;
    max-height: 200px;
    padding: 0;
    border: none;
    background: transparent;
    font-size: var(--font-size-base);
    line-height: var(--line-height-relaxed);
    color: var(--color-text-primary);
    resize: none;
    outline: none;
    font-family: inherit;
  }
  
  textarea::placeholder {
    color: var(--color-text-placeholder);
  }
  
  textarea:disabled {
    cursor: not-allowed;
  }
  
  .send-btn {
    width: 36px;
    height: 36px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: var(--color-text-disabled);
    color: var(--color-white);
    border-radius: var(--radius-lg);
    cursor: not-allowed;
    transition: var(--transition-fast);
  }
  
  .send-btn.active {
    background: var(--color-primary);
    cursor: pointer;
  }
  
  .send-btn.active:hover {
    background: var(--color-primary-dark);
  }
  
  .hints {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: 0 var(--space-3);
    font-size: var(--font-size-xs);
    color: var(--color-text-placeholder);
  }
  
  .hint::before {
    content: '⌘';
    margin-right: var(--space-1);
    opacity: 0.6;
  }
  
  .char-count {
    margin-left: auto;
  }
</style>
