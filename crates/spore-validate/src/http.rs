// SPDX-License-Identifier: AGPL-3.0-or-later

//! Minimal HTTP client and tar/gzip extraction — pure Rust, no TLS.
//!
//! Designed for sovereign Forgejo on LAN (plain HTTP). For extracellular
//! forges requiring HTTPS, the `fetch` module falls back to `GitBackend`
//! which shells out to `git`.
//!
//! Self-contained: `std::net::TcpStream` for HTTP, `flate2` for gzip,
//! hand-rolled POSIX tar reader for extraction. Zero external C dependencies.

use crate::error::Error;
use std::path::Path;

/// Perform an HTTP GET with redirect following, returning the response body.
///
/// Follows up to 5 HTTP redirects (301, 302, 307, 308). Rejects HTTPS
/// redirects — this client is for plain HTTP only.
pub fn get_body(url: &str) -> Result<Vec<u8>, Error> {
    const MAX_REDIRECTS: u8 = 5;
    let mut current_url = url.to_string();

    for _ in 0..MAX_REDIRECTS {
        let (status, headers_str, body) = request_raw(&current_url)?;

        if status == 200 {
            return Ok(body);
        }

        if matches!(status, 301 | 302 | 307 | 308) {
            let location = headers_str
                .lines()
                .find_map(|line| {
                    let lower = line.to_ascii_lowercase();
                    if lower.starts_with("location:") {
                        Some(line[9..].trim().to_string())
                    } else {
                        None
                    }
                })
                .ok_or_else(|| Error::Git(format!("redirect {status} without Location header")))?;

            if location.starts_with("https://") {
                return Err(Error::Git(format!(
                    "redirect to HTTPS not supported by archive backend: {location}"
                )));
            }
            current_url = if location.starts_with("http://") {
                location
            } else {
                format!("http://{}", location.strip_prefix('/').unwrap_or(&location))
            };
            continue;
        }

        let status_line = headers_str.lines().next().unwrap_or("unknown");
        return Err(Error::Git(format!("HTTP error: {status_line}")));
    }

    Err(Error::Git("too many redirects (max 5)".into()))
}

/// Perform a single HTTP GET request, returning (`status_code`, headers, body).
fn request_raw(url: &str) -> Result<(u16, String, Vec<u8>), Error> {
    use std::io::{Read, Write};
    use std::net::TcpStream;

    let url_path = url.strip_prefix("http://").ok_or_else(|| {
        Error::Git(format!("ForgeArchiveBackend only supports plain HTTP: {url}"))
    })?;

    let (host_port, path) = url_path.split_once('/').unwrap_or((url_path, "/"));
    let path = format!("/{path}");
    let host_port = if host_port.contains(':') {
        host_port.to_string()
    } else {
        format!("{host_port}:80")
    };

    let host = host_port.split(':').next().unwrap_or("");

    let mut stream = TcpStream::connect(&host_port)
        .map_err(|e| Error::Git(format!("TCP connect to {host_port} failed: {e}")))?;

    stream
        .set_read_timeout(Some(std::time::Duration::from_secs(30)))
        .ok();

    let request = format!(
        "GET {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\nAccept: */*\r\n\r\n"
    );
    stream
        .write_all(request.as_bytes())
        .map_err(|e| Error::Git(format!("HTTP write failed: {e}")))?;

    let mut response = Vec::new();
    stream
        .read_to_end(&mut response)
        .map_err(|e| Error::Git(format!("HTTP read failed: {e}")))?;

    let header_end = response
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .ok_or_else(|| Error::Git("malformed HTTP response".into()))?;

    let headers = std::str::from_utf8(&response[..header_end]).unwrap_or("");

    let status = headers
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(0);

    let body = response[header_end + 4..].to_vec();

    Ok((status, headers.to_string(), body))
}

/// Gzip decompression using `flate2` (pure Rust via `miniz_oxide`).
pub fn gzip_decompress(data: &[u8]) -> Result<Vec<u8>, Error> {
    use flate2::read::GzDecoder;
    use std::io::Read;

    let mut decoder = GzDecoder::new(data);
    let mut out = Vec::new();
    decoder
        .read_to_end(&mut out)
        .map_err(|e| Error::Git(format!("gzip decompress failed: {e}")))?;
    Ok(out)
}

/// Minimal tar extraction — reads POSIX tar headers and writes regular files.
///
/// Strips the top-level archive directory (e.g., `repo-main/`) so files
/// extract directly into `target`. Handles type flags `0` (regular) and
/// `5` (directory).
pub fn extract_tar(data: &[u8], target: &Path) {
    let mut pos = 0;

    while pos + 512 <= data.len() {
        let header = &data[pos..pos + 512];

        if header.iter().all(|&b| b == 0) {
            break;
        }

        let name_end = header[..100].iter().position(|&b| b == 0).unwrap_or(100);
        let raw_name = std::str::from_utf8(&header[..name_end]).unwrap_or("");

        let size_str = std::str::from_utf8(&header[124..136])
            .unwrap_or("0")
            .trim_matches(|c: char| c == '\0' || c == ' ');
        let size = usize::from_str_radix(size_str, 8).unwrap_or(0);

        let type_flag = header[156];

        pos += 512;

        let rel_path = raw_name
            .find('/')
            .map_or(raw_name, |i| &raw_name[i + 1..]);

        if !rel_path.is_empty() && (type_flag == b'0' || type_flag == 0) {
            let file_path = target.join(rel_path);
            if let Some(parent) = file_path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if pos + size <= data.len() {
                let _ = std::fs::write(&file_path, &data[pos..pos + size]);
            }
        } else if !rel_path.is_empty() && type_flag == b'5' {
            let _ = std::fs::create_dir_all(target.join(rel_path));
        }

        pos += (size + 511) & !511;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tar_header(name: &str, size: usize, type_flag: u8) -> [u8; 512] {
        let mut header = [0u8; 512];
        let name_bytes = name.as_bytes();
        header[..name_bytes.len()].copy_from_slice(name_bytes);
        let size_str = format!("{size:011o}\0");
        header[124..136].copy_from_slice(size_str.as_bytes());
        header[156] = type_flag;
        header
    }

    #[test]
    fn get_body_rejects_https() {
        let result = get_body("https://example.com/file.tar.gz");
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("only supports plain HTTP"), "{msg}");
    }

    #[test]
    fn get_body_rejects_invalid_url() {
        let result = get_body("ftp://example.com/file.tar.gz");
        assert!(result.is_err());
    }

    #[test]
    fn get_body_fails_on_refused_connection() {
        let result = get_body("http://127.0.0.1:1/file.tar.gz");
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("TCP connect") || msg.contains("failed"), "{msg}");
    }

    #[test]
    fn gzip_decompress_rejects_invalid_data() {
        let result = gzip_decompress(b"not gzip data");
        assert!(result.is_err());
    }

    #[test]
    fn gzip_decompress_roundtrips() {
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use std::io::Write;

        let original = b"hello from sporePrint http module tests";
        let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
        encoder.write_all(original).unwrap();
        let compressed = encoder.finish().unwrap();

        let decompressed = gzip_decompress(&compressed).unwrap();
        assert_eq!(decompressed, original);
    }

    #[test]
    fn extract_tar_creates_files() {
        let mut archive = Vec::new();
        let content = b"file content here";
        archive.extend_from_slice(&tar_header("repo-main/hello.txt", content.len(), b'0'));
        let mut data_block = [0u8; 512];
        data_block[..content.len()].copy_from_slice(content);
        archive.extend_from_slice(&data_block);

        archive.extend_from_slice(&[0u8; 512]);

        let dir = tempfile::tempdir().unwrap();
        extract_tar(&archive, dir.path());

        let extracted = std::fs::read_to_string(dir.path().join("hello.txt")).unwrap();
        assert_eq!(extracted, "file content here");
    }

    #[test]
    fn extract_tar_creates_subdirectories() {
        let mut archive = Vec::new();
        archive.extend_from_slice(&tar_header("repo-main/sub/", 0, b'5'));

        let content = b"nested";
        archive.extend_from_slice(&tar_header("repo-main/sub/deep.txt", content.len(), b'0'));
        let mut data_block = [0u8; 512];
        data_block[..content.len()].copy_from_slice(content);
        archive.extend_from_slice(&data_block);

        archive.extend_from_slice(&[0u8; 512]);

        let dir = tempfile::tempdir().unwrap();
        extract_tar(&archive, dir.path());

        assert!(dir.path().join("sub").is_dir());
        let extracted = std::fs::read_to_string(dir.path().join("sub/deep.txt")).unwrap();
        assert_eq!(extracted, "nested");
    }

    #[test]
    fn extract_tar_handles_empty_archive() {
        let archive = [0u8; 512];
        let dir = tempfile::tempdir().unwrap();
        extract_tar(&archive, dir.path());
        assert!(std::fs::read_dir(dir.path()).unwrap().next().is_none());
    }
}
