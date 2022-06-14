# Packed Array aka Sparse Set

Data structure that allows cache-friendly iteration by keeping all data neatly packed.
Similar one is used in full-fledged ECS game engine [EnTT](https://github.com/skypjack/entt)

Order of elements is not garanteed to stay the same so you should save indices of appended elements for future access
