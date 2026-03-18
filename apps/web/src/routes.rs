use axum::{
    extract::{Path, Query, State, WebSocketUpgrade, ws::WebSocket},
    http::StatusCode,
    response::{IntoResponse, Json, Response, sse::Sse},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc;
use futures::{SinkExt, StreamExt, stream::Stream};
use uuid::Uuid;
use std::time::Duration;

use novel_sdk_core::provider::{
    ProviderRegistry, ProviderConfig, ProviderType, ModelConfig,
    ResolutionInput, UserOverride, ChatRequest, ChatMessage, MessageRole as LLMMessageRole,
    HotReloadManager, PartialProviderConfig,
};

use crate::state::AppState;
use crate::book_store::{
    Book, BookStage, CreateBookRequest, UpdateBookRequest,
    Message, SendMessageRequest, AgentStatus, AgentState, StreamEvent,
};

pub fn build_routes() -> Router<AppState> {
    Router::new()
        .route("/health", axum::routing::get(|| async { "OK" }))
        
        // Book API
        .route("/api/books", axum::routing::get(list_books).post(create_book))
        .route("/api/books/:id", axum::routing::get(get_book).delete(delete_book).patch(update_book))
        .route("/api/books/:id/messages", axum::routing::get(get_messages))
        .route("/api/books/:id/chat", axum::routing::post(send_message))
        .route("/api/books/:id/stream", axum::routing::get(stream_events))
        .route("/api/books/:id/agents", axum::routing::get(get_agent_statuses))
        
        // Provider API
        .route("/api/providers", axum::routing::get(list_providers).post(create_provider))
        .route("/api/providers/:id", axum::routing::get(get_provider).delete(delete_provider).patch(update_provider))
        .route("/api/providers/:id/test", axum::routing::post(test_provider))
        
        // LLM API
        .route("/api/llm/resolve", axum::routing::post(resolve_model))
        .route("/api/llm/generate", axum::routing::post(generate_text))
        
        // Config API
        .route("/api/config/version", axum::routing::get(get_config_version))
        .route("/ws/config", axum::routing::get(ws_config_handler))
}

async fn list_books(State(state): State<AppState>) -> impl IntoResponse {
    let books = state.book_store.list_books().await;
    (StatusCode::OK, Json(serde_json::json!({ "books": books })))
}

async fn create_book(
    State(state): State<AppState>,
    Json(request): Json<CreateBookRequest>,
) -> impl IntoResponse {
    if request.title.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": { "code": "INVALID_TITLE", "message": "书名不能为空" }
        })));
    }
    
    let book = state.book_store.create_book(request).await;
    (StatusCode::CREATED, Json(serde_json::to_value(book).unwrap()))
}

async fn get_book(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    match state.book_store.get_book(&id).await {
        Some(book) => (StatusCode::OK, Json(serde_json::to_value(book).unwrap())),
        None => (StatusCode::NOT_FOUND, Json(serde_json::json!({
            "error": { "code": "BOOK_NOT_FOUND", "message": "书籍不存在" }
        }))),
    }
}

async fn update_book(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(request): Json<UpdateBookRequest>,
) -> impl IntoResponse {
    match state.book_store.update_book(&id, request).await {
        Some(book) => (StatusCode::OK, Json(serde_json::to_value(book).unwrap())),
        None => (StatusCode::NOT_FOUND, Json(serde_json::json!({
            "error": { "code": "BOOK_NOT_FOUND", "message": "书籍不存在" }
        }))),
    }
}

async fn delete_book(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if state.book_store.delete_book(&id).await {
        (StatusCode::OK, Json(serde_json::json!({ "success": true })))
    } else {
        (StatusCode::NOT_FOUND, Json(serde_json::json!({
            "error": { "code": "BOOK_NOT_FOUND", "message": "书籍不存在" }
        })))
    }
}

#[derive(Debug, Deserialize)]
struct GetMessagesQuery {
    #[serde(default = "default_limit")]
    limit: usize,
    #[serde(default)]
    before: Option<String>,
}

fn default_limit() -> usize { 50 }

async fn get_messages(
    Path(book_id): Path<String>,
    State(state): State<AppState>,
    Query(query): Query<GetMessagesQuery>,
) -> impl IntoResponse {
    let messages = state.book_store.get_messages(&book_id, Some(query.limit), query.before.as_deref()).await;
    (StatusCode::OK, Json(serde_json::json!({ "messages": messages })))
}

async fn send_message(
    Path(book_id): Path<String>,
    State(state): State<AppState>,
    Json(request): Json<SendMessageRequest>,
) -> impl IntoResponse {
    if request.content.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": { "code": "EMPTY_MESSAGE", "message": "消息内容不能为空" }
        })));
    }
    
    let book = match state.book_store.get_book(&book_id).await {
        Some(b) => b,
        None => {
            return (StatusCode::NOT_FOUND, Json(serde_json::json!({
                "error": { "code": "BOOK_NOT_FOUND", "message": "书籍不存在" }
            })));
        }
    };
    
    let user_message = Message::new_user(book_id.clone(), request.content.trim().to_string());
    let saved_message = state.book_store.add_message(user_message.clone()).await;
    
    let event_tx = state.get_or_create_event_channel(&book_id).await;
    let collaboration_manager = state.create_collaboration_manager(event_tx);
    
    let conversation_history = state.book_store.get_messages(&book_id, Some(20), None).await;
    
    // 异步触发 Agent 协作（不阻塞响应）
    let book_clone = book;
    let user_message_clone = user_message;
    let history_clone = conversation_history;
    
    tokio::spawn(async move {
        if let Err(e) = collaboration_manager.process_user_message(
            &book_clone,
            &user_message_clone,
            history_clone,
        ).await {
            eprintln!("Agent collaboration error: {:?}", e);
        }
    });
    
    (StatusCode::OK, Json(serde_json::to_value(saved_message).unwrap()))
}

async fn get_agent_statuses(
    Path(book_id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if state.book_store.get_book(&book_id).await.is_none() {
        return (StatusCode::NOT_FOUND, Json(serde_json::json!({
            "error": { "code": "BOOK_NOT_FOUND", "message": "书籍不存在" }
        })));
    }
    
    let book = state.book_store.get_book(&book_id).await.unwrap();
    
    let agents = vec![
        AgentStatus { name: "天道".to_string(), role: "剧情编排".to_string(), status: AgentState::Idle, current_task: None, is_locked: book.stage != BookStage::Writing },
        AgentStatus { name: "执笔".to_string(), role: "内容撰写".to_string(), status: AgentState::Idle, current_task: None, is_locked: false },
        AgentStatus { name: "世界观守护者".to_string(), role: "规则检查".to_string(), status: AgentState::Idle, current_task: None, is_locked: false },
        AgentStatus { name: "刘和平".to_string(), role: "人物塑造".to_string(), status: AgentState::Idle, current_task: None, is_locked: false },
        AgentStatus { name: "规划者".to_string(), role: "新书规划".to_string(), status: AgentState::Idle, current_task: None, is_locked: book.stage == BookStage::Writing },
        AgentStatus { name: "审阅".to_string(), role: "质量评估".to_string(), status: AgentState::Idle, current_task: None, is_locked: false },
        AgentStatus { name: "观察者".to_string(), role: "知识库管理".to_string(), status: AgentState::Idle, current_task: None, is_locked: false },
        AgentStatus { name: "调研者".to_string(), role: "爆点分析".to_string(), status: AgentState::Idle, current_task: None, is_locked: book.stage != BookStage::Knowledge },
    ];
    
    (StatusCode::OK, Json(serde_json::json!({ "agents": agents, "stage": book.stage })))
}

async fn stream_events(
    Path(book_id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if state.book_store.get_book(&book_id).await.is_none() {
        return (StatusCode::NOT_FOUND, Json(serde_json::json!({
            "error": { "code": "BOOK_NOT_FOUND", "message": "书籍不存在" }
        }))).into_response();
    }
    
    let event_tx = state.get_or_create_event_channel(&book_id).await;
    let mut rx = event_tx.subscribe();
    
    let (sse_tx, sse_rx) = mpsc::channel::<Result<axum::response::sse::Event, axum::Error>>(100);
    
    tokio::spawn(async move {
        loop {
            match rx.recv().await {
                Ok(event) => {
                    if let Ok(json) = serde_json::to_string(&event) {
                        if sse_tx.send(Ok(axum::response::sse::Event::default().data(json))).await.is_err() {
                            break;
                        }
                    }
                }
                Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
            }
        }
    });
    
    Sse::new(tokio_stream::wrappers::ReceiverStream::new(sse_rx))
        .keep_alive(axum::response::sse::KeepAlive::default())
        .into_response()
}

#[derive(Debug, Serialize)]
struct ProviderListResponse {
    providers: Vec<ProviderSummary>,
}

#[derive(Debug, Serialize)]
struct ProviderSummary {
    id: String,
    name: String,
    provider_type: String,
    enabled: bool,
    model_count: usize,
}

async fn list_providers(State(state): State<AppState>) -> impl IntoResponse {
    match state.provider_registry.list().await {
        Ok(providers) => {
            let summaries: Vec<ProviderSummary> = providers
                .into_iter()
                .map(|p| ProviderSummary {
                    id: p.id,
                    name: p.name,
                    provider_type: p.provider_type.to_string(),
                    enabled: p.enabled,
                    model_count: p.model_count,
                })
                .collect();
            (StatusCode::OK, Json(serde_json::to_value(ProviderListResponse { providers: summaries }).unwrap()))
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": { "code": "INTERNAL_ERROR", "message": e.to_string() }
            })))
        }
    }
}

#[derive(Debug, Serialize)]
struct ProviderDetail {
    id: String,
    name: String,
    provider_type: String,
    base_url: String,
    enabled: bool,
    models: Vec<ModelSummary>,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Serialize)]
struct ModelSummary {
    id: String,
    name: String,
    model_id: String,
    enabled: bool,
}

async fn get_provider(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    match state.provider_registry.get_config(&id).await {
        Ok(Some(config)) => {
            let detail = ProviderDetail {
                id: config.id,
                name: config.name,
                provider_type: config.provider_type.to_string(),
                base_url: config.base_url,
                enabled: config.enabled,
                models: config.models.into_iter()
                    .map(|m| ModelSummary {
                        id: m.id,
                        name: m.name,
                        model_id: m.model_id,
                        enabled: m.enabled,
                    })
                    .collect(),
                created_at: config.created_at.to_rfc3339(),
                updated_at: config.updated_at.to_rfc3339(),
            };
            (StatusCode::OK, Json(serde_json::to_value(detail).unwrap()))
        }
        Ok(None) => {
            (StatusCode::NOT_FOUND, Json(serde_json::json!({
                "error": { "code": "PROVIDER_NOT_FOUND", "message": "Provider not found" }
            })))
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": { "code": "INTERNAL_ERROR", "message": e.to_string() }
            })))
        }
    }
}

#[derive(Debug, Deserialize)]
struct CreateProviderRequest {
    name: String,
    provider_type: String,
    base_url: String,
    api_key: String,
    #[serde(default)]
    headers: std::collections::HashMap<String, String>,
    models: Vec<CreateModelRequest>,
}

#[derive(Debug, Deserialize)]
struct CreateModelRequest {
    name: String,
    model_id: String,
    #[serde(default)]
    supports_thinking: bool,
    #[serde(default = "default_context_length")]
    max_context_length: u32,
}

fn default_context_length() -> u32 { 8192 }

async fn create_provider(
    State(state): State<AppState>,
    Json(request): Json<CreateProviderRequest>,
) -> impl IntoResponse {
    let provider_type = match request.provider_type.as_str() {
        "openai" => ProviderType::OpenAI,
        "anthropic" => ProviderType::Anthropic,
        "openai-compatible" => ProviderType::OpenAICompatible,
        _ => ProviderType::Custom,
    };
    
    let provider_id = format!("custom-{}", chrono::Utc::now().timestamp_millis());
    
    let config = ProviderConfig {
        id: provider_id.clone(),
        name: request.name,
        provider_type,
        base_url: request.base_url,
        api_key: request.api_key,
        headers: request.headers,
        default_model: request.models.first().map(|m| m.model_id.clone()),
        models: request.models.into_iter()
            .map(|m| ModelConfig {
                id: m.model_id.clone(),
                name: m.name,
                model_id: m.model_id,
                supports_thinking: m.supports_thinking,
                supports_streaming: true,
                supports_tool_calling: true,
                max_context_length: m.max_context_length,
                max_output_tokens: None,
                default_temperature: 0.7,
                cost_tier: novel_sdk_core::provider::CostTier::Medium,
                enabled: true,
            })
            .collect(),
        enabled: true,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    match state.provider_registry.register(config).await {
        Ok(()) => {
            let version = state.increment_config_version().await;
            state.hot_reload.set_version(version).await;
            state.broadcast_config_update("created", &provider_id).await;
            (StatusCode::CREATED, Json(serde_json::json!({
                "id": provider_id,
                "name": "Custom Provider",
                "enabled": true
            })))
        }
        Err(e) => {
            (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": { "code": "CREATE_FAILED", "message": e.to_string() }
            })))
        }
    }
}

#[derive(Debug, Deserialize)]
struct UpdateProviderRequest {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    enabled: Option<bool>,
    #[serde(default)]
    default_model: Option<String>,
}

async fn update_provider(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(request): Json<UpdateProviderRequest>,
) -> impl IntoResponse {
    use novel_sdk_core::provider::PartialProviderConfig;
    
    let updates = PartialProviderConfig {
        name: request.name,
        enabled: request.enabled,
        default_model: request.default_model,
        ..Default::default()
    };
    
    match state.provider_registry.update(&id, updates).await {
        Ok(()) => {
            let version = state.increment_config_version().await;
            state.hot_reload.set_version(version).await;
            state.broadcast_config_update("updated", &id).await;
            (StatusCode::OK, Json(serde_json::json!({ "success": true })))
        }
        Err(e) => {
            (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": { "code": "UPDATE_FAILED", "message": e.to_string() }
            })))
        }
    }
}

async fn delete_provider(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    match state.provider_registry.unregister(&id).await {
        Ok(()) => {
            let version = state.increment_config_version().await;
            state.hot_reload.set_version(version).await;
            state.broadcast_config_update("deleted", &id).await;
            (StatusCode::OK, Json(serde_json::json!({ "success": true })))
        }
        Err(e) => {
            (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": { "code": "DELETE_FAILED", "message": e.to_string() }
            })))
        }
    }
}

#[derive(Debug, Serialize)]
struct TestProviderResponse {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    available_models: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

async fn test_provider(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    match state.provider_registry.check_availability(&id).await {
        Ok(true) => {
            match state.provider_registry.get_provider(&id).await {
                Ok(Some(provider)) => {
                    match provider.list_models().await {
                        Ok(models) => {
                            (StatusCode::OK, Json(serde_json::to_value(TestProviderResponse {
                                success: true,
                                available_models: Some(models.into_iter().map(|m| m.id).collect()),
                                error: None,
                            }).unwrap()))
                        }
                        Err(e) => {
                            (StatusCode::OK, Json(serde_json::to_value(TestProviderResponse {
                                success: true,
                                available_models: None,
                                error: Some(e.to_string()),
                            }).unwrap()))
                        }
                    }
                }
                _ => {
                    (StatusCode::OK, Json(serde_json::to_value(TestProviderResponse {
                        success: false,
                        available_models: None,
                        error: Some("Provider not found".to_string()),
                    }).unwrap()))
                }
            }
        }
        Ok(false) => {
            (StatusCode::OK, Json(serde_json::to_value(TestProviderResponse {
                success: false,
                available_models: None,
                error: Some("Provider not available".to_string()),
            }).unwrap()))
        }
        Err(e) => {
            (StatusCode::OK, Json(serde_json::to_value(TestProviderResponse {
                success: false,
                available_models: None,
                error: Some(e.to_string()),
            }).unwrap()))
        }
    }
}

#[derive(Debug, Deserialize)]
struct ResolveModelRequest {
    #[serde(default)]
    agent_name: Option<String>,
    #[serde(default)]
    category: Option<String>,
    #[serde(default)]
    provider_id: Option<String>,
    #[serde(default)]
    model_id: Option<String>,
}

#[derive(Debug, Serialize)]
struct ResolveModelResponse {
    provider_id: String,
    model_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    variant: Option<String>,
    is_fallback: bool,
}

async fn resolve_model(
    State(state): State<AppState>,
    Json(request): Json<ResolveModelRequest>,
) -> impl IntoResponse {
    let user_override = match (request.provider_id, request.model_id) {
        (Some(provider_id), Some(model_id)) => Some(UserOverride {
            provider_id,
            model_id,
            variant: None,
        }),
        _ => None,
    };
    
    let input = ResolutionInput {
        agent_name: request.agent_name,
        category: request.category,
        user_override,
        agent_requirements: None,
        category_requirements: None,
    };
    
    match state.model_resolver.resolve(input).await {
        Ok(resolution) => {
            (StatusCode::OK, Json(serde_json::to_value(ResolveModelResponse {
                provider_id: resolution.provider_id,
                model_id: resolution.model_id,
                variant: resolution.variant,
                is_fallback: resolution.is_fallback,
            }).unwrap()))
        }
        Err(e) => {
            (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": { "code": "RESOLUTION_FAILED", "message": e.to_string() }
            })))
        }
    }
}

#[derive(Debug, Deserialize)]
struct GenerateTextRequest {
    provider_id: String,
    model_id: String,
    messages: Vec<ChatMessageInput>,
    #[serde(default)]
    temperature: Option<f32>,
    #[serde(default)]
    max_tokens: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct ChatMessageInput {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct GenerateTextResponse {
    content: String,
    usage: TokenUsageResponse,
}

#[derive(Debug, Serialize)]
struct TokenUsageResponse {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

async fn generate_text(
    State(state): State<AppState>,
    Json(request): Json<GenerateTextRequest>,
) -> impl IntoResponse {
    let messages: Vec<ChatMessage> = request.messages.into_iter()
        .map(|m| {
            let role = match m.role.as_str() {
                "system" => LLMMessageRole::System,
                "assistant" => LLMMessageRole::Assistant,
                _ => LLMMessageRole::User,
            };
            ChatMessage { role, content: m.content }
        })
        .collect();
    
    let chat_request = ChatRequest {
        model: request.model_id,
        messages,
        system: None,
        temperature: request.temperature,
        max_tokens: request.max_tokens,
    };
    
    match state.provider_registry.get_provider(&request.provider_id).await {
        Ok(Some(provider)) => {
            match provider.chat(chat_request).await {
                Ok(response) => {
                    (StatusCode::OK, Json(serde_json::to_value(GenerateTextResponse {
                        content: response.content,
                        usage: TokenUsageResponse {
                            prompt_tokens: response.usage.prompt_tokens,
                            completion_tokens: response.usage.completion_tokens,
                            total_tokens: response.usage.total_tokens,
                        },
                    }).unwrap()))
                }
                Err(e) => {
                    (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                        "error": { "code": "GENERATION_FAILED", "message": e.to_string() }
                    })))
                }
            }
        }
        Ok(None) => {
            (StatusCode::NOT_FOUND, Json(serde_json::json!({
                "error": { "code": "PROVIDER_NOT_FOUND", "message": "Provider not found" }
            })))
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": { "code": "INTERNAL_ERROR", "message": e.to_string() }
            })))
        }
    }
}

#[derive(Debug, Serialize)]
struct ConfigVersionResponse {
    version: u64,
}

async fn get_config_version(State(state): State<AppState>) -> impl IntoResponse {
    Json(ConfigVersionResponse {
        version: state.get_config_version().await,
    })
}

async fn ws_config_handler(
    State(state): State<AppState>,
    ws: WebSocketUpgrade,
) -> Response {
    ws.on_upgrade(move |socket| handle_config_ws(socket, state.hot_reload))
}

async fn handle_config_ws(socket: WebSocket, hot_reload: Arc<HotReloadManager>) {
    let (mut ws_tx, mut ws_rx) = socket.split();
    let client_id = Uuid::new_v4().to_string();
    
    let (tx, mut rx) = mpsc::channel::<String>(100);
    let tx_for_responses = tx.clone();
    hot_reload.add_client(client_id.clone(), tx).await;
    
    let init_msg = hot_reload.create_init_message().await;
    if tx_for_responses.send(init_msg).await.is_err() {
        hot_reload.remove_client(&client_id).await;
        return;
    }
    
    let hot_reload_for_task = hot_reload.clone();
    let client_id_for_task = client_id.clone();
    let recv_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if ws_tx.send(axum::extract::ws::Message::Text(msg)).await.is_err() {
                break;
            }
        }
        hot_reload_for_task.remove_client(&client_id_for_task).await;
    });
    
    while let Some(msg) = ws_rx.next().await {
        match msg {
            Ok(axum::extract::ws::Message::Text(text)) => {
                if let Some(parsed) = HotReloadManager::parse_message(&text) {
                    match parsed.msg_type.as_str() {
                        "ping" => {
                            let pong = HotReloadManager::create_pong_message();
                            if tx_for_responses.send(pong).await.is_err() {
                                break;
                            }
                        }
                        "get_version" => {
                            let version_msg = hot_reload.create_init_message().await;
                            if tx_for_responses.send(version_msg).await.is_err() {
                                break;
                            }
                        }
                        _ => {}
                    }
                }
            }
            Ok(axum::extract::ws::Message::Close(_)) => break,
            Err(_) => break,
            _ => {}
        }
    }
    
    hot_reload.remove_client(&client_id).await;
    recv_task.abort();
}