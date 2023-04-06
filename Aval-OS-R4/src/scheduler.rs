// scheduler.rs

use crate::process::Process;

pub struct Scheduler 
{
    queue: Vec<Process>,
}

impl Scheduler {
    pub fn new() 
        -> Self 
        {
            Self 
            {
                queue: 
                    Vec::new(),
            }
        }

    pub fn enqueue(&mut self, process: Process) 
    {
        self.queue
            .push(process);
    }

    pub fn dequeue(&mut self) 
        -> Option<Process> 
        {
            self.queue
                .pop()
        }
}
