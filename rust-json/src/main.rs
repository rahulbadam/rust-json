use serde::{Deserialize, Serialize};
use serde_json::Number;

#[derive(Serialize, Deserialize)]
struct Story {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Book {
    name: String,
    author: String,
    pages: Number,
    stories: Vec<Story>,
}

fn main() {
    let json = r#"
    {
        "name" : "book1",
        "author" : "rahul",
        "pages": 5,
        "stories": [
            {
                "name":"stroy 1"
            },
            {
                "name":"stroy 2"
            },
            {
                "name":"stroy 3"
            }
        ]
    }"#;
    let parsed: Book = parsed_json(json);
    for story_name in parsed.stories {
        println!("Story Name{}", story_name.name);
    }
}

fn parsed_json(json: &str) -> Book {
    let parsed: Book = serde_json::from_str(json).unwrap();
    return parsed;
}
