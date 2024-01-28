use colored::*;
use mime::Mime;

pub async fn print_response(resp: reqwest::Response) {
    print_status(&resp);
    print_headers(&resp);
    print_body(resp).await;
}

pub fn print_status(resp: &reqwest::Response) {
    let status = resp.status();
    let status_code = status.to_string();
    let status_code = match status_code.as_str() {
        "200 OK" => status_code.green(),
        "400 Bad Request" => status_code.yellow(),
        "404 Not Found" => status_code.red(),
        "500 Internal Server Error" => status_code.red(),
        _ => status_code.normal(),
    };

    let version = format!("{:?}", resp.version()).blue();

    println!(
        "{} {} {}",
        "Response Status:".to_string().green(),
        version,
        status_code
    );
}

pub fn print_headers(resp: &reqwest::Response) {
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }

    println!()
}

pub async fn print_body(resp: reqwest::Response) {
    let content_type = get_content_type(&resp).await;

    match resp.text().await {
        Ok(text) => match content_type {
            Some(content_type) if content_type == mime::APPLICATION_JSON => {
                println!("{}", jsonxf::pretty_print(&text).unwrap().cyan())
            }
            _ => println!("{}", text.cyan()),
        },
        Err(_) => println!("Response body is not valid UTF-8, cannot print."),
    }
}

pub async fn get_content_type(resp: &reqwest::Response) -> Option<Mime> {
    match resp.headers().get("content-type") {
        Some(content_type) => Some(content_type.to_str().unwrap().parse().unwrap()),
        None => None,
    }
}
