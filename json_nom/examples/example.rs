use json_nom::j_value::JValue;

fn main() {
  let json = r#"
      {
        "a": [
          1,
          "hola",
          {
            "x": true
          }
        ],
        "b": {
          "y": [true, false, null]
        }
      }
    "#;
  let (_, jvalue) = JValue::parse(json).unwrap();
  println!("{jvalue:?}");
}
