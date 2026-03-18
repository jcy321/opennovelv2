//! 应用状态

use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::{RwLock, broadcast};

use novel_sdk_core::provider::{
    InMemoryProviderRegistry, ModelResolver, ProviderConfig, ProviderRegistry, LLMProvider,
    ProviderError, ProviderType, HotReloadManager,
};
use novel_sdk_core::provider::{GitCodeProvider, OpenAIProvider, OpenAIConfig};
use novel_sdk_core::provider::embedding::{EmbeddingConfig, EmbeddingProvider};

use crate::book_store::{BookStore, StreamEvent};
use crate::agent_collaboration::{AgentCollaborationManager, EventTx};

fn create_provider(config: &ProviderConfig) -> Result<Arc<dyn LLMProvider>, ProviderError> {
    match config.provider_type {
        ProviderType::OpenAICompatible | ProviderType::Custom => {
            if config.id == "glm5-provider" {
                return Ok(Arc::new(GitCodeProvider::new(novel_sdk_core::provider::GitCodeConfig {
                    api_key: config.api_key.clone(),
                    default_model: config.default_model.clone().unwrap_or_else(|| "GLM-5".to_string()),
                    base_url: config.base_url.clone(),
                })));
            }
            let openai_config = OpenAIConfig {
                api_key: config.api_key.clone(),
                base_url: config.base_url.clone(),
                default_model: config.default_model.clone().unwrap_or_else(|| "GLM-5".to_string()),
            };
            Ok(Arc::new(OpenAIProvider::new(openai_config)))
        }
        ProviderType::Anthropic => {
            Err(ProviderError::UnsupportedProvider("Anthropic not configured".to_string()))
        }
        ProviderType::OpenAI => {
            let openai_config = OpenAIConfig {
                api_key: config.api_key.clone(),
                base_url: "https://api.openai.com/v1".to_string(),
                default_model: config.default_model.clone().unwrap_or_else(|| "gpt-4".to_string()),
            };
            Ok(Arc::new(OpenAIProvider::new(openai_config)))
        }
    }
}

/// 应用状态
#[derive(Clone)]
pub struct AppState {
    pub provider_registry: Arc<InMemoryProviderRegistry>,
    pub model_resolver: Arc<ModelResolver<Arc<InMemoryProviderRegistry>>>,
    pub config_version: Arc<RwLock<u64>>,
    pub hot_reload: Arc<HotReloadManager>,
    pub book_store: Arc<BookStore>,
    pub event_channels: Arc<RwLock<HashMap<String, EventTx>>>,
    pub embedding_provider: Arc<EmbeddingProvider>,
}

impl AppState {
    pub fn new() -> Self {
        let registry = Arc::new(InMemoryProviderRegistry::new(|config| create_provider(config)));
        
        let gitcode_config = GitCodeProvider::create_builtin_provider_config();
        
        let registry_clone = registry.clone();
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                let _ = registry_clone.register(gitcode_config).await;
            });
        });
        
        let model_resolver = Arc::new(ModelResolver::new(registry.clone(), None, None));
        let hot_reload = Arc::new(HotReloadManager::new());
        let book_store = Arc::new(BookStore::new());
        let event_channels = Arc::new(RwLock::new(HashMap::new()));
        let embedding_provider = Arc::new(EmbeddingProvider::builtin());
        
        Self {
            provider_registry: registry,
            model_resolver,
            config_version: Arc::new(RwLock::new(1)),
            hot_reload,
            book_store,
            event_channels,
            embedding_provider,
        }
    }
    
    pub async fn get_or_create_event_channel(&self, book_id: &str) -> EventTx {
        let mut channels = self.event_channels.write().await;
        if let Some(tx) = channels.get(book_id) {
            return tx.clone();
        }
        
        let (tx, _rx): (EventTx, _) = broadcast::channel(100);
        channels.insert(book_id.to_string(), tx.clone());
        tx
    }
    
    pub fn create_collaboration_manager(&self, event_tx: EventTx) -> AgentCollaborationManager {
        AgentCollaborationManager::new(
            event_tx,
            self.provider_registry.clone(),
        )
    }
    
    pub async fn get_config_version(&self) -> u64 {
        *self.config_version.read().await
    }
    
    pub async fn increment_config_version(&self) -> u64 {
        let mut version = self.config_version.write().await;
        *version += 1;
        *version
    }
    
    pub async fn broadcast_config_update(&self, event_type: &str, provider_id: &str) {
        self.hot_reload.broadcast_update(event_type, provider_id).await;
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}