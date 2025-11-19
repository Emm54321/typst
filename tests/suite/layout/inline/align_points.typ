--- inline-alignpt paged ---
#set page(width: auto)
#let A = { align-point("A"); [A] }
#{A}a aaa #box[bbb\
bb#{A}\
bb\
bb] aa aa #box(rect(radius: 5pt, stroke: 5pt+red)[#{A}ccc cc]) aa aaaa #box[dd\
ddd\
ddd#{A}dd] aaa

--- inline-alignpt-linked paged ---
#set page(width: auto)
#let A = { align-point("A"); [A] }
#let B = { align-point("B"); [B] }
#let C = { align-point("C"); [C] }
#{A}a aaa #box(rect[eee\
ee#{C}ee]) a #box(rect[bbb\
bb#{A}\
b#{B}b\
bb]) aa aa #box(rect[#{A}ccc\
cc]) aa aaaa #box(rect[dd#C\
ddd\
ddd#{B}dd]) aaa

--- inline-alignpt-nested-unused paged ---
#set page(width: auto)
#let A = { align-point("A"); [A] }
#let B = { align-point("B"); [B] }
Bbb #box[cc c cc] #box[aa #box[aaa#{A}aa] aa] bb#B #box(rect[cc\
cc#B\
cc]).

