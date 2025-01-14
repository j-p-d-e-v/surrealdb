use crate::dbs::DB;
use crate::err::Error;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use http_body::Body as HttpBody;
use surrealdb::kvs::{LockType::*, TransactionType::*};

pub(super) fn router<S, B>() -> Router<S, B>
where
	B: HttpBody + Send + 'static,
	S: Clone + Send + Sync + 'static,
{
	Router::new().route("/health", get(handler))
}

async fn handler() -> impl IntoResponse {
	// Get the datastore reference
	let db = DB.get().unwrap();
	// Attempt to open a transaction
	match db.transaction(Read, Optimistic).await {
		// The transaction failed to start
		Err(_) => Err(Error::InvalidStorage),
		// The transaction was successful
		Ok(tx) => {
			// Cancel the transaction
			trace!("Health endpoint cancelling transaction");
			// Attempt to fetch data
			match tx.get(vec![0x00]).await {
				Err(_) => {
					// Ensure the transaction is cancelled
					let _ = tx.cancel().await;
					// Return an error for this endpoint
					Err(Error::InvalidStorage)
				}
				Ok(_) => {
					// Ensure the transaction is cancelled
					let _ = tx.cancel().await;
					// Return success for this endpoint
					Ok(())
				}
			}
		}
	}
}
