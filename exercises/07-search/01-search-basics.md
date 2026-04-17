# Search Basics

## PRACTICE

[2025-01-15 08:23:01] INFO  Server started on port 8080
[2025-01-15 08:23:14] INFO  Connected to database
[2025-01-15 08:24:02] ERROR Failed to load config: file not found
[2025-01-15 08:24:15] INFO  Using default configuration
[2025-01-15 08:25:33] INFO  Processing batch job #4471
[2025-01-15 08:25:34] ERROR Timeout connecting to cache server
[2025-01-15 08:26:01] INFO  Retrying with fallback cache
[2025-01-15 08:27:12] ERROR Disk space critical: 98% used
[2025-01-15 08:27:45] INFO  Cleanup job started
[2025-01-15 08:28:03] ERROR Authentication service unreachable

## EXPECTED

[2025-01-15 08:23:01] INFO  Server started on port 8080
[2025-01-15 08:23:14] INFO  Connected to database
[2025-01-15 08:24:15] INFO  Using default configuration
[2025-01-15 08:25:33] INFO  Processing batch job #4471
[2025-01-15 08:26:01] INFO  Retrying with fallback cache
[2025-01-15 08:27:45] INFO  Cleanup job started
