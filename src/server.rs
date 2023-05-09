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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Claims {
    sub: String,
    iss: String,
    aud: String,
    exp: u64,
    iat: u64,
    nbf: u64,
}

#[tracing::instrument]
fn create_jwt(
    user_id: String,
) -> Result<String, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)?
        .as_secs();
    let ttl = 24 * 60 * 60; // 24 hours in seconds
    let claims = Claims {
        sub: user_id,
        iat: now,
        nbf: now,
        exp: now + ttl,
        // Since we aren't intending for these to be consumed by an external
        // parties, we can set iss and aud to a fixed value.
        // TODO: Eventually allow this to be configured with the base url of
        // the deployment e.g onx.ottr.sh
        iss: "onx".to_string(),
        aud: "onx".to_string(),
    };
    let hdr = jsonwebtoken::Header::default();
    // TODO: Load secret from config
    let key = jsonwebtoken::EncodingKey::from_secret("foo".as_bytes());

    Ok(jsonwebtoken::encode(&hdr, &claims, &key)?)
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
        let id = uuid::Uuid::new_v4();

        let token = create_jwt(id.to_string()).map_err(|err| Status::from_error(err))?;
        let res = pb::RegisterResponse {
            user_id: id.to_string(),
            token: token,
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

    #[tracing::instrument]
    async fn get_user(
        &self,
        req: Request<pb::GetUserRequest>,
    ) -> Result<Response<pb::GetUserResponse>, Status> {
        let user_id = authenticate(req);
        Err(Status::unimplemented("authenticate not implemented"))
    }
}

fn authenticate<T>(req: Request<T>) -> Option<String> {
    let token_string = match req.metadata().get("authorization") {
        Some(t) => t,
        None => return None,
    };

    let decoded: jsonwebtoken::TokenData<Claims> = jsonwebtoken::decode(
        token_string.to_str().unwrap(),
        &jsonwebtoken::DecodingKey::from_secret("foo".as_bytes()),
        &jsonwebtoken::Validation::default(), // TODO: Validate properly
    )
    .unwrap();

    Some(decoded.claims.sub)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::fmt()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .init();

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
