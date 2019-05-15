use base64;
use failure::Error;
use html5ever::{
    parse_document,
    rcdom::{Handle, NodeData::Element, RcDom},
    tendril::{Tendril, TendrilSink},
};
use image;
use reqwest::Client;
use serde_json;
use serializer::serialize;
use std::io::Cursor;
use std::str;

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
    debug!("response: {:?}", res);
    let text = res
        .text()
        .map_err(|err| format_err!("Error getting text from mercury api response: {}", err))?;
    debug!("text: {:?}", text);
    let mut json: ParsedDocument = serde_json::from_str(text.as_str())
        .map_err(|err| format_err!("Error deserializing mercury api response json: {}", err))?;

    debug!("content: {:?}", json.content);

    json.content = inline_images(json.content)?;
    debug!("content with inlined images: {:?}", json.content);

    Ok(json)
}

fn inline_images(input: String) -> Result<String, Error> {
    let mut bytes = input.as_bytes();

    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut bytes)?;
    let doc = walk(dom.document)?;

    let mut output = Vec::new();
    let _ = serialize(&mut output, &doc, Default::default())?;
    str::from_utf8(&output)
        .map(|s| s.into())
        .map_err(|err| format_err!("Error sending request to mercury api: {}", err))
}

fn walk(handle: Handle) -> Result<Handle, Error> {
    let node = handle;
    match node.data {
        Element {
            ref name,
            ref attrs,
            ..
        } => {
            if name.local.eq_str_ignore_ascii_case("img") {
                attrs.borrow_mut().iter_mut().for_each(|ref mut attr| {
                    if attr.name.local.eq_str_ignore_ascii_case("src") {
                        match inline_image(attr.value.to_string().as_str()) {
                            Ok(base64_img) => {
                                let src = format!("data:image/jpeg;base64, {}", base64_img);
                                attr.value = Tendril::from_slice(src.as_str());
                            }
                            Err(_) => {}
                        }
                    }
                });
            }
        }
        _ => {}
    }
    let children: Result<Vec<_>, _> = node
        .children
        .borrow()
        .iter()
        .map(|c| walk(c.clone()))
        .collect();
    children?;
    Ok(node)
}

fn inline_image(url: &str) -> Result<String, Error> {
    let url = sanitize_image_url(url);
    let img = get_image(url)?;
    let buf = compress_image(img)?;
    Ok(base64::encode(&buf))
}

fn sanitize_image_url(url: &str) -> &str {
    match url.split("%20").next() {
        Some(url) => url,
        None => url,
    }
}

fn get_image(url: &str) -> Result<Vec<u8>, Error> {
    let client = Client::builder().build()?;
    let request_uri = String::from(url);
    let mut req = client.get(request_uri.as_str());
    let mut buf: Vec<u8> = vec![];
    let mut res = req
        .send()
        .map_err(|err| format_err!("Error fetching image: {}", err))?;
    res.copy_to(&mut buf)?;
    Ok(buf)
}

fn compress_image(input: Vec<u8>) -> Result<Vec<u8>, Error> {
    let format = image::guess_format(&input)?;
    let output = image::load(Cursor::new(input), format)?;
    let output = output
        .grayscale()
        .resize(1080, 1430, image::FilterType::Nearest);
    let mut buf: Vec<u8> = vec![];
    output.write_to(&mut buf, image::JPEG)?;
    Ok(buf)
}
