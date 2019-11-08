// Import the wasm output
const { Series } = require('./pkg/series.js');

// Get our series object from Rust
const series = Series.okKo();

// Log the properties to show they are set
console.log(series.name);
console.log(series.seasons);

// Log the object, expecting to see a "name" and "seasons" field
console.log(series);

// Log the object as a value rather than by reference
console.log(JSON.parse(JSON.stringify(series)));
