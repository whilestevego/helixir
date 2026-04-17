# Vim to Helix Translation

## PRACTICE

```js
VIM COMMAND          HELIX EQUIVALENT     DESCRIPTION
────────────────────────────────────────────────────────
diw                  ???                  Delete inner word
ci{                  ???                  Change inside braces
dap                  ???                  Delete around paragraph
yiw                  ???                  Yank inner word
vip                  ???                  Select inner paragraph
das                  ???                  Delete around sentence (Zed-specific)
cs"'                 ???                  Change surround " to '

NOW PRACTICE:

Delete the word "DELETEME" from this line: hello DELETEME world
Change "old" to "new" inside the quotes: config("old")
Delete the content inside these braces: { remove all of this }
```

## EXPECTED

```js
VIM COMMAND          HELIX EQUIVALENT     DESCRIPTION
────────────────────────────────────────────────────────
diw                  miwd                 Delete inner word
ci{                  mi{c                 Change inside braces
dap                  mapd                 Delete around paragraph
yiw                  miwy                 Yank inner word
vip                  mip                  Select inner paragraph
das                  masd                 Delete around sentence (Zed-specific)
cs"'                 mr"'                 Change surround " to '

NOW PRACTICE:

Delete the word "DELETEME" from this line: hello world
Change "old" to "new" inside the quotes: config("new")
Delete the content inside these braces: {}
```
