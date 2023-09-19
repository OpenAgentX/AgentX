use super::agent_manager::{AgentIdAccessor, AgentManager};
use super::system::System;

use super::component::Component;
use async_trait::async_trait;

use anyhow::{Ok, Result};
use reqwest::Client;
use futures;
use tokio::task::JoinError;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::{
    self, runtime,
    sync::{self as tsync, mpsc, Mutex},
};


async fn fetch_url(
    url: &str,
    id: usize,
    shared_data_clone: Arc<Mutex<Vec<String>>>,
    sender: mpsc::Sender<i32>,
) -> Result<String> {
    // loop {
    let response = reqwest::get(url).await?;
    let body;
    // 检查响应状态码
    if response.status().is_success() {
        body = response.text().await?;
        // println!("Response from {}: {}", url, body);
    } else {
        body = String::new();
        // eprintln!("Request to {} failed with status code: {:?}", url, response.status());
    }
    {
        let mut data = shared_data_clone.lock().await;
        // 修改数据
        data.push(body.clone());
        // 打印修改后的数据
        // println!("AgentSystem {} Task {:?}", id, *data);
    }
    // sender.send(id as i32).await?;
    // }
    Ok(body)
}


#[derive(Default, Debug)]
pub struct GlobalMemory {
    pub shared_data: Arc<Mutex<Vec<i32>>>,
}

impl Component for GlobalMemory {}

// 定义一个结构体 MyStruct，其中的属性 data 是泛型
pub struct GlobalMemorySystem<T: Send + Sync> {
    pub data: T,
    pub global_memory_store: Arc<Mutex<Vec<String>>>,
}

#[async_trait]
impl<T> System for GlobalMemorySystem<T>
where
    T: Send + Sync + Debug,
{
    async fn update(
        &mut self,
        manager: &mut AgentManager,
        accessor: &mut AgentIdAccessor,
    ) -> Result<()> {

        let agent_ids = accessor.borrow_ids::<GlobalMemory>(manager).unwrap();

        let connections = agent_ids.len() as usize;

        // start workers by connection number
        let mut jobs = Vec::with_capacity(connections);
        let (tx, mut rx) = mpsc::channel::<i32>(500);

        // start all worker and send request
        for id in agent_ids.iter() {

            let shared_data_clone = Arc::clone(&self.global_memory_store);
            // 修改数据
            // let agent_id = Arc::new(Mutex::new(id));
            let id = id.clone();
            let worker = tokio::spawn(fetch_url(
                "https://httpbin.org/get",
                id,
                shared_data_clone,
                tx.clone(),
            ));

            jobs.push(worker);

        }

        // Wait for all jobs to finish and collect the results
        let results: Vec<Result<String, JoinError> > = futures::future::join_all(jobs)
            .await
            .into_iter()
            .map(|result| result.map(|res| res.unwrap()))
            .collect();

        println!("{}", results.len());
        for res in results {
            println!("{}", res.unwrap());
            // let _ = res.unwrap().await.unwrap();
        }

        Ok(())
    }
}