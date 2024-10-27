#import "template.typ" as template: *
#import "/src/lib.typ" as my-package

#let package-meta = toml("/typst.toml").package
#let date = none
// #let date = datetime(year: ..., month: ..., day: ...)

#show: manual(
  title: "Template",
  // subtitle: "...",
  authors: package-meta.authors.map(a => a.split("<").at(0).trim()),
  abstract: [
    A template for typst packages
  ],
  url: package-meta.repository,
  version: package-meta.version,
  date: date,
)

// the scope for evaluating expressions and documentation
#let scope = (my-package: my-package)

= Introduction

This is a template for typst packages. It provides the #ref-fn("parse()") and #ref-fn("eval()") functions:

#file-code("lib.typ", {
  let lib = raw(block: true, lang: "typ", read("/src/lib.typ").trim(at: end))
  lib = crudo.lines(lib, "14-17,30-")
  lib
})

Here they are in action:
#man-style.show-example(mode: "markup", dir: ttb, scope: scope, ```typ
$2 * (2 + x) arrow.double.long$ #my-package.parse("2 * (2 + x)")
```)

#man-style.show-example(mode: "markup", dir: ttb, scope: scope, ```typ
$2 * (2 + x) arrow.double.long^(x=3)$ #my-package.eval("2 * (2 + x)", x: 3)
```)

= Plugin implementation

This plugin uses #link("https://lalrpop.github.io/lalrpop/")[LALRPOP] to implement a parser for a domain specific language (DSL) in Rust; in particular, the parser is for mathematical expressions based on the #link("https://lalrpop.github.io/lalrpop/tutorial/007_fallible_actions.html")[LALRPOP tutorial].

Here are some relevant excerpts from the plugin's code:

#file-code("ast.rs", {
  let lib = raw(block: true, lang: "rust", read("/plugin/src/ast.rs").trim(at: end))
  lib = crudo.lines(lib, "7-33")
  lib
})

#file-code("parser/mod.rs", {
  let lib = raw(block: true, lang: "rust", read("/plugin/src/parser/mod.rs").trim(at: end))
  lib = crudo.lines(lib, "4-17")
  lib
})

#file-code("grammar.lalrpop", {
  show: template.codly.local.with(languages: (
    rust: (name: "lalrpop"),
  ))

  let lib = raw(block: true, lang: "rust", read("/plugin/src/parser/grammar.lalrpop").trim(at: end))
  lib = crudo.lines(lib, "7-8, 13-22, 24-29, 41-")
  lib
})

#pagebreak(weak: true)

= Module reference

#module(
  read("/src/lib.typ"),
  name: "my-package",
  label-prefix: none,
  scope: scope,
)
