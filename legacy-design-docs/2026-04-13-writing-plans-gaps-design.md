# Writing-Plans Gaps — Design Spec

## Summary

Adopt 5 insights from superpowers:writing-plans into vsdd-factory's story decomposition and creation skills.

## Deliverables

### 1. Hard Gate Language

- **decompose-stories:** Do NOT skip to implementation or story delivery. ALL stories MUST be decomposed, dependency-ordered into waves, and approved before any code is written.
- **create-story:** Do NOT skip to implementation or deliver the story before it is fully elaborated. Every mandatory section MUST be completed — no stub stories.

### 2. Scope Decomposition Check (decompose-stories only)

Quick check before decomposing: verify the PRD describes a single product. If multiple independent products, stop and split first.

### 3. "Plan Failures" Anti-Pattern List (both skills)

Explicit ban: "TBD"/"TODO", "add appropriate error handling", "write tests for the above", "similar to STORY-NNN", untestable ACs, vague file lists, how-less tasks.

### 4. Self-Review Checklist (both skills)

- decompose-stories: spec coverage, placeholder scan, consistency, sizing
- create-story: completeness, consistency, testability, context budget

### 5. Execution Reference (story-template.md)

Add `/vsdd-factory:deliver-story STORY-NNN` pointer to the template.

## Files Modified

- `skills/decompose-stories/SKILL.md`
- `skills/create-story/SKILL.md`
- `templates/story-template.md`
