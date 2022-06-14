#![allow(dead_code)]
use std::ptr::NonNull;
use std::marker::PhantomData;
use std::alloc::{self, Layout};
use std::ptr;
use array_init::array_init;

pub struct PackedArray<T, const N : usize> {
    size : usize,
    index_to_entry : [usize; N],
    entry_to_index : [usize; N],
    ptr : NonNull<T>,
    _marker : PhantomData<T>
}

impl<T, const N : usize> PackedArray<T, N> {
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

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn get(&self, index : usize) -> & T {
        assert!(self.index_to_entry[index] < self.size);

        unsafe {
            &*self.get_ptr(self.index_to_entry[index])
        }
    }

    pub fn get_mut(&mut self, index : usize) -> &mut T {
        assert!(self.index_to_entry[index] < self.size);

        unsafe {
            &mut *self.get_ptr(self.index_to_entry[index])
        }
    }

    pub fn append(&mut self, value : T) -> usize {
        assert!(self.size < N);

        unsafe {
            ptr::write(self.ptr.as_ptr().add(self.size), value);
        }

        self.size += 1;
        self.entry_to_index[self.size-1]
    }

    pub fn remove(&mut self, index : usize) {
        assert!(self.index_to_entry[index] < self.size);

        self.size -= 1;
        if self.index_to_entry[index] != self.size {
            let entry_to_delete = self.index_to_entry[index];
            let last_index = self.entry_to_index[self.size];

            unsafe {
                ptr::copy_nonoverlapping(self.get_ptr(self.size), self.get_ptr(entry_to_delete), 1);
            }
            
            self.index_to_entry[index] = self.size;
            self.entry_to_index[self.size] = index;

            self.index_to_entry[last_index] = entry_to_delete;
            self.entry_to_index[entry_to_delete] = last_index;
        }
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(self.ptr.as_ptr(), self.size)
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [T] {
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.size)
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
       self.as_slice().iter() 
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.as_slice_mut().iter_mut()
    }

    fn get_ptr(&self, entry : usize) -> *mut T {
        unsafe {
            self.ptr.as_ptr().add(entry)
        }
    }
}
