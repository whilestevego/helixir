# Surround Workflows

## PRACTICE

```css
.header { color: var(--primary); }
.sidebar { background: var(--surface); }
.button { border: 1px solid var(--accent); }
.footer { color: var(--muted); }
```

```js
log("INFO", "Server started on port 3000");
log("WARN", "Cache miss for user session");
log("ERROR", "Connection refused by upstream");
```

```json
{
  host: "localhost",
  port: 8080,
  debug: true
}
```

## EXPECTED

```css
.header { color: var[--primary]; }
.sidebar { background: var[--surface]; }
.button { border: 1px solid var[--accent]; }
.footer { color: var[--muted]; }
```

```js
log(INFO, "Server started on port 3000");
log(WARN, "Cache miss for user session");
log(ERROR, "Connection refused by upstream");
```

```json
{
  "host": "localhost",
  "port": 8080,
  "debug": true
}
```
