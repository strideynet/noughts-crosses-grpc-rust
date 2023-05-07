use std::{env, error::Error, path};

use clap::{Args, Parser, Subcommand};
use tonic::transport::{self, Channel};

mod pb;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct MainCommand {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long, default_value = "http://[::1]:50058")]
    addr: String,
}

impl MainCommand {
    async fn health_client(
        &self,
    ) -> Result<pb::health_service_client::HealthServiceClient<Channel>, transport::Error> {
        pb::health_service_client::HealthServiceClient::connect(self.addr.clone()).await
    }
    async fn user_client(
        &self,
    ) -> Result<pb::user_service_client::UserServiceClient<Channel>, transport::Error> {
        pb::user_service_client::UserServiceClient::connect(self.addr.clone()).await
    }
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Play a game of Noughts and Crosses
    Play(PlayArgs),
    /// Register with the game server
    Register(RegisterArgs),
    /// Ping the server and receive debugging information
    Ping,
}

#[derive(Args, Debug)]
struct PlayArgs {}

#[derive(Args, Debug)]
struct RegisterArgs {
    #[arg(long)]
    /// The username to register as
    username: String,
    #[arg(long)]
    /// The password to register with
    password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct PersistedState {
    token: String,
}

impl PersistedState {
    fn path() -> Result<path::PathBuf, Box<dyn Error>> {
        let home_path: path::PathBuf;
        // TODO: Not use deprecated home_dir
        match env::home_dir() {
            Some(path) => home_path = path,
            None => {
                return Err("unable to determine home dir".into());
            }
        }

        Ok(home_path.join(".onx-state.json"))
    }
    fn load() -> Result<PersistedState, Box<dyn Error>> {
        let path = PersistedState::path()?;
        let bytes = std::fs::read(path)?;
        Ok(serde_json::from_slice(&bytes)?)
    }
    fn save(&self) -> Result<(), Box<dyn Error>> {
        let path = PersistedState::path()?;
        let bytes = serde_json::to_vec(self)?;
        std::fs::write(path, bytes)?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let args = MainCommand::parse();
    tracing::info!("parsed args");

    match &args.command {
        Commands::Play(_play_args) => {
            println!("play? {}", "epp");
        }
        Commands::Register(register_args) => {
            println!("Registering as {}...", register_args.username);
            let mut client = args.user_client().await?;
            let req = tonic::Request::new(pb::RegisterRequest {
                username: register_args.username.clone(),
                password: register_args.password.clone(),
            });
            let res: tonic::Response<pb::RegisterResponse> = client.register(req).await?;

            let state = PersistedState {
                token: res.into_inner().token,
            };
            state.save()?;

            println!("Registered! You can now start playing.");
        }
        Commands::Ping => {
            let mut client = args.health_client().await?;
            let req = tonic::Request::new(pb::PingRequest {});
            let res: tonic::Response<pb::PingResponse> = client.ping(req).await?;
            println!("RESPONSE={:?}", res);
        }
    }

    Ok(())
}
