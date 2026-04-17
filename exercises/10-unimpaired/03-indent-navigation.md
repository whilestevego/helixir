# Indent Navigation

## PRACTICE

INDENT NAVIGATION REFERENCE
════════════════════════════

From current line, jump to:
  Next more-indented line:          ???
  Previous more-indented line:      ???
  Next less-indented line:          ???
  Previous less-indented line:      ???
  Next same-indentation line:       ???
  Previous same-indentation line:   ???

Buffer navigation:
  Next buffer:                      ???
  Previous buffer:                  ???
  Last buffer in list:              ???
  First buffer in list:             ???

## EXPECTED

INDENT NAVIGATION REFERENCE
════════════════════════════

From current line, jump to:
  Next more-indented line:          ] +
  Previous more-indented line:      [ +
  Next less-indented line:          ] -
  Previous less-indented line:      [ -
  Next same-indentation line:       ] =
  Previous same-indentation line:   [ =

Buffer navigation:
  Next buffer:                      ] b
  Previous buffer:                  [ b
  Last buffer in list:              ] B
  First buffer in list:             [ B
