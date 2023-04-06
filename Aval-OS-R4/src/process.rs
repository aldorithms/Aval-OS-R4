// process.rs

use core::ptr::NonNull;
use crate::heap::Heap;

#[derive(Debug)]
pub struct Process 
{
    pub id:
        u32,

    pub name: 
        String,

    pub memory_map: 
        Option<MemoryMap>,
}

impl Process 
{
    pub fn new(id: u32, name: String, heap: &mut Heap) 
        -> Option<Process> 
        {
            let process_ptr 
                = heap
                    .allocate(core::mem::size_of::<Process>(), core::mem::align_of::<Process>())?;

            let process_ref 
                = unsafe { process_ptr.as_ptr().cast::<Process>() };

            Some
            (
                Process 
                {
                    id: 
                        id,

                    name:
                        name,
                        
                    memory_map: 
                        None,
                }
            )
    }
}
