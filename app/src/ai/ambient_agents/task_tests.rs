use chrono::Utc;

use super::{AmbientAgentTask, AmbientAgentTaskState, TaskStatusErrorCode, TaskStatusMessage};

fn task_with_name_and_title(name: Option<&str>, title: &str) -> AmbientAgentTask {
    AmbientAgentTask {
        task_id: "550e8400-e29b-41d4-a716-446655440000".parse().unwrap(),
        parent_run_id: None,
        name: name.map(|n| n.to_string()),
        title: title.to_string(),
        state: AmbientAgentTaskState::Queued,
        prompt: "prompt".to_string(),
        created_at: Utc::now(),
        started_at: None,
        updated_at: Utc::now(),
        status_message: None,
        source: None,
        session_id: None,
        session_link: None,
        creator: None,
        executor: None,
        conversation_id: None,
        request_usage: None,
        agent_config_snapshot: None,
        artifacts: vec![],
        is_sandbox_running: false,
        last_event_sequence: None,
        children: vec![],
    }
}

#[test]
fn display_name_prefers_orchestrator_supplied_name() {
    let task = task_with_name_and_title(Some("api"), "Implement REST endpoints for users service");
    assert_eq!(task.display_name(), "api");
}

#[test]
fn display_name_falls_back_to_title_when_name_missing() {
    let task = task_with_name_and_title(None, "Implement REST endpoints for users service");
    assert_eq!(
        task.display_name(),
        "Implement REST endpoints for users service"
    );
}

#[test]
fn display_name_falls_back_to_title_when_name_is_whitespace_only() {
    let task = task_with_name_and_title(Some("   "), "Implement REST endpoints");
    assert_eq!(task.display_name(), "Implement REST endpoints");
}

#[test]
fn display_name_returns_literal_agent_when_name_and_title_blank() {
    let task = task_with_name_and_title(Some("   "), "");
    assert_eq!(task.display_name(), "Agent");
}

#[test]
fn display_name_trims_whitespace_padded_name() {
    let task = task_with_name_and_title(Some("  api  "), "Implement endpoints");
    assert_eq!(task.display_name(), "api");
}

#[test]
fn task_status_error_code_deserializes_public_api_casing() {
    let message: TaskStatusMessage = serde_json::from_str(
        "{\"message\":\"setup failed\",\"error_code\":\"environment_setup_failed\"}",
    )
    .unwrap();

    assert_eq!(
        message.error_code,
        Some(TaskStatusErrorCode::EnvironmentSetupFailed)
    );
    assert!(message.is_environment_setup_failure());
}

#[test]
fn task_status_error_code_deserializes_graphql_casing() {
    let message: TaskStatusMessage = serde_json::from_str(
        "{\"message\":\"setup failed\",\"errorCode\":\"ENVIRONMENT_SETUP_FAILED\"}",
    )
    .unwrap();

    assert_eq!(
        message.error_code,
        Some(TaskStatusErrorCode::EnvironmentSetupFailed)
    );
    assert!(message.is_environment_setup_failure());
}

#[test]
fn task_status_error_code_deserializes_unknown_codes() {
    let message: TaskStatusMessage =
        serde_json::from_str("{\"message\":\"failed\",\"error_code\":\"new_error\"}").unwrap();

    assert_eq!(message.error_code, Some(TaskStatusErrorCode::Unknown));
    assert!(!message.is_environment_setup_failure());
}
