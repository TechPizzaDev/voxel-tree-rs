== @NCA:lo
The generation of @fig_nca_orig converges after $80$ iterations,
or roughly $34.7$ seconds on CPU.
On the other hand, introducing randomness to cell propagation creates
spurious growths seen in @fig_nca_growths. 
Growth quickly collapses when scaling weights with values less than one,
making the tree disappear.

#line(stroke: 0pt)

#grid(
  columns: 2,
  gutter: 2pt,
  [#figure(
    image("../../img/NCA,original.png"),
    caption: [Original @NCA tree],
  ) <fig_nca_orig>],
  [#figure(
    image("../../img/NCA,modified,lifemask_0.2,fire_0.75.png"),
    caption: [@NCA tree with factors: $"mask"=0.2, "fire"=0.75$],
  ) <fig_nca_growths>],
)