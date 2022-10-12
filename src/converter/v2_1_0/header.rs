use crate::{error::PostresError, postman::v2_1_0, restclient};

pub(crate) fn convert_headers(
    postman_req: &v2_1_0::RequestClass,
) -> Result<Vec<restclient::Header>, PostresError> {
    if postman_req.header.is_none() {
        return Ok(vec![]);
    }
    let postman_headers = postman_req.header.as_ref().unwrap();
    match postman_headers {
        v2_1_0::HeaderUnion::HeaderArray(headers) => http_header_from_header_array(headers),
        v2_1_0::HeaderUnion::String(header) => http_header_from_string(header),
    }
}

fn http_header_from_string(header: &str) -> Result<Vec<restclient::Header>, PostresError> {
    // postman specification does not clarify what should be done if the header field is a string
    // we assume that it expects a single header in form of name: value.
    // therefore, we will try to parse something in this format.
    let mut h: Vec<_> = header.splitn(2, ":").collect();
    if h.len() < 2 {
        return Err(PostresError::invalid_postman_header(format!(
            "could not parse header {header}"
        )));
    }
    // name will be everything to the left of the first :
    let name = h[0];
    // value will be everything to the right of the first :
    // it could be only a segment, but it could also be a list. Comsider "a: b:c"
    // RUST does not offer a limit on the split function, so that we will have to code it
    let value = h[1].trim_start();

    Ok(vec![restclient::Header {
        name: name.to_string(),
        value: value.to_string(),
    }])
}

fn http_header_from_header_array(
    headers: &Vec<v2_1_0::Header>,
) -> Result<Vec<restclient::Header>, PostresError> {
    Ok(headers
        .iter()
        .filter(|header| {
            matches!(header.disabled, None) || matches!(header.disabled, Some(d) if !d)
        })
        .map(|header| restclient::Header {
            name: header.key.clone(),
            value: header.value.clone(),
        })
        .collect())
}
/*
    see 009
*/
#[cfg(test)]
mod tests {

    use super::super::tests::*;
    use super::*;

    #[test]
    fn should_return_empty_list_of_headers_if_postman_headers_are_not_present() {
        let mut req = default_postman_request_class();
        assert_eq!(convert_headers(&req).unwrap(), vec![]);
    }

    #[test]
    fn should_convert_header_from_string() {
        use restclient::Header;

        assert_eq!(
            http_header_from_string("name: value").unwrap(),
            vec![Header {
                name: "name".to_string(),
                value: "value".to_string()
            }]
        );

        assert_eq!(
            http_header_from_string("name: value:aaa,bbbc").unwrap(),
            vec![Header {
                name: "name".to_string(),
                value: "value:aaa,bbbc".to_string()
            }]
        );
    }

    #[test]
    #[should_panic(expected = "InvalidPostmanHeader")]
    fn should_fail_to_convert_header_from_string_if_string_format_is_invalid() {
        http_header_from_string("name value").unwrap();
    }

    #[test]
    fn should_convert_header_from_header_array() {
        use restclient::Header;

        let postman_headers = vec![
            v2_1_0::Header {
                description: Default::default(),
                disabled: None,
                key: "name1".to_string(),
                value: "value1".to_string(),
            },
            v2_1_0::Header {
                description: Default::default(),
                disabled: Some(false),
                key: "name2".to_string(),
                value: "value2".to_string(),
            },
            v2_1_0::Header {
                description: Default::default(),
                disabled: Some(true),
                key: "disabled1".to_string(),
                value: "should not appear".to_string(),
            },
        ];
        let converted = http_header_from_header_array(&postman_headers).unwrap();
        assert_eq!(
            converted,
            vec![
                Header {
                    name: "name1".to_string(),
                    value: "value1".to_string(),
                },
                Header {
                    name: "name2".to_string(),
                    value: "value2".to_string(),
                },
            ],
        )
    }
}
