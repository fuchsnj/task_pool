# task_pool
A growable pool to manage concurrent tasks.

## Installation

If you're using `Cargo`, just add task_pool to your `Cargo.toml`:

```toml
[dependencies.task_pool]
git = "https://github.com/fuchsnj/task_pool.git"
```

##Example
```rust
	extern crate task_pool;
	
	use task_pool::Pool;
	
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
