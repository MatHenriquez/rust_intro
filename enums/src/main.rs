fn main() {
    enum Response {
        Success,
        Error(u32), // Error with a code (u32) associated with it (like HTTP status code)
    }

    let response = Response::Success; // Success response
    let response = Response::Error(404); // Error response with code 404

    match response {
        Response::Success => println!("Request succeeded"), // Print if response is Success
        Response::Error(404) => println!("Request failed: Not Found"), // Print if response is Error with code 404
        Response::Error(code) => println!("Request failed with code: {}", code), // Print if response is Error with any other code
    }
}
