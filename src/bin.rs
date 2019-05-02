extern crate libnixstore_sys;


fn main () {

    let mut threads = vec![];
    for i in 0..10 {
        let n = format!("asfdsafasfsafasfdafsfsasafsafsaafsfoo{}", i);
        let thread = std::thread::spawn(move || {
            let mut instance = libnixstore_sys::Instance::new().unwrap();
            println!("{}: {:?}", i, instance.query_path_info(n))
        });
        threads.push(thread);
    }

    std::thread::sleep(std::time::Duration::new(5, 0));
}
