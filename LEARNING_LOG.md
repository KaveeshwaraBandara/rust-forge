# 📓 Learning Log

Write a short entry after every session. You'll use it to review before each new lesson, and in a few months it'll show you how far you've come.

---

## Template

```markdown
### YYYY-MM-DD · L0X
- **Learned:**
- **Confused by:**
- **Compiler error of the day:** E0XXX: what it meant
- **Next:**
```

---

## L01 · Basics: concept check

> Answer in your own words. No copy-paste from the Book. If you can't explain it, you don't own it yet.

1. Shadowing vs `mut`: what's the difference, and when would you choose each?
   -
2. Why does adding a `;` to the last line of a function change its return type?
   -
3. What does `300_i32 as u8` evaluate to, and why? Name two safer alternatives.
   -
4. What happens when a `u8` overflows in a debug build vs a release build? How did `encoder_delta` use wrapping deliberately?
   -
5. Why must function signatures be fully annotated when `let` bindings can be inferred?
   -
6. What is the type of `if c { 1 } else { 2 }`? What happens if you put `;` after the `1`?
   -

---

## Session entries

### 2026-11-06 · Setup
- **Learned:** Repo scaffolded. Workflow is issue → branch → tests green → PR → review → merge.
- **Next:** L01 compiler drills, then exercises E01–E06.
