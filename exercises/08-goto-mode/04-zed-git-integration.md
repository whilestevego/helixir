# Zed Git Integration

## PRACTICE

GIT STAGING WORKFLOW
═══════════════════════════

To review changes in a file:
  1. Open the modified file
  2. Press ??? to see inline diffs for changed regions

To selectively stage changes:
  1. Navigate to the first hunk you want to stage
  2. Press ??? to stage it and auto-jump to the next hunk
  3. If a hunk should NOT be staged, just press ] c to skip it
  4. To unstage a hunk you staged by mistake, press ???

To discard unwanted changes:
  1. Navigate to the hunk you want to discard
  2. Press ??? to restore the original content

QUICK REFERENCE:
  Toggle diff view:     ???
  Stage + next:         ???
  Unstage + next:       ???
  Discard changes:      ???
  Next hunk:            ] c
  Previous hunk:        [ c

## EXPECTED

GIT STAGING WORKFLOW
═══════════════════════════

To review changes in a file:
  1. Open the modified file
  2. Press g o to see inline diffs for changed regions

To selectively stage changes:
  1. Navigate to the first hunk you want to stage
  2. Press g u to stage it and auto-jump to the next hunk
  3. If a hunk should NOT be staged, just press ] c to skip it
  4. To unstage a hunk you staged by mistake, press g U

To discard unwanted changes:
  1. Navigate to the hunk you want to discard
  2. Press g R to restore the original content

QUICK REFERENCE:
  Toggle diff view:     g o
  Stage + next:         g u
  Unstage + next:       g U
  Discard changes:      g R
  Next hunk:            ] c
  Previous hunk:        [ c
