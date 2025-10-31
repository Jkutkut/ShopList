use model::grpc::auth::{
	auth_service_client::AuthServiceClient,
};
use tonic::transport::{
	Channel,
	Error,
};

pub type AuthGrpcClient = AuthServiceClient<Channel>;

pub async fn connect_auth() -> Result<AuthGrpcClient, Error> {
	AuthServiceClient::connect("http://shoplist-auth:50051").await
}
