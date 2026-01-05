
extern crate alloc;
use alloc::boxed::Box;
use zephyr::printkln;
use crate::time::{Duration,sleep};

use core::ffi::c_int;
use crate::sync::channel::Sender;
use crate::sync::channel::unbounded;
#[allow(dead_code)]
#[allow(non_camel_case_types)]
/// Constant Bandwidth Server (CBS) struct
pub struct cbs {
    /// CBS parameter priorty
                pub priority: c_int,
    /// CBS parameter name
                pub name: Box<[u8]>,
    /// CBS parameter budget
                pub budget: i64,
    /// CBS parameter period
                pub period: i64,
                /// CBS task sender
                pub sender: Option<Sender<Box<dyn FnOnce() + Send + 'static>>>,
}
/// CBS implementation
#[allow(dead_code)]
impl cbs {

/// Create a new CBS instance    
pub  fn new(priority: c_int, name: &[u8], budget: i64, period: i64) -> Self {
        Self {
            priority,
            name: name.into(),
            budget,
            period,
            sender: None,
        }

    }
/// Start the CBS thread    
pub fn start(&mut self) {
    let (sender, receiver) = unbounded::<Box<dyn FnOnce() + Send + 'static>>();
    self.sender = Some(sender);

   let server_cbs = cbs_thread(receiver,self.budget,self.period);
       server_cbs.set_priority(self.priority);
     //  server_cbs.set_name(&self.name);
         server_cbs.start();


}


}



#[zephyr::thread(stack_size = 2048, pool_size = 1)]
/// CBS thread function
fn cbs_thread(receiver: zephyr::sync::channel::Receiver<Box<dyn FnOnce() + Send + 'static>>, budget: i64, period: i64) {
    let _receiver = receiver;



    
    loop {
        // Thread logic goes here
       printkln!("CBS thread is running");
       //  sleep(Duration::from_ticks(500));
    }
} 
    