//观察者模式

#[derive(Debug, Clone)]
struct Work {
    id: i32
}

struct WorkMaster {
    workers: Vec<Box<Worker>>
}

impl WorkMaster {
    fn register_worker(&mut self, worker: Box<Worker>) {
        self.workers.push(worker);
    }
    fn dispatch_new_work(&self, work: Work) {
        for worker in &self.workers {
            worker.on_new_work(work.clone())
        }
    }
}

trait Worker {
    fn on_new_work(&self, work: Work);
}
struct LocalWorker {}
impl Worker for LocalWorker {
    fn on_new_work(&self, work: Work) {
        println!("local worker receiver new work {:?}", work)
    }
}
struct RemoteWorker {}
impl Worker for RemoteWorker {
    fn on_new_work(&self, work: Work) {
        println!("remote worker receive new work {:?}", work)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn observer_mode_test() {
        let mut master = WorkMaster{workers: Vec::new()};
        let local_worker = LocalWorker{};
        let remote_worker = RemoteWorker{};
        master.register_worker(Box::new(local_worker));
        master.register_worker(Box::new(remote_worker));
        // local worker receiver new work Work { id: 1 }
        // remote worker receive new work Work { id: 1 }
        master.dispatch_new_work(Work{id: 1});
    }
}