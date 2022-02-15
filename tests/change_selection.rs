use std::ffi::OsString;

use crossterm::event::{Event, KeyCode};

use helper::{create_tree, MockTerminal, MockTerminalEvent};
use thwack::entrypoint;

mod helper;

#[test]
fn cannot_move_up_because_selection_reaches_to_top() {
    let dir = create_tree().unwrap();
    let args = args![
        "thwack",
        "--starting-point",
        dir.path().to_str().unwrap(),
        "--status-line=relative"
    ];
    let mut event = MockTerminalEvent::new();
    event.add(Some(Event::Key(KeyCode::Up.into())));
    event.add(Some(Event::Key(KeyCode::Up.into())));
    event.add(Some(Event::Key(KeyCode::Up.into())));
    event.add(Some(Event::Key(KeyCode::Esc.into())));
    let mut buffer = buf!();
    let mut stderr = buf!();
    let result = entrypoint(args, &mut buffer, &mut stderr, MockTerminal, event);
    assert!(result.is_ok());
    assert_eq!(
        buffer.normalize_path(),
        buf!(
            b"\x1b[?1049h\x1b[0m",
            b"\x1b[1;1H\x1b[J",
            b"Search: \x1b7\x1b[1E",
            b"> .browserslistrc\x1b[1E",
            b"  .editorconfig\x1b[1E",
            b"  .env\x1b[1E",
            b"  .env.local\x1b[1E",
            b"  .gitignore\x1b[1E",
            b"  .npmrc\x1b[1E",
            b"  .nvmrc\x1b[1E",
            b"  Dockerfile\x1b[1E",
            b"  LICENSE\x1b[1E",
            b"  README.md\x1b[1E",
            b"  package-lock.json\x1b[1E",
            b"  package.json\x1b[1E",
            b"  tsconfig.json\x1b[1E",
            b"  \xe2\x98\x95.txt\x1b[1E",
            b"  .config/bar.toml\x1b[1E",
            b"  .config/ok.toml\x1b[1E",
            b"  lib/bar.js\x1b[1E",
            b"\x1b[19d\x1b[1m\x1b[7m.browserslistrc                                                                 \x1b[0m\x1b[1E",
            b"\x1b[1m<Up>/<Ctrl-p>:\x1b[0m\x1b[1CUp\x1b[2C\x1b[1m<Down>/<Ctrl-n>:\x1b[0m\x1b[1CDown\x1b[2C\x1b[1m<Enter>:\x1b[0m\x1b[1CExecute",
            b"\x1b8",
            b"\x1b[1;1H\x1b[J",
            b"Search: \x1b7\x1b[1E",
            b"> .browserslistrc\x1b[1E",
            b"  .editorconfig\x1b[1E",
            b"  .env\x1b[1E",
            b"  .env.local\x1b[1E",
            b"  .gitignore\x1b[1E",
            b"  .npmrc\x1b[1E",
            b"  .nvmrc\x1b[1E",
            b"  Dockerfile\x1b[1E",
            b"  LICENSE\x1b[1E",
            b"  README.md\x1b[1E",
            b"  package-lock.json\x1b[1E",
            b"  package.json\x1b[1E",
            b"  tsconfig.json\x1b[1E",
            b"  \xe2\x98\x95.txt\x1b[1E",
            b"  .config/bar.toml\x1b[1E",
            b"  .config/ok.toml\x1b[1E",
            b"  lib/bar.js\x1b[1E",
            b"\x1b[19d\x1b[1m\x1b[7m.browserslistrc                                                                 \x1b[0m\x1b[1E",
            b"\x1b[1m<Up>/<Ctrl-p>:\x1b[0m\x1b[1CUp\x1b[2C\x1b[1m<Down>/<Ctrl-n>:\x1b[0m\x1b[1CDown\x1b[2C\x1b[1m<Enter>:\x1b[0m\x1b[1CExecute",
            b"\x1b8",
            b"\x1b[1;1H\x1b[J",
            b"Search: \x1b7\x1b[1E",
            b"> .browserslistrc\x1b[1E",
            b"  .editorconfig\x1b[1E",
            b"  .env\x1b[1E",
            b"  .env.local\x1b[1E",
            b"  .gitignore\x1b[1E",
            b"  .npmrc\x1b[1E",
            b"  .nvmrc\x1b[1E",
            b"  Dockerfile\x1b[1E",
            b"  LICENSE\x1b[1E",
            b"  README.md\x1b[1E",
            b"  package-lock.json\x1b[1E",
            b"  package.json\x1b[1E",
            b"  tsconfig.json\x1b[1E",
            b"  \xe2\x98\x95.txt\x1b[1E",
            b"  .config/bar.toml\x1b[1E",
            b"  .config/ok.toml\x1b[1E",
            b"  lib/bar.js\x1b[1E",
            b"\x1b[19d\x1b[1m\x1b[7m.browserslistrc                                                                 \x1b[0m\x1b[1E",
            b"\x1b[1m<Up>/<Ctrl-p>:\x1b[0m\x1b[1CUp\x1b[2C\x1b[1m<Down>/<Ctrl-n>:\x1b[0m\x1b[1CDown\x1b[2C\x1b[1m<Enter>:\x1b[0m\x1b[1CExecute",
            b"\x1b8",
            b"\x1b[1;1H\x1b[J",
            b"Search: \x1b7\x1b[1E",
            b"> .browserslistrc\x1b[1E",
            b"  .editorconfig\x1b[1E",
            b"  .env\x1b[1E",
            b"  .env.local\x1b[1E",
            b"  .gitignore\x1b[1E",
            b"  .npmrc\x1b[1E",
            b"  .nvmrc\x1b[1E",
            b"  Dockerfile\x1b[1E",
            b"  LICENSE\x1b[1E",
            b"  README.md\x1b[1E",
            b"  package-lock.json\x1b[1E",
            b"  package.json\x1b[1E",
            b"  tsconfig.json\x1b[1E",
            b"  \xe2\x98\x95.txt\x1b[1E",
            b"  .config/bar.toml\x1b[1E",
            b"  .config/ok.toml\x1b[1E",
            b"  lib/bar.js\x1b[1E",
            b"\x1b[19d\x1b[1m\x1b[7m.browserslistrc                                                                 \x1b[0m\x1b[1E",
            b"\x1b[1m<Up>/<Ctrl-p>:\x1b[0m\x1b[1CUp\x1b[2C\x1b[1m<Down>/<Ctrl-n>:\x1b[0m\x1b[1CDown\x1b[2C\x1b[1m<Enter>:\x1b[0m\x1b[1CExecute",
            b"\x1b8",
            b"\x1b[?1049l"
        )
    );
}

#[test]
fn cannot_move_down_because_selection_reaches_to_bottom() {
    let dir = create_tree().unwrap();
    let args = args![
        "thwack",
        "--starting-point",
        dir.path().to_str().unwrap(),
        "--status-line=relative"
    ];
    let mut event = MockTerminalEvent::new();
    event.add(Some(Event::Key(KeyCode::Char('b').into())));
    event.add(Some(Event::Key(KeyCode::Char('r').into())));
    event.add(Some(Event::Key(KeyCode::Char('o').into())));
    event.add(Some(Event::Key(KeyCode::Char('w').into())));
    event.add(Some(Event::Key(KeyCode::Char('s').into())));
    event.add(Some(Event::Key(KeyCode::Char('e').into())));
    event.add(Some(Event::Key(KeyCode::Char('r').into())));
    event.add(None);
    event.add(Some(Event::Key(KeyCode::Down.into())));
    event.add(Some(Event::Key(KeyCode::Down.into())));
    event.add(Some(Event::Key(KeyCode::Esc.into())));
    let mut buffer = buf!();
    let mut stderr = buf!();
    let result = entrypoint(args, &mut buffer, &mut stderr, MockTerminal, event);
    assert!(result.is_ok());
    assert_eq!(
        buffer.normalize_path(),
        buf!(
            b"\x1b[?1049h\x1b[0m",
            b"\x1b[1;1H\x1b[J",
            b"Search: \x1b7\x1b[1E",
            b"> .browserslistrc\x1b[1E",
            b"  .editorconfig\x1b[1E",
            b"  .env\x1b[1E",
            b"  .env.local\x1b[1E",
            b"  .gitignore\x1b[1E",
            b"  .npmrc\x1b[1E",
            b"  .nvmrc\x1b[1E",
            b"  Dockerfile\x1b[1E",
            b"  LICENSE\x1b[1E",
            b"  README.md\x1b[1E",
            b"  package-lock.json\x1b[1E",
            b"  package.json\x1b[1E",
            b"  tsconfig.json\x1b[1E",
            b"  \xe2\x98\x95.txt\x1b[1E",
            b"  .config/bar.toml\x1b[1E",
            b"  .config/ok.toml\x1b[1E",
            b"  lib/bar.js\x1b[1E",
            b"\x1b[19d\x1b[1m\x1b[7m.browserslistrc                                                                 \x1b[0m\x1b[1E",
            b"\x1b[1m<Up>/<Ctrl-p>:\x1b[0m\x1b[1CUp\x1b[2C\x1b[1m<Down>/<Ctrl-n>:\x1b[0m\x1b[1CDown\x1b[2C\x1b[1m<Enter>:\x1b[0m\x1b[1CExecute",
            b"\x1b8",
            b"\x1b[1;1H\x1b[J",
            b"Search: b\x1b7\x1b[1E",
            b"> .browserslistrc\x1b[1E",
            b"  .editorconfig\x1b[1E",
            b"  .env\x1b[1E",
            b"  .env.local\x1b[1E",
            b"  .gitignore\x1b[1E",
            b"  .npmrc\x1b[1E",
            b"  .nvmrc\x1b[1E",
            b"  Dockerfile\x1b[1E",
            b"  LICENSE\x1b[1E",
            b"  README.md\x1b[1E",
            b"  package-lock.json\x1b[1E",
            b"  package.json\x1b[1E",
            b"  tsconfig.json\x1b[1E",
            b"  \xe2\x98\x95.txt\x1b[1E",
            b"  .config/bar.toml\x1b[1E",
            b"  .config/ok.toml\x1b[1E",
            b"  lib/bar.js\x1b[1E",
            b"\x1b[19d\x1b[1m\x1b[7m.browserslistrc                                                                 \x1b[0m\x1b[1E",
            b"\x1b[1m<Up>/<Ctrl-p>:\x1b[0m\x1b[1CUp\x1b[2C\x1b[1m<Down>/<Ctrl-n>:\x1b[0m\x1b[1CDown\x1b[2C\x1b[1m<Enter>:\x1b[0m\x1b[1CExecute",
            b"\x1b8",
            b"\x1b[1;1H\x1b[J",
            b"Search: br\x1b7\x1b[1E",
            b"> .browserslistrc\x1b[1E",
            b"  .editorconfig\x1b[1E",
            b"  .env\x1b[1E",
            b"  .env.local\x1b[1E",
            b"  .gitignore\x1b[1E",
            b"  .npmrc\x1b[1E",
            b"  .nvmrc\x1b[1E",
            b"  Dockerfile\x1b[1E",
            b"  LICENSE\x1b[1E",
            b"  README.md\x1b[1E",
            b"  package-lock.json\x1b[1E",
            b"  package.json\x1b[1E",
            b"  tsconfig.json\x1b[1E",
            b"  \xe2\x98\x95.txt\x1b[1E",
            b"  .config/bar.toml\x1b[1E",
            b"  .config/ok.toml\x1b[1E",
            b"  lib/bar.js\x1b[1E",
            b"\x1b[19d\x1b[1m\x1b[7m.browserslistrc                                                                 \x1b[0m\x1b[1E",
            b"\x1b[1m<Up>/<Ctrl-p>:\x1b[0m\x1b[1CUp\x1b[2C\x1b[1m<Down>/<Ctrl-n>:\x1b[0m\x1b[1CDown\x1b[2C\x1b[1m<Enter>:\x1b[0m\x1b[1CExecute",
            b"\x1b8",
            b"\x1b[1;1H\x1b[J",
            b"Search: bro\x1b7\x1b[1E",
            b"> .browserslistrc\x1b[1E",
            b"  .editorconfig\x1b[1E",
            b"  .env\x1b[1E",
            b"  .env.local\x1b[1E",
            b"  .gitignore\x1b[1E",
            b"  .npmrc\x1b[1E",
            b"  .nvmrc\x1b[1E",
            b"  Dockerfile\x1b[1E",
            b"  LICENSE\x1b[1E",
            b"  README.md\x1b[1E",
            b"  package-lock.json\x1b[1E",
            b"  package.json\x1b[1E",
            b"  tsconfig.json\x1b[1E",
            b"  \xe2\x98\x95.txt\x1b[1E",
            b"  .config/bar.toml\x1b[1E",
            b"  .config/ok.toml\x1b[1E",
            b"  lib/bar.js\x1b[1E",
            b"\x1b[19d\x1b[1m\x1b[7m.browserslistrc                                                                 \x1b[0m\x1b[1E",
            b"\x1b[1m<Up>/<Ctrl-p>:\x1b[0m\x1b[1CUp\x1b[2C\x1b[1m<Down>/<Ctrl-n>:\x1b[0m\x1b[1CDown\x1b[2C\x1b[1m<Enter>:\x1b[0m\x1b[1CExecute",
            b"\x1b8",
            b"\x1b[1;1H\x1b[J",
            b"Search: brow\x1b7\x1b[1E",
            b"> .browserslistrc\x1b[1E",
            b"  .editorconfig\x1b[1E",
            b"  .env\x1b[1E",
            b"  .env.local\x1b[1E",
            b"  .gitignore\x1b[1E",
            b"  .npmrc\x1b[1E",
            b"  .nvmrc\x1b[1E",
            b"  Dockerfile\x1b[1E",
            b"  LICENSE\x1b[1E",
            b"  README.md\x1b[1E",
            b"  package-lock.json\x1b[1E",
            b"  package.json\x1b[1E",
            b"  tsconfig.json\x1b[1E",
            b"  \xe2\x98\x95.txt\x1b[1E",
            b"  .config/bar.toml\x1b[1E",
            b"  .config/ok.toml\x1b[1E",
            b"  lib/bar.js\x1b[1E",
            b"\x1b[19d\x1b[1m\x1b[7m.browserslistrc                                                                 \x1b[0m\x1b[1E",
            b"\x1b[1m<Up>/<Ctrl-p>:\x1b[0m\x1b[1CUp\x1b[2C\x1b[1m<Down>/<Ctrl-n>:\x1b[0m\x1b[1CDown\x1b[2C\x1b[1m<Enter>:\x1b[0m\x1b[1CExecute",
            b"\x1b8",
            b"\x1b[1;1H\x1b[J",
            b"Search: brows\x1b7\x1b[1E",
            b"> .browserslistrc\x1b[1E",
            b"  .editorconfig\x1b[1E",
            b"  .env\x1b[1E",
            b"  .env.local\x1b[1E",
            b"  .gitignore\x1b[1E",
            b"  .npmrc\x1b[1E",
            b"  .nvmrc\x1b[1E",
            b"  Dockerfile\x1b[1E",
            b"  LICENSE\x1b[1E",
            b"  README.md\x1b[1E",
            b"  package-lock.json\x1b[1E",
            b"  package.json\x1b[1E",
            b"  tsconfig.json\x1b[1E",
            b"  \xe2\x98\x95.txt\x1b[1E",
            b"  .config/bar.toml\x1b[1E",
            b"  .config/ok.toml\x1b[1E",
            b"  lib/bar.js\x1b[1E",
            b"\x1b[19d\x1b[1m\x1b[7m.browserslistrc                                                                 \x1b[0m\x1b[1E",
            b"\x1b[1m<Up>/<Ctrl-p>:\x1b[0m\x1b[1CUp\x1b[2C\x1b[1m<Down>/<Ctrl-n>:\x1b[0m\x1b[1CDown\x1b[2C\x1b[1m<Enter>:\x1b[0m\x1b[1CExecute",
            b"\x1b8",
            b"\x1b[1;1H\x1b[J",
            b"Search: browse\x1b7\x1b[1E",
            b"> .browserslistrc\x1b[1E",
            b"  .editorconfig\x1b[1E",
            b"  .env\x1b[1E",
            b"  .env.local\x1b[1E",
            b"  .gitignore\x1b[1E",
            b"  .npmrc\x1b[1E",
            b"  .nvmrc\x1b[1E",
            b"  Dockerfile\x1b[1E",
            b"  LICENSE\x1b[1E",
            b"  README.md\x1b[1E",
            b"  package-lock.json\x1b[1E",
            b"  package.json\x1b[1E",
            b"  tsconfig.json\x1b[1E",
            b"  \xe2\x98\x95.txt\x1b[1E",
            b"  .config/bar.toml\x1b[1E",
            b"  .config/ok.toml\x1b[1E",
            b"  lib/bar.js\x1b[1E",
            b"\x1b[19d\x1b[1m\x1b[7m.browserslistrc                                                                 \x1b[0m\x1b[1E",
            b"\x1b[1m<Up>/<Ctrl-p>:\x1b[0m\x1b[1CUp\x1b[2C\x1b[1m<Down>/<Ctrl-n>:\x1b[0m\x1b[1CDown\x1b[2C\x1b[1m<Enter>:\x1b[0m\x1b[1CExecute",
            b"\x1b8",
            b"\x1b[1;1H\x1b[J",
            b"Search: browser\x1b7\x1b[1E",
            b"> .browserslistrc\x1b[1E",
            b"  .editorconfig\x1b[1E",
            b"  .env\x1b[1E",
            b"  .env.local\x1b[1E",
            b"  .gitignore\x1b[1E",
            b"  .npmrc\x1b[1E",
            b"  .nvmrc\x1b[1E",
            b"  Dockerfile\x1b[1E",
            b"  LICENSE\x1b[1E",
            b"  README.md\x1b[1E",
            b"  package-lock.json\x1b[1E",
            b"  package.json\x1b[1E",
            b"  tsconfig.json\x1b[1E",
            b"  \xe2\x98\x95.txt\x1b[1E",
            b"  .config/bar.toml\x1b[1E",
            b"  .config/ok.toml\x1b[1E",
            b"  lib/bar.js\x1b[1E",
            b"\x1b[19d\x1b[1m\x1b[7m.browserslistrc                                                                 \x1b[0m\x1b[1E",
            b"\x1b[1m<Up>/<Ctrl-p>:\x1b[0m\x1b[1CUp\x1b[2C\x1b[1m<Down>/<Ctrl-n>:\x1b[0m\x1b[1CDown\x1b[2C\x1b[1m<Enter>:\x1b[0m\x1b[1CExecute",
            b"\x1b8",
            b"\x1b[1;1H\x1b[J",
            b"Search: browser\x1b7\x1b[1E",
            b"> .\x1b[1mbrowse\x1b[0mrslist\x1b[1mr\x1b[0mc\x1b[1E",
            b"\x1b[19d\x1b[1m\x1b[7m.browserslistrc                                                                 \x1b[0m\x1b[1E",
            b"\x1b[1m<Up>/<Ctrl-p>:\x1b[0m\x1b[1CUp\x1b[2C\x1b[1m<Down>/<Ctrl-n>:\x1b[0m\x1b[1CDown\x1b[2C\x1b[1m<Enter>:\x1b[0m\x1b[1CExecute",
            b"\x1b8",
            b"\x1b[?1049l"
        )
    );
}
