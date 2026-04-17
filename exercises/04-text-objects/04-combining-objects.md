# Combining Objects with Actions

## PRACTICE

```js
const env = "PLACEHOLDER";
const apiUrl = "http://staging.internal/api";

if (env === "production") {
  console.log("Starting production server...");
  console.log("Loaded " + modules.length + " modules");
}

// [LEGACY] This handler will be removed in v3.0
function onConnect(socket) {
  fetchData(userId, sessionToken, { cache: true });
  processData(TODO);
}
```

## EXPECTED

```js
const env = "production";
const apiUrl = "https://api.example.com/v2";

if (env === "production") {
}

// This handler will be removed in v3.0
function onConnect(socket) {
  fetchData(userId, sessionToken, { cache: true });
  processData(userId, sessionToken, { cache: true });
}
```
