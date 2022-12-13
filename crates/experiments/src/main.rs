mod tsq;

use serde::de::{self, MapAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum PrereqTree {
    Only(String),
    And { and: Vec<PrereqTree> },
    Or { or: Vec<PrereqTree> },
}

#[derive(Debug)]
pub struct Person {
    name: String,
}

impl Person {
    fn new(name: String) -> Self {
        Self { name }
    }
}

impl Serialize for Person {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Person", 1)?;
        state.serialize_field("name", &self.name)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Person {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Name,
        }
        struct PersonVisitor;

        impl<'de> Visitor<'de> for PersonVisitor {
            type Value = Person;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("struct Person")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Person, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut name = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Name => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                    }
                }
                let name =
                    name.ok_or_else(|| de::Error::missing_field("name"))?;
                Ok(Person::new(name))
            }
        }

        const FIELDS: &'static [&'static str] = &["name"];
        deserializer.deserialize_struct("Person", FIELDS, PersonVisitor)
    }
}

fn main() {
    println!("HELLO!");
    let data = r#"
        {
            "name": "John Doe"
        }"#;
    let john: Person = serde_json::from_str(data).unwrap();
    println!("{john:?}");
}
