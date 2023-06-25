# `fibash`

An experimental tool for maintaining safe, useful, portable `bash` scripts using a subset of `fish`.

Goals:

- Safe, useful, portable output (in that order).
- Support maintaining the `fibash` source directly in a `bash` script (using a trailing `HEREDOC`).
- Make it difficult to introduce common security/correctness bugs:
  - Fail on command error by default.
  - Enforce good string escaping practices.
  - Enforce good practices for piping, process substitution, status checks, and logic.
- Accept a strict subset of `fish`.
  - Concession: it may be worth adding small extensions that can be compiled into `fish` as well as `bash`.
- Output idiomatic `bash` where possible.

Stretch goals:

- Find a way to introduce types (string, "stringly typed" numbers, status codes).
- Kind errors that suggest alternative syntax.

Non-goals:

- Enable arbitrary `bash` scripts.
- Support more than one self-contained script file at a time.
- Become a replacement scripting language.
  - [`fish`](https://fishshell.com/), [YSH](https://www.oilshell.org/cross-ref.html?tag=YSH#YSH), and [Nushell](https://www.nushell.sh/) are viable replacements for `bash` where they can be installed.
  - [OSH](http://www.oilshell.org/release/latest/doc/osh-help.html) allows authoring `bash`-compatible code.

## Example use cases

- Initialization scripts:
  - Dependency installation code.
  - Container initialization code.
  - Dotfile ["bootstrap"](https://docs.github.com/en/codespaces/customizing-your-codespace/personalizing-github-codespaces-for-your-account#dotfiles) scripts
- One-off scripts that need to be portable above else.
- Scritps for lightweight projects that don't want to require additional dependencies.
