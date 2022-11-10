use enum_as_inner::EnumAsInner;
use fxhash::FxHashMap;

use crate::{container::ContainerID, InternalString};

/// [LoroValue] is used to represents the state of CRDT at a given version
#[derive(Debug, PartialEq, Clone, serde::Serialize, EnumAsInner)]
pub enum LoroValue {
    Null,
    Bool(bool),
    Double(f64),
    I32(i32),
    String(Box<str>),
    List(Box<Vec<LoroValue>>),
    Map(Box<FxHashMap<InternalString, LoroValue>>),
    Unresolved(Box<ContainerID>),
}

impl Default for LoroValue {
    fn default() -> Self {
        LoroValue::Null
    }
}

impl From<FxHashMap<InternalString, LoroValue>> for LoroValue {
    fn from(map: FxHashMap<InternalString, LoroValue>) -> Self {
        LoroValue::Map(Box::new(map))
    }
}

impl From<Vec<LoroValue>> for LoroValue {
    fn from(vec: Vec<LoroValue>) -> Self {
        LoroValue::List(Box::new(vec))
    }
}

impl From<InsertValue> for LoroValue {
    fn from(v: InsertValue) -> Self {
        match v {
            InsertValue::Null => LoroValue::Null,
            InsertValue::Bool(b) => LoroValue::Bool(b),
            InsertValue::Double(d) => LoroValue::Double(d),
            InsertValue::Int32(i) => LoroValue::I32(i),
            InsertValue::String(s) => LoroValue::String(s),
            InsertValue::Container(c) => LoroValue::Unresolved(c),
        }
    }
}

impl From<LoroValue> for InsertValue {
    fn from(v: LoroValue) -> Self {
        match v {
            LoroValue::Null => InsertValue::Null,
            LoroValue::Bool(b) => InsertValue::Bool(b),
            LoroValue::Double(d) => InsertValue::Double(d),
            LoroValue::I32(i) => InsertValue::Int32(i),
            LoroValue::String(s) => InsertValue::String(s),
            LoroValue::Unresolved(c) => InsertValue::Container(c),
            _ => unreachable!("Unsupported convert from LoroValue to InsertValue"),
        }
    }
}

impl From<i32> for LoroValue {
    fn from(v: i32) -> Self {
        LoroValue::I32(v)
    }
}

impl From<f64> for LoroValue {
    fn from(v: f64) -> Self {
        LoroValue::Double(v)
    }
}

impl From<bool> for LoroValue {
    fn from(v: bool) -> Self {
        LoroValue::Bool(v)
    }
}

impl From<&str> for LoroValue {
    fn from(v: &str) -> Self {
        LoroValue::String(v.into())
    }
}

impl From<String> for LoroValue {
    fn from(v: String) -> Self {
        LoroValue::String(v.into())
    }
}

/// [InsertValue] can be inserted to Map or List
/// It's different from [LoroValue] because some of the states in [LoroValue] are illegal to be inserted
#[derive(Debug, PartialEq, Clone)]
pub enum InsertValue {
    Null,
    Bool(bool),
    Double(f64),
    Int32(i32),
    String(Box<str>),
    Container(Box<ContainerID>),
}

#[cfg(feature = "wasm")]
pub mod wasm {
    use js_sys::{Array, Object};
    use wasm_bindgen::{JsValue, __rt::IntoJsResult};

    use crate::LoroValue;

    use super::InsertValue;

    pub fn convert(value: LoroValue) -> JsValue {
        match value {
            LoroValue::Null => JsValue::NULL,
            LoroValue::Bool(b) => JsValue::from_bool(b),
            LoroValue::Double(f) => JsValue::from_f64(f),
            LoroValue::I32(i) => JsValue::from_f64(i as f64),
            LoroValue::String(s) => JsValue::from_str(&s),
            LoroValue::List(list) => {
                let arr = Array::new_with_length(list.len() as u32);
                for v in list.into_iter() {
                    arr.push(&convert(v));
                }

                arr.into_js_result().unwrap()
            }
            LoroValue::Map(m) => {
                let map = Object::new();
                for (k, v) in m.into_iter() {
                    let str: &str = &k;
                    js_sys::Reflect::set(&map, &JsValue::from_str(str), &convert(v)).unwrap();
                }

                map.into_js_result().unwrap()
            }
            LoroValue::Unresolved(_) => {
                unreachable!()
            }
        }
    }

    impl From<LoroValue> for JsValue {
        fn from(value: LoroValue) -> Self {
            convert(value)
        }
    }

    impl InsertValue {
        pub fn try_from_js(value: JsValue) -> Result<InsertValue, JsValue> {
            if value.is_null() {
                Ok(InsertValue::Null)
            } else if value.as_bool().is_some() {
                Ok(InsertValue::Bool(value.as_bool().unwrap()))
            } else if value.as_f64().is_some() {
                Ok(InsertValue::Double(value.as_f64().unwrap()))
            } else if value.is_string() {
                Ok(InsertValue::String(value.as_string().unwrap().into()))
            } else {
                Err(value)
            }
        }
    }
}

#[cfg(test)]
pub(crate) mod proptest {
    use proptest::prelude::*;
    use proptest::prop_oneof;

    use super::InsertValue;

    pub fn gen_insert_value() -> impl Strategy<Value = InsertValue> {
        prop_oneof![
            Just(InsertValue::Null),
            any::<f64>().prop_map(InsertValue::Double),
            any::<i32>().prop_map(InsertValue::Int32),
            any::<bool>().prop_map(InsertValue::Bool),
            any::<String>().prop_map(|s| InsertValue::String(s.into())),
        ]
    }
}
