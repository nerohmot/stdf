use serde::Serialize;

#[derive(Serialize)]
struct S {
    a: i32,
    b: i32,
    c: String,
}

impl S {
    pub fn to_atdf(&self) -> String {
        let serialized = serde_json::to_string(self).unwrap();
        // println!("{}", serialized); // Output: {"a":1,"b":2,"c":"Hello"}
        let json: serde_json::Value = serde_json::from_str(&serialized).unwrap();
        println!("{:?}", json); // Output: {"a":1,"b":2,"c":"Hello"}

        if let serde_json::Value::Object(map) = json {
            let result = map.values()
                .map(|value| format!("{}", value))
                .collect::<Vec<String>>()
                .join("|");
            return format!("{}\n", result.replace("\"", ""));
        }

        String::new() // fallback
    }
}

fn main() {
    let s = S {
        a: 1,
        b: 2,
        c: String::from("Hello"),
    };

    let result = s.to_atdf(); //.replace("\"", "");
    println!("{}", result); // Output: 1|2|Hello\n
}
