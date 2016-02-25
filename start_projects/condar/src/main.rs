use std::sync::{Arc, Mutex, Condvar};
use std::thread;



fn main(){
// Inside of our lock, spawn a new thread, and then wait for it to start
	let pair = Arc::new((Mutex::new(false), Condvar::new()));
	let pair2 = pair.clone();
	let pair3 = pair.clone();


	// wait for the thread to start up
	let &(ref lock, ref cvar) = &*pair;
	let mut started = lock.lock().unwrap();
	println!("Get here 1");

	thread::spawn(move|| {
	    let &(ref lock, ref cvar) = &*pair2;
	    let mut started = lock.lock().unwrap();
	    println!("Lock by thread");
	    started = true;
	    cvar.notify_all();
	    println!("Cloned send notification");
	});

	while !*started {
		println!("Get here 2");
	    started = cvar.wait(started).unwrap();
		println!("Origin value initialed");
		// lock.unlock().unwrap();
	}

	println!("Get out");

	// // //test for notify notify one 
	let &(ref lock3, ref cvar3) = &*pair3;
	println!("Before lock3");
	let mut started3 = lock3.lock().unwrap();  // waiting and try to lock
	println!("Before lock3");   //after lock 3
}
