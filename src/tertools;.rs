use itertools::Itertools;
use serde::Deserialize;
use serde_json::value::RawValue;
use std::hash::{Hash, Hasher};

#[derive(Deserialize, Debug)]
#[serde(transparent)]
struct RawValueHashed<'a> {
    #[serde(borrow)]
    raw_value: &'a RawValue,
}

impl<'a> Hash for RawValueHashed<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw_value.get().hash(state);
    }
}

impl<'a> PartialEq for RawValueHashed<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.raw_value.get() == other.raw_value.get()
    }
}

impl<'a> Eq for RawValueHashed<'a> {}

fn json_loads(raw: &str) -> serde_json::Result<Vec<RawValueHashed>> {
    let _req: Vec<RawValueHashed> = serde_json::from_str(raw)?;

    Ok(_req)
}

// fn permutate(items: &Vec<&RawValueHashed>) {
//     for perm in items.iter().permutations(items.len()).unique() {
//         println!("{:?}", perm);
//     }
// }

fn main() {
    let source = r#"
[1,2,3]
"#;
    let items = json_loads(&source).unwrap();
    //println!("{:?}", items);
    for perm in items.iter().permutations(items.len()).unique() {
        println!("{:?}", perm);
    }
}
