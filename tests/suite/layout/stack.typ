// Test stack layouts.

--- stack-basic ---
// Test stacks with different directions.
#let widths = (
  30pt, 20pt, 40pt, 15pt,
  30pt, 50%, 20pt, 100%,
)

#let shaded(i, w) = {
  let v = (i + 1) * 10%
  rect(width: w, height: 10pt, fill: rgb(v, v, v))
}

#let items = for (i, w) in widths.enumerate() {
  (align(right, shaded(i, w)),)
}

#set page(width: 50pt, margin: 0pt)
#stack(dir: btt, ..items)

--- stack-spacing ---
// Test spacing.
#set page(width: 50pt, margin: 0pt)

#let x = square(size: 10pt, fill: eastern)
#stack(
  spacing: 5pt,
  stack(dir: rtl, spacing: 5pt, x, x, x),
  stack(dir: ltr, x, 20%, x, 20%, x),
  stack(dir: ltr, spacing: 5pt, x, x, 7pt, 3pt, x),
)

--- stack-overflow ---
// Test overflow.
#set page(width: 50pt, height: 30pt, margin: 0pt)
#box(stack(
  rect(width: 40pt, height: 20pt, fill: conifer),
  rect(width: 30pt, height: 13pt, fill: forest),
))

--- stack-fr ---
#set page(height: 3.5cm)
#stack(
  dir: ltr,
  spacing: 1fr,
  ..for c in "ABCDEFGHI" {([#c],)}
)

Hello
#v(2fr)
from #h(1fr) the #h(1fr) wonderful
#v(1fr)
World! 🌍

--- stack-rtl-align-and-fr ---
// Test aligning things in RTL stack with align function & fr units.
#set page(width: 50pt, margin: 5pt)
#set block(spacing: 5pt)
#set text(8pt)
#stack(dir: rtl, 1fr, [A], 1fr, [B], [C])
#stack(dir: rtl,
  align(center, [A]),
  align(left, [B]),
  [C],
)

--- issue-1240-stack-h-fr ---
// This issue is sort of horrible: When you write `h(1fr)` in a `stack` instead
// of directly `1fr`, things go awry. To fix this, we now transparently detect
// h/v children.
#stack(dir: ltr, [a], 1fr, [b], 1fr, [c])
#stack(dir: ltr, [a], h(1fr), [b], h(1fr), [c])

--- issue-1240-stack-v-fr ---
#set page(height: 60pt)
#stack(
  dir: ltr,
  spacing: 1fr,
  stack([a], 1fr, [b]),
  stack([a], v(1fr), [b]),
)

--- issue-1918-stack-with-infinite-spacing ---
// https://github.com/typst/typst/issues/1918
#set page(width: auto)
#context layout(available => {
  let infinite-length = available.width
  // Error: 3-40 stack spacing is infinite
  stack(spacing: infinite-length)[A][B]
})

--- stack-alignpt ---
#set page(width: auto)
#let A = { align-point("A"); [A] }
#let B = { align-point("B"); [B] }
#stack(
  spacing: 5pt,
  [aaa#{A}aaa],
  [a#{A}aaaaaaa],
  [bbbbbbb#{B}b],
  [aaaaa#{A}aa],
  [b#{B}bbb],
  [bb#{B}b],
)

--- stack-alignpt-right ---
#set page(width: auto)
#let A = { align-point("A"); [A] }
#let B = { align-point("B"); [B] }
#rect(stack(
  spacing: 5pt,
  align(right, [aaa#{A}aaa]),
  align(right, [a#{A}aaaaaaa]),
  align(right, [bbbbbbb#{B}bbbbb]),
  align(right, [aaaaa#{A}aa]),
  align(right, [b#{B}bbb]),
  align(right, [bb#{B}b]),
))

--- stack-alignpt-right-fixed-width ---
#set page(width: auto)
#let A = { align-point("A"); [A] }
#let B = { align-point("B"); [B] }
#rect(width: 30mm, stack(
  spacing: 5pt,
  align(right, [aaa#{A}aaa]),
  align(right, [a#{A}aaaaaaa]),
  align(right, [bbbbbbb#{B}bbbbb]),
  align(right, [aaaaa#{A}aa]),
  align(right, [b#{B}bbb]),
  align(right, [bb#{B}b]),
))

--- stack-alignpt-center ---
#set page(width: auto)
#let A = { align-point("A"); [A] }
#let B = { align-point("B"); [B] }
#rect(stack(
  spacing: 5pt,
  align(center, [aaa#{A}aaa]),
  align(center, [a#{A}aaaaaaa]),
  align(center, [bbbbbbb#{B}b]),
  align(center, [xxxxxxxxxxxxxxxxxxx]),
  align(center, [aaaaa#{A}aa]),
  align(center, [b#{B}bbb]),
  align(center, [bb#{B}b]),
))

--- stack-alignpt-linked ---
#set page(width: auto)
#let A = { align-point("A"); [A] }
#let B = { align-point("B"); [B] }
#let C = { align-point("C"); [C] }
#stack(
  spacing: 5pt,
  [aaa#{A}aaa],
  [cc#{C}c],
  [a#{A}aaaaaaa],
  [bbb#{A}bbbb#{B}b],
  [aaaaa#{A}aa],
  [b#{B}bb#{C}b],
  [bb#{B}b],
  [cccc#{C}cc],
)

--- stack-alignpt-ltr ---
#set page(width: auto)
#let A = { align-point("A"); [A] }
#let B = { align-point("B"); [B] }
#stack(
  dir: ltr,
  spacing: 5pt,
  [aa\
   a#{A}a\aaaa],
  [a#{A}aa\
   aaaaa],
  [bbb\
   bbbb#{B}b\
   bb\
   bbb],
  [a\
   aa\
   aa#{A}aa],
  [b#{B}b\
   bb],
  [bb#{B}b],
)

--- stack-alignpt-ltr-align ---
#set page(width: auto)
#let A = { align-point("A"); [A] }
#let B = { align-point("B"); [B] }
#stack(
  dir: ltr,
  spacing: 5pt,
  align(top, [aa\
   a#{A}a\aaaa]),
  align(top, [a#{A}aa\
   aaaaa]),
  [x\
   x\
   x\
   x\
   x\
   x],
  align(bottom, [bbb\
   bbbb#{B}b\
   bb\
   bbb]),
  align(top, [a\
   aa\
   aa#{A}aa]),
  align(bottom, [b#{B}b\
   bb]),
  align(bottom, [bb#{B}b]),
)
