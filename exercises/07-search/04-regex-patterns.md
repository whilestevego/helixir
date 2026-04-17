# Regex Patterns

## PRACTICE

Replace every number in the log lines with the word "REDACTED".
Use a regex pattern to match all numeric sequences at once.

ERROR 500: connection timeout after 30 seconds (retry 3)
ERROR 401: unauthorized at line 142, column 8
WARN 200: slow query took 1250ms over threshold 800ms
INFO 100: served 4096 requests in 60 seconds

## EXPECTED

Replace every number in the log lines with the word "REDACTED".
Use a regex pattern to match all numeric sequences at once.

ERROR REDACTED: connection timeout after REDACTED seconds (retry REDACTED)
ERROR REDACTED: unauthorized at line REDACTED, column REDACTED
WARN REDACTED: slow query took REDACTEDms over threshold REDACTEDms
INFO REDACTED: served REDACTED requests in REDACTED seconds
