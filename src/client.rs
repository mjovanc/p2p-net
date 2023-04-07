use reqwest::blocking::Client;

pub(crate) fn send_message(message: String) {
    println!("Sending message: {}", message);

    let client = Client::new();
    let res = client.get("http://localhost:8477").send();

    match res {
        Ok(response) => {
            if response.status().is_success() {
                println!("Server response: {:?}", response.text());
            } else {
                println!("Server returned error: {}", response.status());
            }
        },
        Err(e) => println!("Error sending request: {}", e),
    }
}
