The plan is to have three-actor architecture: `MasterEmitter`, `Counter` and `EvenOdd`.


`Master` can turn the other two on and off `Start` and `Stop` and emits `Signal`s with `id`s, random numbers (between 1 and 100) for data every 2 sec.

`Counter` counts the number of `Signal`s that reaches it and collects its sum. 

`Combo` keeps track of whether the last signal it has received was bigger or smaller than 75. If it has -- the `Counter` has to add this number twice. 

