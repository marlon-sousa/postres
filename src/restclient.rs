use derive_builder::Builder;
use strum::Display;
use url::Url;

use crate::error::PostresError;

/// represents a http file collection, understandable by RestClient
pub struct HttpRequests {
    /// list of requests in collection
    pub requests: Vec<HttpRequest>,
}

/*
    def 011: builder structs with derive_builder
    We want to use a builder pattern on our HttpRequest struct.
    This is needed because the struct will not be initialized at once.
    We want to set url, headers, body, query params and others in phases, because each of these elements will be converted by a specialized function.
    The builder pattern creates a temporary (builder) struct, where everything is Optional (can at any time be Some(value) or None, meaning that it hasn't yet been initialized).
    When we are ready to build the real structure, we call the build() method on the builder structure.
    This method checks that every required fields are set (with Some(value)), and unwraps them, building the structure.
    Obviously, if any of the required fields are still set as None, an error will be thrown.
    We could implement all this by hand, but there is a nice crate called derive_builder which, through procedure macros, generate all the code for us.
    We also have a chance to perform a post build operation, something we want to do on the build struct as soon as it finishes being built, but before returning it for use.
    Greater explanations of what is being done in this method can be located at 012
*/
/// represent a http request
#[derive(Builder, Debug, PartialEq, Eq)]
#[builder(
    setter(into),
    build_fn(post_build = "Self::post_build", error = "PostresError")
)]
pub struct HttpRequest {
    /// headers
    #[builder(default)]
    headers: Vec<Header>,
    /// method
    method: Method,
    /*
    /// request body
    body: Body,
    */
    /// request name
    name: String,
    /// query parameters
    #[builder(default)]
    query_params: Vec<QueryParam>,
    /// url
    url: String,
}

impl HttpRequestBuilder {
    pub fn post_build(instance: &mut HttpRequest) -> Result<(), String> {
        /*
            def 012: post build function
            This function executs as soon as an instance of HttpRequest is built, but before the HttpRequestBuilder build method returns.
            We have then a chance to do some post build adjustments on the instance just built.

            Note: Some will argue that post build functions should be considered under the domain of the target struct, not of the builder struct.
            While this makes sense, the notion that post build functions are executed by the builder before returning the built instance also leads us to think that this could be executed in the domain of the builder
            There is no right answer. As far as we are concerned, we think that this fits more on the builder domain, as parameters are being adjusted before the instance can be finally used.

            Urls can come with query strings appended (http://host/path?querystring)
            We want to extract the query string parameters and let the url without them as the request url, mainly because RestClient allows for a nicer way to represent query parameters
            This means that we might need to change the url field. If it contains query strings, we will need to remove them and store them im the query_params field.
            We will use the url crate to extract the query params (if they exist) and fill the vecor of query parameters of this request appropriately
        */

        // try to find the first "?" on url

        if let Some(p) = instance.url.find("?") {
            // there is a "?", because the pattern matched Some(p). Parsing the url
            let url = Url::parse(&instance.url).unwrap();
            // with a copy of the url parsed, truncate the original url to cut the query string part.
            // the p wrapped in Some contains the position of the found "?"
            // we can change the url field because we have a &mut reference.
            instance.url.truncate(p);

            // add the query params extracted from the url to the request query params
            for (k, v) in url.query_pairs() {
                instance.query_params.push(QueryParam::new(k, v));
            }
        }
        Ok(())
    }
}

impl HttpRequest {
    /// converts a request to rest client format
    pub fn to_restclient(&self) -> String {
        let mut builder = string_builder::Builder::default();
        builder.append(format!("#####\n# @name {}\n\n", self.name));
        builder.append(format!(
            "{} {} http/1.1\n",
            &self.method.to_string(),
            &self.url
        ));
        builder.string().unwrap()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Header {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QueryParam {
    name: String,
    value: String,
}

impl QueryParam {
    pub fn new(name: impl ToString, value: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Body {
    Empty,
    FileSource(String),
    FormData(Vec<FormDataParamSpec>),
    Graphql(GraphqlSpec),
    Raw(String),
    UrlEncoded(Vec<QueryParam>),
}

impl Default for Body {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Clone, Debug, Display, PartialEq, Eq)]
#[strum(serialize_all = "UPPERCASE")]
pub enum Method {
    Copy,
    Delete,
    Get,
    Head,
    Link,
    Lock,
    Options,
    Patch,
    Post,
    Propfind,
    Purge,
    Put,
    Unlink,
    Unlock,
    View,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GraphqlSpec {
    pub spec: String,
    pub variables: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FormDataParamSpec {
    pub content_type: Option<String>,
    pub name: String,
    pub value: FormParamValue,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FormParamValue {
    File(Vec<String>),
    Text(String),
}

/*
    see 009
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

            GET http://127.0.0.1:3000/a/b http/1.1
        "#};
        let req = HttpRequestBuilder::default()
            .name("testReq")
            .method(Method::Get)
            .url("http://127.0.0.1:3000/a/b")
            .build()
            .unwrap();

        assert_eq!(req.to_restclient(), result);
    }
}
