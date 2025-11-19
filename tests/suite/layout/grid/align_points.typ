--- table-alignpt-fixed-cols paged ---
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

--- table-alignpt-auto-cols paged ---
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

--- table-alignpt-multiple-points paged ---
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

--- table-alignpt-linked-points paged ---
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
  [bbbb#{B}bbb],
  [b#{B}bbbb],
)

--- table-alignpt-colspan paged ---
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
  table.cell(colspan: 3, [cccccccccccccccccccccccccc#{C}c]),
)

--- table-alignpt-linked-colspans paged ---
#set page(width: auto)
#let A = { align-point("A"); [A] }
#let B = { align-point("B"); [B] }
#let C = { align-point("C"); [C] }
#table(
  columns: 3,
  gutter: 3pt,
  [aaaa#{A}aaa], [bb#{B}b], [ccc#{C}c],
  [aa#{A}aaa], [bbbb#{B}bbb], [ccc#{C}],
  table.cell(colspan: 2, [aaa#{A}aaaaaaaaaaaaaa#{B}a]), [c#{C}c],
  [a#{A}aa], table.cell(colspan: 2, [bbbbbbb#{B}bb]),
  [aa#{A}], table.cell(colspan: 2, [bb#{B}bbbbbbbb]),
  table.cell(colspan: 3, [cc#{A}cccccccccccccccccccccccccccc#{C}c]),
)
