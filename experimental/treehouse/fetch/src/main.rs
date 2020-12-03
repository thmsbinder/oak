use oauth2::{
    basic::BasicClient, reqwest::http_client, AuthUrl, AuthorizationCode, ClientId, ClientSecret,
    CsrfToken, PkceCodeChallenge, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::TcpListener,
};
use url::Url;

const OAUTH_TOKEN_FILE: &str = ".oauth_token.secret";

#[derive(Deserialize, Debug)]
struct ClientConfig {
    web: ClientConfigWeb,
}

#[derive(Deserialize, Debug)]
struct ClientConfigWeb {
    client_id: String,
    project_id: String,
    auth_uri: String,
    token_uri: String,
    auth_provider_x509_cert_url: String,
    client_secret: String,
    redirect_uris: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CalendarEvents {
    kind: String,
    etag: String,
    summary: String,
    updated: String,
    time_zone: String,
    access_role: String,
    items: Vec<CalendarEvent>,
}

/// See https://developers.google.com/calendar/v3/reference/events.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CalendarEvent {
    kind: String,
    etag: String,
    id: String,
    status: String,
    #[serde(default)]
    html_link: String,
    #[serde(default)]
    created: String,
    #[serde(default)]
    updated: String,
    #[serde(default)]
    summary: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    location: String,
    #[serde(default)]
    color_id: String,
    start: Option<CalendarTime>,
    end: Option<CalendarTime>,
    #[serde(default)]
    attachments: Vec<CalendarAttachment>,
    #[serde(default)]
    attendees: Vec<CalendarAttendee>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CalendarAttachment {
    file_url: String,
    title: String,
    mime_type: String,
    icon_link: String,
    file_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CalendarAttendee {
    #[serde(default)]
    id: String,
    #[serde(default)]
    email: String,
    #[serde(default)]
    display_name: String,
    #[serde(default)]
    organizer: bool,
    #[serde(default)]
    self_: bool,
    #[serde(default)]
    resource: bool,
    #[serde(default)]
    optional: bool,
    #[serde(default)]
    response_status: String,
    #[serde(default)]
    comment: String,
    #[serde(default)]
    additional_guests: u16,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CalendarTime {
    #[serde(default)]
    date: String,
    #[serde(default)]
    date_time: String,
    #[serde(default)]
    time_zone: String,
}

fn get_token() -> String {
    if let Ok(mut f) = std::fs::File::open(OAUTH_TOKEN_FILE) {
        let mut token = String::new();
        f.read_to_string(&mut token).unwrap();
        return token;
    }
    let client_secret_file = std::fs::File::open("client_secret_691249393555-anhtgr54cajv9l96apgfi3hv1n3a63e2.apps.googleusercontent.com.json").unwrap();
    let client_config: ClientConfig = serde_json::from_reader(client_secret_file).unwrap();
    println!("loaded client config");
    let auth_url =
        AuthUrl::new(client_config.web.auth_uri).expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new(client_config.web.token_uri).expect("Invalid token endpoint URL");

    // Set up the config for the Google OAuth2 process.
    let client = BasicClient::new(
        ClientId::new(client_config.web.client_id),
        Some(ClientSecret::new(client_config.web.client_secret)),
        auth_url,
        Some(token_url),
    )
    // This example will be running its own server at localhost:8080.
    // See below for the server implementation.
    .set_redirect_url(
        RedirectUrl::new("http://localhost:8080".to_string()).expect("Invalid redirect URL"),
    );

    // Google supports Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
    // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        // This example is requesting access to the "calendar" features and the user's profile.
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/calendar.readonly".to_string(),
        ))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/gmail.readonly".to_string(),
        ))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/drive.readonly".to_string(),
        ))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/drive.photos.readonly".to_string(),
        ))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/plus.me".to_string(),
        ))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    println!(
        "Open this URL in your browser:\n{}\n",
        authorize_url.to_string()
    );

    // A very naive implementation of the redirect server.
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let code;
            let state;
            {
                let mut reader = BufReader::new(&stream);

                let mut request_line = String::new();
                reader.read_line(&mut request_line).unwrap();

                let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                let code_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "code"
                    })
                    .unwrap();

                let (_, value) = code_pair;
                code = AuthorizationCode::new(value.into_owned());

                let state_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "state"
                    })
                    .unwrap();

                let (_, value) = state_pair;
                state = CsrfToken::new(value.into_owned());
            }

            let message = "Go back to your terminal :)";
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes()).unwrap();

            println!("Google returned the following code:\n{}\n", code.secret());
            println!(
                "Google returned the following state:\n{} (expected `{}`)\n",
                state.secret(),
                csrf_state.secret()
            );

            // Exchange the code with a token.
            let token = client
                .exchange_code(code)
                .set_pkce_verifier(pkce_code_verifier)
                .request(http_client);

            let secret = token.unwrap().access_token().secret().to_string();
            std::fs::File::create(OAUTH_TOKEN_FILE)
                .unwrap()
                .write_all(secret.as_bytes())
                .unwrap();
            return secret;
        }
    }
    unreachable!()
}

fn relevant_people(events: &CalendarEvents) -> String {
    let mut people = std::collections::HashMap::<String, u16>::new();
    for event in events.items.iter() {
        for attendee in event.attendees.iter() {
            people
                .entry(attendee.email.clone())
                .and_modify(|v| *v += 1)
                .or_default();
        }
    }
    let mut people = people.iter().collect::<Vec<_>>();
    people.sort_by_key(|(_, n)| *n);
    people.reverse();
    people
        .iter()
        .take(10)
        .map(|(p, n)| format!("{}: {}", p, n))
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    let token = get_token();
    let client = reqwest::blocking::Client::new();
    let req = client.get("https://www.googleapis.com/calendar/v3/calendars/primary/events");
    let req = req.bearer_auth(token);

    // See https://developers.google.com/calendar/v3/reference/events/list.
    let req = req.query(&[("timeMin", "2020-11-01T00:00:00Z"), ("maxResults", "2000")]);

    let res = req.send().unwrap();
    if res.status().is_success() {
        let events = res.json::<CalendarEvents>().unwrap();
        println!("calendar events: {:?}", events.items.len());
        println!("relevant people:\n{:}", relevant_people(&events));
    } else {
        println!("error: {}", res.status());
        println!("error: {}", res.text().unwrap());
    }
}
