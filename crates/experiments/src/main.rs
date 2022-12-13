use serde::{Deserialize, Deserializer};

enum Semester {
    Semester1,
    Semester2,
    SpecialTerm1,
    SpecialTerm2,
}

struct Test {
    semesters: Vec<Semester>,
}

impl<'de> Deserialize<'de> for Semester {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Semester::Semester1)
    }
}

fn main() {
    let data = r#"
        {
            "semesters": [1, 2, 4],
        }"#;
    // let t: Test = serde_json::from_str(data).unwrap();

    println!("Hello, world!");
}
