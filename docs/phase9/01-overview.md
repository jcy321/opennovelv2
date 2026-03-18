# Phase 9: 测试与优化

**生成日期**: 2026-03-17
**预计工期**: 1-2 周
**前置依赖**: Phase 8 完成
**状态**: 规划中

---

## 一、Phase 9 概述

### 1.1 目标

确保 OpenNovel 系统的稳定性和性能：
- 完善单元测试覆盖率
- 实现集成测试
- 端到端测试（E2E）
- 性能优化
- 错误处理完善

### 1.2 测试金字塔

```
                    ┌─────────┐
                   │   E2E   │  (5%)
                  │   Tests  │
                 └───────────┘
                ┌───────────────┐
               │  Integration  │  (25%)
              │     Tests      │
             └─────────────────┘
            ┌─────────────────────┐
           │    Unit Tests       │  (70%)
          │                       │
         └─────────────────────────┘
```

### 1.3 测试工具栈

| 类型 | 工具 | 说明 |
|------|------|------|
| 单元测试 | Rust内置 `#[test]` | 每个 crate 的测试 |
| 集成测试 | `cargo test` | 跨模块测试 |
| E2E 测试 | Playwright | Web UI 测试 |
| 性能测试 | Criterion | 基准测试 |
| 覆盖率 | tarpaulin | 代码覆盖率 |

---

## 二、单元测试

### 2.1 测试范围

| 模块 | 测试重点 | 目标覆盖率 |
|------|---------|-----------|
| SDK Core | Agent trait, IntentGate, PermissionMatrix | 80% |
| Knowledge | CharacterDB, ForeshadowingPool, Timeline | 75% |
| Text Tools | Editor, Counter, StyleChecker | 85% |
| Collaboration | Annotation, ConflictArbitration | 70% |
| Analysis | StyleAnalyzer, EmotionAnalyzer | 70% |
| Provider | Registry, Resolver, FallbackChain | 80% |

### 2.2 示例测试

```rust
// packages/sdk/core/src/agent/trait.rs

#[cfg(test)]
mod tests {
    use super::*;

    struct MockAgent {
        id: AgentId,
    }

    #[async_trait]
    impl Agent for MockAgent {
        fn id(&self) -> &AgentId { &self.id }
        fn name(&self) -> &str { "Mock Agent" }
        fn role(&self) -> AgentRole { AgentRole::Writer }
        
        async fn process(
            &self,
            message: &str,
            context: &SessionContext,
        ) -> Result<AgentResponse> {
            Ok(AgentResponse {
                content: format!("Processed: {}", message),
                annotations: vec![],
            })
        }
    }

    #[tokio::test]
    async fn test_agent_process() {
        let agent = MockAgent { id: AgentId::new("mock") };
        let context = SessionContext::default();
        
        let response = agent.process("test message", &context).await.unwrap();
        
        assert_eq!(response.content, "Processed: test message");
        assert!(response.annotations.is_empty());
    }

    #[test]
    fn test_agent_permission_check() {
        let matrix = PermissionMatrix::default();
        
        // 天道可以写伏笔库
        assert!(matrix.check_permission(
            AgentRole::TianDao,
            KnowledgeBase::Foreshadowing,
            Permission::Write
        ));
        
        // 执笔不能写伏笔库
        assert!(!matrix.check_permission(
            AgentRole::Writer,
            KnowledgeBase::Foreshadowing,
            Permission::Write
        ));
    }
}
```

### 2.3 IntentGate 测试

```rust
// packages/sdk/core/src/intent/gate.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verbalize_intent_implementation() {
        let gate = IntentGate::new();
        let context = SessionContext::default();
        
        // 实施意图
        let result = gate.verbalize_intent("写第一章", &context);
        assert_eq!(result.intent_type, IntentType::Implementation);
        assert!(result.reason.contains("写"));
        
        // 研究意图
        let result = gate.verbalize_intent("如何设计人物弧光？", &context);
        assert_eq!(result.intent_type, IntentType::Research);
    }

    #[test]
    fn test_classify_request() {
        let gate = IntentGate::new();
        let context = SessionContext::default();
        
        // 简单请求
        let result = gate.classify_request("写第一章", &context);
        assert_eq!(result, RequestClassification::Explicit);
        
        // 开放式请求
        let result = gate.classify_request("优化这段文字", &context);
        assert_eq!(result, RequestClassification::OpenEnded);
    }

    #[test]
    fn test_check_ambiguity() {
        let gate = IntentGate::new();
        
        // 无歧义
        let result = gate.check_ambiguity("写第一章", RequestClassification::Explicit);
        assert!(!result.has_ambiguity);
        
        // 有歧义
        let result = gate.check_ambiguity("修改人物", RequestClassification::Ambiguous);
        assert!(result.has_ambiguity);
        assert!(result.interpretations.len() > 1);
    }
}
```

### 2.4 运行单元测试

```bash
# 运行所有单元测试
cargo test --workspace

# 运行特定包的测试
cargo test -p novel-sdk-core

# 运行特定测试
cargo test test_intent_gate -- --nocapture

# 生成覆盖率报告
cargo tarpaulin --workspace --out Html
```

---

## 三、集成测试

### 3.1 测试目录结构

```
/tests/
├── integration/
│   ├── agent_collaboration_test.rs    # Agent 协作测试
│   ├── knowledge_flow_test.rs         # 知识库流转测试
│   ├── provider_resolution_test.rs    # Provider 解析测试
│   └── chat_flow_test.rs              # 聊天流程测试
│
├── fixtures/
│   ├── sample_novel.json              # 测试小说数据
│   ├── sample_characters.json         # 测试人物数据
│   └── sample_worldview.json          # 测试世界观
│
└── mocks/
    ├── mock_llm_provider.rs           # Mock LLM Provider
    └── mock_agent.rs                  # Mock Agent
```

### 3.2 Agent 协作集成测试

```rust
// tests/integration/agent_collaboration_test.rs

use novel_sdk_core::*;
use novel_knowledge::*;

/// 测试天道与执笔的协作流程
#[tokio::test]
async fn test_tiandao_writer_collaboration() {
    // 1. 初始化系统
    let registry = AgentRegistry::new();
    let knowledge = KnowledgeBase::new_in_memory();
    
    // 2. 注册 Agent
    registry.register(TianDaoAgent::new(knowledge.clone())).await;
    registry.register(WriterAgent::new(knowledge.clone())).await;
    
    // 3. 模拟用户请求
    let context = SessionContext {
        book_id: "test-book".into(),
        stage: CreationStage::Writing,
        ..Default::default()
    };
    
    // 4. 天道编排剧情
    let tiandao = registry.get("tian-dao").await.unwrap();
    let plan = tiandao.process("设计第二章剧情", &context).await.unwrap();
    
    // 5. 验证规划包含关键事件
    assert!(plan.content.contains("关键事件"));
    
    // 6. 执笔撰写
    let writer = registry.get("writer").await.unwrap();
    let chapter = writer.process(&plan.content, &context).await.unwrap();
    
    // 7. 验证章节输出
    assert!(chapter.content.len() > 100);
}

/// 测试世界观守护者的违规检测
#[tokio::test]
async fn test_world_guardian_violation_detection() {
    let knowledge = KnowledgeBase::new_in_memory();
    
    // 添加世界观规则
    knowledge.worldview.add_rule(WorldviewRule {
        rule_id: "no-modern-slang".into(),
        description: "古代背景不能使用现代网络流行语".into(),
        scope: RuleScope::Global,
        violation_action: ViolationAction::Warn,
    }).await;
    
    let guardian = WorldGuardianAgent::new(knowledge.clone());
    
    // 测试违规检测
    let result = guardian.check_content("主角说：YYDS，这个操作太绝了！").await;
    
    assert!(result.has_violations);
    assert!(result.violations.iter().any(|v| v.rule_id == "no-modern-slang"));
}
```

### 3.3 Provider 解析集成测试

```rust
// tests/integration/provider_resolution_test.rs

#[tokio::test]
async fn test_provider_fallback_chain() {
    let registry = ProviderRegistry::new();
    let resolver = ModelResolver::new(&registry);
    
    // 注册多个 Provider
    registry.register(ProviderConfig {
        id: "primary".into(),
        provider_type: ProviderType::OpenAI,
        priority: 100,
        enabled: true,
        models: vec![/* ... */],
        ..Default::default()
    }).await;
    
    registry.register(ProviderConfig {
        id: "fallback".into(),
        provider_type: ProviderType::OpenAICompatible,
        priority: 50,
        enabled: true,
        models: vec![/* ... */],
        ..Default::default()
    }).await;
    
    // 测试正常解析
    let resolution = resolver.resolve(ResolutionInput {
        agent_name: Some("tian-dao".into()),
        category: Some("ultrabrain".into()),
        ..Default::default()
    }).await.unwrap();
    
    assert_eq!(resolution.provider_id, "primary");
    
    // 模拟主 Provider 故障
    registry.set_available("primary", false).await;
    
    // 测试 Fallback
    let resolution = resolver.resolve(ResolutionInput::default()).await.unwrap();
    assert_eq!(resolution.provider_id, "fallback");
    assert!(resolution.is_fallback);
}
```

### 3.4 运行集成测试

```bash
# 运行所有集成测试
cargo test --test '*'

# 运行特定集成测试
cargo test --test agent_collaboration_test

# 带日志输出
RUST_LOG=debug cargo test --test chat_flow_test -- --nocapture
```

---

## 四、E2E 测试（Playwright）

### 4.1 测试场景

| 场景 | 说明 | 优先级 |
|------|------|--------|
| 用户注册/登录 | 基础认证流程 | P0 |
| 创建书籍 | 创建新小说项目 | P0 |
| 群聊交互 | 发送消息，Agent 响应 | P0 |
| Provider 配置 | 添加/测试 Provider | P1 |
| 可视化查看 | 查看人物关系图 | P1 |
| 流式输出 | 验证 SSE 正常工作 | P1 |

### 4.2 测试实现

```typescript
// tests/e2e/chat.spec.ts

import { test, expect } from '@playwright/test';

test.describe('群聊功能', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/books/test-book/chat');
  });

  test('发送消息并收到 Agent 响应', async ({ page }) => {
    // 输入消息
    await page.fill('[data-testid="message-input"]', '写第一章');
    await page.click('[data-testid="send-button"]');
    
    // 等待用户消息出现
    await expect(page.locator('[data-testid="user-message"]').last()).toContainText('写第一章');
    
    // 等待 Agent 响应（流式输出）
    await expect(page.locator('[data-testid="agent-message"]').first()).toBeVisible({ timeout: 30000 });
    
    // 验证 Agent 身份标识
    await expect(page.locator('[data-testid="agent-badge"]').first()).toBeVisible();
  });

  test('@提及 Agent', async ({ page }) => {
    await page.fill('[data-testid="message-input"]', '@天道 ');
    
    // 验证提及提示出现
    await expect(page.locator('[data-testid="mention-dropdown"]')).toBeVisible();
    
    // 选择 Agent
    await page.click('[data-testid="mention-option-tiandao"]');
    
    // 验证输入框包含提及
    await expect(page.locator('[data-testid="message-input"]')).toHaveValue(/@天道/);
  });

  test('流式输出正常工作', async ({ page }) => {
    await page.fill('[data-testid="message-input"]', '继续写');
    await page.click('[data-testid="send-button"]');
    
    // 等待流式消息开始
    const streamingMsg = page.locator('[data-testid="streaming-message"]');
    await expect(streamingMsg).toBeVisible({ timeout: 5000 });
    
    // 等待流式完成
    await expect(streamingMsg).not.toBeVisible({ timeout: 60000 });
    
    // 验证完整消息存在
    await expect(page.locator('[data-testid="agent-message"]').last()).toBeVisible();
  });
});
```

### 4.3 Provider 配置测试

```typescript
// tests/e2e/provider.spec.ts

test.describe('Provider 配置', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/settings/providers');
  });

  test('添加新 Provider', async ({ page }) => {
    // 点击添加按钮
    await page.click('[data-testid="add-provider-button"]');
    
    // 填写表单
    await page.fill('[name="name"]', 'Test Provider');
    await page.fill('[name="base_url"]', 'https://api.example.com/v1');
    await page.fill('[name="api_key"]', 'test-key');
    
    // 添加模型
    await page.click('[data-testid="add-model-button"]');
    await page.fill('[name="model_id"]', 'gpt-4');
    
    // 保存
    await page.click('[data-testid="save-button"]');
    
    // 验证保存成功
    await expect(page.locator('[data-testid="provider-card"]').filter({ hasText: 'Test Provider' })).toBeVisible();
  });

  test('测试 Provider 连接', async ({ page }) => {
    // 找到已存在的 Provider
    const providerCard = page.locator('[data-testid="provider-card"]').first();
    
    // 点击测试按钮
    await providerCard.locator('[data-testid="test-button"]').click();
    
    // 等待测试结果
    await expect(providerCard.locator('[data-testid="test-result"]')).toBeVisible({ timeout: 10000 });
    
    // 验证成功状态
    await expect(providerCard.locator('[data-testid="test-success"]')).toBeVisible();
  });
});
```

### 4.4 运行 E2E 测试

```bash
# 安装 Playwright
npm install -D @playwright/test
npx playwright install

# 运行所有 E2E 测试
npx playwright test

# 运行特定测试
npx playwright test tests/e2e/chat.spec.ts

# UI 模式调试
npx playwright test --ui

# 生成覆盖率报告
npx playwright test --reporter=html
```

---

## 五、性能测试

### 5.1 基准测试（Criterion）

```rust
// benches/agent_benchmark.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use novel_sdk_core::*;

fn bench_intent_gate(c: &mut Criterion) {
    let gate = IntentGate::new();
    let context = SessionContext::default();
    
    c.bench_function("intent_gate_verbalize", |b| {
        b.iter(|| {
            gate.verbalize_intent(black_box("写第一章"), &context)
        })
    });
    
    c.bench_function("intent_gate_classify", |b| {
        b.iter(|| {
            gate.classify_request(black_box("写第一章"), &context)
        })
    });
}

fn bench_knowledge_search(c: &mut Criterion) {
    let knowledge = KnowledgeBase::new_in_memory();
    // 添加测试数据...
    
    c.bench_function("knowledge_search_character", |b| {
        b.iter(|| {
            knowledge.character_db.search(black_box("主角"))
        })
    });
}

criterion_group!(benches, bench_intent_gate, bench_knowledge_search);
criterion_main!(benches);
```

### 5.2 负载测试

```rust
// tests/load/chat_load_test.rs

use tokio::time::{duration, sleep};

/// 测试并发聊天请求
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_concurrent_chat_requests() {
    let client = reqwest::Client::new();
    let mut handles = vec![];
    
    // 模拟 100 个并发用户
    for i in 0..100 {
        let client = client.clone();
        let handle = tokio::spawn(async move {
            let response = client
                .post("http://localhost:3000/api/books/test/chat")
                .json(&serde_json::json!({
                    "content": format!("Test message {}", i)
                }))
                .send()
                .await
                .unwrap();
            
            assert!(response.status().is_success());
        });
        handles.push(handle);
    }
    
    // 等待所有请求完成
    for handle in handles {
        handle.await.unwrap();
    }
}
```

### 5.3 性能指标

| 指标 | 目标值 | 测量方法 |
|------|--------|---------|
| API 响应时间 | P95 < 500ms | Prometheus metrics |
| 消息发送延迟 | < 200ms | 前端性能 API |
| SSE 连接稳定性 | > 99.9% | 监控断开率 |
| 内存占用 | < 500MB | 系统监控 |
| 并发支持 | 100 用户 | 负载测试 |

---

## 六、错误处理完善

### 6.1 错误类型定义

```rust
// packages/sdk/core/src/error.rs

#[derive(Debug, thiserror::Error)]
pub enum OpenNovelError {
    #[error("Agent not found: {0}")]
    AgentNotFound(String),
    
    #[error("Permission denied: {agent} cannot {action}")]
    PermissionDenied { agent: String, action: String },
    
    #[error("Knowledge base error: {0}")]
    KnowledgeBase(#[from] KnowledgeError),
    
    #[error("LLM provider error: {0}")]
    LlmProvider(#[from] ProviderError),
    
    #[error("Stage locked: {agent} is locked until {stage}")]
    StageLocked { agent: String, stage: String },
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, OpenNovelError>;
```

### 6.2 错误处理中间件

```rust
// apps/web/src/middleware/error.rs

use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};

pub async fn error_handler(
    error: axum::BoxError,
) -> impl IntoResponse {
    let (status, message) = if error.is::<OpenNovelError>() {
        match error.downcast_ref::<OpenNovelError>().unwrap() {
            OpenNovelError::AgentNotFound(_) => (StatusCode::NOT_FOUND, error.to_string()),
            OpenNovelError::PermissionDenied { .. } => (StatusCode::FORBIDDEN, error.to_string()),
            OpenNovelError::Validation(_) => (StatusCode::BAD_REQUEST, error.to_string()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error".to_string()),
        }
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error".to_string())
    };
    
    (status, Json(serde_json::json!({
        "error": {
            "code": status.as_u16(),
            "message": message
        }
    })))
}
```

### 6.3 前端错误处理

```typescript
// src/lib/api/client.ts

export class ApiError extends Error {
  constructor(
    public status: number,
    public code: string,
    message: string
  ) {
    super(message);
  }
}

export async function apiClient<T>(
  endpoint: string,
  options?: RequestInit
): Promise<T> {
  const response = await fetch(endpoint, options);
  
  if (!response.ok) {
    const error = await response.json();
    throw new ApiError(
      response.status,
      error.error?.code || 'UNKNOWN',
      error.error?.message || 'Unknown error'
    );
  }
  
  return response.json();
}

// 使用示例
try {
  const result = await apiClient('/api/books', { method: 'POST', body: JSON.stringify(data) });
} catch (error) {
  if (error instanceof ApiError) {
    if (error.status === 403) {
      showToast('权限不足', 'error');
    } else {
      showToast(error.message, 'error');
    }
  }
}
```

---

## 七、任务分解

### Task 9.1: 单元测试完善（3 天）

| 子任务 | 工时 | 说明 |
|--------|------|------|
| 9.1.1 SDK Core 测试 | 1 天 | Agent, IntentGate, Permission |
| 9.1.2 Knowledge 测试 | 1 天 | CharacterDB, ForeshadowingPool |
| 9.1.3 Provider 测试 | 1 天 | Registry, Resolver, Fallback |

### Task 9.2: 集成测试实现（2 天）

| 子任务 | 工时 | 说明 |
|--------|------|------|
| 9.2.1 Agent 协作测试 | 1 天 | 天道-执笔，刘和平-世界观守护者 |
| 9.2.2 Provider 解析测试 | 0.5 天 | Fallback Chain |
| 9.2.3 聊天流程测试 | 0.5 天 | 端到端流程 |

### Task 9.3: E2E 测试实现（2 天）

| 子任务 | 工时 | 说明 |
|--------|------|------|
| 9.3.1 群聊测试 | 1 天 | 消息发送，流式输出，@提及 |
| 9.3.2 Provider 测试 | 0.5 天 | 添加/测试 Provider |
| 9.3.3 可视化测试 | 0.5 天 | 人物关系图，世界地图 |

### Task 9.4: 性能优化（2 天）

| 子任务 | 工时 | 说明 |
|--------|------|------|
| 9.4.1 基准测试 | 1 天 | Criterion 基准 |
| 9.4.2 性能分析 | 0.5 天 | 瓶颈定位 |
| 9.4.3 优化实施 | 0.5 天 | 针对性优化 |

### Task 9.5: 错误处理（1 天）

| 子任务 | 工时 | 说明 |
|--------|------|------|
| 9.5.1 错误类型定义 | 0.5 天 | 统一错误类型 |
| 9.5.2 中间件实现 | 0.5 天 | 错误处理中间件 |

---

## 八、验收标准

### 8.1 测试覆盖率

- [ ] 单元测试覆盖率 > 75%
- [ ] 集成测试覆盖核心流程
- [ ] E2E 测试覆盖主要用户场景

### 8.2 性能指标

- [ ] API 响应 P95 < 500ms
- [ ] 消息发送延迟 < 200ms
- [ ] 内存占用 < 500MB（无负载）

### 8.3 稳定性

- [ ] 连续运行 24 小时无崩溃
- [ ] 错误处理覆盖率 100%
- [ ] 无已知 P0/P1 Bug

---

## 九、CI/CD 集成

### 9.1 GitHub Actions 配置

```yaml
# .github/workflows/test.yml

name: Test

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  unit-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      
      - name: Cache cargo
        uses: actions/cache@v4
        with:
          path: ~/.cargo
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Run unit tests
        run: cargo test --workspace
      
      - name: Generate coverage
        run: cargo tarpaulin --workspace --out Xml
      
      - name: Upload coverage
        uses: codecov/codecov-action@v4
        with:
          file: cobertura.xml

  e2e-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
      
      - name: Install Playwright
        run: |
          npm install -D @playwright/test
          npx playwright install
      
      - name: Run E2E tests
        run: npx playwright test
      
      - name: Upload test results
        uses: actions/upload-artifact@v4
        if: always()
        with:
          name: playwright-report
          path: playwright-report/
```