//! HTTP [`Transport`] backed by `reqwless`.
//!
//! Gated behind the `http-reqwless` feature so the crate's default build
//! keeps its zero-network-stack dependency footprint — a project feeding OTA
//! bytes from a filesystem or a caller-driven channel implements
//! [`Transport`] itself and never pulls `embassy-net` in.
//!
//! What this covers, and what it deliberately does not:
//!
//! - **No TLS.** `reqwless` is taken with `default-features = false`, which
//!   drops `embedded-tls`. Transport-level confidentiality is not what makes
//!   an update safe here: the bootloader verifies each image's ECC signature
//!   before ever booting it, so a tampered download fails signature checking
//!   and is discarded rather than run. Adding HTTPS later means enabling the
//!   TLS feature and swapping the client construction — the core state
//!   machine is unaffected.
//! - **No redirects.** `reqwless` does not follow 3xx, so a redirecting URL
//!   surfaces as [`HttpError::Status`] rather than being chased silently.
//!   Point the updater at a direct URL.
//! - **No DNS.** The host must be an IPv4 literal; see [`LiteralDns`].

use core::net::{IpAddr, Ipv4Addr};

use embedded_nal_async::{AddrType, Dns};
use reqwless::client::{HttpClient, HttpResource};
use reqwless::request::Method;

use crate::transport::Transport;

/// Bytes reserved for the response status line and headers.
///
/// `reqwless` parses headers into the buffer handed to `send()`, so this
/// bounds the *header* block, not the body — the body is streamed past it.
pub const HEADER_BUF_SIZE: usize = 1024;

/// Failures specific to the HTTP transport.
///
/// `reqwless::Error` implements none of `Copy`/`Clone`/`PartialEq`, so this
/// enum derives only `Debug` rather than discarding the underlying error just
/// to widen the derive list.
#[derive(Debug)]
pub enum HttpError {
    /// Host was not an IPv4 literal — this transport does not resolve names.
    NotAnIpLiteral,
    /// Reverse lookup was requested; never needed for an outbound fetch.
    ReverseLookupUnsupported,
    /// The server answered with a non-2xx status.
    Status(u16),
    /// Underlying `reqwless`/socket failure.
    Request(reqwless::Error),
    /// Reading the response body failed mid-stream.
    Body(reqwless::Error),
}

/// A [`Dns`] implementation that only accepts addresses already in literal
/// form.
///
/// `reqwless` requires *some* resolver to construct a client, but resolving
/// names is a separate concern from fetching an image, and pulling a DNS
/// client in would mean another socket and another failure mode on the
/// update path. Parsing the literal keeps the type requirement satisfied
/// without pretending to offer resolution: a hostname URL fails fast with
/// [`HttpError::NotAnIpLiteral`] instead of timing out somewhere less
/// obvious.
pub struct LiteralDns;

impl Dns for LiteralDns {
    type Error = HttpError;

    async fn get_host_by_name(
        &self,
        host: &str,
        _addr_type: AddrType,
    ) -> Result<IpAddr, Self::Error> {
        host.parse::<Ipv4Addr>()
            .map(IpAddr::V4)
            .map_err(|_| HttpError::NotAnIpLiteral)
    }

    async fn get_host_by_address(
        &self,
        _addr: IpAddr,
        _result: &mut [u8],
    ) -> Result<usize, Self::Error> {
        Err(HttpError::ReverseLookupUnsupported)
    }
}

/// An in-progress HTTP GET whose body is the OTA image.
///
/// Produced by [`http_get`]; hand it to
/// [`OtaSession::new`](crate::core::OtaSession::new).
///
/// Generic over the body reader rather than naming `reqwless`'s concrete
/// buffering reader — that type is crate-private, and `BodyReader` implements
/// [`embedded_io_async::Read`] for every backing reader anyway, so the bound
/// is all this wrapper needs.
pub struct HttpTransport<R>
where
    R: embedded_io_async::Read<Error = reqwless::Error>,
{
    body: R,
}

impl<R> Transport for HttpTransport<R>
where
    R: embedded_io_async::Read<Error = reqwless::Error>,
{
    type Error = HttpError;

    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.body.read(buf).await.map_err(HttpError::Body)
    }
}

/// Issue `GET` against `resource` and return its body as a [`Transport`].
///
/// The response status is checked before returning, so a 404 or a redirect
/// fails here rather than being programmed into flash as if it were firmware.
///
/// `header_buf` must outlive the returned transport; size it with
/// [`HEADER_BUF_SIZE`].
pub async fn http_get<'a, C>(
    resource: &'a mut HttpResource<'a, C>,
    path: &'a str,
    header_buf: &'a mut [u8],
) -> Result<HttpTransport<impl embedded_io_async::Read<Error = reqwless::Error> + 'a>, HttpError>
where
    C: embedded_io_async::Read + embedded_io_async::Write,
{
    let response = resource
        .request(Method::GET, path)
        .send(header_buf)
        .await
        .map_err(HttpError::Request)?;

    let status = response.status;
    if !status.is_successful() {
        return Err(HttpError::Status(status.0));
    }

    Ok(HttpTransport {
        body: response.body().reader(),
    })
}

/// Convenience alias for the client type this transport is built on.
pub type OtaHttpClient<'a, T> = HttpClient<'a, T, LiteralDns>;
