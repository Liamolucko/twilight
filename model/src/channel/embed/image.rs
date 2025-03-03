use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EmbedImage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::EmbedImage;
    use serde_test::Token;

    #[test]
    fn test_embed_image() {
        let value = EmbedImage {
            height: Some(1440),
            proxy_url: Some("https://cdn.example.com/1-hash.png".to_owned()),
            url: "https://example.com/1.png".to_owned(),
            width: Some(2560),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "EmbedImage",
                    len: 4,
                },
                Token::Str("height"),
                Token::Some,
                Token::U64(1440),
                Token::Str("proxy_url"),
                Token::Some,
                Token::Str("https://cdn.example.com/1-hash.png"),
                Token::Str("url"),
                Token::Str("https://example.com/1.png"),
                Token::Str("width"),
                Token::Some,
                Token::U64(2560),
                Token::StructEnd,
            ],
        );
    }
}
