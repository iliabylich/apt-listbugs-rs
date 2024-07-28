use xml::{writer::XmlEvent, EmitterConfig, EventWriter};

pub(crate) struct Envelope {
    writer: EventWriter<Vec<u8>>,
}

impl Envelope {
    pub(crate) fn new(method: &str) -> Self {
        let mut writer = EmitterConfig::new()
            .perform_indent(true)
            .create_writer(vec![]);

        writer
            .write(
                XmlEvent::start_element("soap:Envelope")
                    .attr("xmlns:soap", "http://schemas.xmlsoap.org/soap/envelope/"),
            )
            .unwrap();
        writer.write(XmlEvent::start_element("soap:Body")).unwrap();
        writer
            .write(
                XmlEvent::start_element(method)
                    .attr("xmlns", "http://www.dataaccess.com/webservicesserver/"),
            )
            .unwrap();

        Self { writer }
    }

    pub(crate) fn add_key_value(mut self, key: &str, value: &str) -> Self {
        self.writer.write(XmlEvent::start_element("key")).unwrap();
        self.writer.write(XmlEvent::characters(key)).unwrap();
        self.writer.write(XmlEvent::end_element()).unwrap();

        self.writer.write(XmlEvent::start_element("value")).unwrap();
        self.writer.write(XmlEvent::characters(value)).unwrap();
        self.writer.write(XmlEvent::end_element()).unwrap();

        self
    }

    pub(crate) fn add_node(mut self, tag: &str, content: &str) -> Self {
        self.writer.write(XmlEvent::start_element(tag)).unwrap();
        self.writer.write(XmlEvent::characters(content)).unwrap();
        self.writer.write(XmlEvent::end_element()).unwrap();

        self
    }

    pub(crate) fn add_space(mut self) -> Self {
        self.writer.write(XmlEvent::comment("")).unwrap();
        self
    }

    pub(crate) fn finish(mut self) -> String {
        self.writer.write(XmlEvent::end_element()).unwrap();
        self.writer.write(XmlEvent::end_element()).unwrap();
        self.writer.write(XmlEvent::end_element()).unwrap();

        String::from_utf8(self.writer.into_inner()).unwrap()
    }
}
