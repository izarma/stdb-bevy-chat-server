use bevy::{prelude::*, tasks::IoTaskPool};
use bevy_http_client::{HttpClient, HttpRequest, HttpResponse, HttpResponseError};
use crossbeam_channel::bounded;
use serde::Deserialize;

use crate::{
    module_bindings::set_name,
    socials::{ChatState, SpacetimeDB, chatui::LoginMessage},
    utils::{generate_csrf_state, pkce_challenge, pkce_verifier},
};

#[derive(Resource, Default)]
pub struct LoginData {
    pub access_token: Option<String>,
    pub id_token: Option<String>,
}

pub(crate) fn login_event_handler(
    mut events: MessageReader<LoginMessage>,
    stdb: SpacetimeDB,
    mut state: ResMut<NextState<ChatState>>,
    mut ev_request: MessageWriter<HttpRequest>,
) {
    for event in events.read() {
        match event {
            LoginMessage::Username(usr) => {
                stdb.reducers().set_name(usr.to_string()).unwrap();
                state.set(ChatState::LoggedIn);
            }
            LoginMessage::Discord => {
                let verifier = pkce_verifier();
                let challenge = pkce_challenge(&verifier);
                let state = generate_csrf_state();
                let stdb_client_id = "client_031LPfBM8jHxvge4NX9CNr";
                let redirect_uri = "http://127.0.0.1:42069";
                let auth_url = format!(
                    "https://auth.spacetimedb.com/oidc/auth?client_id={}&redirect_uri={}&scope=openid%20email%20profile&response_type=code&response_mode=query&code_challenge_method=S256&code_challenge={}&state={}",
                    stdb_client_id, redirect_uri, challenge, state
                );
                // info!("Auth URL: {}", auth_url);
                let expected_state = state.clone();
                let (s1, r1) = bounded::<String>(2);
                let (s2, _) = (s1.clone(), r1.clone());
                IoTaskPool::get()
                    .spawn(async move {
                        info!("Starting server");
                        let server = tiny_http::Server::http("127.0.0.1:42069").unwrap();
                        let request = server.recv().unwrap();
                        // Parse query parameters from URL
                        let query_string = request.url().trim_start_matches("/?");
                        let mut code = None;
                        let mut state = None;

                        for param in query_string.split('&') {
                            let parts: Vec<&str> = param.split('=').collect();
                            if parts.len() == 2 {
                                match parts[0] {
                                    "code" => code = Some(parts[1].to_string()),
                                    "state" => state = Some(parts[1].to_string()),
                                    _ => {}
                                }
                            }
                        }
                        if let (Some(received_state), Some(auth_code)) = (state, code) {
                            if received_state == expected_state {
                                info!("Successfully received auth code: {}", auth_code);
                                s2.send(auth_code).unwrap();
                            }
                        }
                    })
                    .detach();
                let _jh = open::that_in_background(auth_url);
                let url = format!("https://auth.spacetimedb.com/oidc/token");
                let auth_code = r1.recv().unwrap();
                let verifier_str =
                    String::from_utf8(verifier).expect("Code verifier bytes were not valid UTF-8");
                let body = format!(
                    "client_id={}&\
                                  code={}&\
                                  code_verifier={}&\
                                  grant_type=authorization_code&\
                                  redirect_uri=http://127.0.0.1:42069",
                    stdb_client_id, auth_code, &verifier_str
                );
                match HttpClient::new().post(url).form_encoded(&body).try_build() {
                    Ok(request) => {
                        ev_request.write(request);
                    }
                    Err(e) => {
                        eprintln!("Failed to build request: {}", e);
                    }
                }
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    pub access_token: String,
    pub id_token: String,
}

pub(crate) fn handle_response(
    mut commands: Commands,
    mut ev_resp: MessageReader<HttpResponse>,
    mut login_data: ResMut<LoginData>,
) {
    for response in ev_resp.read() {
        // info!("response {}", response.text().unwrap());
        let token_response: TokenResponse = response.json().unwrap();
        info!("response {:#?}", token_response);
        login_data.access_token = Some(token_response.access_token);
        login_data.id_token = Some(token_response.id_token);
    }
}

pub(crate) fn handle_error(mut ev_error: MessageReader<HttpResponseError>) {
    for error in ev_error.read() {
        println!("Error retrieving IP: {}", error.err);
    }
}
