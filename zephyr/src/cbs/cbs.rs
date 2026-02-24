
extern crate alloc;
use alloc::boxed::Box;
use zephyr::printkln;
use crate::time::{Duration,sleep};
use core::error::Error;
use core::fmt;
use core::ffi::c_int;
use crate::sync::channel::Sender;
use crate::sync::channel::unbounded;
use zephyr::sync::{SpinMutex};
use core::mem::MaybeUninit;
use core::ptr;
use core::ffi::c_char;
use crate::thread::RunningThread;
 use crate::raw::{k_cbs, k_timeout_t, k_thread_cbs_init, k_thread_cbs_start, k_thread_cbs_stop};
#[allow(dead_code)]
#[allow(non_camel_case_types)]
/// Constant Bandwidth Server (CBS) struct

const CBS_SERVER_NUMBER: usize = zephyr::kconfig::CONFIG_CBS_COUNT  as usize;
const CBS_SERVER_SIZE: usize = zephyr::kconfig::CONFIG_CBS_THREAD_STACK_SIZE  as usize;
static CBS_COUNT: SpinMutex<usize> = SpinMutex::new(0);
/// CBS error enum
#[derive(Debug)]
pub enum CBSError {
    /// Error when creating CBS thread
    ThreadCreationFailed,
    /// Error when joining CBS thread
    ThreadJoinFailed,
}

impl fmt::Display for CBSError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ThreadCreationFailed => write!(f, "Failed to create CBS thread"),
            Self::ThreadJoinFailed => write!(f, "Failed to join CBS thread"),
        }
    }
}
/// Implement the Error trait for CBSError
impl Error for CBSError {}

/// cbs struct 
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
pub fn start(&mut self)-> Result<RunningThread, CBSError> {

     if *CBS_COUNT.lock().unwrap() < CBS_SERVER_NUMBER {
        *CBS_COUNT.lock().unwrap() += 1;
    

    let (sender, receiver) = unbounded::<Box<dyn FnOnce() + Send + 'static>>();
    self.sender = Some(sender);
   let mut server_cbs = Some(cbs_thread(receiver,self.budget,self.period,self.name.clone()));

let server = match server_cbs{
    Some(handle) => handle,
    None => {
        printkln!("Failed to create CBS thread");
        return Err(CBSError::ThreadCreationFailed);
    }
};




// unwrap or return Error
server.set_priority(self.priority);

  let cbs_server=Some(server.start());

let cbs_start=  match cbs_server {
    Some(handle) => handle,
    None => {
        printkln!("Failed to start CBS thread");
        return Err(CBSError::ThreadCreationFailed);
    }

};

Ok(cbs_start)
     }
     else {
        printkln!("CBS thread limit reached");
        Err(CBSError::ThreadCreationFailed)
}
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



#[zephyr::thread(stack_size = CBS_SERVER_SIZE, pool_size = CBS_SERVER_NUMBER)]
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
    



