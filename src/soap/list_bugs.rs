use xml::{reader::XmlEvent, EventReader};

use crate::{
    error::AppError,
    log::log,
    soap::{envelope::Envelope, send_soap_req_to_debian_bts},
};

fn make_xml_request<T, I>(bugs: I) -> String
where
    T: AsRef<str>,
    I: IntoIterator<Item = T>,
{
    let mut envelope = Envelope::new("get_status");

    for bug in bugs {
        envelope = envelope.add_node("bugnumber", bug.as_ref());
    }

    envelope.finish()
}

#[test]
fn test_make_xml_request() {
    let actual = make_xml_request(["1073508"]);

    let expected = include_str!("../../fixtures/list_bugs-request.xml").trim();

    assert_eq!(actual, expected);
}

#[derive(Debug, PartialEq)]
pub(crate) struct Bug {
    pub(crate) id: String,
    pub(crate) package: String,
    pub(crate) severity: String,
    pub(crate) subject: String,
}

fn parse_xml_response(response: impl AsRef<str>) -> Result<Vec<Bug>, AppError> {
    let mut ids = vec![];
    let mut packages = vec![];
    let mut severities = vec![];
    let mut subjects = vec![];

    #[derive(PartialEq)]
    enum PrevWas {
        Id,
        Package,
        Severity,
        Subject,
        None,
    }

    let mut prev_was = PrevWas::None;

    for e in EventReader::from_str(response.as_ref()) {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) if name.local_name == "bug_num" => {
                prev_was = PrevWas::Id;
            }
            Ok(XmlEvent::Characters(id)) if prev_was == PrevWas::Id => {
                ids.push(id);
                prev_was = PrevWas::None;
            }

            Ok(XmlEvent::StartElement { name, .. }) if name.local_name == "package" => {
                prev_was = PrevWas::Package;
            }
            Ok(XmlEvent::Characters(package)) if prev_was == PrevWas::Package => {
                packages.push(package);
                prev_was = PrevWas::None;
            }

            Ok(XmlEvent::StartElement { name, .. }) if name.local_name == "severity" => {
                prev_was = PrevWas::Severity;
            }
            Ok(XmlEvent::Characters(severity)) if prev_was == PrevWas::Severity => {
                severities.push(severity);
                prev_was = PrevWas::None;
            }

            Ok(XmlEvent::StartElement { name, .. }) if name.local_name == "subject" => {
                prev_was = PrevWas::Subject;
            }
            Ok(XmlEvent::Characters(subject)) if prev_was == PrevWas::Subject => {
                subjects.push(subject);
                prev_was = PrevWas::None;
            }

            Err(_) => {
                return Err(AppError::MalformedResponseFromDebianService);
            }
            _ => {}
        }
    }

    let bugs = ids
        .into_iter()
        .zip(packages)
        .zip(severities)
        .zip(subjects)
        .map(|(((id, package), severity), subject)| Bug {
            id,
            package,
            severity,
            subject,
        })
        .collect::<Vec<_>>();
    Ok(bugs)
}

#[test]
fn test_parse_ok() {
    let xml = include_str!("../../fixtures/list_bugs-response.xml");
    assert_eq!(
        parse_xml_response(xml),
        Ok(vec![Bug {
            id: "1073508".to_string(),
            package: "libxml2".to_string(),
            severity: "serious".to_string(),
            subject: "libxml2: just another API+ABI break; please bump soname".to_string()
        }])
    )
}
#[test]
fn test_parse_malformed() {
    let xml = "foo bar";
    assert_eq!(
        parse_xml_response(xml),
        Err(AppError::MalformedResponseFromDebianService)
    );
}

pub(crate) fn list_bugs(bug_numbers: Vec<String>) -> Result<Vec<Bug>, AppError> {
    let req = make_xml_request(&bug_numbers);
    log!("==> {req}");
    let res = send_soap_req_to_debian_bts(req)?;
    log!("<== {res}");

    let bugs = parse_xml_response(res)?;

    if bugs.len() != bug_numbers.len() {
        return Err(AppError::MalformedResponseFromDebianService);
    }

    Ok(bugs)
}
