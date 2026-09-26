#import "../../markers.typ": todo, refine

== @NCA:lo <sec:artifact_nca>

#refine[
  Describe the experiment around @NCA @growing_3d_artefacts: 
  - Reproducing the open-source build found on GitHub by updating Python packages. 
  - Exploring the provided Jupyter notebook.

  - Modifying the loss function to achieve different growth patterns. The original loss function is a combination of _Softmax_ and _negative log likelihood_ loss #footnote[https://docs.pytorch.org/docs/stable/nn].
]

#refine[
  - This algo is a black-box, and may be related to evolutionary algos.
  - This algo struggles to settle on novel shapes, i.e. anything outside training data.
]

// TODO: most of these can go
#todo[
  - Pruning methods; why is it important, how to measure?

  - How easy is it to control appearance?

  - Run a test? Benchmark?

  - Insights around @NCA loss function (helps reach a goal), not particulary a ready product.
]