# Lessons Log — v1.0-brownfield-backfill

## L-EDP1-007 — Always check path guards before reading files

Category: implementation
Date: 2026-04-01

Path guards using `Path::file_name()` are required for all four target file checks. Using
`ends_with` is insufficient because it matches partial path components.

Next application: every new hook plugin added to the registry must use the canonical
`Path::file_name()` form.
