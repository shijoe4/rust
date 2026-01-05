
extern crate alloc;
use alloc::boxed::Box;
use zephyr::printkln;
use crate::time::{Duration,sleep};
 use crate::Error;
use core::ffi::c_int;
use crate::sync::channel::Sender;
use crate::sync::channel::unbounded;
use core::mem::MaybeUninit;
use core::ptr;
use core::ffi::c_char;
use crate::thread::RunningThread;
 use crate::raw::{k_cbs, k_timeout_t, k_thread_cbs_init, k_thread_cbs_start, k_thread_cbs_stop};
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
pub fn start(&mut self)-> Result<RunningThread, Error>   {
    let (sender, receiver) = unbounded::<Box<dyn FnOnce() + Send + 'static>>();
    self.sender = Some(sender);
   let server_cbs = Some(cbs_thread(receiver,self.budget,self.period,self.name.clone()));
     match server_cbs {
        None => {
            printkln!("Failed to create CBS thread");
            return Err(Error::new("ThreadCreationFailed"));
        }
        Some(ref mut s) => s,
    };

    let server = server_cbs.take().ok_or(Error(1))?; // unwrap or return Error
server.set_priority(self.priority);
      // server_cbs.set_priority(self.priority);
     //  server_cbs.set_name(&self.name);
     let cbs_server = Some(server_cbs.start());
        match cbs_server {
            None => {
                printkln!("Failed to start CBS thread");
                 return Err(Error::new("ThreadCreationFailed"));
            }
            Some(t) => t,


};
}

/// Push a job to the CBS thread
 pub fn push<F>(&mut self, f: F)
                where
                    F: FnOnce() + Send + 'static,
                {
                    if let Some(tx) = &self.sender {
                        let _ = tx.send(Box::new(f));
                    }
                }


}



#[zephyr::thread(stack_size = 2048, pool_size = 2)]
/// CBS thread function
fn cbs_thread(receiver: zephyr::sync::channel::Receiver<Box<dyn FnOnce() + Send + 'static>>, budget: i64, period: i64, name: Box<[u8]>) {
    // let _receiver = receiver;
unsafe {
        
let mut cbs_str: MaybeUninit<k_cbs> = MaybeUninit::uninit();
let ptr: *mut k_cbs = cbs_str.as_mut_ptr();
 u8_to_c_char_ptr(&name,(*ptr).name.as_mut_ptr() as *mut c_char);
 let t1 = k_timeout_t { ticks: budget};
let t2 = k_timeout_t { ticks: period};
k_thread_cbs_init(ptr, t1, t2);
    loop {
        // Thread logic goes here
       printkln!("CBS thread is running");
       //  sleep(Duration::from_ticks(500));

                               match receiver.recv() {
                                Ok(job) => {
                                    printkln!("CBS executing job");
                                     k_thread_cbs_start(ptr);
                                    job();
                                    k_thread_cbs_stop(ptr);
                                }
                                Err(_) => {
                                    printkln!("Channel closed for ");
                                     k_thread_cbs_stop(ptr); 
                                    break;
                                }
                            }

 sleep(Duration::from_ticks(500));


    }
}


} 


fn u8_to_c_char_ptr(slice: &[u8], dest: *mut c_char) {
    unsafe {
        for (i, &b) in slice.iter().enumerate() {
            *dest.add(i) = b as c_char;
        }
        *dest.add(slice.len()) = 0; // Null terminator
    }
}
    