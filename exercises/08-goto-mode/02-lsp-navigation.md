# LSP Navigation

## PRACTICE

```ts
interface Logger {
  log(message: string): void;
}

class ConsoleLogger implements Logger {
  log(message: string): void {
    console.log(message);
  }
}

function createLogger(): Logger {
  return new ConsoleLogger();
}

// With cursor on "Logger" in createLogger's return type:
// g d would go to:
// g i would go to:

// With cursor on "log" in console.log(message):
// g d would go to:
// g r would go to:
```

## EXPECTED

```ts
interface Logger {
  log(message: string): void;
}

class ConsoleLogger implements Logger {
  log(message: string): void {
    console.log(message);
  }
}

function createLogger(): Logger {
  return new ConsoleLogger();
}

// With cursor on "Logger" in createLogger's return type:
// g d would go to: interface Logger (line 1)
// g i would go to: class ConsoleLogger (line 5)

// With cursor on "log" in console.log(message):
// g d would go to: log method in ConsoleLogger (line 6)
// g r would go to: all usages of log in the file
```
