use serde_json::json;
use vercel_runtime::{ run, Body, Error, Request, Response, StatusCode };
use postgres::{ Client, NoTls };
use std::net::Ipv4Addr;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    if req.method() == "POST" {
        match serde_json::from_slice::<serde_json::Value>(req.body().as_ref()) {
            Ok(json_body) => {
                let ip = match json_body["ip"].as_str() {
                    Some(ip) => ip,
                    None => {
                        return create_error_response(
                            StatusCode::BAD_REQUEST,
                            "ip is missing or invalid"
                        );
                    }
                };
                let ip_num = ip_to_number(ip);
                let mut client = Client::connect("YOUR_DB", NoTls)?;
                let row = client.query_opt(
                    "SELECT ip, asn FROM ip_info WHERE ip > $1 ORDER BY ip LIMIT 1",
                    &[&ip_num]
                )?;

                if let Some(row) = row {
                    Ok(
                        Response::builder()
                            .status(StatusCode::OK)
                            .header("Content-Type", "application/json")
                            .body(
                                json!({
                                "asn": row.get::<usize, &str>(1),
                            })
                                    .to_string()
                                    .into()
                            )?
                    )
                } else {
                    return create_error_response(StatusCode::BAD_REQUEST, "ip not found");
                }
            }
            Err(_) => create_error_response(StatusCode::BAD_REQUEST, "Invalid JSON"),
        }
    } else {
        create_error_response(StatusCode::METHOD_NOT_ALLOWED, "Method Not Allowed")
    }
}

fn create_error_response(status: StatusCode, message: &str) -> Result<Response<Body>, Error> {
    Ok(
        Response::builder()
            .status(status)
            .header("Content-Type", "application/json")
            .body(
                json!({
                    "error": message,
                }).to_string().into()
            )?
    )
}

fn ip_to_number(ip: &str) -> u32 {
    let ipv4: Ipv4Addr = ip.parse().expect("bad ip");
    let octets = ipv4.octets();
    ((octets[0] as u32) << 24) |
        ((octets[1] as u32) << 16) |
        ((octets[2] as u32) << 8) |
        (octets[3] as u32)
}
