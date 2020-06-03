# train

Describes the pure abstraction of a trainable function through automatic gradients

Goal: Make it so that operations can be strung together and concatenated as differentiable operations. Two successive `Trainable` should be able to combine into one which hides the type in the middle, but it should also be possible to have forks and joins in the directed acyclic graph of computation
that still have a common input and output type by hiding the state internally.
