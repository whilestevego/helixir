# Register Relay

## PRACTICE

```sql
Using named registers, assemble the SQL query below.
Yank each fragment into a separate register, then paste
them in order on the RESULT line.

Fragment A: SELECT name, email
Fragment B: FROM users
Fragment C: WHERE active = true
Fragment D: ORDER BY name;

RESULT:

```

## EXPECTED

```sql
Using named registers, assemble the SQL query below.
Yank each fragment into a separate register, then paste
them in order on the RESULT line.

Fragment A: SELECT name, email
Fragment B: FROM users
Fragment C: WHERE active = true
Fragment D: ORDER BY name;

RESULT:
SELECT name, email FROM users WHERE active = true ORDER BY name;
```
