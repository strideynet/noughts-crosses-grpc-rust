use tonic::{transport::Server, Request, Response, Status};

mod built;
mod pb;

#[derive(Debug, Default)]
struct HealthService {}

#[tonic::async_trait]
impl pb::health_service_server::HealthService for HealthService {
    #[tracing::instrument]
    async fn ping(
        &self,
        _: Request<pb::PingRequest>,
    ) -> Result<Response<pb::PingResponse>, Status> {
        let res = pb::PingResponse {
            version: built::PKG_VERSION.to_string(),
        };

        Ok(Response::new(res))
    }
}

#[derive(Debug, Default)]
struct UserService {}

#[tonic::async_trait]
impl pb::user_service_server::UserService for UserService {
    #[tracing::instrument]
    async fn register(
        &self,
        req: Request<pb::RegisterRequest>,
    ) -> Result<Response<pb::RegisterResponse>, Status> {
        tracing::info!("hi");
        let id = uuid::Uuid::new_v4();
        let res = pb::RegisterResponse {
            user_id: id.to_string(),
            token: "bar".to_string(),
        };

        Ok(Response::new(res))
    }
    #[tracing::instrument]
    async fn authenticate(
        &self,
        req: Request<pb::AuthenticateRequest>,
    ) -> Result<Response<pb::AuthenticateResponse>, Status> {
        Err(Status::unimplemented("authenticate not implemented"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let addr: std::net::SocketAddr = "[::1]:50058".parse()?;
    let health_service = HealthService::default();
    let user_service = UserService::default();

    Server::builder()
        .trace_fn(|_| tracing::info_span!("grpc_server"))
        .add_service(pb::health_service_server::HealthServiceServer::new(
            health_service,
        ))
        .add_service(pb::user_service_server::UserServiceServer::new(
            user_service,
        ))
        .serve(addr)
        .await?;

    Ok(())
}
