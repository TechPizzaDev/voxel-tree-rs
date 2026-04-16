
#let headings = outline(
  title: block("Index"),
  indent: auto,
  target: selector.or(..(1, 2, 3).map(l => heading.where(level: l))),
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
