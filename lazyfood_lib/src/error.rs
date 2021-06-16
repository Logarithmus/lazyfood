use thiserror::Error;

#[derive(Error, Debug)]
pub enum OrderError {
	#[error("Failed to place an order")]
	Failed,
}

#[derive(Error, Debug)]
pub enum MenuError {
	#[error("Failed to fetch menu")]
	Failed,
	#[error("reqwest error")]
	ReqwestErr(#[from] reqwest::Error),
}
