

(label) @type
(sublabel) @attribute

((label) @comment (#match? @comment "_:"))
((sublabel) @comment (#match? @comment "._:"))

(comment) @comment

[ "(" ")" ] @punctuation.bracket
(string) @string

(hex_content) @constant
(number) @number

(decimal) @function.call
(zero_pad) @function.call
