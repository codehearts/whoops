# Debugging WASM-exported Rust Structs

## Reproducing the Issue

Clone this repo:

```sh
git clone https://github.com/codehearts/whoops.git
cd whoops/rust/debug-wasm-exported-rust-struct/
```

Or perform a sparse checkout of this example only:

```sh
git clone --no-checkout --depth 1 https://github.com/codehearts/whoops.git
cd whoops/
git config core.sparseCheckout true
echo "rust/debug-wasm-exported-rust-struct/" > .git/info/sparse-checkout
git checkout master
cd rust/debug-wasm-exported-rust-struct/
```

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

## The Solution

For Node.js at least, the output of `console.log` can be customized by defining a `[util.inspect.custom]` method within your class. `JSON.stringify` always calls `toJSON` on an object if it is defined. Knowing this, the generated `pkg/series.js` can be updated as follows:

```javascript
const util = require('util');

class Series {

    // ...

    // Overrides the output of `JSON.stringify`
    toJSON() {
        return {
          name: this.name,
          seasons: this.seasons,
        };
    }

    // Returns the JSON representation when inspected by `console.log` and friends
    [util.inspect.custom](depth, options) {
        return this.toJSON();
    }

    // ...

}
```

The output is now much more useful:

```javascript
console.log(series); // { name: "OK K.O.! Let's Be Heroes", seasons: 3 }
console.log(JSON.parse(JSON.stringify(series))); // { name: "OK K.O.! Let's Be Heroes", seasons: 3 }
```

The `toJSON` method can be defined in the Rust bindings, but it is not possible to create a binding for `[util.inspect.custom]` as the name creates an invalid symbol during compilation
