# Rust Template `rust-lib-template`

A template for `cargo-generate` to create a new Rust package.

[![Apache-2.0 License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![MIT License](https://img.shields.io/badge/license-mit-118811.svg)](https://opensource.org/license/mit)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-lib-template.svg)](<https://github.com/johnstonskj/rust-lib-template/stargazers>)

## Installation

Install [cargo-generate](https://github.com/cargo-generate/cargo-generate);
installing cargo, if necessary, using [rustup](https://rustup.rs/).

```bash
❱ cargo install cargo-generate
```

### Usage

The simplest form of the command is shown in the example below with the
template identified using the Github `<username>/<reponame>` form. The
`--allow-commands` flag is added as the template configuration executes
a number of commands to get environment settings, which otherwise would
prompt you each time whether you are OK with executing an external command.

```bash
❱ cargo generate johnstonskj/rust-lib-template --allow-commands
🔧   Destination: /Users/skj/Projects/rust/mylib ...
🔧   project-name: mylib ...
🔧   Generating template ...
🤷   One-line package description: My nice new shiny crate
🤷   Package version (semver): 0.1.0
✔ 🤷   Initialize git? · true
✔ 🤷   Include mdbook documentation? · false
✔ 🤷   Include command-line tool? · false
🤷   Github repository name prefix: rust-
🤷   Github user name: johnstonskj
[ 1/37]   Done: ...
```

The name of the project can be specified using `--name` and the destination
with `--init` for the current directory, `--destination` for a specified path,
or the default is a directory with the same name as `--name` in the current
directory.

The rest of the *placeholder values* can be included on the command line using
the `--define` flag. The following table is the set of placeholders in this
template and the example following shows the tool operating in *silent* mode
where all values are provided by the command-line arguments.

| Placeholder           | Type    | Default | Description                   |
|-----------------------|---------|---------|-------------------------------|
| `package_description` | string  | None    | One-line package description  |
| `package_version`     | string  | `0.1.0` | Package version (semver)      |
| `gh-repo-prefix`      | string  | `rust-` | Github repository name prefix |
| `use_git`             | boolean | `true`  | Initialize git?               |
| `has_book`            | boolean | `true`  | Include mdbook documentation? |
| `has_cli`             | boolean | `true`  | Include command-line tool?    |

```bash
❱ cargo generate johnstonskj/rust-lib-template \
⋯⋯❱ --name crate-name \
⋯⋯❱ --destination ./rust-crate-name \
⋯⋯❱ --define package_description="My nice new shiny crate" \
⋯⋯❱ --define has_book=false \
⋯⋯❱ --define has_cli=false \
⋯⋯❱ --allow-commands \
⋯⋯❱ --silent
```

## Template Content

This template provides the following:

- Generic Project:
  - `README.md`, including short form of licenses and contributing pointer.
  - `LICENSE-APACHE`, `LICENSE-MIT` license files.
  - `CODE_OF_CONDUCT.md`
  - `CONTRIBUTING.md`
- Rust library:
  - `lib.rs` with extensive lint settings.
  - `error.rs` with initial `Error` and `Return` implementations.
  - `bin/main.rs` with skeleton; note this can be surpressed with the template's `has_cli` flag.
  - `tests` folder with skeletons.
  - `doc/book` mdbook skeleton; note this can be surpressed with the template's `has_book` flag.
- Rust automation:
  - `.rustfmt.toml`
  - `cliff.toml`
  - `codecov.yml`
  - `deny.toml`
  - `dist-workspace.toml`
  - `release-plz.toml`
- Git-related:
  - `.gitignore` for Emacs, VSCode and Rust.
  - `rust-pre-release-hook`, copy to `.git/hooks/` as `pre-release` if you want.
- Github-related:
  - issue templates for *bug_report* and *feature_request*.
  - template for *pull_request*.
  - `settings.yml` for the repository itself if you use the [settings app](https://github.com/repository-settings).
  - `dependabot.yml` config for [dependabot](https://github.com/dependabot).
  - `mergify.yml` config for [mergify](https://docs.mergify.com/integrations/github/) to automate dependabot PRs.
  - Workflows
    - *mdbook to pages* build mdbook documentation and publish to Github pages. Note this can be surpressed with the template's `has_book` flag.
    - *Release distributions* release binary distributions on Github release pages (uses `dist-workspace.toml`).
    - *Release to crates.io* release versions to crates.io (uses `release-plz.toml` and `cliff.toml`).
    - *Rust build and test* full build and test workflow (uses `.rustfmt.toml` and `codecov.yml`).
    - *Rust security audit* security audits, cargo **audit** and cargo **deny** (uses `deny.toml`).
    - *Spell check* the typos tool.

## License(s)

The contents of this repository are made available under the following
licenses:

### Apache-2.0

> ```text
> Copyright 2025 Simon Johnston <johnstonskj@gmail.com>
> 
> Licensed under the Apache License, Version 2.0 (the "License");
> you may not use this file except in compliance with the License.
> You may obtain a copy of the License at
> 
>     http://www.apache.org/licenses/LICENSE-2.0
> 
> Unless required by applicable law or agreed to in writing, software
> distributed under the License is distributed on an "AS IS" BASIS,
> WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> See the License for the specific language governing permissions and
> limitations under the License.
> ```

See the enclosed file [LICENSE-Apache](https://github.com/johnstonskj/rust-lib-template/blob/main/LICENSE-Apache).

### MIT

> ```text
> Copyright 2025 Simon Johnston <johnstonskj@gmail.com>
> 
> Permission is hereby granted, free of charge, to any person obtaining a copy
> of this software and associated documentation files (the “Software”), to deal
> in the Software without restriction, including without limitation the rights to
> use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
> the Software, and to permit persons to whom the Software is furnished to do so,
> subject to the following conditions:
> 
> The above copyright notice and this permission notice shall be included in all
> copies or substantial portions of the Software.
> 
> THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
> INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
> PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
> HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
> OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
> SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
> ```

See the enclosed file [LICENSE-MIT](https://github.com/johnstonskj/rust-lib-template/blob/main/LICENSE-MIT).

### Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING](https://github.com/johnstonskj/rust-lib-template/blob/main/template/CONTRIBUTING.md) and the
project's [CODE_OF_CONDUCT](https://github.com/johnstonskj/rust-lib-template/blob/main/template/CODE_OF_CONDUCT.md).
