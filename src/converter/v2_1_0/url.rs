use crate::{error::PostresError, postman::v2_1_0};

use super::extract_path_variables;

pub(crate) fn convert_url(postman_req: &v2_1_0::RequestClass) -> Result<String, PostresError> {
    let postman_url = postman_req
        .url
        .as_ref()
        .ok_or(PostresError::PostmanRequestUrlNotPresent)?;
    let res = match postman_url {
        v2_1_0::Url::String(url) => url,
        v2_1_0::Url::UrlClass(c) => {
            // per postman specification, the raw field should contain the complete url
            c.raw
                .as_ref()
                .ok_or(PostresError::PostmanRequestUrlNotPresent)?
        }
    };
    Ok(extract_path_variables(res))
}
