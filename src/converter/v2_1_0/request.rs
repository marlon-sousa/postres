use crate::{error::PostresError, postman::v2_1_0, restclient};

use super::{
    extract_path_variables, header::convert_headers, method::convert_method, url::convert_url,
};

pub(crate) fn convert_request(
    name: &str,
    item: &v2_1_0::Items,
) -> Result<restclient::HttpRequest, PostresError> {
    let postman_request = item
        .request
        .as_ref()
        .ok_or_else(|| PostresError::postman_request_not_present(""))?;
    match postman_request {
        v2_1_0::RequestUnion::RequestClass(r) => http_request_from_request_class(name, r),
        v2_1_0::RequestUnion::String(r) => http_request_from_string(&name, &r),
    }
}

fn http_request_from_string(
    name: &str,
    req: &str,
) -> Result<restclient::HttpRequest, PostresError> {
    // per postman specification, a single string request is a Get request with the string as url
    let req = extract_path_variables(req);
    Ok(restclient::HttpRequestBuilder::default()
        .name(name.to_string())
        .method(restclient::Method::Get)
        .url(req)
        .build()?)
}

fn http_request_from_request_class(
    name: &str,
    postman_req: &v2_1_0::RequestClass,
) -> Result<restclient::HttpRequest, PostresError> {
    let mut request_builder = restclient::HttpRequestBuilder::default();
    let postman_url = convert_url(postman_req)?;
    request_builder.url(postman_url);
    let method = convert_method(postman_req)?;
    request_builder.method(method);

    let headers = convert_headers(postman_req)?;
    Ok(restclient::HttpRequestBuilder::default().build()?)
}

/*
    see 009
*/
#[cfg(test)]
mod tests {

    use super::super::tests::*;
    use super::*;

    #[test]
    fn should_convert_request_from_string_handling_path_variable() {
        // path variales in requests terminated with path
        let converted =
            http_request_from_string("testReq", "http://127.0.0.1:3000/a/:b/c/:d/e").unwrap();
        assert_eq!(
            converted,
            restclient::HttpRequestBuilder::default()
                .method(restclient::Method::Get)
                .name("testReq")
                .url("http://127.0.0.1:3000/a/{{b}}/c/{{d}}/e")
                .build()
                .unwrap()
        );
        // path variables in requests terminated with /
        let converted =
            http_request_from_string("testReq", "http://127.0.0.1:3000/a/:b/c/:d/").unwrap();
        assert_eq!(
            converted,
            restclient::HttpRequestBuilder::default()
                .method(restclient::Method::Get)
                .name("testReq")
                .url("http://127.0.0.1:3000/a/{{b}}/c/{{d}}/")
                .build()
                .unwrap()
        );
        // path variables in requests terminated with path variables
        let converted =
            http_request_from_string("testReq", "http://127.0.0.1:3000/a/:b/c/:d").unwrap();
        assert_eq!(
            converted,
            restclient::HttpRequestBuilder::default()
                .method(restclient::Method::Get)
                .name("testReq")
                .url("http://127.0.0.1:3000/a/{{b}}/c/{{d}}")
                .build()
                .unwrap()
        );
    }

    #[test]
    fn should_convert_request_from_string() {
        let converted = http_request_from_string("testReq", "http://127.0.0.1:3000/a/b").unwrap();
        assert_eq!(
            converted,
            restclient::HttpRequestBuilder::default()
                .method(restclient::Method::Get)
                .name("testReq")
                .url("http://127.0.0.1:3000/a/b")
                .build()
                .unwrap()
        );
    }

    #[test]
    fn should_convert_request_from_string_handling_query_parameters() {
        let converted =
            http_request_from_string("testReq", "http://127.0.0.1:3000/a/b?aaa=111&bbb=222")
                .unwrap();
        assert_eq!(
            converted,
            restclient::HttpRequestBuilder::default()
                .method(restclient::Method::Get)
                .name("testReq")
                .query_params(vec![
                    restclient::QueryParam::new("aaa", "111"),
                    restclient::QueryParam::new("bbb", "222"),
                ])
                .url("http://127.0.0.1:3000/a/b")
                .build()
                .unwrap()
        );
    }

    #[test]
    #[should_panic(expected = "PostManRequestNotPresent { path: \"\" }")]
    fn should_fail_to_convert_request_if_request_cannot_be_found() {
        let req = default_postman_items();
        convert_request("", &req).unwrap();
    }
}
