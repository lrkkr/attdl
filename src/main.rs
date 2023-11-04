extern crate imap;

use mail_parser::{MessageParser, MimeHeaders};
use native_tls::TlsConnector;

fn main() {
    // load environment variables
    dotenvy::dotenv().ok();
    let email_address: String = dotenvy::var("EMAIL_ADDRESS").unwrap();
    let password: String = dotenvy::var("PASSWORD").unwrap();
    let imap_server: String = dotenvy::var("IMAP_SERVER").unwrap();
    let search_mailbox: &str = "INBOX";
    let from_address: String = dotenvy::var("FROM_ADDRESS").unwrap();
    let attachment_path: &str = "./attachments/";
    // create a new tls connector
    let tls = TlsConnector::builder().build().unwrap();
    // create a new imap client
    let client = imap::connect((imap_server.clone(), 993), imap_server, &tls)
        .expect("Failed to build client");

    // create a new session
    let mut session = client
        .login(email_address, password)
        .expect("Failed to login");

    // select the INBOX mailbox
    session
        .select(search_mailbox)
        .expect("Failed to select mailbox");

    // search mailbox for all messages
    let messages = session
        .search("ALL")
        .expect("Failed to search for messages");

    for &message in &messages {
        // fetch message
        let fetched_message = session
            .fetch(message.to_string(), "BODY[HEADER]")
            .expect("Failed to fetch message");

        // parse message
        let message_data = fetched_message.iter().next();

        if let Some(message_data) = message_data {
            // check from address
            let header = message_data.header().expect("has no header");
            let parsed_header = MessageParser::default().parse(header).unwrap();
            let from = parsed_header
                .from()
                .unwrap()
                .first()
                .unwrap()
                .address()
                .unwrap();
            if from.contains(&from_address) {
                // check attachments
                let fetched_body = session
                    .fetch(message.to_string(), "BODY[]")
                    .expect("Failed to fetch message");
                let body_data = fetched_body.iter().next();
                if let Some(body_data) = body_data {
                    // fetch body
                    let body = body_data.body().expect("has no body");
                    // parse body
                    let parsed_body = mail_parser::MessageParser::default().parse(body).unwrap();
                    // get attachment
                    let attachment = parsed_body.attachment(0).unwrap();
                    // save attachment
                    let attachment_name = attachment.attachment_name().unwrap();
                    let attachment_data = attachment.contents();
                    // create attachment directory if it doesn't exist
                    std::fs::create_dir_all(attachment_path).unwrap();
                    // check if attachment already exists
                    if std::path::Path::new(&format!("{}{}", attachment_path, attachment_name))
                        .exists()
                    {
                        println!("Attachment already exists: {}", attachment_name);
                        continue;
                    }
                    // write attachment to file
                    std::fs::write(
                        format!("{}{}", attachment_path, attachment_name),
                        attachment_data,
                    )
                    .unwrap();
                    println!("Saved attachment: {}", attachment_name);
                }
            }
        }
    }
}
