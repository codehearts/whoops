# Deserializing `avro_rs` Types to `rsgen_avro` Unions

## Reproducing the Issue

Clone this repo:

```sh
git clone https://github.com/codehearts/whoops.git
cd whoops/rust/deserializing-avro_rs-types-to-rsgen_avro-unions/
```

Or perform a sparse checkout of this example only:

```sh
git clone --no-checkout --depth 1 https://github.com/codehearts/whoops.git
cd whoops/
git config core.sparseCheckout true
echo "rust/deserializing-avro_rs-types-to-rsgen_avro-unions/" > .git/info/sparse-checkout
git checkout master
cd rust/deserializing-avro_rs-types-to-rsgen_avro-unions/
```

To build and run this example:

```sh
docker-compose up
```

The container will run tests exhibiting the issue and showing that the easy fix is not perfect.

## The Issue

Consider the following Rust types generated for an event by `rsgen_avro`:

```rust
/// Auto-generated type for unnamed Avro union variants.
#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub enum UnionIntLongBool {
    Int(i32),
    Long(i64),
    Bool(bool),
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Event {
    pub a: Option<UnionIntLongBool>,
}

impl Default for Event {
    fn default() -> Event {
        Event { a: None }
    }
}
```

When deserializing from an `avro_rs` value where the union's type is `null`, this code executes just fine:

```rust
let avro_value = Value::Record(vec![("a".to_string(), Value::Union(Box::new(Value::Null)))]);
let my_event = from_value::<Event>(&avro_value)?; // Ok(Event { a: None })
```

However, if the union's type is _not_ `null` the deserialization will fail:

```rust
let avro_value = Value::Record(vec![("a".to_string(), Value::Union(Box::new(Value::Int(123))))]);
let my_event = from_value::<Event>(&avro_value)?; // Err(avro_rs::Error::DeserializeValue("not an enum"))
```

The reason, as far as I understand it, is that `avro_rs` uses its own `serde::Deserializer` type when deserializing `avro_rs::types::Value`, and the generated `#[derive(serde::Deseralize)]` implementation on `UnionIntLongBool` simply calls `deserialize_enum()` on that deserializer. That's not correct in this case because the `avro_rs::types::Value::Union` contains either an `int` or a `long`, which their deserializer then fails with the error message `"not an enum"`.

## The Easy Solution

The easiest solution is to annotate the auto-generated enum types with `#[serde(untagged)]`.

```rust
/// Auto-generated type for unnamed Avro union variants.
#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum UnionIntLongBool {
    Int(i32),
    Long(i64),
    Bool(bool),
}
```

The only issue seems to be when deserializing similar types, such as `int` and `long`. The following example shows how a `long` value gets deserialized to the `Int` variant:

```rust
let avro_value = Value::Record(vec![("a".to_string(), Value::Union(Box::new(Value::Long(123))))]);
let my_event = from_value::<Event>(&avro_value)?; // Ok(Event{ a: Some(UnionIntLongBool::Int(123)) })
```

This might be an acceptable tradeoff, since the fix is simple and the deserialization worked. If the developer needs the type as an `i64`, they can always cast the inner value of the `Int` variant.

## The Painful Solution

There is a solution that avoids the caveat of the easy solution, but it's a huge pain to implement. It involves removing the `#[derive(serde::Deserialize)]` annotation and implementing a `serde::de::Visitor` and `serde::Deserializer` for the auto-generated union type. The code for this is long, but you can find it in [`src/painful-fix.rs`](https://github.com/codehearts/whoops/blob/master/rust/deserializing-avro_rs-types-to-rsgen_avro-unions/src/painful-fix.rs). The obvious downside here is the sheer amount of additional code to generate, which may not be worth it.
