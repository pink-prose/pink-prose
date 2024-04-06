use super::*;

pub trait ServerAuthenticatedRequest: Sized {
	type Error: From<Error>;

	fn receive_request(&mut self) -> fut!(AuthenticatedRequest);
	fn get_stored_session_info(&mut self, session_id: &SessionID) -> fut!(SessionServerInfo);
	fn finalise_invalid_signature(self) -> fut!(());
	fn perform_request(&mut self, body: &[u8]) -> fut!(AuthenticatedResponse);
	// TODO: likely some kind of "return response" fn here, after any processing we might want to do?
	fn finalise(self) -> fut!(());

	fn run(mut self) -> sealed_fut!(()) {
		seal!(async move {
			let AuthenticatedRequest {
				session_id,
				body_signature,
				body
			} = self.receive_request().await?;

			let SessionServerInfo {
				session_id: _,
				session_public_key
			} = self.get_stored_session_info(&session_id).await?;

			let valid = session_public_key.verify(&body, &body_signature);
			if !valid { return self.finalise_invalid_signature().await }

			// TODO figure this out? body? whatever stuff
			let response = self.perform_request(&body).await?;

			self.finalise().await
		})
	}
}