#import "indices.typ"

#let body(abbr) = [
  #include "intro.typ"

  #include "concepts/mod.typ"

  #include "algos/mod.typ"

  #include "artifacts/mod.typ"

  #include "results.typ"

  #include "discussion/mod.typ"

  #include "future_work/mod.typ"

  #include "conclusion.typ"

  #abbr.list(title: "Glossary", columns: 1)

  #colbreak() // Nudge end-of-document headers to next column

  #indices.figures

  #indices.tables
]
