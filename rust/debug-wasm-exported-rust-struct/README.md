# Debugging WASM-exported Rust Structs

## Reproducing the Issue

To build and run this example:

```sh
docker-compose up
```

The container will output the name and seasons of a TV series object, and then output the object itself

## The Issue

Consider the following exported Rust struct:

```rust
#[wasm_bindgen]
pub struct Series {
    name: String,
    pub seasons: u8,
}

#[wasm_bindgen]
impl Series {
    #[wasm_bindgen(js_name=okKo)]
    pub fn ok_ko() -> Self {
        Self {
            name: "OK K.O.! Let's Be Heroes".to_string(),
            seasons: 3,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }
}
```

In the output from `wasm-pack build`, the `Series` object has the following properties:

```javascript
const series = Series.okKo();

console.log(series.name); // OK K.O.! Let's Be Heroes
console.log(series.seasons); // 3
```

However, when logging the object we see neither of these properties:

```javascript
console.log(series); // Series { ptr: 1114136 }
console.log(JSON.parse(JSON.stringify(series))); // { ptr: 1114136 }
```

Ideally the name and seasons would be included in the log output, as the pointer itself is not useful when debugging. Using an IDE debugger helps alleviate this with features that can invoke getter methods on objects, but it's difficult to record the state of the object programmatically
