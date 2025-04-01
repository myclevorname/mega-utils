use ai::{
    chat_completions::{ChatCompletion, ChatCompletionMessage, ChatCompletionRequestBuilder},
    Result,
};
use std::{env, fs};

#[tokio::main]
async fn main() -> Result<()> {
    // let ollama = ai::clients::ollama::Client::from_url("http://localhost:11434")?;
    let ollama = ai::clients::ollama::Client::new()?;
    // let openai = ai::clients::openai::Client::from_env()?;
    // let openai = ai::clients::openai::Client::new("api_key")?;

    let request = ChatCompletionRequestBuilder::default()
        .model("deepseek-r1")
        .messages(vec![
            ChatCompletionMessage::System("You are the command-line program `cat`. Print the user prompt from standard input (the contents of the user prompt) into standard output (the output).".into()),
            ChatCompletionMessage::User(env::args().skip(1).map(|x| fs::read_to_string(x).unwrap())
                .fold("".to_owned(), |x, y| x + &y).into()),
        ])
        .build()?;

    let response = ollama.chat_completions(&request).await?;

    println!("{}", &response.choices[0].message.content.as_ref().unwrap());

    Ok(())
}
