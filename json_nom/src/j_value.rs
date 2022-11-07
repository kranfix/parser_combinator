//use std::collections::HashMap;

use crate::j_num::Dec;

#[derive(Debug)]
pub enum JValue {
  //Str(String),
  Num(Dec),
  //Obj(HashMap<String, JValue>),
  //Array(Vec<JValue>),
  Bool(bool),
  Null,
}
