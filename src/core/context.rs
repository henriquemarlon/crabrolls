use super::application::Application;
use super::environment::Rollup;
use super::types::{FinishStatus, Input};
use std::error::Error;

pub struct RunOptions {
    pub rollup_url: String,
}

impl Default for RunOptions {
    fn default() -> Self {
        Self {
            rollup_url: String::from("http://127.0.0.1:5004"),
        }
    }
}

impl RunOptions {
    pub fn new(rollup_url: String) -> Self {
        Self { rollup_url }
    }
}

pub async fn run(app: impl Application, options: RunOptions) -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let rollup = Rollup::new(options.rollup_url.clone());

    let status = FinishStatus::Accept;

    println!(
        "Starting the application... Listening for inputs on {}",
        options.rollup_url
    );

    loop {
        let input = rollup.finish_and_get_next(status.clone()).await?;

        match input {
            Some(Input::Advance(advance_input)) => {
                debug!("Advance input: {:?}", advance_input);
                if let Err(e) = app
                    .advance(&rollup, advance_input.metadata, advance_input.payload)
                    .await
                {
                    error!("Error in advance: {}", e);
                    return Err(e);
                }
            }
            Some(Input::Inspect(inspect_input)) => {
                debug!("Inspect input: {:?}", inspect_input);
                if let Err(e) = app.inspect(&rollup, inspect_input.payload).await {
                    error!("Error in inspect: {}", e);
                    return Err(e);
                }
            }
            None => {
                debug!("Waiting for next input");
            }
        }
    }
}
