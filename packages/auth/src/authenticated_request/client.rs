use super::*;

pub trait ClientAuthenticatedRequest: Sized {
	type Error: From<Error>;

	fn get_stored_session_info(&mut self) -> fut!(SessionClientInfo);
	fn get_request_body(&mut self) -> fut!(Vec<u8>);
	fn submit_request(&mut self, request: &AuthenticatedRequest) -> fut!(AuthenticatedResponse);
	fn finalise(self) -> fut!(());

	fn run(self) -> sealed_fut!(AuthenticatedResponse) {
		seal!(self, |mut client| async move {
			let SessionClientInfo {
				session_id,
				session_secret_key
			} = client.get_stored_session_info().await?;

			let body = client.get_request_body().await?;
			let body_signature = session_secret_key.sign(&body);

			let request = AuthenticatedRequest {
				session_id,
				body_signature,
				body
			};
			let response = client.submit_request(&request).await?;

			client.finalise().await?;
			Ok(response)
		})
	}
}
