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
    pub url: Option<String>,
}

pub fn parse(uri: &str, key: String) -> Result<ParsedDocument, Error> {
    let client = Client::builder().build()?;

    let mut request_uri = String::from(MERCURY_URL);
    request_uri.push_str("?url=");
    request_uri.push_str(uri);
    debug!("{:?}", request_uri);

    let mut req = client.get(request_uri.as_str());
    req.header(XApiKey(key.to_string()));
    let req_string = format!("{:?}", req).as_str().replace(&key, "[redacted]");
    debug!("{}", req_string);
    let mut res = req
        .send()
        .map_err(|err| format_err!("Error sending request to mercury api: {}", err))?;
    debug!("{:?}", res);
    let text = res
        .text()
        .map_err(|err| format_err!("Error getting text from mercury api response: {}", err))?;
    debug!("{:?}", text);
    let json: ParsedDocument = serde_json::from_str(text.as_str())
        .map_err(|err| format_err!("Error deserializing mercury api response json: {}", err))?;
    Ok(json)
}
