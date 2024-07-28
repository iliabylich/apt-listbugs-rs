mod envelope;

mod get_bugs;
pub(crate) use get_bugs::get_bugs;

mod list_bugs;
pub(crate) use list_bugs::list_bugs;

use crate::error::AppError;

pub(crate) fn send_soap_req_to_debian_bts(req: String) -> Result<String, AppError> {
    let res = ureq::post("http://bugs.debian.org/cgi-bin/soap.cgi")
        .set("Content-Type", "text/xml")
        .send(std::io::Cursor::new(req))
        .map_err(|error| AppError::FailedToAccessDebianService {
            kind: error.kind(),
            response: error.into_response().and_then(|res| res.into_string().ok()),
        })?;

    res.into_string()
        .map_err(|_| AppError::MalformedResponseFromDebianService)
}
