
#let headings = outline(
  title: block("Index"),
  depth: 3,
  indent: auto,
  target: heading,
);

#let figures = outline(
  title: block("Index of Figures"),
  indent: auto,
  target: figure.where(kind: image),
)

#let tables = outline(
  title: [#block("Index of Tables")],
  indent: auto,
  target: figure.where(kind: table),
)
