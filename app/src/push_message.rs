use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub data: Vec<PushTicket>,
}

#[derive(Debug, Deserialize)]
pub struct PushTicket {
    pub status: String,
    pub id: String,
}

pub async fn push_message(
    expo_push_token: &[&str],
    title: &str,
    body: &str,
) -> Result<ApiResponse, String> {
    const URL: &str = "https://exp.host/--/api/v2/push/send";
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = reqwest::Client::new();

    for token in expo_push_token {
        if !token.starts_with("ExponentPushToken[") {
            let error_message = format!("Invalid expo push token: {}", token);
            return Err(error_message);
        }
    }

    if title.is_empty() {
        let error_message = format!("Title is empty");
        return Err(error_message);
    }

    if body.is_empty() {
        let error_message = format!("Body is empty");
        return Err(error_message);
    }

    let payload = json!({
        "to": expo_push_token,
        "title": title,
        "body": body,
    });

    match client
        .post(URL)
        .headers(headers)
        .json::<Value>(&payload)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                let body = response
                    .json::<ApiResponse>()
                    .await
                    .expect("Failed to parse response body");
                Ok(body)
            } else {
                let error_message = format!(
                    "Request failed with status code {}: {}",
                    response.status(),
                    response
                        .text()
                        .await
                        .expect("Failed to parse response body")
                );
                Err(error_message)
            }
        }
        Err(err) => {
            let error_message = format!("Failed to send request: {:?}", err);
            Err(error_message)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito;

    #[tokio::test]
    async fn test_invalid_expo_push_token() {
        let result = push_message(&["invalid_token"], "Hello", "World");
        assert_eq!(
            result.await.unwrap_err(),
            "Invalid expo push token: invalid_token"
        );
    }

    #[tokio::test]
    async fn test_empty_title() {
        let result = push_message(&["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"], "", "World");
        assert_eq!(result.await.unwrap_err(), "Title is empty");
    }

    #[tokio::test]
    async fn test_empty_body() {
        let result = push_message(&["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"], "Hello", "");
        assert_eq!(result.await.unwrap_err(), "Body is empty");
    }

    #[tokio::test]
    async fn test_valid_post() {
        let mut server = mockito::Server::new();
        server
            .mock("POST", "https://exp.host/--/api/v2/push/send")
            .with_status(200)
            .with_header("content-type", "application/json")
            .match_body(mockito::Matcher::JsonString(
                r#"{"to":["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"],"title":"Hello","body":"World"}"#
                    .to_string(),
            )).create();
        let result = push_message(
            &["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"],
            "Hello",
            "World",
        );
        assert_eq!(result.await.is_ok(), true);
    }

    #[tokio::test]
    #[ignore]
    async fn test_invalid_post() {
        todo!("test invalid post")
    }
}
