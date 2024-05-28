use std::thread;
use std::time::Duration;
use std::sync::{ mpsc, Arc, Mutex }; //multi processor single consumer

fn main() {
    let count_handle = thread::spawn(|| {
        for i in 1..=15 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1))
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    let v = vec![1, 2, 3];

    // move causes this clsoure(and thus its thread) to take ownership of mentioned vars
    thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    }); // no need to join the main thread because this will complete before count
    // this is creating a handle that isn't stored anywhere

    //# Channels
    
    let (tx, rx) = mpsc::channel(); // channel() -> (transmitter<T>, receiver<U> )
    
    let tx1 = tx.clone();
    thread::spawn(move || { 
        let val = String::from("beep");
        tx1.send(val).unwrap(); // send() -> Result<T, E>
        tx1.send(String::from("boop")).unwrap();
    });
   
    let received = rx.recv().unwrap();
    println!("Get: {}", received);
    /* like a promise recv holds the thread it's in until tx has a result
     * thus is tx has closed recv() will automatically return an error 
     * there is also try_recv which is non blocking
     * notice that unlike async try_recv must be executed again rather than breaking
     * out a new thread implicitly*/

    // multiple outputs passed through a channel
    
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
    
    //# Mutex
    
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
        //unlock?
    }
    
    println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push( 
            thread::spawn(move || {
                let mut num = counter.lock().unwrap();

                *num += 1;
            })
        );
    }

    for handle in handles {
        handle.join().unwrap();
    }
    // Even tho we're waiting for count_handle we must wait for correct Result value

    println!("Result: {}", *counter.lock().unwrap()); 

    // handle joined with the main thread so main doesn't stop before handle finishes
    count_handle.join().unwrap();
}
