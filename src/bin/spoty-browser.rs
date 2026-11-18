//! Native messaging host for the Spoty browser extension.

use std::io::{self, Read, Write};
use std::process::{Command, Stdio};

use serde::Deserialize;

const MAX_MESSAGE: usize = 1024;

#[derive(Deserialize)]
struct Request {
    kind: String,
    id: String,
}

impl Request {
    fn link(&self) -> Option<String> {
        if self.id.len() != 22 || !self.id.bytes().all(|byte| byte.is_ascii_alphanumeric()) {
            return None;
        }
        match self.kind.as_str() {
            "track" => Some(format!("spoty:play:track:{}", self.id)),
            "album" | "artist" | "playlist" | "show" | "episode" => {
                Some(format!("spotify:{}:{}", self.kind, self.id))
            }
            _ => None,
        }
    }
}

fn open_request(mut input: impl Read) -> io::Result<bool> {
    let mut header = [0; 4];
    input.read_exact(&mut header)?;
    let size = u32::from_ne_bytes(header) as usize;
    if size > MAX_MESSAGE {
        return Ok(false);
    }
    let mut message = vec![0; size];
    input.read_exact(&mut message)?;
    let Some(link) = serde_json::from_slice::<Request>(&message)
        .ok()
        .and_then(|request| request.link())
    else {
        return Ok(false);
    };
    let exe = std::env::current_exe()?.with_file_name("spoty.exe");
    Command::new(exe)
        .arg(link)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    Ok(true)
}

fn main() -> io::Result<()> {
    let success = open_request(io::stdin().lock()).unwrap_or(false);
    let message = if success { r#"{"ok":true}"# } else { r#"{"ok":false}"# };
    let mut output = io::stdout().lock();
    output.write_all(&(message.len() as u32).to_ne_bytes())?;
    output.write_all(message.as_bytes())?;
    output.flush()
}
