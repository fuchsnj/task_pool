#![feature(scoped)]

use std::thread;
use std::thread::JoinGuard;
use std::sync::mpsc::channel;
use std::collections::HashMap;
use std::sync::{Mutex,Arc};
use std::sync::mpsc::Sender;

#[test]
fn it_works() {
	let counter = Mutex::new(0);
	let mut pool = Pool::new();

	for _ in 0..100{
		pool.add_task(||{
			*(counter.lock().unwrap()) += 1;
		});
	}

	pool.join();
	let value = *(counter.lock().unwrap());
	assert!(value == 100);
}
enum PoolEvent{
	ThreadDone(u64),
	Stop
}
pub struct Pool<'a>{
	join_guards: Arc<Mutex<HashMap<u64, JoinGuard<'a,()>>>>,
	manager_guard: JoinGuard<'a,()>,
	next_id: u64,
	end_out: Sender<PoolEvent>
}
impl<'a> Pool<'a>{
	pub fn new() -> Pool<'a>{
		let join_guards = Arc::new(Mutex::new(HashMap::new()));

		let (end_out,end_in) = channel();
		let manager_join_guards = join_guards.clone();
		let manager_guard = thread::scoped(move||{
			let mut running = true;
			while running ||  manager_join_guards.lock().unwrap().len() != 0{
				match end_in.recv().unwrap(){
					PoolEvent::ThreadDone(id) => {
						manager_join_guards.lock().unwrap().remove(&id);
					},
					PoolEvent::Stop => {
						running = false;
					}
				}
			}
		});
		Pool{
			join_guards: join_guards.clone(),
			manager_guard: manager_guard,
			next_id: 0,
			end_out: end_out
		}
	}
	pub fn add_task<T>(&mut self, task:T)
	where T: FnOnce() + Send + Sync + 'a{
		let id = self.next_id;
		self.next_id += 1;
		let mut map = self.join_guards.lock().unwrap();
		let end_out = self.end_out.clone();
		let guard = thread::scoped(move||{
			task();
			end_out.send(
				PoolEvent::ThreadDone(id)
			).unwrap();
		});
		map.insert(id,guard);
	}

	pub fn join(self){
		self.end_out.send(
			PoolEvent::Stop
		).unwrap();
		self.manager_guard.join();
	}

	pub fn size(&self) -> usize{
		self.join_guards.lock().unwrap().len()
	}
}