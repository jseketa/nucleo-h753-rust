# Lesson format

Embedded Rust on the NUCLEO-H753ZI. One program, `src/main.rs`, grows lesson by lesson; git keeps the record.

Draft. Every lesson has the same six parts, in this order. One file per lesson: `lesson-NN.md`.

## 1. Goal

- One line: what the board can do afterwards.
- **Done when**: the visible win that ends the lesson (LED behaviour, a value in the log, terminal text).

## 2. Steps

Each step is a short block:

```markdown
## Step NN · title                                   tag: lesson-NN-stepNN

1. One action per line                                  → where to look (reference section, RM0433 page)
2. …

Done when: the visible result.
```

- About 30–45 minutes per step, each ending in something visible.
- One step = one commit + one annotated tag; push with `git push --follow-tags`.
- Check values (expected register values) folded away at the end of the steps: work them out first.

## 3. Reference

What the steps point to; read it when a step sends you there.

- **Crates**: which crates the lesson uses, and what each one supplies.
- **Registers**: one table, register | RM0433 page | key fact; finding address and bits is the reader's work.
- **Snippets**: one code block, each snippet under a `// name` comment the steps refer to. Snippets are generic: they make the reader think, but not
  too much.
    - The syntax is complete and correct (`unsafe { }`, `as`, `;`); syntax is never the puzzle.
    - The values are placeholders (`SOME_REG`, `PIN`) or a neighbouring register or bit.
    - Finding the right register and bit in RM0433 is the reader's work.
    - One snippet per concept, not one per step.
- **Concepts** used in more than one lesson live in `lessons-concepts.md`; the lesson links there in 2–3 lines.

## 4. Syntax

- Every piece of Rust syntax that is new in this lesson, one row each: syntax | meaning | C equivalent.
- Syntax only, no task patterns (those are the snippets); revision material to gloss over.
- Syntax from earlier lessons is not repeated.

## 5. Pain points

- What can be made better.
- What can be reduced to a helper function.
- What refactoring saves time later.
- Not fixed in this lesson; they are where the next lesson or an interlude starts.

## 6. Learned

- 3–5 lines, written at the end.
- Also the message of the lesson's last tag.
