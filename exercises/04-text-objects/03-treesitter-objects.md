# Tree-sitter Objects

## PRACTICE

```js
function formatUser(name, email, role) {
  const display = `${name} <${email}>`;
  return { display, role };
}

// DEPRECATED: use schema validation instead
function validateEmail(input) {
  const regex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return regex.test(input);
}

function calculateTotal(items, taxRate, discount) {
  const subtotal = items.reduce((sum, item) => sum + item.price, 0);
  const tax = subtotal * taxRate;
  const total = subtotal + tax - discount;
  return Math.round(total * 100) / 100;
}
```

## EXPECTED

```js
function formatUser(name, email) {
  const display = `${name} <${email}>`;
  return { display };
}

function calculateTotal(items, taxRate, discount) {
}
```
