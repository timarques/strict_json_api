use core::fmt::Debug;
use serde::{Deserialize, Serialize};
use strict_json_api::document::DocumentSuccessResponse;
use strict_json_api::present::NotPresent;
use strict_json_api::resource::Resource;
use strict_json_api::resource_identifier::ResourceIdentifierWithoutLid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArticleAttributes {
    pub title: String,
    pub word_count: u32,
    pub author_email: Option<String>,
}

type ArticleIdentifier = ResourceIdentifierWithoutLid<String, String, NotPresent>;
type ArticleResource =
    Resource<ArticleIdentifier, Option<ArticleAttributes>, NotPresent, NotPresent>;
type ArticleDocument =
    DocumentSuccessResponse<ArticleResource, NotPresent, NotPresent, NotPresent, NotPresent>;

#[test]
fn test_01() {
    let json_string = r#"{
        "data": {
            "type": "articles",
            "id": "123",
            "attributes": {
                "title": "JSON:API Explained",
                "word_count": 1500
            }
        }
    }"#;

    let doc: ArticleDocument = serde_json::from_str(json_string).expect("Failed to deserialize");

    let resource = doc.data();

    let resource_type: &String = resource.identifier().resource_type();
    let resource_id: &String = resource.identifier().id();
    assert_eq!(resource_type, "articles");
    assert_eq!(resource_id, "123");
    assert!(resource.attributes().is_some());

    let result = serde_json::to_string(json_string);
    assert!(result.is_ok());
}
