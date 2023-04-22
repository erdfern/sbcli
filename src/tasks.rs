use std::{collections::HashMap, fs, path::PathBuf};

use reqwest::header::{AUTHORIZATION, COOKIE};
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Deserialize)]
struct SubmissionResult {
    user: String,
    course: String,
    taskid: usize,
    timestamp: usize,
    content: String, // figure out how to deserialize this
    #[serde(rename = "resultType")]
    result_type: String,
    simplified: Simplified,
    details: HashMap<String, String>,
    score: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Simplified {
    compiler: Compiler,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Compiler {
    stdout: String,
    exitCode: i32,
}

#[derive(Debug, Deserialize)]
struct SubmissionResponse {
    result: SubmissionResult,
    #[serde(skip)]
    #[serde(rename = "newUnlockedAssets")]
    new_unlocked_assets: Vec<String>, // don't know the structure of this object, if it's not just a string
}

pub fn submit_task(task_id: isize, path: PathBuf) -> anyhow::Result<()> {
    let cfg: Config = Config::load()?;
    let url = format!(
        "{}/api/courses/{}/tasks/{}/submissions",
        cfg.host, cfg.course, task_id
    );
    // read task content from path as string
    let contents = fs::read_to_string(path)?;

    let client = reqwest::blocking::Client::new();

    let mut request_body = HashMap::new();
    request_body.insert("submission", &contents);
    let res = client
        .post(url)
        .json(&request_body)
        // .header(
        //     AUTHORIZATION,
        //     format!("Bearer: {}", cfg.token.clone().unwrap()),
        // )
        .header(COOKIE, format!("token={}", cfg.token.unwrap()))
        .send()?;

    if res.status().is_success() {
        // dbg!(&res.text()?);

        let res: SubmissionResponse = res.json()?;
        dbg!(&res);
    } else {
        dbg!(res);
        return Err(anyhow::anyhow!("Response indicates failure"));
    }

    Ok(())
}

pub fn get_tasks() -> anyhow::Result<()> {
    let cfg: Config = Config::load()?;
    let url = format!("{}/api/courses/{}/tasks", cfg.host, cfg.course);

    let client = reqwest::blocking::Client::new();

    let res = client
        .get(url)
        // .header(
        //     AUTHORIZATION,
        //     format!("Bearer: {}", cfg.token.clone().unwrap()),
        // )
        .header(COOKIE, format!("token={}", cfg.token.unwrap()))
        .send()?;

    if res.status().is_success() {
        dbg!(&res.status());
        let body: String = res.text()?;
        dbg!(&body);
    } else {
        return Err(anyhow::anyhow!("Response indicates failure"));
    }

    Ok(())
}