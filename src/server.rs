use tonic::{transport::Server, Request, Response, Status};

mod pb;
use pb::health_service_server;
use pb::knoughts_and_crosses_service_server;
use pb::{PingRequest, PingResponse};

mod built;

#[derive(Debug, Default)]
struct HealthService {}

#[tonic::async_trait]
impl health_service_server::HealthService for HealthService {
    async fn ping(&self, _: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        let res = PingResponse {
            version: built::PKG_VERSION.to_string(),
        };

        Ok(Response::new(res))
    }
}

#[derive(Debug, Default)]
struct KnoughtsAndCrossesService {}

impl knoughts_and_crosses_service_server::KnoughtsAndCrossesService for KnoughtsAndCrossesService {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let addr: std::net::SocketAddr = "[::1]:50058".parse()?;
    let health_service = HealthService::default();
    let knoughts_and_crosses_service = KnoughtsAndCrossesService::default();
    Server::builder()
        .add_service(
            health_service_server::HealthServiceServer::with_interceptor(
                health_service,
                log_interceptor,
            ),
        )
        .add_service(
            knoughts_and_crosses_service_server::KnoughtsAndCrossesServiceServer::with_interceptor(
                knoughts_and_crosses_service_server,
                log_interceptor,
            ),
        )
        .serve(addr)
        .await?;

    Ok(())
}

fn log_interceptor(req: Request<()>) -> Result<Request<()>, Status> {
    log::trace!("{:?}", req);

    Ok(req)
}
