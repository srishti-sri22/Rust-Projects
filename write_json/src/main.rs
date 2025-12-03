use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Paragraph{
    name: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Article{
    article:String, 
    author: String,
    paragraphs: Vec<Paragraph>
}

fn main() {
    let article: Article = Article {
	article: String::from("how to work with json in Rust"),
	author: String::from("akhil"),
	paragraphs: vec![
	    Paragraph {
		name: String::from("first sentence")
	    },
	    Paragraph {
		name: String::from("body of the article")
	    },
	    Paragraph {
		name: String::from("end of paragraph")
	    }
	]
    };

    let json = serde_json::to_string(&article).unwrap();
    println!("Value to JSON: {}", json);
}
