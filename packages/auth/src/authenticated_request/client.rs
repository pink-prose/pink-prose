use super::*;

pub trait ClientAuthenticatedRequest: Sized {
	type Error: From<Error>;

	fn get_stored_session_info(&mut self) -> fut!(SessionClientInfo);
	fn get_request_body(&mut self) -> fut!(Vec<u8>);
	fn submit_request(&mut self, request: &AuthenticatedRequest) -> fut!(AuthenticatedResponse);
	fn finalise(self) -> fut!(());

	fn run(mut self) -> sealed_fut!(AuthenticatedResponse) {
		seal!(async move {
			let SessionClientInfo {
				session_id,
				session_secret_key
			} = self.get_stored_session_info().await?;

			let body = self.get_request_body().await?;
			let body_signature = session_secret_key.sign(&body);

			let request = AuthenticatedRequest {
				session_id,
				body_signature,
				body
			};
			let response = self.submit_request(&request).await?;

			self.finalise().await?;
			Ok(response)
		})
	}
}
