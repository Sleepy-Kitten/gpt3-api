use serde::{Deserialize, Serialize};
use crate::api::{Action, Auth};
use crate::OPENAI_URL;
use crate::client::NormalRequest;

/// # OpenAi documentation
/// 
/// Delete a file.
#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    pub file_id: String,
}
impl Request {
    pub fn new(file_id: String) -> Self {
        Request { file_id }
    }
}
impl Action for Request {
    type Response = Response;

    fn build_request(&self, client: &crate::Client) -> crate::RequestBuilder {
        client
            .reqwest_client()
            .delete(format!("{OPENAI_URL}/files/{}", self.file_id))
            .auth(client.gpt_token())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    /// The file id used to identify the file
    pub id: String,
    /// The object of the request
    pub object: String,
    /// Whether the deletion was successful or not
    pub deleted: bool,
}

impl NormalRequest for Request {}
