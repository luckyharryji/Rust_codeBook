use std::sync::{Arc, Mutex, Condvar};
use std::thread;



fn main(){
// Inside of our lock, spawn a new thread, and then wait for it to start
	let pair = Arc::new((Mutex::new(false), Condvar::new()));
	let pair2 = pair.clone();


	// wait for the thread to start up
	let &(ref lock, ref cvar) = &*pair;
	let mut started = lock.lock().unwrap();
	println!("Get here 1");

	thread::spawn(move|| {
	    let &(ref lock, ref cvar) = &*pair2;
	    let mut started = lock.lock().unwrap();
	    *started = true;
	    cvar.notify_one();
	    println!("Cloned send notification");
	});
	while !*started {
		println!("Get here 2");
	    started = cvar.wait(started).unwrap();
		println!("Origin value initialed");
	}

}
