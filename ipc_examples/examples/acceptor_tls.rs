//! Example of acceptor.
//! # Note
//! - A file path to a credential file must be set on `KDBPLUS_ACCOUNT_FILE`. See the README for a detail of format.
//! - A file path to pkcs12 file must be set on `KDBPLUS_TLS_KEY_FILE` and its password on `KDBPLUS_TLS_KEY_FILE_SECRET`.

use std::io;
use kdbplus::ipc::*;

#[tokio::main]
async fn main() -> io::Result<()>{

  // Start listenening over TCP at the port 7000.
  let mut socket_tcp=QStream::accept(ConnectionMethod::TLS, "canaan.com", 7000).await?;

  // Send a query with the socket.
  let greeting=socket_tcp.send_sync_message(&"string `Hello").await?;
  println!("Greeting: {}", greeting);

  socket_tcp.shutdown().await?;

  Ok(())
}