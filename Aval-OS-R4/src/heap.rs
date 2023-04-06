// heap.rs

use core::ptr::NonNull;

pub struct Heap 
{
    start: 
        NonNull<u8>,

    end: 
        NonNull<u8>,

    next:
        NonNull<u8>,
}

impl Heap 
{
    pub fn new(start: NonNull<u8>, size: usize) 
        -> Heap 
        {
            Heap 
            {
                start: 
                    start,

                end: 
                    unsafe { start.as_ptr().add(size) }
                        .into(),

                next: 
                    start,
            }
        }

    pub fn allocate(&mut self, size: usize, align: usize) 
        -> Option<NonNull<u8>> 
        {
            let layout 
                = core::alloc::Layout::from_size_align(size, align)
                    .ok()?;

            let ptr 
                = self.next
                    .as_ptr()
                    .align_up(layout.align());

            if ptr 
                >= self
                    .end
                        .as_ptr() 
                        {
                            None
                        } 
                        else 
                        {
                            let next_ptr 
                                = unsafe { ptr.add(layout.size()) }
                                    .into();

                            self.next 
                                = next_ptr;
                                
                            Some(unsafe { NonNull::new_unchecked(ptr) })
                        }
        }
}
