#![allow(dead_code)]
use std::ptr::NonNull;
use std::marker::PhantomData;
use std::alloc::{self, Layout};
use std::ptr;
use array_init::array_init;

/// Densly packed data structure for fast iteration
pub struct PackedArray<T, const N : usize> {
    size : usize,
    index_to_entry : [usize; N],
    entry_to_index : [usize; N],
    ptr : NonNull<T>,
    _marker : PhantomData<T>
}

impl<T, const N : usize> PackedArray<T, N> {

    /// Return new PackedArray with allocated storage
    pub fn new() -> Self {
        let layout = Layout::array::<T>(N).unwrap();
        let ptr = match NonNull::new(unsafe { alloc::alloc(layout) } as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error( layout ),
        };

        PackedArray { 
            size : 0,
            index_to_entry : array_init(|i| i),
            entry_to_index : array_init(|i| i),
            ptr, 
            _marker : PhantomData
        }
    }

    /// Returns current number of elements stored
    pub fn len(&self) -> usize {
        self.size
    }

    /// Returns true if there is a valid entry under index
    pub fn has(&self, index : usize) -> bool {
        self.index_to_entry[index] < self.size
    }

    /// Returns true if allocated memory is exhausted
    /// True also means that no more entries can be added
    pub fn full(&self) -> bool {
        self.size == N
    }

    /// Returns reference to an element at index
    /// Panics if there is no valid element with such index 
    pub fn get(&self, index : usize) -> & T {
        assert!(self.has(index));

        unsafe {
            &*self.get_ptr(self.index_to_entry[index])
        }
    }

    /// Return mutable reference to an element at index
    /// Panics if there is no valid element with such index 
    pub fn get_mut(&mut self, index : usize) -> &mut T {
        assert!(self.has(index));

        unsafe {
            &mut *self.get_ptr(self.index_to_entry[index])
        }
    }

    /// Adds element to the end of array.
    /// Return index of newly-added element for future access
    /// Panics if structure is full
    pub fn append(&mut self, value : T) -> usize {
        assert!(!self.full());

        unsafe {
            ptr::write(self.ptr.as_ptr().add(self.size), value);
        }

        self.size += 1;
        self.entry_to_index[self.size-1]
    }

    /// Sets value of passed index
    /// If there is no valid element with such index, element is added
    pub fn set(&mut self, index : usize, value : T) {
        if !self.has(index) {
            self.swap_with_back(index);
            self.size += 1;
        }
        self[index] = value;
    }

    /// Removes element by given index
    /// Panics if there is no valid element with such index 
    pub fn remove(&mut self, index : usize) {
        assert!(self.has(index));

        self.size -= 1;
        if self.index_to_entry[index] != self.size {
            let entry_to_delete = self.index_to_entry[index];

            unsafe {
                ptr::copy_nonoverlapping(self.get_ptr(self.size), self.get_ptr(entry_to_delete), 1);
            }
            self.swap_with_back(index);
        }
    }

    /// Returns an underlying storage as slice
    pub fn as_slice(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(self.ptr.as_ptr(), self.size)
        }
    }

    /// Returns an underlying storage as mutable slice
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.size)
        }
    }

    /// Returns an iterator to underlying storage
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
       self.as_slice().iter() 
    }

    /// Return a mutable iterator to underlying storage
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.as_slice_mut().iter_mut()
    }

    fn swap_with_back(&mut self, index : usize) {
        let entry = self.index_to_entry[index];
        let last_index = self.entry_to_index[self.size];

        self.index_to_entry.swap(index, last_index);
        self.entry_to_index.swap(entry, self.size);
    }

    fn get_ptr(&self, entry : usize) -> *mut T {
        unsafe {
            self.ptr.as_ptr().add(entry)
        }
    }
}

impl<T, const N : usize> std::ops::Index<usize> for PackedArray<T, N> {
    type Output = T;

    fn index(&self, index : usize) -> &Self::Output {
        self.get(index)
    }
}

impl<T, const N : usize> std::ops::IndexMut<usize> for PackedArray<T, N> {
    fn index_mut(&mut self, index : usize) -> &mut Self::Output {
        self.get_mut(index)
    }
}
