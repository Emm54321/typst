--- table-alignpt-fixed-cols-rtl paged ---
#set text(dir: rtl)
#set page(width: auto)
#let A = { align-point("A"); [A] }
#let B = { align-point("B"); [B] }
#let C = { align-point("C"); [C] }
#table(
  columns: (30mm, 30mm, 30mm),
  align: (left, center, right),
  [left], [center], [right],
  [aaaa#{A}aaa], [bb#{B}b], [ccc#{C}c],
  [aa#{A}aaa], [bbbb#{B}bbb], [ccc#{C}c],
  [aaa#{A}aaaaaa], [b#{B}bbbb], [c#{C}cccc],
)

--- table-alignpt-auto-cols-rtl paged ---
#set text(dir: rtl)
#set page(width: auto)
#let A = { align-point("A"); [A] }
#let B = { align-point("B"); [B] }
#let C = { align-point("C"); [C] }
#table(
  columns: 3,
  [aaaa#{A}aaa], [bb#{B}b], [ccc#{C}c],
  [aa#{A}aaa], [bbbb#{B}bbb], [ccc#{C}c],
  [aaa#{A}aaaaaa], [b#{B}bbbb], [c#{C}cccc],
)

--- table-alignpt-multiple-points-rtl paged ---
#set text(dir: rtl)
#set page(width: auto)
#let A = { align-point("A"); [A] }
#let B = { align-point("B"); [B] }
#let C = { align-point("C"); [C] }
#table(
  columns: 1,
  table.cell(align: left, [a#{A}aaa]),
  table.cell(align: right, [bb#{B}b]),
  table.cell(align: center, [cccccc#{C}c]),
  table.cell(align: left, [aa#{A}aaa]),
  table.cell(align: right, [bbbb#{B}bbb]),
  table.cell(align: center, [ccc#{C}c]),
  table.cell(align: left, [aaa#{A}aaaaaaaaaa]),
  table.cell(align: right, [b#{B}bbbb]),
  table.cell(align: center, [ccccc#{C}cc]),
)

--- table-alignpt-linked-points-rtl paged ---
#set text(dir: rtl)
#set page(width: auto)
#let A = { align-point("A"); [A] }
#let B = { align-point("B"); [B] }
#let C = { align-point("C"); [C] }
#table(
  columns: 1,
  align: (center,),
  [a#{A}aaa],
  [aa#{A}aaa],
  [aaa#{A}aaaaaa],
  [c#{A}cc#{C}c],
  [ccccc#{C}cc],
  [ccc#{B}ccc#{C}c],
  [bb#{B}b],
  [bbbbbbbb#{B}bbb],
  [b#{B}bbbb],
)

--- table-alignpt-colspan-rtl paged ---
#set text(dir: rtl)
#set page(width: auto)
#let A = { align-point("A"); [A] }
#let B = { align-point("B"); [B] }
#let C = { align-point("C"); [C] }
#table(
  columns: 3,
  gutter: 3pt,
  [aaaa#{A}aaa], [bb#{B}b], [ccc#{C}c],
  [aa#{A}aaa], [bbbb#{B}bbb], [ccc#{C}],
  table.cell(colspan: 2, [aaa#{A}aaaaaaaa]), [c#{C}c],
  [a#{A}aa], table.cell(colspan: 2, [bbbbbbb#{B}bb]),
  [aa#{A}], table.cell(colspan: 2, [bb#{B}bbbbbbbb]),
  table.cell(colspan: 3, [cccccccc#{C}cccccccccccccccccc]),
)

--- table-alignpt-linked-colspans-rtl paged ---
#set text(dir: rtl)
#set page(width: auto)
#let A = { align-point("A"); [A] }
#let B = { align-point("B"); [B] }
#let C = { align-point("C"); [C] }
#table(
  columns: 3,
  gutter: 3pt,
  [aaaa#{A}aaa], [bb#{B}b], [ccc#{C}c],
  [aa#{A}aaa], [bbbb#{B}bbb], [ccc#{C}],
  table.cell(colspan: 2, [aaa#{B}aaaaaaaaaaaaaa#{A}a]), [c#{C}c],
  [a#{A}aa], table.cell(colspan: 2, [bbbbbbb#{B}bb]),
  [aa#{A}], table.cell(colspan: 2, [bb#{B}bbbbb]),
  table.cell(colspan: 3, [cc#{C}cccccccccccccccccccccccccccc#{A}c]),
)
