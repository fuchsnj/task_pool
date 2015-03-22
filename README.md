# task_pool
A growable pool to manage concurrent tasks.

###Example
```rust

	let mut pool = Pool::new();

	let counter = Mutex::new(0);
	for _ in 0..100{
		pool.add_task(||{
			*(counter.lock().unwrap()) += 1;
		});
	}

	pool.join();
	let value = *(counter.lock().unwrap());
	assert!(value == 100);
```
