# DEC-020 - Markdown vision authority

- Status: accepted
- Class: C
- Date: 2026-07-11

## Decision

`docs/vision.md` is the maintained and authoritative vision source. The original
`doc/Vision_Atlas_Executable_MVP1-4.docx` remains an immutable snapshot of
version 0.1 and is not edited further.

## Consequences

Vision changes are textual, diffable, and reviewable in Git. Conversion artifacts
such as HTML tables may be simplified incrementally without changing meaning.
When Markdown and DOCX differ, Markdown is authoritative.
