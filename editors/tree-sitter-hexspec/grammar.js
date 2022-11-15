module.exports = grammar({
  name: "hexspec",

  rules: {
    source_file: $ => seq(
      optional(repeat($._content)), 
      optional(repeat($.region))),

    region: $ => seq(
      $.label, 
      optional(repeat($._content)), 
      optional(repeat($.subregion))),

    subregion: $ => seq($.sublabel, repeat($._content)),

    _content: $ => choice(
      $.hex_content,
      $.string,
      $.decimal,
      $.zero_pad
    ),

    decimal: $ => seq(
      '@',
      choice("b", "l16", "l32", "l64", "b16", "b32", "b64",
      "wl", "dl", "ql", "wb", "db", "qb"),
      "(", $.number, ")"),

    zero_pad: $ => seq("@", "zp", "(", $.number, ")"),

    hex_content: $ => /([0-9a-fA-F]{2})+/,
    string: $ => seq('@', '"', /[^"]*/, '"'),
    number: $ => /-?[0-9]+/,

    label: $ => /[a-zA-Z_][a-zA-Z0-9_-]*:/,
    sublabel: $ => /\.[a-zA-Z_][a-zA-Z0-9_-]*:/,

    comment: $ => seq(';', /.*/),
  },

  extras: $ => [$.comment, /\s+/, /\n/],
})
