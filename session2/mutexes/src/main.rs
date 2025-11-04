use std::sync::Mutex;

const N_THREADS: usize = 1_000;
const N_ITERATIONS: usize = 10_000;
// static NUMBERS: Mutex<Vec<u32>> = Mutex::new(Vec::new());
static COUNTER: Mutex<u32> = Mutex::new(0);

fn main() {
    let mut handles = Vec::new();
    // for _ in 0..10 {
    //     let handle = std::thread::spawn(|| {
    //         let mut lock = NUMBERS.lock().unwrap();
    //         lock.push(1);
    //     });
    //     handles.push(handle);
    // }
    // handles.into_iter().for_each(|h| h.join().unwrap());
    // let lock = NUMBERS.lock().unwrap();
    // println!("{:#?}", lock);

    for _ in 0..N_THREADS {
        let handle = std::thread::spawn(|| {
            let mut n = 0;
            for _ in 0..N_ITERATIONS {
                n += 1;
            }
            *COUNTER.lock().unwrap() += n;
        });
        handles.push(handle);
    }
    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("{}", COUNTER.lock().unwrap());
}
