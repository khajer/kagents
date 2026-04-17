use axum::{extract::State, http::StatusCode, response::Json};

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tokio::fs;
use tracing::{error, info};

use crate::db_func;
use crate::call_agent;


const AGENTS_FOLDER: &str = "workspace";
const MSG_SUCCESS: &str = "Agent created successfully";

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct Agent {
    pub id: i64,
    pub name: String,
    pub token: String,
    pub model: String,
    pub status: String,
    pub created_at: String,
}

impl std::fmt::Display for Agent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ID: {} | Name: {} | Model: {} | Created: {}",
            self.id, self.name, self.model, self.created_at
        )
    }
}

#[derive(Serialize)]
pub(crate) struct ListResponse {
    agents: Vec<Agent>,
}

#[derive(Serialize)]
pub(crate) struct AgentResponse {
    agent: Agent,
}


#[derive(Deserialize)]
pub(crate) struct CreateAgent {
    pub(crate) name: String,
    pub(crate) token: String,
    pub(crate) model: String,
    pub(crate) status: String,
}

#[derive(Serialize)]
pub(crate) struct CreateAgentResponse {
    id: i64,
    message: String,
}

#[derive(Serialize)]
pub(crate) struct ErrorResponse {
    error: String,
}

#[derive(Deserialize)]
pub(crate) struct DataAgent {
    id: i64,
    prompt : String
}

#[derive(Serialize)]
pub(crate) struct RemoveAgentResponse {
    message: String,
}

pub async fn list_handler(State(pool): State<SqlitePool>) -> Result<Json<ListResponse>, (StatusCode, Json<ErrorResponse>)> {
    match db_func::list_agents(&pool).await {
        Ok(agents) => Ok(Json(ListResponse { agents })),
        Err(e) => {
            error!("Failed to list agents: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to list agents: {}", e),
                }),
            ))
        }
    }
}

pub async fn prompt_handler(State(pool): State<SqlitePool>, Json(payload): Json<DataAgent>) -> Result<Json<AgentResponse>, (StatusCode, Json<ErrorResponse>)> {
    let agent_result = db_func::get_agent_by_id(&pool, payload.id).await;


    match agent_result {
        Ok(Some(agent)) => {
            let agent_brand = "openai";

            if agent_brand == "openai" {
                match call_agent::call_openai(&payload.prompt, &agent.token, &agent.model).await {
                    Ok(response) => {
                        info!("OpenAI response: {}", response);
                    }
                    Err(e) => {
                        error!("Failed to call OpenAI: {}", e);
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(ErrorResponse {
                                error: format!("Failed to call OpenAI: {}", e),
                            }),
                        ));
                    }
                }
            } else {
                match call_agent::call_anthropic(&payload.prompt, &agent.token, &agent.model).await {
                    Ok(response) => {
                        info!("Anthropic response: {}", response);
                    }
                    Err(e) => {
                        error!("Failed to call Anthropic: {}", e);
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(ErrorResponse {
                                error: format!("Failed to call Anthropic: {}", e),
                            }),
                        ));
                    }
                }
            }

            Ok(Json(AgentResponse { agent }))
        }
        Ok(None) => {
            Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: format!("Agent {} not found", payload.id),
                }),
            ))
        }
        Err(e) => {
            error!("Failed to query agent: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to query agent: {}", e),
                }),
            ))
        }
    }


}

pub async fn process_handler(State(pool): State<SqlitePool>) -> Result<Json<ListResponse>, (StatusCode, Json<ErrorResponse>)> {
    match db_func::list_agents(&pool).await {
        Ok(agents) => Ok(Json(ListResponse { agents })),
        Err(e) => {
            error!("Failed to process agents: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to list agents: {}", e),
                }),
            ))
        }
    }
}
pub async fn add_agent_handler(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateAgent>,
) -> Result<Json<CreateAgentResponse>, (StatusCode, Json<ErrorResponse>)> {

    let result = db_func::insert_agent(&pool, &payload).await;

    match result {
        Ok(query_result) => {

            gen_agent_folder(&payload).await;

            Ok(Json(CreateAgentResponse {
                id: query_result.last_insert_rowid(),
                message: MSG_SUCCESS.to_string(),
            }))
        }
        Err(e) => {
            error!("Failed to create agent: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to create agent: {}", e),
                }),
            ))
        }
    }
}

pub async fn remove_agent_handler(
    State(pool): State<SqlitePool>,
    Json(payload): Json<DataAgent>,
) -> Result<Json<RemoveAgentResponse>, (StatusCode, Json<ErrorResponse>)> {
    let agent_result = db_func::get_agent_by_id(&pool, payload.id).await;

    match agent_result {
        Ok(Some(agent)) => {
            // Delete the folder & file
            let folder_path = format!("./{}/{}", AGENTS_FOLDER, agent.name);
            if let Err(e) = fs::remove_dir_all(&folder_path).await {
                error!("Failed to remove folder for agent {}: {}", agent.name, e);
            } else {
                info!("Removed folder for agent: {}", folder_path);
            }

            let delete_result = db_func::delete_agent_by_id(&pool, payload.id).await;
            match delete_result {
                Ok(_) => {
                    Ok(Json(RemoveAgentResponse {
                        message: format!("Agent {} removed successfully", payload.id),
                    }))
                }
                Err(e) => {
                    error!("Failed to remove agent from database: {}", e);
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            error: format!("Failed to remove agent: {}", e),
                        }),
                    ))
                }
            }
        }
        Ok(None) => {
            Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: format!("Agent {} not found", payload.id),
                }),
            ))
        }
        Err(e) => {
            error!("Failed to query agent: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to query agent: {}", e),
                }),
            ))
        }
    }
}



async fn gen_agent_folder(payload: &CreateAgent) {
    let folder_path = format!("./{}/{}", AGENTS_FOLDER, payload.name);
    if let Err(e) = fs::create_dir_all(&folder_path).await {
        error!("Failed to create folder for agent {}: {}", payload.name, e);
    } else {
        info!("Created folder for agent: {}", folder_path);
    }

    let created_at = Utc::now().to_rfc3339();
    let readme_content = format!(r#"# {}
- model : {},
- created : {}
"#, payload.name, payload.model, &created_at
    );

    let readme_path = format!("{}/readme.md", folder_path);
    if let Err(e) = fs::write(&readme_path, &readme_content).await {
        error!("Failed to create readme for agent {}: {}", payload.name, e);
    } else {
        info!("Created readme file for agent: {}", readme_path);
    }

}
