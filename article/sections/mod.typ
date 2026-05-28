#import "indices.typ"

#let body(abbr) = [
  #colbreak() // Nudge index to next column

  #indices.headings

  #colbreak() // Nudge intro header to next column

  #include "intro.typ"

  #include "concepts/mod.typ"

  #include "algos/mod.typ"

  #include "artifacts.typ"

  #include "results.typ"

  #include "discussion.typ"

  #include "future_work.typ"

  #include "conclusion.typ"

  #colbreak() // Nudge end-of-document headers to next column

  #abbr.list(title: "Glossary", columns: 1)

  #indices.figures

  #indices.tables
]
