use base64;
use failure::Error;
use html5ever::{ns, LocalName, QualName};
use image;
use kuchiki::{parse_html, traits::TendrilSink, Attribute, Attributes, ExpandedName, NodeRef};
use reqwest::Client;
use serde_json;
use serializer::serialize;
use std::collections::BTreeMap;
use std::io::Cursor;
use std::io::ErrorKind::NotFound;
use std::process::Command;
use std::str;

header! { (XApiKey, "x-api-key") => [String] }

#[derive(Deserialize, Debug)]
pub struct ParsedDocument {
    pub title: String,
    pub author: Option<String>,
    pub content: String,
    pub domain: Option<String>,
    pub date_published: Option<String>,
    pub url: Option<String>,
}

pub fn parse(uri: &str) -> Result<ParsedDocument, Error> {
    let output = Command::new("mercury-parser").arg(uri).output().map_err(|err| {
        match err.kind() {
            NotFound => format_err!("Couldn't find executable `mercury-parser`. Make sure you've installed it and it's in your $PATH: https://github.com/postlight/mercury-parser"),
            _ => format_err!("{}", err)
        }
    })?;

    let text = String::from_utf8(output.stdout)?;
    debug!("text: {:?}", text);
    let mut json: ParsedDocument = serde_json::from_str(text.as_str())
        .map_err(|err| format_err!("Error deserializing mercury api response json: {}", err))?;

    debug!("content: {:?}", json.content);

    json.content = inline_images(json.content)?;
    debug!("content with inlined images: {:?}", json.content);

    Ok(json)
}

fn inline_images(input: String) -> Result<String, Error> {
    let doc = parse_html().one(input);

    // inline base64 encoded image
    for img in doc
        .select("img")
        .map_err(|err| format_err!("Error selecting img: {:?}", err))?
    {
        if let Some(element) = img.as_node().as_element() {
            let mut attr = element.attributes.borrow_mut();
            if let Some(src) = attr.get("src") {
                let base64_img = inline_image(src)?;
                let src = format!("data:image/jpeg;base64, {}", base64_img);
                *attr = Attributes {
                    map: BTreeMap::new(),
                };
                attr.insert("src", src);
            }
        }
    }

    // pull `img`s out of `picture`s and remove `picture` elements because kindle seems to do a
    // weird thing with `picture`s where the content width gets narrower after each one.
    for picture in doc
        .select("picture")
        .map_err(|err| format_err!("Error removing picture: {:?}", err))?
    {
        let picture_node = picture.as_node();
        let img = picture_node
            .select_first("img")
            .map_err(|err| format_err!("Error selecting img {:?}", err))?;
        if let Some(element) = img.as_node().as_element() {
            let attr = element.attributes.borrow();
            if let Some(src) = attr.get("src") {
                let name = QualName::new(None, ns!(html), LocalName::from("img"));
                let expanded_name = ExpandedName::new(ns!(), LocalName::from("src"));
                let attr = Attribute {
                    prefix: None,
                    value: src.to_string(),
                };
                let attr = vec![(expanded_name, attr)];
                let img = NodeRef::new_element(name, attr);
                picture_node.insert_before(img);
            }
        }
    }

    let mut done = false;

    while !done {
        if let Ok(p) = doc.select_first("picture") {
            p.as_node().detach();
        } else {
            done = true;
        }
    }

    done = false;

    // strip iframes
    while !done {
        if let Ok(p) = doc.select_first("iframe") {
            p.as_node().detach();
        } else {
            done = true;
        }
    }

    let mut output = Vec::new();
    let _ = serialize(&mut output, &doc, Default::default())?;

    str::from_utf8(&output)
        .map(|s| s.into())
        .map_err(|err| format_err!("Error sending request to mercury api: {}", err))
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
