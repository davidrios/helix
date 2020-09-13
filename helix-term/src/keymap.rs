use crossterm::{
    event::{KeyCode, KeyEvent as Key, KeyModifiers as Modifiers},
    execute,
    style::Print,
};
use helix_core::commands::{self, Command};
use std::collections::HashMap;

// Kakoune-inspired:
// mode = {
//      normal = {
//          q = record_macro
//          w = (next) word
//          e = end of word
//          r =
//          t = 'till char
//          y = yank
//          u = undo
//          U = redo
//          i = insert
//          I = INSERT (start of line)
//          o = open below (insert on new line below)
//          O = open above (insert on new line above)
//          p = paste (before cursor)
//          P = PASTE (after cursor)
//          ` =
//          [ = select to text object start (alt = select whole object)
//          ] = select to text object end
//          { = extend to inner object start
//          } = extend to inner object end
//          a = append
//          A = APPEND (end of line)
//          s = split
//          S = select
//          d = delete()
//          f = find_char()
//          g = goto (gg, G, gc, gd, etc)
//
//          h = move_char_left(n)
//          j = move_line_down(n)
//          k = move_line_up(n)
//          l = move_char_right(n)
//          : = command line
//          ; = collapse selection to cursor
//          " = use register
//          ` = convert case? (to lower) (alt = swap case)
//          ~ = convert to upper case
//          . = repeat last command
//          \ = disable hook?
//          / = search
//          > = indent
//          < = deindent
//          % = select whole buffer (in vim = jump to matching bracket)
//          * = search pattern in selection
//          ( = rotate main selection backward
//          ) = rotate main selection forward
//          - = trim selections? (alt = merge contiguous sel together)
//          @ = convert tabs to spaces
//          & = align cursor
//          ? = extend to next given regex match (alt = to prev)
//
//          z = save selections
//          Z = restore selections
//          x = select line
//          X = extend line
//          c = change selected text
//          C = copy selection?
//          v = view menu (viewport manipulation)
//          b = select to previous word start
//          B = select to previous WORD start
//
//
//
//
//
//
//          = = align?
//          + =
//      }
// }

type Keymap = HashMap<Key, Command>;

pub fn default() -> Keymap {
    hashmap!(
        Key {
            code: KeyCode::Char('h'),
            modifiers: Modifiers::NONE
        } => commands::move_char_left as Command,
        Key {
            code: KeyCode::Char('j'),
            modifiers: Modifiers::NONE
        } => commands::move_line_down as Command,
        Key {
            code: KeyCode::Char('k'),
            modifiers: Modifiers::NONE
        } => commands::move_line_up as Command,
        Key {
            code: KeyCode::Char('l'),
            modifiers: Modifiers::NONE
        } => commands::move_char_right as Command,
        Key {
            code: KeyCode::Char('i'),
            modifiers: Modifiers::NONE
        } => commands::insert_mode as Command,
        Key {
            code: KeyCode::Char('I'),
            modifiers: Modifiers::SHIFT,
        } => commands::prepend_to_line as Command,
        Key {
            code: KeyCode::Char('a'),
            modifiers: Modifiers::NONE
        } => commands::append_mode as Command,
        Key {
            code: KeyCode::Char('A'),
            modifiers: Modifiers::SHIFT,
        } => commands::append_to_line as Command,
        Key {
            code: KeyCode::Char('o'),
            modifiers: Modifiers::NONE
        } => commands::open_below as Command,
        Key {
            code: KeyCode::Esc,
            modifiers: Modifiers::NONE
        } => commands::normal_mode as Command,
    )

    // hashmap!(
    //     Key {
    //         code: KeyCode::Esc,
    //         modifiers: Modifiers::NONE
    //     } => commands::normal_mode as Command,
    // )
}
