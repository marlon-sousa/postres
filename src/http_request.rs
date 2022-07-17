/// represents a http file collection, understandable by RestClient
pub struct HttpRequests {
    /// list of requests in collection
    pub requests: Vec<HttpRequest>,
}

/// represent a http request
pub struct HttpRequest {
    /// request name
    pub name: String,
}

impl HttpRequest {
    /// converts a request to rest client format
    pub fn to_restclient(&self) -> String {
        format!("#####\n# @name {}\n", self.name)
    }
}

/*
    see 010
*/
#[cfg(test)]
mod tests {

    use super::*;
    use indoc::indoc;
    #[test]
    fn http_request_is_generated_in_valid_format() {
        let result = indoc! {r#"
            #####
            # @name testReq
        "#};
        let req = HttpRequest {
            name: "testReq".to_string(),
        };
        assert_eq!(req.to_restclient(), result);
    }
}
