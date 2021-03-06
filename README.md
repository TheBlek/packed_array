# Packed Array aka Sparse Set

Data structure that allows cache-friendly iteration by keeping all data neatly packed.
Similar one is used in full-fledged ECS game engine [EnTT](https://github.com/skypjack/entt)

Order of elements is not garanteed to stay the same so you should save indices of appended elements for future access

# Usage
```rust
    #[test]
    fn creation() {
        let a = PackedArray::<u32, 10>::new();
        assert_eq!(a.len(), 0);
    }

    #[test]
    fn append_remove() {
        let mut a = PackedArray::<i16, 15>::new();
        let i1 = a.append(90);
        let i2 = a.append(18);
        let i3 = a.append(-80);
        assert_eq!(a.len(), 3);
        assert_eq!(a[i1], 90);
        assert_eq!(a[i2], 18);
        assert_eq!(a[i3], -80);

        a.remove(i3);
        assert_eq!(a.len(), 2);
        assert_eq!(a[i1], 90);
        assert_eq!(a[i2], 18);

        a.remove(i1);
        assert_eq!(a.len(), 1);
        assert_eq!(a[i2], 18);
    }

    #[test]
    fn references() {
        let mut a = PackedArray::<i16, 15>::new();
        let i1 = a.append(90);
        let i2 = a.append(18);

        let number = &mut a[i2];
        *number = 80;
        assert_eq!(a[i2], 80);
        assert_eq!(a[i1], 90);
    }

    #[test]
    fn iteration() {
        let mut a = PackedArray::<i16, 15>::new();
        a.append(90);
        a.append(18);

        let mut i = 0;
        for item in a.iter() {
            assert_eq!(*item, a[i]);
            i += 1;
        }
    }

    #[test]
    fn iteration_mut() {
        let mut a = PackedArray::<i16, 15>::new();
        let i1 = a.append(90);
        let i2 = a.append(18);

        for item in a.iter_mut() {
            *item += 1;
        }
        assert_eq!(a[i1], 91);
        assert_eq!(a[i2], 19);
    }
```
