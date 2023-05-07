use clap::{Args, Parser, Subcommand};

mod pb;
use pb::health_service_client::HealthServiceClient;
use pb::knoughts_and_crosses_service_client::KnoughtsAndCrossesServiceClient;
use pb::PingRequest;
use tonic::transport::Channel;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct MainCommand {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long, default_value = "http://[::1]:50058")]
    addr: String,
}

impl MainCommand {
    async fn health_client(self) -> Result<HealthServiceClient<Channel>, tonic::transport::Error> {
        HealthServiceClient::connect(self.addr).await
    }
    async fn knoughts_and_crosses_client(
        self,
    ) -> Result<KnoughtsAndCrossesServiceClient<Channel>, tonic::transport::Error> {
        KnoughtsAndCrossesServiceClient::connect(self.addr).await
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Play a game of Noughts and Crosses
    Play(PlayArgs),
    /// Ping the server and receive debugging information
    Ping,
}

#[derive(Args)]
struct PlayArgs {
    #[arg(long)]
    /// The username you wish to play as
    username: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = MainCommand::parse();

    match &args.command {
        Commands::Play(play_args) => {
            println!("play? {}", play_args.username)
        }
        Commands::Ping => {
            let mut client = args.health_client().await?;
            let request = tonic::Request::new(PingRequest {});
            let response = client.ping(request).await?;
            println!("RESPONSE={:?}", response);
        }
    }

    Ok(())
}
