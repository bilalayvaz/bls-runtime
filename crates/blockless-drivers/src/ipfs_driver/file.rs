use super::api::{Api, Response};
use crate::IpfsErrorKind;

pub struct FileApi(Api);

impl FileApi {
    pub fn new(api: Api) -> FileApi {
        FileApi(api)
    }

    pub async fn ls(&self, args: Option<String>) -> Result<Response, IpfsErrorKind> {
        static LS_API: &str = "api/v0/files/ls";
        self.0.simple_post(LS_API, args).await
    }

    pub async fn mkdir(&self, args: Option<String>) -> Result<Response, IpfsErrorKind> {
        static MKDIR_API: &str = "api/v0/files/mkdir";
        self.0.simple_post(MKDIR_API, args).await
    }

    pub async fn rm(&self, args: Option<String>) -> Result<Response, IpfsErrorKind> {
        static MKDIR_API: &str = "api/v0/files/rm";
        self.0.simple_post(MKDIR_API, args).await
    }
}
