use reqwest::header::COOKIE;

use crate::config::Config;

use super::models::{SubmissionGet, Task};

/// GET /api/courses/{courseId}/tasks
/// Needs to be authenticated
pub fn get_tasks() -> anyhow::Result<Vec<Task>> {
    let cfg = Config::load()?;
    let url = format!("{}/api/courses/{}/tasks", cfg.host, cfg.course);

    let client = reqwest::blocking::Client::new();
    let res = client
        .get(url)
        .header(COOKIE, format!("token={}", cfg.token.unwrap()))
        .send()?;
    let tasks: Vec<Task> = res.json()?;
    Ok(tasks)
}

/// GET /api/courses/{courseId}/tasks/{taskId}/submissions
/// Needs to be authenticated
/// Returns a list of submissions for the given task
pub fn get_submissions(task_id: usize) -> anyhow::Result<Vec<SubmissionGet>> {
    let cfg = Config::load()?;
    let url = format!(
        "{}/api/courses/{}/tasks/{}/submissions",
        cfg.host, cfg.course, task_id
    );

    let client = reqwest::blocking::Client::new();
    let res = client
        .get(url)
        .header(COOKIE, format!("token={}", cfg.token.unwrap()))
        .send()?;

    Ok(res.json()?)
}

/// GET /api/courses/{courseId}/progress
/// Needs to be authenticated
/// Returns a list of task ids that have been solved
pub fn get_progress() -> anyhow::Result<Vec<usize>> {
    let cfg = Config::load()?;
    let url = format!("{}/api/courses/{}/progress", cfg.host, cfg.course);

    let client = reqwest::blocking::Client::new();
    let res = client
        .get(url)
        .header(COOKIE, format!("token={}", cfg.token.unwrap()))
        .send()?;
    let progress: Vec<usize> = res.json()?;
    Ok(progress)
}
