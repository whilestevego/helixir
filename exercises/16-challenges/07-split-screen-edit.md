# Split Screen Edit

## PRACTICE

Fill in the window management commands for each workflow.

WORKFLOW 1: Edit a test file alongside its source
  Step 1: Open src/auth.ts
  Step 2: ???  (vertical split)
  Step 3: ???  (open file picker)
  Step 4: Open tests/auth.test.ts

WORKFLOW 2: View three files at once
  Step 1: Open file A
  Step 2: ???  (vertical split)
  Step 3: Open file B
  Step 4: ???  (horizontal split)
  Step 5: Open file C

WORKFLOW 3: Focus on one file, then restore
  Step 1: ???  (close all other panes)
  Step 2: ... edit ...
  Step 3: ???  (re-split to get back)

## EXPECTED

Fill in the window management commands for each workflow.

WORKFLOW 1: Edit a test file alongside its source
  Step 1: Open src/auth.ts
  Step 2: Ctrl-w v  (vertical split)
  Step 3: Space f  (open file picker)
  Step 4: Open tests/auth.test.ts

WORKFLOW 2: View three files at once
  Step 1: Open file A
  Step 2: Ctrl-w v  (vertical split)
  Step 3: Open file B
  Step 4: Ctrl-w s  (horizontal split)
  Step 5: Open file C

WORKFLOW 3: Focus on one file, then restore
  Step 1: Ctrl-w o  (close all other panes)
  Step 2: ... edit ...
  Step 3: Ctrl-w v  (re-split to get back)
