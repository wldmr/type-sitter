# type-sitter: generate typed wrappers for tree-sitter grammars from node-types.json and (WIP) ~~queries~~

***Note:** type-sitter is still in the very early stages and as such the API is subject to change.*

## Overview

type-sitter is a library, CLI tool, and procedural-macro which generates type-safe wrappers for tree-sitter nodes from a tree-sitter grammar. These wrappers contain methods to access the node's fields and children, and nodes with subtypes are represented as `enum`s. The wrappers also encourage good practices by explicitly handling "error" and "extra" nodes, while also providing convenience methods like variant selectors and `flatten` to ease some of the verbosity.

### tree-sitter-wrapper

Additionally, in `type-sitter-lib`, the `tree-sitter-wrapper` feature provides another wrapper over tree-sitter nodes. This wrapper is general-purpose: it provides the ability to get node's text directly from the node itself as a `&str`, assign arbitrary bitmasks ("marks") to each node, and other convenience methods.

`type-sitter` can generate typed nodes which wrap `tree_sitter_wrapper::Node`, or it can generate nodes which only wrap `tree_sitter::Node` directly. Or, using the library `type-sitter-gen` you can generate nodes which wrap an arbitrary `...::Node` datatype.

## Drawbacks

`type-sitter`'s main drawback is that as of now, the generated wrapper code is very large: the generated node wrappers for `tree-sitter-rust` are 30755 LOC. On my M1 Macbook Air running IntelliJ, this makes code analysis startup time fairly long (~1 minute), though fortunately completions still seem to be fast and everything works. There are potential future steps to reduce code size such as replacing enums with generic types, but these have their own drawbacks (more complex resolution, may not be effective).

Another issue is that some grammars may generate duplicate datatype definitions (see [Naming Rules](#naming-rules)): this should be very uncommon because it will only happen if their names are weirdly similar, but if it does, there is currently no workaround as there is no way to rename generated datatypes.

`tree-sitter-wrapper`'s drawbacks are slightly lower performance and limitations. The lower performance is because `tree-sitter-wrapper` requires all nodes and most other datatypes to have an additional reference to `tree_sitter_wrapper::Tree` for capabilities like the ability for nodes to get their own text. The limitations are that `tree_sitter_wrapper` requires the parsed source code to be UTF-8 compliant and be stored along with the tree itself, and certain aspects of incremental parsing may not work with the convenience methods (specifically, text offsets may be messed up).

Lastly, keep in mind that both libraries are still in the early stages of development, so they may have bugs and API is subject to change (though will try to preserve naming rules).

## Naming Rules

`type-sitter` generates datatype based on the names of the nodes in the grammar. However, these nodes are in snake-case and contain punctuation which is illegal in Rust, so we convert them to camel-case and perform the following illegal-character substitutions:

- `&` ⇒ `And`
- `|` ⇒ `Or`
- `!` ⇒ `Not`
- `=` ⇒ `Eq`
- `<` ⇒ `Lt`
- `>` ⇒ `Gt`
- `+` ⇒ `Add`
- `-` ⇒ `Sub`
- `*` ⇒ `Mul`
- `/` ⇒ `Div`
- `~` ⇒ `BitNot`
- `%` ⇒ `Mod`
- `^` ⇒ `BitXor`
- `?` ⇒ `Question`
- `:` ⇒ `Colon`
- `.` ⇒ `Dot`
- `,` ⇒ `Comma`
- `;` ⇒ `Semicolon`
- `(` ⇒ `LParen`
- `)` ⇒ `RParen`
- `[` ⇒ `LBracket`
- `]` ⇒ `RBracket`
- `{` ⇒ `LBrace`
- `}` ⇒ `RBrace`
- `\` ⇒ `Backslash`
- `'` ⇒ `Quote`
- `"` ⇒ `DoubleQuote`
- `#` ⇒ `Hash`
- `@` ⇒ `At`
- `$` ⇒ `Dollar`
- `` ` `` ⇒ `Backtick`
- ` ` ⇒ `Space`
- `\t` ⇒ `Tab`
- `\n` ⇒ `Newline`
- `\r` ⇒ `CarriageReturn`
- Any other character ⇒ `U` + the character's Unicode codepoint in upper-hex

For method names (variant selectors), we simply convert back to snake case.

Additionally, if a node is implicit (starts with `_`), we remove the prepended `_`

Lastly, if a type or method name is an illegal definition identifier (`Self`, `self`, `super`, `crate`, `_`, or anything which starts with a number), `type-sitter` prepends an `_`. If it's a Rust keyword, `type-sitter` prepends `r#`.

Naming rules also determine the module. Unnamed nodes and symbols are in modules specifically to reduce naming conflicts without having to actually rename the nodes.

- Unnamed and contains symbols: `symbol::`
- Unnamed and doesn't contain symbols: `unnamed::`
- Otherwise the node is at the toplevel of the generated source

The source for all this is at [`type-sitter-gen/src/names.rs`](type-sitter-gen/src/names.rs).

### Naming Rule Examples

- `_declaration_statement` ⇒ `DeclarationStatement`
- `use_declaration` ⇒ `UseDeclaration`
- `self` ⇒ `unnamed::_Self`
- `%` ⇒ `symbols::Mod`
- `mod` ⇒ `unnamed::Mod`
- `true` selector ⇒ `r#true` (`true` ⇒ `unnamed::True`)

## Example

```rust
use tree_sitter::{Parser, Tree};
use type_sitter_lib::{Either2, TypedNode};

pub fn get_import_paths_unsafe(tree: &Tree, text: &str) -> Vec<String> {
    // BAD: what if we spell the field names wrong?
    tree.root_node().children(&mut tree.walk())
        .filter(|n| n.kind() == "use_declaration")
        .filter_map(|n| n.child_by_field_name("argument"))
        .filter_map(|n| n.child_by_field_name("path"))
        .filter_map(|n| n.utf8_text(text.as_bytes()))
        .map(|s| s.to_string())
        .collect()
}

pub fn get_import_paths_safe(tree: &Tree, text: &str) -> Vec<String> {
    // GOOD: fields are type-safe, and we get IDE inference
    rust::SourceFile::try_from(tree.root_node()).unwrap().children(&mut tree.walk())
        .filter_map(|n| n.declaration_statement())
        .filter_map(|n| n.use_declaration())
        .filter_map(|n| n.argument())
        .filter_map(|n| n.scoped_identifier())
        .filter_map(|n| n.path().flatten())
        .filter_map(|n| n.identifier())
        .filter_mao(|n| n.utf8_text(code_str.as_bytes()))
        .map(|s| s.to_string())
        .collect()
}

// We can also define methods which only take nodes of certain types
pub fn process_declaration(decl: rust::DeclarationStatement<'_>) {
    // ...
}
```

## Usage

In order to generate the bindings, you can either invoke `type-sitter-cli` directly, or use the procedural macros in `type-sitter-proc`. The CLI tool is recommended, as it's more flexible and will give your IDE better inference.

The generated code depends on `type-sitter-lib`, so you must include `type-sitter-lib` as a dependency.

### Basic usage

```shell
# If not already installed
cargo install type-sitter-cli
# In your cargo project root directory
type-sitter-cli path/to/node-types.json
# To add type-sitter-lib as a dependency (also in cargo root)
cargo add type-sitter-lib
```

#### Advanced usage

```shell
# Add type-sitter-lib with the tree-sitter-wrapper feature (see above section)
cargo add type-sitter-lib --features tree-sitter-wrapper
# Specify a custom output directory and use tree-sitter-wrapper
type-sitter-cli vendor/tree-sitter-rust/node-types.json -o generated_src --use-wrapper
# You can generate bindings for multiple grammars in the same project
type-sitter-cli vendor/tree-sitter-rust/node-types.json -o generated_src --use-wrapper
# To see help for the CLI program
type-sitter-cli --help
```

## Future plans

Also generate typed query wrappers, which have type-safe selectors that also return typed node wrappers.

(Also reduce LOC and improve API where possible, and fix bugs)

## Comparison to [rust-sitter](https://www.shadaj.me/writing/introducing-rust-sitter)

[rust-sitter](https://www.shadaj.me/writing/introducing-rust-sitter) is the primary alternative which also provides convenience over tree-sitter's Rust API. However, rust-sitter takes a much different approach by fully generating the tree-sitter grammar from a Rust file.

Advantages of type-sitter:

- arbitrary tree-sitter grammars, not only ones written in Rust
- Error node and incremental parsing support, since typed nodes directly wrap `tree-sitter` nodes
- Less API difference from the native tree-sitter API: if you don't use `tree-sitter-wrapper` feature it only provides typed wrappers for nodes, and even `tree-sitter-wrapper` really only provides convenience methods on top of the base API
- Less complexity because of the above

Advantages of rust-sitter:

- More control over the typed nodes, since you define them yourself
- May generate less boilerplate especially because of the extra control
- type-sitter is in the much earlier stages, and it's more likely to have bugs and API changes

## Contributing

Feel free to submit an issue or pull request if you want a new feature or anything is missing, and don't hesitate to submit an issue if you encounter any bugs or have any questions.

## Licence

The code is licensed under MIT or Apache 2.0 (you choose), which is the norm for Rust packages.
