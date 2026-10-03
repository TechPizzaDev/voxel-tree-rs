== @NCA:lo <sec:artifact_nca>

This was pursued since a few of the pre-trained examples were of trees
@growing_3d_artefacts, giving a useful baseline that worked. 
We reproduced the open-source build on GitHub
#footnote[https://github.com/real-itu/3d-artefacts-nca],
which included updating some yanked Python packages.
This allowed us to explore the provided _Jupyter notebooks_,
and train @NCA from scratch using the @MC server controlled by Python.

The original loss function is a combination of
_Softmax_ and _negative log likelihood_ loss
#footnote[https://docs.pytorch.org/docs/stable/nn].
We attempted to modify the loss function 
to achieve different growth patterns.
That may be effective for tree variation but was difficult to control.
The growth is too sporadic and unnatural,
and generally struggles to settle on novel shapes outside of training data.
We found a paper that takes a different approach to automata
using convolutional @NN:pla @learning_generate_3d_shapes, 
and the method presented there seemed more plausible than
messing with @NCA internals.