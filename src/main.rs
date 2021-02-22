mod ngrok;

use url::Url;
fn main() -> std::io::Result<()> {
    let host = std::env::var("HOST").expect("Expected hostname.");
    let port = std::env::var("PORT").expect("Expected port.");
    let auth = std::env::var("AUTH").expect("Expected auth crendentials in format 'usr:pwd'.");
    let authtoken = std::env::var("NGROK_AUTHTOKEN").expect("Expected ngrok authtoken for your ngrok user.");

    ngrok::auth_ngrok(authtoken);

    let tunnel = ngrok::builder()
        .http()
        .host(host)
        .port(port)
        .auth(auth)
        .run()?;

    let public_url: &Url = tunnel.http()?;

    println!("Tunnel is open at {:?}", public_url);

    // Sleep for a year
    std::thread::sleep(std::time::Duration::from_secs(31_536_000));

    println!("Woke up after a year. Stopping this process.");

    Ok(())
}