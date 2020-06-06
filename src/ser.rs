use lexpr::Value;

use serde::ser::{SerializeSeq, SerializeTuple};
use serde::{Serialize, Serializer};

use crate::LispIR;

fn serialize_type<T, S>(serializer: S, typ: &str, val: T) -> Result<S::Ok, S::Error>
where
    T: Serialize,
    S: Serializer,
{
    let mut tup = serializer.serialize_tuple(2)?;
    tup.serialize_element(typ)?;
    tup.serialize_element(&val)?;
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
        serialize_type(self.0, "u64", n)
    }

    fn visit_i64(self, n: i64) -> Result<Self::Value, Self::Error> {
        serialize_type(self.0, "i64", n)
    }

    fn visit_f64(self, n: f64) -> Result<Self::Value, Self::Error> {
        serialize_type(self.0, "f64", n)
    }
}

impl Serialize for LispIR<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match &self.0 {
            Value::Nil => serializer.serialize_none(),
            Value::Null => {
                let seq = serializer.serialize_seq(Some(0))?;
                seq.end()
            }
            Value::Bool(val) => serialize_type(serializer, "bool", val),
            Value::Number(num) => num.visit(LispNumberSerializer(serializer)),
            Value::Char(val) => serialize_type(serializer, "char", val),
            Value::String(val) => serialize_type(serializer, "str", val),
            Value::Symbol(val) => serialize_type(serializer, "symbol", val),
            Value::Keyword(val) => serialize_type(serializer, "keyword", val),
            Value::Bytes(bytes) => serializer.serialize_bytes(&bytes),
            Value::Cons(val) => {
                let mut tup = serializer.serialize_tuple(2)?;
                tup.serialize_element(&LispIR(val.car()))?;
                tup.serialize_element(&LispIR(val.car()))?;
                tup.end()
            }
            Value::Vector(val) => {
                let mut seq = serializer.serialize_seq(Some(val.len()))?;
                for e in val.iter() {
                    seq.serialize_element(&LispIR(e))?;
                }
                seq.end()
            }
        }
    }
}
