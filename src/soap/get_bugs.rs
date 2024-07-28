use xml::{reader::XmlEvent, EventReader};

use crate::{
    error::AppError,
    log::log,
    soap::{envelope::Envelope, send_soap_req_to_debian_bts},
};

fn make_xml_request<T, I>(packages: I) -> String
where
    T: AsRef<str>,
    I: IntoIterator<Item = T>,
{
    let mut envelope = Envelope::new("get_bugs")
        .add_key_value("severity", "critical")
        .add_key_value("severity", "grave")
        .add_key_value("severity", "serious")
        .add_space()
        .add_key_value("status", "forwarded")
        .add_key_value("status", "open")
        .add_space();

    for package in packages {
        envelope = envelope.add_key_value("package", package.as_ref());
    }

    envelope.finish()
}

#[test]
fn test_make_xml_request() {
    let actual = make_xml_request(["libxml2", "sccache"]);

    let expected = include_str!("../../fixtures/get_bugs-request.xml").trim();

    assert_eq!(actual, expected);
}

fn parse_xml_response(response: impl AsRef<str>) -> Result<Vec<String>, AppError> {
    let mut bugs = vec![];
    for e in EventReader::from_str(response.as_ref()) {
        match e {
            Ok(XmlEvent::Characters(bug)) => {
                bugs.push(bug);
            }
            Err(_) => {
                return Err(AppError::MalformedResponseFromDebianService);
            }
            _ => {}
        }
    }
    Ok(bugs)
}

#[test]
fn test_parse_ok() {
    let xml = include_str!("../../fixtures/get_bugs-response.xml");
    assert_eq!(parse_xml_response(xml), Ok(vec!["1073508".to_string()]))
}
#[test]
fn test_parse_malformed() {
    let xml = "foo bar";
    assert_eq!(
        parse_xml_response(xml),
        Err(AppError::MalformedResponseFromDebianService)
    );
}

pub(crate) fn get_bugs(packages: Vec<String>) -> Result<Vec<String>, AppError> {
    let req = make_xml_request(packages);
    log!("==> {req}");
    let res = send_soap_req_to_debian_bts(req)?;
    log!("<== {res}");
    parse_xml_response(res)
}
