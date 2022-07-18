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
    def 012
    We want to use a builder pattern on our HttpRequest struct.
    This is needed because the struct will not be initialized at once.
    We want to set url, headers, body, query params and others in phases, because each of these elements will be converted by a specialized function.
    The builder pattern creates a temporary (builder) struct, where everything is Optional (can at any time be Some(value) or None, meaning that it hasn't yet been initialized).
    When we are ready to build the real structure, we call the build() method on the builder structure.
    This method checks that every required pfields are set (with Some(value)), and unwraps them, building the structure.
    Obviously, if any of the required fields are still set as None, an error will be thrown.
    We could implement all this by hand, but there is a nice crate called derive_builder which, through procedure macros, generate all the code for us.
    There are though two fields which can't use the default setters from the macro.
    Explanations on why and what we need to do on the custom setters
*/
/// represent a http request
#[derive(Builder, Debug, PartialEq, Eq)]
#[builder(setter(into), build_fn(error = "PostresError"))]
pub struct HttpRequest {
    /// headers
    #[builder(default)]
    headers: Vec<Header>,
    /// method
    method: Method,
    /// request name
    name: String,
    /// query parameters
    #[builder(default)]
    #[builder(setter(custom))]
    query_params: Vec<QueryParam>,
    /// url
    #[builder(setter(custom))]
    url: String,
}

impl HttpRequestBuilder {
    pub fn url(&mut self, mut value: impl ToString) -> &mut Self {
        /*
            def 013
            Postman urls can come with query strings appended (http://host/path?querystring)
            We want to extract the query string parameters and let the url without them as the request url, mainly because RestClient allows for a nicer way to represent query parameters
            This means that we can't take a string the way it is and store in the url field, this default *** and reasonable *** behavior would be available for us cortesy of the derive_builder crate
            We will use the url crate to extract the query params (if they exist) and fill the vecor of query parameters of this request appropriately
        */

        // try to find the first "?" on url
        let mut value = value.to_string();
        if let Some(p) = value.find("?") {
            // there is a "?", because the pattern matched Some(p). Parsing the url
            let url = Url::parse(&value).unwrap();
            // with a copy of the url parsed, truncate the original url to cut the query string part.
            // the p wrapped in Some contains the position of the found "?"
            value.truncate(p);
            // now, there are two possibilities:
            // 1. Some one already initialized the vector of query parameters on this builder.
            // 2. Nobody initialized the vector of query parameters.
            // If this vector is not still initialized, we will do it now, because we know we have some query parameters to add to the request
            if self.query_params.is_none() {
                self.query_params = Some(vec![]);
            }
            /*
                def 014
                Rust ownership model dictates that if you are changing something through a shared reference (like we are, see the &mut self) in this function parameters list) you can do whatever you want, except moving an object field and leaving nothing in place.
                We need to change the vector of query parameters. It has the value Some(val), and in order to change its value we need to get the vec wrapped by Some.
                The most obvious thing to do would be call unwrap(), but unwrap() invalidates the option by moving the val wrapped in Some() to a variable you own or panicking if the Option is set to None
                this means that if we make let params = self.query_params.unwrap(), we moved the self.query_params into a variable called params that we own ... and left nothing in place.
                What to do so?
                We still need the vector behind Some to push our new query params.
                Well, Option has a method called take. This method replaces the option in memory by another option, always set to None, and gives us the ownership of the Option previously on that memory spot.
                Now, we can unwrap (the option was under our ownership, the vec inside it will also be), push to the vector and then, when we are set, update again the self.query_params field with Some(our_vec)
            */
            let mut params = self.query_params.take().unwrap();
            for (k, v) in url.query_pairs() {
                params.push(QueryParam::new(k, v));
            }
            self.query_params = Some(params);
        }
        self.url = Some(value);
        self
    }

    pub fn query_params(&mut self, value: Vec<QueryParam>) -> &mut Self {
        if self.query_params.is_some() {
            let mut params = self.query_params.take().unwrap();
            params.extend(value);
            self.query_params = Some(params);
            return self;
        }
        self.query_params = Some(value);
        self
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

#[derive(Clone, Debug, Display, PartialEq, Eq)]
#[strum(serialize_all = "UPPERCASE")]
pub enum Method {
    Delete,
    Get,
    Head,
    Post,
    Put,
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
