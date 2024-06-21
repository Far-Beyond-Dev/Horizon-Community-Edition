// auth.rs
use serde_json::{Value, json};

// Assuming you have a struct to represent the user
struct User {
    id: String,
    username: String,
    // Other user fields
}

// Private function to authenticate the user against the external API
pub async fn authenticate_user_api(data: Value) -> Option<User> {
    // Extract the necessary data from the `data` Value
    let username = data.get("username")?.as_str()?;
    let password = data.get("password")?.as_str()?;

    // Prepare the JSON data as a byte vector
    let request_body = json!({
        "username": username,
        "password": password
    });
    let request_body = request_body.to_string().into_bytes();

    // Make a request to the external authentication API
    let client = reqwest::Client::new();
    let response = client
        .post("https://example.com/auth/login")
        .body(request_body)
        .send()
        .await
        .ok()?
        .error_for_status();

    // we get the data in the response body, so I think so// Get the response text
    // Thanks a million man silly issue but glad you got it
    // let response_data = response();
    let response_string = String::from_utf8_lossy(&response);

    // Parse the response text and create a User instance
    let user_data: Value = serde_json::from_str(&response_data).ok()?;
    let user = User {
        id: user_data.get("id")?.as_str()?.to_owned(),
        username: user_data.get("username")?.as_str()?.to_owned(),
        // Other user fields
    };
    Some(user)
}

// Public function to authenticate the user
pub async fn authenticate_user(data: Value) -> Option<User> {
    authenticate_user_api(data).await
}

// Function to authorize the user for a specific namespace and socket ID
pub fn authorize_request(user: &User, namespace: &str, socket_id: &str) -> bool {
    // Implement your authorization logic here
    // For example, you can check if the user has permissions to access the namespace
    // or if the socket ID matches the user's session
    true
}
// note: remove any `use`s here; vscode is a bit bugged and will put them back for some reason
// why??
// pub use authorize_request;pub use authorize_request;pub use authorize_request;pub use authorize_request;pub use authorize_request;pub use authorize_request;pub use authorize_request;pub use authorize_request;pub use authorize_request;pub use authorize_request;