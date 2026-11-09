use super::to_snake_case;

#[test]
fn snake_case_matches_the_model_derives() {
    // The same table as tideorm-macros' case test, so both derive names alike;
    // changing a row renames what an existing `Iden` writes.
    let cases = [
        ("UserProfile", "user_profile"),
        ("HTTPRequest", "http_request"),
        ("MyXMLParser", "my_xml_parser"),
        ("CustomerID", "customer_id"),
        ("camelCase", "camel_case"),
        ("created_at", "created_at"),
        ("sha256", "sha_256"),
        ("address_line1", "address_line_1"),
        ("S3Object", "s_3_object"),
        ("OAuth2Token", "o_auth_2_token"),
        ("a1b2", "a_1_b_2"),
        ("_private", "_private"),
        ("field__double", "field__double"),
        ("Größe", "größe"),
        // `ọ̀` is `ọ` plus a combining grave accent: one grapheme, one letter.
        ("Ọ̀rọ̀Ìwé", "ọ̀rọ̀_ìwé"),
    ];
    for (ident, snake) in cases {
        assert_eq!(to_snake_case(ident), snake, "snake_case of {ident}");
    }
}
