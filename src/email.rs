use chrono::{DateTime, Local};
use configuration::EmailConfig;
use failure::Error;
use lettre::{
    smtp::{authentication::Credentials, SmtpClient},
    Transport,
};
use lettre_email::EmailBuilder;
use mime::TEXT_HTML;
use parser::ParsedDocument;

pub fn send(doc: ParsedDocument, config: EmailConfig) -> Result<(), Error> {
    let EmailConfig {
        to,
        username,
        password,
    } = config;

    let ParsedDocument {
        title,
        author,
        content,
        domain,
        date_published,
    } = doc;

    let mut mailer = SmtpClient::new_simple("smtp.gmail.com")?
        .credentials(Credentials::new(username.to_owned(), password))
        .transport();

    let mut file_name = title.to_owned();

    if let Some(author) = author {
        file_name.push_str(&format!(" - {}", author));
    }

    if let Some(domain) = domain {
        file_name.push_str(&format!(" - {}", domain));
    }

    let date: Option<DateTime<Local>> = date_published.and_then(|date| date.parse().ok());
    if let Some(date) = date {
        file_name.push_str(date.format(" - %Y-%m-%d").to_string().as_str());
    }
    file_name.push_str(".html".into());
    debug!("file_name: {:?}", file_name);

    let email = EmailBuilder::new()
        .to(to)
        .from(username)
        .subject("CONVERT")
        .text("Sent from tl;rl")
        .attachment(&content.into_bytes(), file_name.as_str(), &TEXT_HTML)?
        .build()?;

    trace!("{:?}", email);

    trace!("sending...");
    mailer.send(email.into())?;
    trace!("sent");
    trace!("closing...");
    mailer.close();
    trace!("closed");
    Ok(())
}
