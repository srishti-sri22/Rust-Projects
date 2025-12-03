use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug, Deserialize)]
struct Paragraph{
    name:String
}

#[derive(Serialize, Deserialize)]
struct Article{
    article:String,
    author:String,
    paragraph: Vec<Paragraph>
}

fn main() {
          let json = r#"
{
  "article": "how to work with json in Rust",
  "author": "akhil",
  "paragraph": [
    {
      "name": "starting sentences"
    },
    {
      "name": "body of the paragraph"
    },
    {
      "name": "end of the paragraph"
    }
  ]
}
"#;    

let parsed: Article = read_json_typed(json);
println!("The name of the first pragraph is :{}", parsed.paragraph[0].name);
}

fn read_json_typed(raw_json: &str) -> Article{
   let parsed = serde_json::from_str(raw_json).unwrap();
   return parsed;
}


