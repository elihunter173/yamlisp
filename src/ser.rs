use lexpr::Value;

use serde::ser::{SerializeSeq, SerializeMap, SerializeTuple};
use serde::{Serialize, Serializer};

use crate::LispIR;

fn serialize_special<T, S>(serializer: S, typ: &str, val: T) -> Result<S::Ok, S::Error>
where
    T: Serialize,
    S: Serializer,
{
    let mut tup = serializer.serialize_map(Some(1))?;
    tup.serialize_entry(typ, &val)?;
    tup.end()
}

struct LispNumberSerializer<S: Serializer>(S);

impl<S: Serializer> lexpr::number::Visitor for LispNumberSerializer<S> {
    type Value = S::Ok;
    type Error = S::Error;

    fn error<T: Into<String>>(msg: T) -> Self::Error {
        use serde::ser::Error;
        S::Error::custom(msg.into())
    }

    fn visit_u64(self, n: u64) -> Result<Self::Value, Self::Error> {
        self.0.serialize_u64(n)
    }

    fn visit_i64(self, n: i64) -> Result<Self::Value, Self::Error> {
        self.0.serialize_i64(n)
    }

    fn visit_f64(self, n: f64) -> Result<Self::Value, Self::Error> {
        self.0.serialize_f64(n)
    }
}

// TODO: Improve rendering of cons

impl Serialize for LispIR<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match &self.0 {
            Value::Nil => serializer.serialize_none(),
            Value::Null => {
                let tup = serializer.serialize_tuple(0)?;
                tup.end()
            }
            Value::Bool(val) => serializer.serialize_bool(*val),
            Value::Number(num) => num.visit(LispNumberSerializer(serializer)),
            // Value::Char(val) => serialize_special(serializer, "c", val),
            Value::Char(val) => serializer.serialize_char(*val),
            Value::String(val) => serializer.serialize_str(val),
            Value::Symbol(val) => serialize_special(serializer, "s", val),
            Value::Keyword(val) => serialize_special(serializer, "keyword", val),
            Value::Bytes(bytes) => serializer.serialize_bytes(&bytes),
            Value::Cons(val) => {
                if let Value::Cons(_) = val.cdr() {
                    let mut seq = serializer.serialize_seq(None)?;
                    for cell in val.iter() {
                        seq.serialize_element(&LispIR(cell.car()))?;
                        match cell.cdr() {
                            Value::Cons(_) | Value::Null => {},
                            _ => seq.serialize_element(&LispIR(cell.cdr()))?,
                        }
                    }
                    seq.end()
                } else {
                    serialize_special(serializer, "cons", (&LispIR(val.car()), &LispIR(val.cdr())))
                }
            }
            Value::Vector(val) => {
                serialize_special(serializer, "vector", val.iter().map(|e| LispIR(e)).collect::<Vec<_>>())
            }
        }
    }
}
