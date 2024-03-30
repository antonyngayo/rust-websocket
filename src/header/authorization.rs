use std::{ops::Deref, str::FromStr};

use hyper::header::{parsing::from_comma_delimited, Header, HeaderFormat};
use websocket_base::result::WebSocketError;

use crate::WebSocketResult;

/// Represents a Authorization header
#[derive(PartialEq, Clone, Debug)]
pub struct Authorization(pub String);

impl Deref for Authorization {
	type Target = String;

	fn deref(&self) -> &String {
		&self.0
	}
}

impl FromStr for Authorization {
	type Err = WebSocketError;

	fn from_str(s: &str) -> WebSocketResult<Authorization> {
		Ok(Authorization(s.to_string()))
	}
}

impl Header for Authorization {
	fn header_name() -> &'static str {
		"Authorization"
	}
	fn parse_header(raw: &[Vec<u8>]) -> hyper::Result<Authorization> {
		from_comma_delimited(raw)
			.map(|x: Vec<u8>| Authorization(String::from_utf8_lossy(&x).to_string()))
	}
}

impl HeaderFormat for Authorization {
	fn fmt_header(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let Authorization(ref value) = *self;
		write!(f, "{}", value)
	}
}

impl std::fmt::Display for Authorization {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	// use hyper::header::Header;
	use test;

	#[test]
	fn test_header_authorization() {
		use crate::header::Headers;
		let authorization = Authorization("Bearer SomeValue".to_string());
		let mut headers = Headers::new();
		headers.set(authorization);
		assert_eq!(
			&headers.to_string()[..],
			"Authorization: Bearer SomeValue\r\n"
		);
	}
}
