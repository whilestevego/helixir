# Search with Selection

## PRACTICE

```js
function validateUser(userName) {
    if (!userName || userName.length === 0) {
        throw new Error("userName is required");
    }
    const sanitized = userName.trim().toLowerCase();
    console.log("Validating userName:", userName);
    return sanitized;
}
```

## EXPECTED

```js
function validateUser(accountName) {
    if (!accountName || accountName.length === 0) {
        throw new Error("accountName is required");
    }
    const sanitized = accountName.trim().toLowerCase();
    console.log("Validating accountName:", accountName);
    return sanitized;
}
```
