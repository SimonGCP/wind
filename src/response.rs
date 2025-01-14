#[derive(Debug, PartialEq, Clone)]
pub struct Response {
    pub code: HTTPCodes,
    pub contents: Vec<u8>,
    pub next: bool,
}

impl Response {
    pub fn new(code: HTTPCodes, contents: Vec<u8>) -> Response {
        Response{code, contents, next: false}
    }

    pub fn ok(contents: Vec<u8>) -> Response {
        Response{code: HTTPCodes::OK, contents, next: false}
    }

    pub fn next() -> Response {
        Response{code: HTTPCodes::OK, contents: Vec::new(), next: true}
    }

    pub fn header_string(&self) -> String {
        format!("HTTP/1.1 {}\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n",
            self.code.as_str(),
            self.contents.len()
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum HTTPCodes {
    // Informational Codes (100–199)
    Continue,
    SwitchingProtocols,
    Processing,
    EarlyHints,

    // Successful Responses (200–299)
    OK,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,

    // Redirection Messages (300–399)
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    UseProxy,
    TemporaryRedirect,
    PermanentRedirect,

    // Client Error Responses (400–499)
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    RequestTimeout,

    // Server Error Responses (500–599)
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HTTPVersionNotSupported,
}

impl HTTPCodes {
    pub fn as_str(&self) -> &'static str {
        match self {
            // Informational Codes
            HTTPCodes::Continue => "100 Continue",
            HTTPCodes::SwitchingProtocols => "101 Switching Protocols",
            HTTPCodes::Processing => "102 Processing",
            HTTPCodes::EarlyHints => "103 Early Hints",

            // Successful Responses
            HTTPCodes::OK => "200 OK",
            HTTPCodes::Created => "201 Created",
            HTTPCodes::Accepted => "202 Accepted",
            HTTPCodes::NonAuthoritativeInformation => "203 Non-Authoritative Information",
            HTTPCodes::NoContent => "204 No Content",
            HTTPCodes::ResetContent => "205 Reset Content",
            HTTPCodes::PartialContent => "206 Partial Content",

            // Redirection Messages
            HTTPCodes::MultipleChoices => "300 Multiple Choices",
            HTTPCodes::MovedPermanently => "301 Moved Permanently",
            HTTPCodes::Found => "302 Found",
            HTTPCodes::SeeOther => "303 See Other",
            HTTPCodes::NotModified => "304 Not Modified",
            HTTPCodes::UseProxy => "305 Use Proxy",
            HTTPCodes::TemporaryRedirect => "307 Temporary Redirect",
            HTTPCodes::PermanentRedirect => "308 Permanent Redirect",

            // Client Error Responses
            HTTPCodes::BadRequest => "400 Bad Request",
            HTTPCodes::Unauthorized => "401 Unauthorized",
            HTTPCodes::PaymentRequired => "402 Payment Required",
            HTTPCodes::Forbidden => "403 Forbidden",
            HTTPCodes::NotFound => "404 Not Found",
            HTTPCodes::MethodNotAllowed => "405 Method Not Allowed",
            HTTPCodes::NotAcceptable => "406 Not Acceptable",
            HTTPCodes::RequestTimeout => "408 Request Timeout",

            // Server Error Responses
            HTTPCodes::InternalServerError => "500 Internal Server Error",
            HTTPCodes::NotImplemented => "501 Not Implemented",
            HTTPCodes::BadGateway => "502 Bad Gateway",
            HTTPCodes::ServiceUnavailable => "503 Service Unavailable",
            HTTPCodes::GatewayTimeout => "504 Gateway Timeout",
            HTTPCodes::HTTPVersionNotSupported => "505 HTTP Version Not Supported",
        }
    }
}
