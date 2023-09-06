#![warn(unused_variables)]
use std::error::Error;

use tracing::{info, debug};

use futures::StreamExt;
use async_trait::async_trait;

// use async_openai::config::OpenA/IConfig;
use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    config::OpenAIConfig,
    Client,
};

// use agent_schema::Message;
use crate::llmbase::LLMBase;



#[derive(Debug)]
pub struct OpenAIGPTAPI{
    client: Client<OpenAIConfig>
}
#[async_trait]
impl LLMBase for OpenAIGPTAPI {
    // async fn acompletion(&self) -> String {

    // }
    // async fn ask(&self, msg: &Message) -> String {

    // }
    async fn aask(&self, msg: String) -> String {
        // debug!("OpenAIGPTAPI msg: {}", msg);
        debug!("chat with OpenAIGPTAPI...");
        let data = self.aask(&msg).await.expect("msg is not a message");
        data
    }
}

impl Default for OpenAIGPTAPI {
    fn default() -> Self {
        Self::new()
    }
}

impl OpenAIGPTAPI {

    pub fn new() -> Self {
        let client = Client::new();
        OpenAIGPTAPI{client}
    }

    pub async fn aask(&self, content: &str) -> Result<String, Box<dyn Error>> {
        
        let request = CreateChatCompletionRequestArgs::default()
            .model(std::env::var("OPENAI_API_MODEL").unwrap_or("gpt-3.5-turbo".to_string()))
            // .max_tokens(4096u16)
            .messages([ChatCompletionRequestMessageArgs::default()
                .content(content)
                .role(Role::User)
                .build()?])
            .build()?;

        let mut stream = self.client.chat().create_stream(request).await?;

        // From Rust docs on print: https://doc.rust-lang.org/std/macro.print.html
        //
        //  Note that stdout is frequently line-buffered by default so it may be necessary
        //  to use io::stdout().flush() to ensure the output is emitted immediately.
        //
        //  The print! macro will lock the standard output on each call.
        //  If you call print! within a hot loop, this behavior may be the bottleneck of the loop.
        //  To avoid this, lock stdout with io::stdout().lock():

        // let mut lock = stdout().lock();
        let mut rsp = String::new();
        while let Some(result) = stream.next().await {
            match result {
                Ok(response) => {
                    response.choices.iter().for_each(|chat_choice| {
                        if let Some(ref content) = chat_choice.delta.content {
                            // debug!("{}", content);
                            rsp += content
                        }
                    });
                }
                Err(_err) => {
                    // writeln!(lock, "error: {err}").unwrap();
                }
            }
            // stdout().flush()?;
        }

        info!("rsp: {:?}", rsp);

        Ok(rsp)
    }

}