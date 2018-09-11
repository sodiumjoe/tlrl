use failure::Error;
use reqwest::Client;
use serde_json;

header! { (XApiKey, "x-api-key") => [String] }

const MERCURY_URL: &str = "https://mercury.postlight.com/parser";

#[derive(Deserialize, Debug)]
pub struct ParsedDocument {
    pub title: String,
    pub author: Option<String>,
    pub content: String,
    pub domain: Option<String>,
    pub date_published: Option<String>,
}

pub fn parse(uri: &str, key: String) -> Result<ParsedDocument, Error> {
    let client = Client::builder().build()?;

    let mut request_uri = String::from(MERCURY_URL);
    request_uri.push_str("?url=");
    request_uri.push_str(uri);
    debug!("{:?}", request_uri);

    let mut req = client.get(request_uri.as_str());
    req.header(XApiKey(key.to_string()));
    // TODO: redact api key
    debug!("{:?}", req);
    let mut res = req.send()?;
    debug!("{:?}", res);
    let text = res.text()?;
    debug!("{:?}", text);
    let json: ParsedDocument = serde_json::from_str(text.as_str())?;
    Ok(json)
}
