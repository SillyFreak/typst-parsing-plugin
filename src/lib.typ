#let _p = plugin("parser.wasm")


/// The identity function
///
/// #example(mode: "markup", ```typ
/// #my-package.id(1)
/// #my-package.id[one]
/// ```)
///
/// - x (any): some parameter
/// -> any
#let id(x) = {
  let foo = "foo"
  let foofoo = str(_p.concatenate(bytes(foo), bytes(foo)))
  assert.eq(foofoo, foo * 2)
  x
}
