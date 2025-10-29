use model::grpc::auth::{
	auth_service_client::AuthServiceClient,
};
use tonic::transport::Channel;

pub async fn connect_auth() -> Result<AuthServiceClient<Channel>, tonic::transport::Error> {
	AuthServiceClient::connect("http://shoplist-auth:50051").await
}
