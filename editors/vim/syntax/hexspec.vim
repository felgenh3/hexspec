if exists("b:current_syntax")
  finish
endif

syn case ignore

syn region hexspecComment start=';' keepend end='$' contains=hexspecCommentTodo
syn keyword hexspecCommentTodo contained TODO FIXME

syn keyword hexspecNullLabel _: ._:

syn match hexspecLabel "[_a-zA-Z0-9]\+:" 
syn match hexspecRelLabel "\.[_a-zA-Z0-9]\+:"
syn keyword hexspecCommentTodo contained .


syn match hexspecStringEscapes "\\[0\\\"]" contained
syn region hexspecString start="@\"" end="\"" contains=hexspecStringEscapes


syn region hexspecInterp start=+@\(b\|wl\|wb\|dl\|db\|ql\|qb\|zp\|b16\|b32\|b64\|l16\|l32\|l64\)(+ end=+)+ contains=hexspecInterpBound keepend
syn region hexspecInterp start=+@\(b\|wl\|wb\|dl\|db\|ql\|qb\|zp\|b16\|b32\|b64\|l16\|l32\|l64\)\[+ end=+\]+ contains=hexspecInterpBound keepend
syn region hexspecInterp start=+@\(b\|wl\|wb\|dl\|db\|ql\|qb\|zp\|b16\|b32\|b64\|l16\|l32\|l64\){+ end=+}+ contains=hexspecInterpBound keepend

syn region hexspecInterpBound start=+(+ end=+)+ contains=hexspecIntLit contained
syn region hexspecInterpBound start=+\[+ end=+\]+ contains=hexspecIntLit contained 
syn region hexspecInterpBound start=+{+ end=+}+ contains=hexspecIntLit contained 

syn match hexspecIntLit "-\?\d\+" contained
syn match hexspecHex "[0-9A-Fa-f]\+" 

hi def link hexspecComment        Comment
hi def link hexspecCommentTodo    Todo

hi def link hexspecNullLabel      Comment
hi def link hexspecLabel          Identifier
hi def link hexspecRelLabel       Define

hi def link hexspecString         String
hi def link hexspecStringEscapes  SpecialChar

hi def link hexspecInterp         Type
hi def link hexspecInterpBound    Type
hi def link hexspecIntLit         Constant
hi def link hexspecHex            Statement


" hi def link hexspecIntLit         Constant
"
let b:current_syntax = "hexspec"
