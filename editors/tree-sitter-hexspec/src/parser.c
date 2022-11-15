#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 13
#define STATE_COUNT 33
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 37
#define ALIAS_COUNT 0
#define TOKEN_COUNT 26
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 5
#define PRODUCTION_ID_COUNT 1

enum {
  anon_sym_AT = 1,
  anon_sym_b = 2,
  anon_sym_l16 = 3,
  anon_sym_l32 = 4,
  anon_sym_l64 = 5,
  anon_sym_b16 = 6,
  anon_sym_b32 = 7,
  anon_sym_b64 = 8,
  anon_sym_wl = 9,
  anon_sym_dl = 10,
  anon_sym_ql = 11,
  anon_sym_wb = 12,
  anon_sym_db = 13,
  anon_sym_qb = 14,
  anon_sym_LPAREN = 15,
  anon_sym_RPAREN = 16,
  anon_sym_zp = 17,
  sym_hex_content = 18,
  anon_sym_DQUOTE = 19,
  aux_sym_string_token1 = 20,
  sym_number = 21,
  sym_label = 22,
  sym_sublabel = 23,
  anon_sym_SEMI = 24,
  aux_sym_comment_token1 = 25,
  sym_source_file = 26,
  sym_region = 27,
  sym_subregion = 28,
  sym__content = 29,
  sym_decimal = 30,
  sym_zero_pad = 31,
  sym_string = 32,
  sym_comment = 33,
  aux_sym_source_file_repeat1 = 34,
  aux_sym_source_file_repeat2 = 35,
  aux_sym_region_repeat1 = 36,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_AT] = "@",
  [anon_sym_b] = "b",
  [anon_sym_l16] = "l16",
  [anon_sym_l32] = "l32",
  [anon_sym_l64] = "l64",
  [anon_sym_b16] = "b16",
  [anon_sym_b32] = "b32",
  [anon_sym_b64] = "b64",
  [anon_sym_wl] = "wl",
  [anon_sym_dl] = "dl",
  [anon_sym_ql] = "ql",
  [anon_sym_wb] = "wb",
  [anon_sym_db] = "db",
  [anon_sym_qb] = "qb",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [anon_sym_zp] = "zp",
  [sym_hex_content] = "hex_content",
  [anon_sym_DQUOTE] = "\"",
  [aux_sym_string_token1] = "string_token1",
  [sym_number] = "number",
  [sym_label] = "label",
  [sym_sublabel] = "sublabel",
  [anon_sym_SEMI] = ";",
  [aux_sym_comment_token1] = "comment_token1",
  [sym_source_file] = "source_file",
  [sym_region] = "region",
  [sym_subregion] = "subregion",
  [sym__content] = "_content",
  [sym_decimal] = "decimal",
  [sym_zero_pad] = "zero_pad",
  [sym_string] = "string",
  [sym_comment] = "comment",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_source_file_repeat2] = "source_file_repeat2",
  [aux_sym_region_repeat1] = "region_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_AT] = anon_sym_AT,
  [anon_sym_b] = anon_sym_b,
  [anon_sym_l16] = anon_sym_l16,
  [anon_sym_l32] = anon_sym_l32,
  [anon_sym_l64] = anon_sym_l64,
  [anon_sym_b16] = anon_sym_b16,
  [anon_sym_b32] = anon_sym_b32,
  [anon_sym_b64] = anon_sym_b64,
  [anon_sym_wl] = anon_sym_wl,
  [anon_sym_dl] = anon_sym_dl,
  [anon_sym_ql] = anon_sym_ql,
  [anon_sym_wb] = anon_sym_wb,
  [anon_sym_db] = anon_sym_db,
  [anon_sym_qb] = anon_sym_qb,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_zp] = anon_sym_zp,
  [sym_hex_content] = sym_hex_content,
  [anon_sym_DQUOTE] = anon_sym_DQUOTE,
  [aux_sym_string_token1] = aux_sym_string_token1,
  [sym_number] = sym_number,
  [sym_label] = sym_label,
  [sym_sublabel] = sym_sublabel,
  [anon_sym_SEMI] = anon_sym_SEMI,
  [aux_sym_comment_token1] = aux_sym_comment_token1,
  [sym_source_file] = sym_source_file,
  [sym_region] = sym_region,
  [sym_subregion] = sym_subregion,
  [sym__content] = sym__content,
  [sym_decimal] = sym_decimal,
  [sym_zero_pad] = sym_zero_pad,
  [sym_string] = sym_string,
  [sym_comment] = sym_comment,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_source_file_repeat2] = aux_sym_source_file_repeat2,
  [aux_sym_region_repeat1] = aux_sym_region_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_AT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_b] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_l16] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_l32] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_l64] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_b16] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_b32] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_b64] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_wl] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_dl] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ql] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_wb] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_db] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_qb] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_zp] = {
    .visible = true,
    .named = false,
  },
  [sym_hex_content] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_DQUOTE] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_string_token1] = {
    .visible = false,
    .named = false,
  },
  [sym_number] = {
    .visible = true,
    .named = true,
  },
  [sym_label] = {
    .visible = true,
    .named = true,
  },
  [sym_sublabel] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_SEMI] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_comment_token1] = {
    .visible = false,
    .named = false,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym_region] = {
    .visible = true,
    .named = true,
  },
  [sym_subregion] = {
    .visible = true,
    .named = true,
  },
  [sym__content] = {
    .visible = false,
    .named = true,
  },
  [sym_decimal] = {
    .visible = true,
    .named = true,
  },
  [sym_zero_pad] = {
    .visible = true,
    .named = true,
  },
  [sym_string] = {
    .visible = true,
    .named = true,
  },
  [sym_comment] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_source_file_repeat2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_region_repeat1] = {
    .visible = false,
    .named = false,
  },
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(28);
      if (lookahead == '\n') SKIP(0)
      if (lookahead == '"') ADVANCE(66);
      if (lookahead == '(') ADVANCE(56);
      if (lookahead == ')') ADVANCE(57);
      if (lookahead == '-') ADVANCE(24);
      if (lookahead == '.') ADVANCE(26);
      if (lookahead == ';') ADVANCE(73);
      if (lookahead == '@') ADVANCE(29);
      if (lookahead == 'b') ADVANCE(30);
      if (lookahead == 'd') ADVANCE(13);
      if (lookahead == 'l') ADVANCE(2);
      if (lookahead == 'q') ADVANCE(14);
      if (lookahead == 'w') ADVANCE(15);
      if (lookahead == 'z') ADVANCE(16);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(17);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(70);
      if (('G' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'y')) ADVANCE(18);
      END_STATE();
    case 1:
      if (lookahead == '\n') SKIP(1)
      if (lookahead == '"') ADVANCE(66);
      if (lookahead == '-') ADVANCE(24);
      if (lookahead == ';') ADVANCE(73);
      if (lookahead == 'b') ADVANCE(31);
      if (lookahead == 'd') ADVANCE(20);
      if (lookahead == 'l') ADVANCE(3);
      if (lookahead == 'q') ADVANCE(21);
      if (lookahead == 'w') ADVANCE(22);
      if (lookahead == 'z') ADVANCE(23);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(1)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(69);
      END_STATE();
    case 2:
      if (lookahead == '1') ADVANCE(10);
      if (lookahead == '3') ADVANCE(4);
      if (lookahead == '6') ADVANCE(7);
      if (lookahead == ':') ADVANCE(71);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(18);
      END_STATE();
    case 3:
      if (lookahead == '1') ADVANCE(12);
      if (lookahead == '3') ADVANCE(6);
      if (lookahead == '6') ADVANCE(9);
      END_STATE();
    case 4:
      if (lookahead == '2') ADVANCE(35);
      if (lookahead == ':') ADVANCE(71);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(18);
      END_STATE();
    case 5:
      if (lookahead == '2') ADVANCE(40);
      END_STATE();
    case 6:
      if (lookahead == '2') ADVANCE(34);
      END_STATE();
    case 7:
      if (lookahead == '4') ADVANCE(37);
      if (lookahead == ':') ADVANCE(71);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(18);
      END_STATE();
    case 8:
      if (lookahead == '4') ADVANCE(42);
      END_STATE();
    case 9:
      if (lookahead == '4') ADVANCE(36);
      END_STATE();
    case 10:
      if (lookahead == '6') ADVANCE(33);
      if (lookahead == ':') ADVANCE(71);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(18);
      END_STATE();
    case 11:
      if (lookahead == '6') ADVANCE(38);
      END_STATE();
    case 12:
      if (lookahead == '6') ADVANCE(32);
      END_STATE();
    case 13:
      if (lookahead == ':') ADVANCE(71);
      if (lookahead == 'b') ADVANCE(53);
      if (lookahead == 'l') ADVANCE(47);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(63);
      if (lookahead == '-' ||
          ('G' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(18);
      END_STATE();
    case 14:
      if (lookahead == ':') ADVANCE(71);
      if (lookahead == 'b') ADVANCE(55);
      if (lookahead == 'l') ADVANCE(49);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(18);
      END_STATE();
    case 15:
      if (lookahead == ':') ADVANCE(71);
      if (lookahead == 'b') ADVANCE(51);
      if (lookahead == 'l') ADVANCE(45);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(18);
      END_STATE();
    case 16:
      if (lookahead == ':') ADVANCE(71);
      if (lookahead == 'p') ADVANCE(59);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(18);
      END_STATE();
    case 17:
      if (lookahead == ':') ADVANCE(71);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(63);
      if (lookahead == '-' ||
          ('G' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(18);
      END_STATE();
    case 18:
      if (lookahead == ':') ADVANCE(71);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(18);
      END_STATE();
    case 19:
      if (lookahead == ':') ADVANCE(72);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(19);
      END_STATE();
    case 20:
      if (lookahead == 'b') ADVANCE(52);
      if (lookahead == 'l') ADVANCE(46);
      END_STATE();
    case 21:
      if (lookahead == 'b') ADVANCE(54);
      if (lookahead == 'l') ADVANCE(48);
      END_STATE();
    case 22:
      if (lookahead == 'b') ADVANCE(50);
      if (lookahead == 'l') ADVANCE(44);
      END_STATE();
    case 23:
      if (lookahead == 'p') ADVANCE(58);
      END_STATE();
    case 24:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(69);
      END_STATE();
    case 25:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(65);
      END_STATE();
    case 26:
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(19);
      END_STATE();
    case 27:
      if (eof) ADVANCE(28);
      if (lookahead == '\n') SKIP(27)
      if (lookahead == '.') ADVANCE(26);
      if (lookahead == ';') ADVANCE(73);
      if (lookahead == '@') ADVANCE(29);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(27)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(25);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(17);
      if (('G' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(18);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(anon_sym_AT);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(anon_sym_b);
      if (lookahead == '1') ADVANCE(62);
      if (lookahead == '3') ADVANCE(60);
      if (lookahead == '6') ADVANCE(61);
      if (lookahead == ':') ADVANCE(71);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(63);
      if (lookahead == '-' ||
          ('G' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(18);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(anon_sym_b);
      if (lookahead == '1') ADVANCE(11);
      if (lookahead == '3') ADVANCE(5);
      if (lookahead == '6') ADVANCE(8);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(anon_sym_l16);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(anon_sym_l16);
      if (lookahead == ':') ADVANCE(71);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(18);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(anon_sym_l32);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(anon_sym_l32);
      if (lookahead == ':') ADVANCE(71);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(18);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(anon_sym_l64);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(anon_sym_l64);
      if (lookahead == ':') ADVANCE(71);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(18);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(anon_sym_b16);
      END_STATE();
    case 39:
      ACCEPT_TOKEN(anon_sym_b16);
      if (lookahead == ':') ADVANCE(71);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(63);
      if (lookahead == '-' ||
          ('G' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(18);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(anon_sym_b32);
      END_STATE();
    case 41:
      ACCEPT_TOKEN(anon_sym_b32);
      if (lookahead == ':') ADVANCE(71);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(63);
      if (lookahead == '-' ||
          ('G' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(18);
      END_STATE();
    case 42:
      ACCEPT_TOKEN(anon_sym_b64);
      END_STATE();
    case 43:
      ACCEPT_TOKEN(anon_sym_b64);
      if (lookahead == ':') ADVANCE(71);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(63);
      if (lookahead == '-' ||
          ('G' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(18);
      END_STATE();
    case 44:
      ACCEPT_TOKEN(anon_sym_wl);
      END_STATE();
    case 45:
      ACCEPT_TOKEN(anon_sym_wl);
      if (lookahead == ':') ADVANCE(71);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(18);
      END_STATE();
    case 46:
      ACCEPT_TOKEN(anon_sym_dl);
      END_STATE();
    case 47:
      ACCEPT_TOKEN(anon_sym_dl);
      if (lookahead == ':') ADVANCE(71);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(18);
      END_STATE();
    case 48:
      ACCEPT_TOKEN(anon_sym_ql);
      END_STATE();
    case 49:
      ACCEPT_TOKEN(anon_sym_ql);
      if (lookahead == ':') ADVANCE(71);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(18);
      END_STATE();
    case 50:
      ACCEPT_TOKEN(anon_sym_wb);
      END_STATE();
    case 51:
      ACCEPT_TOKEN(anon_sym_wb);
      if (lookahead == ':') ADVANCE(71);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(18);
      END_STATE();
    case 52:
      ACCEPT_TOKEN(anon_sym_db);
      END_STATE();
    case 53:
      ACCEPT_TOKEN(anon_sym_db);
      if (lookahead == ':') ADVANCE(71);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(17);
      if (lookahead == '-' ||
          ('G' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(18);
      END_STATE();
    case 54:
      ACCEPT_TOKEN(anon_sym_qb);
      END_STATE();
    case 55:
      ACCEPT_TOKEN(anon_sym_qb);
      if (lookahead == ':') ADVANCE(71);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(18);
      END_STATE();
    case 56:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 57:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 58:
      ACCEPT_TOKEN(anon_sym_zp);
      END_STATE();
    case 59:
      ACCEPT_TOKEN(anon_sym_zp);
      if (lookahead == ':') ADVANCE(71);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(18);
      END_STATE();
    case 60:
      ACCEPT_TOKEN(sym_hex_content);
      if (lookahead == '2') ADVANCE(41);
      if (lookahead == ':') ADVANCE(71);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(17);
      if (lookahead == '-' ||
          ('G' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(18);
      END_STATE();
    case 61:
      ACCEPT_TOKEN(sym_hex_content);
      if (lookahead == '4') ADVANCE(43);
      if (lookahead == ':') ADVANCE(71);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(17);
      if (lookahead == '-' ||
          ('G' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(18);
      END_STATE();
    case 62:
      ACCEPT_TOKEN(sym_hex_content);
      if (lookahead == '6') ADVANCE(39);
      if (lookahead == ':') ADVANCE(71);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(17);
      if (lookahead == '-' ||
          ('G' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(18);
      END_STATE();
    case 63:
      ACCEPT_TOKEN(sym_hex_content);
      if (lookahead == ':') ADVANCE(71);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(17);
      if (lookahead == '-' ||
          ('G' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(18);
      END_STATE();
    case 64:
      ACCEPT_TOKEN(sym_hex_content);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(70);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(25);
      END_STATE();
    case 65:
      ACCEPT_TOKEN(sym_hex_content);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(25);
      END_STATE();
    case 66:
      ACCEPT_TOKEN(anon_sym_DQUOTE);
      END_STATE();
    case 67:
      ACCEPT_TOKEN(aux_sym_string_token1);
      if (lookahead == '\n') ADVANCE(67);
      if (lookahead == ';') ADVANCE(75);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(67);
      if (lookahead != 0 &&
          lookahead != '"') ADVANCE(68);
      END_STATE();
    case 68:
      ACCEPT_TOKEN(aux_sym_string_token1);
      if (lookahead != 0 &&
          lookahead != '"') ADVANCE(68);
      END_STATE();
    case 69:
      ACCEPT_TOKEN(sym_number);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(69);
      END_STATE();
    case 70:
      ACCEPT_TOKEN(sym_number);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(64);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(65);
      END_STATE();
    case 71:
      ACCEPT_TOKEN(sym_label);
      END_STATE();
    case 72:
      ACCEPT_TOKEN(sym_sublabel);
      END_STATE();
    case 73:
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    case 74:
      ACCEPT_TOKEN(anon_sym_SEMI);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(77);
      END_STATE();
    case 75:
      ACCEPT_TOKEN(anon_sym_SEMI);
      if (lookahead != 0 &&
          lookahead != '"') ADVANCE(68);
      END_STATE();
    case 76:
      ACCEPT_TOKEN(aux_sym_comment_token1);
      if (lookahead == ';') ADVANCE(74);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(76);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(77);
      END_STATE();
    case 77:
      ACCEPT_TOKEN(aux_sym_comment_token1);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(77);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 27},
  [2] = {.lex_state = 1},
  [3] = {.lex_state = 27},
  [4] = {.lex_state = 27},
  [5] = {.lex_state = 27},
  [6] = {.lex_state = 27},
  [7] = {.lex_state = 27},
  [8] = {.lex_state = 27},
  [9] = {.lex_state = 27},
  [10] = {.lex_state = 27},
  [11] = {.lex_state = 27},
  [12] = {.lex_state = 27},
  [13] = {.lex_state = 27},
  [14] = {.lex_state = 27},
  [15] = {.lex_state = 27},
  [16] = {.lex_state = 27},
  [17] = {.lex_state = 27},
  [18] = {.lex_state = 27},
  [19] = {.lex_state = 27},
  [20] = {.lex_state = 27},
  [21] = {.lex_state = 27},
  [22] = {.lex_state = 76},
  [23] = {.lex_state = 1},
  [24] = {.lex_state = 1},
  [25] = {.lex_state = 0},
  [26] = {.lex_state = 67},
  [27] = {.lex_state = 0},
  [28] = {.lex_state = 0},
  [29] = {.lex_state = 0},
  [30] = {.lex_state = 0},
  [31] = {.lex_state = 0},
  [32] = {(TSStateId)(-1)},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [sym_comment] = STATE(0),
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_AT] = ACTIONS(1),
    [anon_sym_b] = ACTIONS(1),
    [anon_sym_l16] = ACTIONS(1),
    [anon_sym_l32] = ACTIONS(1),
    [anon_sym_l64] = ACTIONS(1),
    [anon_sym_b16] = ACTIONS(1),
    [anon_sym_b32] = ACTIONS(1),
    [anon_sym_b64] = ACTIONS(1),
    [anon_sym_wl] = ACTIONS(1),
    [anon_sym_dl] = ACTIONS(1),
    [anon_sym_ql] = ACTIONS(1),
    [anon_sym_wb] = ACTIONS(1),
    [anon_sym_db] = ACTIONS(1),
    [anon_sym_qb] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_zp] = ACTIONS(1),
    [sym_hex_content] = ACTIONS(1),
    [anon_sym_DQUOTE] = ACTIONS(1),
    [sym_number] = ACTIONS(1),
    [sym_label] = ACTIONS(1),
    [sym_sublabel] = ACTIONS(1),
    [anon_sym_SEMI] = ACTIONS(3),
  },
  [1] = {
    [sym_source_file] = STATE(31),
    [sym_region] = STATE(21),
    [sym__content] = STATE(13),
    [sym_decimal] = STATE(10),
    [sym_zero_pad] = STATE(10),
    [sym_string] = STATE(10),
    [sym_comment] = STATE(1),
    [aux_sym_source_file_repeat1] = STATE(5),
    [aux_sym_source_file_repeat2] = STATE(18),
    [ts_builtin_sym_end] = ACTIONS(5),
    [anon_sym_AT] = ACTIONS(7),
    [sym_hex_content] = ACTIONS(9),
    [sym_label] = ACTIONS(11),
    [anon_sym_SEMI] = ACTIONS(3),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 6,
    ACTIONS(3), 1,
      anon_sym_SEMI,
    ACTIONS(13), 1,
      anon_sym_b,
    ACTIONS(17), 1,
      anon_sym_zp,
    ACTIONS(19), 1,
      anon_sym_DQUOTE,
    STATE(2), 1,
      sym_comment,
    ACTIONS(15), 12,
      anon_sym_l16,
      anon_sym_l32,
      anon_sym_l64,
      anon_sym_b16,
      anon_sym_b32,
      anon_sym_b64,
      anon_sym_wl,
      anon_sym_dl,
      anon_sym_ql,
      anon_sym_wb,
      anon_sym_db,
      anon_sym_qb,
  [30] = 11,
    ACTIONS(3), 1,
      anon_sym_SEMI,
    ACTIONS(7), 1,
      anon_sym_AT,
    ACTIONS(9), 1,
      sym_hex_content,
    ACTIONS(23), 1,
      sym_sublabel,
    STATE(3), 1,
      sym_comment,
    STATE(4), 1,
      aux_sym_source_file_repeat1,
    STATE(13), 1,
      sym__content,
    STATE(16), 1,
      aux_sym_region_repeat1,
    STATE(20), 1,
      sym_subregion,
    ACTIONS(21), 2,
      ts_builtin_sym_end,
      sym_label,
    STATE(10), 3,
      sym_decimal,
      sym_zero_pad,
      sym_string,
  [67] = 11,
    ACTIONS(3), 1,
      anon_sym_SEMI,
    ACTIONS(7), 1,
      anon_sym_AT,
    ACTIONS(9), 1,
      sym_hex_content,
    ACTIONS(23), 1,
      sym_sublabel,
    STATE(4), 1,
      sym_comment,
    STATE(8), 1,
      aux_sym_source_file_repeat1,
    STATE(13), 1,
      sym__content,
    STATE(15), 1,
      aux_sym_region_repeat1,
    STATE(20), 1,
      sym_subregion,
    ACTIONS(25), 2,
      ts_builtin_sym_end,
      sym_label,
    STATE(10), 3,
      sym_decimal,
      sym_zero_pad,
      sym_string,
  [104] = 11,
    ACTIONS(3), 1,
      anon_sym_SEMI,
    ACTIONS(7), 1,
      anon_sym_AT,
    ACTIONS(9), 1,
      sym_hex_content,
    ACTIONS(11), 1,
      sym_label,
    ACTIONS(27), 1,
      ts_builtin_sym_end,
    STATE(5), 1,
      sym_comment,
    STATE(8), 1,
      aux_sym_source_file_repeat1,
    STATE(13), 1,
      sym__content,
    STATE(17), 1,
      aux_sym_source_file_repeat2,
    STATE(21), 1,
      sym_region,
    STATE(10), 3,
      sym_decimal,
      sym_zero_pad,
      sym_string,
  [140] = 8,
    ACTIONS(3), 1,
      anon_sym_SEMI,
    ACTIONS(7), 1,
      anon_sym_AT,
    ACTIONS(9), 1,
      sym_hex_content,
    STATE(6), 1,
      sym_comment,
    STATE(8), 1,
      aux_sym_source_file_repeat1,
    STATE(13), 1,
      sym__content,
    ACTIONS(29), 3,
      ts_builtin_sym_end,
      sym_label,
      sym_sublabel,
    STATE(10), 3,
      sym_decimal,
      sym_zero_pad,
      sym_string,
  [169] = 8,
    ACTIONS(3), 1,
      anon_sym_SEMI,
    ACTIONS(7), 1,
      anon_sym_AT,
    ACTIONS(9), 1,
      sym_hex_content,
    STATE(6), 1,
      aux_sym_source_file_repeat1,
    STATE(7), 1,
      sym_comment,
    STATE(13), 1,
      sym__content,
    ACTIONS(31), 3,
      ts_builtin_sym_end,
      sym_label,
      sym_sublabel,
    STATE(10), 3,
      sym_decimal,
      sym_zero_pad,
      sym_string,
  [198] = 7,
    ACTIONS(3), 1,
      anon_sym_SEMI,
    ACTIONS(35), 1,
      anon_sym_AT,
    ACTIONS(38), 1,
      sym_hex_content,
    STATE(13), 1,
      sym__content,
    STATE(8), 2,
      sym_comment,
      aux_sym_source_file_repeat1,
    ACTIONS(33), 3,
      ts_builtin_sym_end,
      sym_label,
      sym_sublabel,
    STATE(10), 3,
      sym_decimal,
      sym_zero_pad,
      sym_string,
  [225] = 4,
    ACTIONS(3), 1,
      anon_sym_SEMI,
    ACTIONS(43), 1,
      sym_hex_content,
    STATE(9), 1,
      sym_comment,
    ACTIONS(41), 4,
      ts_builtin_sym_end,
      anon_sym_AT,
      sym_label,
      sym_sublabel,
  [241] = 4,
    ACTIONS(3), 1,
      anon_sym_SEMI,
    ACTIONS(47), 1,
      sym_hex_content,
    STATE(10), 1,
      sym_comment,
    ACTIONS(45), 4,
      ts_builtin_sym_end,
      anon_sym_AT,
      sym_label,
      sym_sublabel,
  [257] = 4,
    ACTIONS(3), 1,
      anon_sym_SEMI,
    ACTIONS(51), 1,
      sym_hex_content,
    STATE(11), 1,
      sym_comment,
    ACTIONS(49), 4,
      ts_builtin_sym_end,
      anon_sym_AT,
      sym_label,
      sym_sublabel,
  [273] = 4,
    ACTIONS(3), 1,
      anon_sym_SEMI,
    ACTIONS(55), 1,
      sym_hex_content,
    STATE(12), 1,
      sym_comment,
    ACTIONS(53), 4,
      ts_builtin_sym_end,
      anon_sym_AT,
      sym_label,
      sym_sublabel,
  [289] = 4,
    ACTIONS(3), 1,
      anon_sym_SEMI,
    ACTIONS(59), 1,
      sym_hex_content,
    STATE(13), 1,
      sym_comment,
    ACTIONS(57), 4,
      ts_builtin_sym_end,
      anon_sym_AT,
      sym_label,
      sym_sublabel,
  [305] = 5,
    ACTIONS(3), 1,
      anon_sym_SEMI,
    ACTIONS(63), 1,
      sym_sublabel,
    STATE(20), 1,
      sym_subregion,
    ACTIONS(61), 2,
      ts_builtin_sym_end,
      sym_label,
    STATE(14), 2,
      sym_comment,
      aux_sym_region_repeat1,
  [323] = 6,
    ACTIONS(3), 1,
      anon_sym_SEMI,
    ACTIONS(23), 1,
      sym_sublabel,
    STATE(14), 1,
      aux_sym_region_repeat1,
    STATE(15), 1,
      sym_comment,
    STATE(20), 1,
      sym_subregion,
    ACTIONS(66), 2,
      ts_builtin_sym_end,
      sym_label,
  [343] = 6,
    ACTIONS(3), 1,
      anon_sym_SEMI,
    ACTIONS(23), 1,
      sym_sublabel,
    STATE(14), 1,
      aux_sym_region_repeat1,
    STATE(16), 1,
      sym_comment,
    STATE(20), 1,
      sym_subregion,
    ACTIONS(25), 2,
      ts_builtin_sym_end,
      sym_label,
  [363] = 6,
    ACTIONS(3), 1,
      anon_sym_SEMI,
    ACTIONS(11), 1,
      sym_label,
    ACTIONS(68), 1,
      ts_builtin_sym_end,
    STATE(17), 1,
      sym_comment,
    STATE(19), 1,
      aux_sym_source_file_repeat2,
    STATE(21), 1,
      sym_region,
  [382] = 6,
    ACTIONS(3), 1,
      anon_sym_SEMI,
    ACTIONS(11), 1,
      sym_label,
    ACTIONS(27), 1,
      ts_builtin_sym_end,
    STATE(18), 1,
      sym_comment,
    STATE(19), 1,
      aux_sym_source_file_repeat2,
    STATE(21), 1,
      sym_region,
  [401] = 5,
    ACTIONS(3), 1,
      anon_sym_SEMI,
    ACTIONS(70), 1,
      ts_builtin_sym_end,
    ACTIONS(72), 1,
      sym_label,
    STATE(21), 1,
      sym_region,
    STATE(19), 2,
      sym_comment,
      aux_sym_source_file_repeat2,
  [418] = 3,
    ACTIONS(3), 1,
      anon_sym_SEMI,
    STATE(20), 1,
      sym_comment,
    ACTIONS(75), 3,
      ts_builtin_sym_end,
      sym_label,
      sym_sublabel,
  [430] = 3,
    ACTIONS(3), 1,
      anon_sym_SEMI,
    STATE(21), 1,
      sym_comment,
    ACTIONS(77), 2,
      ts_builtin_sym_end,
      sym_label,
  [441] = 3,
    ACTIONS(79), 1,
      anon_sym_SEMI,
    ACTIONS(81), 1,
      aux_sym_comment_token1,
    STATE(22), 1,
      sym_comment,
  [451] = 3,
    ACTIONS(3), 1,
      anon_sym_SEMI,
    ACTIONS(83), 1,
      sym_number,
    STATE(23), 1,
      sym_comment,
  [461] = 3,
    ACTIONS(3), 1,
      anon_sym_SEMI,
    ACTIONS(85), 1,
      sym_number,
    STATE(24), 1,
      sym_comment,
  [471] = 3,
    ACTIONS(3), 1,
      anon_sym_SEMI,
    ACTIONS(87), 1,
      anon_sym_DQUOTE,
    STATE(25), 1,
      sym_comment,
  [481] = 3,
    ACTIONS(79), 1,
      anon_sym_SEMI,
    ACTIONS(89), 1,
      aux_sym_string_token1,
    STATE(26), 1,
      sym_comment,
  [491] = 3,
    ACTIONS(3), 1,
      anon_sym_SEMI,
    ACTIONS(91), 1,
      anon_sym_LPAREN,
    STATE(27), 1,
      sym_comment,
  [501] = 3,
    ACTIONS(3), 1,
      anon_sym_SEMI,
    ACTIONS(93), 1,
      anon_sym_LPAREN,
    STATE(28), 1,
      sym_comment,
  [511] = 3,
    ACTIONS(3), 1,
      anon_sym_SEMI,
    ACTIONS(95), 1,
      anon_sym_RPAREN,
    STATE(29), 1,
      sym_comment,
  [521] = 3,
    ACTIONS(3), 1,
      anon_sym_SEMI,
    ACTIONS(97), 1,
      anon_sym_RPAREN,
    STATE(30), 1,
      sym_comment,
  [531] = 3,
    ACTIONS(3), 1,
      anon_sym_SEMI,
    ACTIONS(99), 1,
      ts_builtin_sym_end,
    STATE(31), 1,
      sym_comment,
  [541] = 1,
    ACTIONS(101), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 30,
  [SMALL_STATE(4)] = 67,
  [SMALL_STATE(5)] = 104,
  [SMALL_STATE(6)] = 140,
  [SMALL_STATE(7)] = 169,
  [SMALL_STATE(8)] = 198,
  [SMALL_STATE(9)] = 225,
  [SMALL_STATE(10)] = 241,
  [SMALL_STATE(11)] = 257,
  [SMALL_STATE(12)] = 273,
  [SMALL_STATE(13)] = 289,
  [SMALL_STATE(14)] = 305,
  [SMALL_STATE(15)] = 323,
  [SMALL_STATE(16)] = 343,
  [SMALL_STATE(17)] = 363,
  [SMALL_STATE(18)] = 382,
  [SMALL_STATE(19)] = 401,
  [SMALL_STATE(20)] = 418,
  [SMALL_STATE(21)] = 430,
  [SMALL_STATE(22)] = 441,
  [SMALL_STATE(23)] = 451,
  [SMALL_STATE(24)] = 461,
  [SMALL_STATE(25)] = 471,
  [SMALL_STATE(26)] = 481,
  [SMALL_STATE(27)] = 491,
  [SMALL_STATE(28)] = 501,
  [SMALL_STATE(29)] = 511,
  [SMALL_STATE(30)] = 521,
  [SMALL_STATE(31)] = 531,
  [SMALL_STATE(32)] = 541,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [9] = {.entry = {.count = 1, .reusable = false}}, SHIFT(10),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [13] = {.entry = {.count = 1, .reusable = false}}, SHIFT(28),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [21] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_region, 1),
  [23] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [25] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_region, 2),
  [27] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [29] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_subregion, 2),
  [31] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_subregion, 1),
  [33] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2),
  [35] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(2),
  [38] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(10),
  [41] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_zero_pad, 5),
  [43] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_zero_pad, 5),
  [45] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__content, 1),
  [47] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__content, 1),
  [49] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_decimal, 5),
  [51] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_decimal, 5),
  [53] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 4),
  [55] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string, 4),
  [57] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 1),
  [59] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 1),
  [61] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_region_repeat1, 2),
  [63] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_region_repeat1, 2), SHIFT_REPEAT(7),
  [66] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_region, 3),
  [68] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 2),
  [70] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat2, 2),
  [72] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat2, 2), SHIFT_REPEAT(3),
  [75] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_region_repeat1, 1),
  [77] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat2, 1),
  [79] = {.entry = {.count = 1, .reusable = false}}, SHIFT(22),
  [81] = {.entry = {.count = 1, .reusable = false}}, SHIFT(32),
  [83] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [85] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [87] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [89] = {.entry = {.count = 1, .reusable = false}}, SHIFT(25),
  [91] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [93] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [95] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [97] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [99] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [101] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_comment, 2),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_hexspec(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
