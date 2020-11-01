use std::{
    future::Future, pin::Pin, sync::{mpsc::{channel, Sender}, Arc, Mutex},
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker}, mem,
    thread::{self, JoinHandle}, time::{Duration, Instant}, collections::HashMap,
};

fn main() {
    // This is just to make it easier for us to see when our Future was resolved
    //Future 执行时间开始
    let start = Instant::now();

    // Many runtimes create a glocal `reactor` we pass it as an argument
    //许多运行时创建一个全局的 “reactor”，我们将其作为参数传递
    let reactor = Reactor::new();
    
    // We create two tasks:
    //我们创建两个 Task：
    // - first parameter is the `reactor`
    //第一个参数是 “reactor”
    // - the second is a timeout in seconds
    //第二个参数是多少秒后超时
    // - the third is an `id` to identify the task
    //第三个参数是 Task 的自增长id
    let future1 = Task::new(reactor.clone(), 2, 1);
    let future2 = Task::new(reactor.clone(), 1, 2);

    // an `async` block works the same way as an `async fn` in that it compiles
    // our code into a state machine, `yielding` at every `await` point.
    //“async”块的工作方式与“async fn”相同，因为它将我们的代码编译成一个状态机，在每个“await”点都“yielding”。
    let fut1 = async {
        let val = future1.await;
        let dur = (Instant::now() - start).as_secs_f32();
        println!("Future got {} at time: {:.2}.", val, dur);
    };

    let fut2 = async {
        let val = future2.await;
        let dur = (Instant::now() - start).as_secs_f32();
        println!("Future got {} at time: {:.2}.", val, dur);
    };

    // Our executor can only run one and one future, this is pretty normal
    // though. You have a set of operations containing many futures that
    // ends up as a single future that drives them all to completion.
    //我们的遗嘱执行人只能经营一个又一个 Future，这很正常不过，你呢拥有一套包含许多未来的操作，最终成为一个单一的 Future，推动它们全部完成。
    let mainfut = async {
        fut1.await;
        fut2.await;
    };

    // This executor will block the main thread until the futures is resolved
    //这个执行器将阻塞主线程，直到 Future 的问题得到解决.
    block_on(mainfut);
    // When we're done, we want to shut down our reactor thread so our program
    // ends nicely.
    //当我们完成后，我们想关闭reactor线程，这样我们的程序就可以很好地结束了。
    reactor.lock().map(|mut r| r.close()).unwrap();
    

}

//// ============================ EXECUTOR ====================================

// Our executor takes any object which implements the `Future` trait
//我们的执行器接受任何实现“未来”特征的对象
fn block_on<F: Future>(mut future: F) -> F::Output {
    // the first thing we do is to construct a `Waker` which we'll pass on to
    // the `reactor` so it can wake us up when an event is ready. 
    //我们可以先做一个唤醒我们的反应器。
    let mywaker = Arc::new(MyWaker{ thread: thread::current() }); 
    let waker = waker_into_waker(Arc::into_raw(mywaker));
    // The context struct is just a wrapper for a `Waker` object. Maybe in the
    // future this will do more, but right now it's just a wrapper.
    //上下文结构只是“Waker”对象的包装器。也许将来这会做得更多，但现在它只是一个包装。
    let mut cx = Context::from_waker(&waker);

    // We poll in a loop, but it's not a busy loop. It will only run when
    // an event occurs, or a thread has a "spurious wakeup" (an unexpected wakeup
    // that can happen for no good reason).
    //我们在一个循环中轮询，但它不是一个繁忙的循环。它只会在事件发生时运行，或者线程有一个“错误的唤醒”（一个意外的唤醒，可能发生在没有正当理由的情况下）。
    let val = loop {
        // So, since we run this on one thread and run one future to completion
        // we can pin the `Future` to the stack. This is unsafe, but saves an
        // allocation. We could `Box::pin` it too if we wanted. This is however
        // safe since we don't move the `Future` here.
        //因此，由于我们在一个线程上运行这个函数，并运行一个future来完成，所以我们可以将“future”固定到堆栈上。
        //这是不安全的，但节省了分配。如果我们想的话，我们也可以用“Box::pin”来代替它。但这是安全的，因为我们不把“Future”移到这里。
        let pinned = unsafe { Pin::new_unchecked(&mut future) };
        match Future::poll(pinned, &mut cx) {
            // when the Future is ready we're finished
            //当 Future 准备好了，我们就完成了
            Poll::Ready(val) => break val,
            // If we get a `pending` future we just go to sleep...
            //如果我们有一个“pending”状态的未来，我们就去睡觉...
            Poll::Pending => thread::park(),
        };
    };
    val
}

// ====================== FUTURE IMPLEMENTATION ==============================

// This is the definition of our `Waker`. We use a regular thread-handle here.
// It works but it's not a good solution. If one of our `Futures` holds a handle
// to our thread and takes it with it to a different thread the followinc could
// happen:
// 1. Our future calls `unpark` from a different thread
// 2. Our `executor` thinks that data is ready and wakes up and polls the future
// 3. The future is not ready yet but one nanosecond later the `Reactor` gets
// an event and calles `wake()` which also unparks our thread.
// 4. This could all happen before we go to sleep again since these processes
// run in parallel.
// 5. Our reactor has called `wake` but our thread is still sleeping since it was
// awake alredy at that point.
// 6. We're deadlocked and our program stops working
// There are many better soloutions, here are some:
// - Use `std::sync::CondVar`
// - Use [crossbeam::sync::Parker](https://docs.rs/crossbeam/0.7.3/crossbeam/sync/struct.Parker.html)

//这就是“Waker”的定义。我们这里使用普通的线程句柄。这是可行的，但不是一个好的解决办法。
//如果我们的某个“Future”持有线程的句柄并将其带到另一个线程，则可能发生以下情况：
// 1. 我们的Future从另一个线程调用“unpark”
// 2. 我们的“executor”认为数据已经准备好了，然后唤醒并轮询 future
// 3. future 还没有准备好，但是一纳秒后，“Reactor”会收到一个事件并调用“wake()”，这也会断开线程的连接。
// 4. 因为这些进程是并行运行的，所以这一切都可能在我们再次入睡之前发生。
// 5. 我们的"reactor"(反应堆)已经叫醒了，但是我们的线程仍然在休眠，因为它在那时是醒着的。
// 6. 我们陷入僵局，程序停止工作
// 有很多更好的解决方案，下面是一些：
//-使用`std::sync::CondVar`
//-使用[crossbeam::sync::Parker](https://docs.rs/crossbeam/0.7.3/crossbeam/sync/struct.Parker.html)
#[derive(Clone)]
struct MyWaker {
    thread: thread::Thread,
}

// This is the definition of our `Future`. It keeps all the information we
// need. This one holds a reference to our `reactor`, that's just to make
// this example as easy as possible. It doesn't need to hold a reference to
// the whole reactor, but it needs to be able to register itself with the
// reactor.

//这就是我们“Future”的定义。它保存了我们需要的所有信息。这个引用了我们的“reactor”，这只是为了使这个例子尽可能简单。它不需要保存对整个反应堆的引用，但它需要能够在反应堆中注册自己。
#[derive(Clone)]
pub struct Task {
    id: usize,
    reactor: Arc<Mutex<Box<Reactor>>>,
    data: u64,
}

// These are function definitions we'll use for our waker. Remember the
// "Trait Objects" chapter from the book.
//这些是我们将用于唤醒器的函数定义。记住书中的“Trait Objects”一章。
fn mywaker_wake(s: &MyWaker) {
    let waker_ptr: *const MyWaker = s;
    let waker_arc = unsafe {Arc::from_raw(waker_ptr)};
    waker_arc.thread.unpark();
}

// Since we use an `Arc` cloning is just increasing the refcount on the smart
// pointer.
//我们使用“Arc”克隆会增加智能指针上的引用计数。
fn mywaker_clone(s: &MyWaker) -> RawWaker {
    let arc = unsafe { Arc::from_raw(s).clone() };
    std::mem::forget(arc.clone()); // increase ref count 增加引用计数
    RawWaker::new(Arc::into_raw(arc) as *const (), &VTABLE)
}

// This is actually a "helper funtcion" to create a `Waker` vtable. In contrast
// to when we created a `Trait Object` from scratch we don't need to concern
// ourselves with the actual layout of the `vtable` and only provide a fixed
// set of functions
//这实际上是创建“Waker”vtable的“helper function”。与我们从头开始创建“Trait Object”相比，我们不需要关心“vtable”的实际布局，只需提供一组固定的函数
const VTABLE: RawWakerVTable = unsafe {
    RawWakerVTable::new(
        |s| mywaker_clone(&*(s as *const MyWaker)),     // clone
        |s| mywaker_wake(&*(s as *const MyWaker)),      // wake
        |s| mywaker_wake(*(s as *const &MyWaker)),      // wake by ref
        |s| drop(Arc::from_raw(s as *const MyWaker)),   // decrease refcount
    )
};

// Instead of implementing this on the `MyWaker` oject in `impl Mywaker...` we
// just use this pattern instead since it saves us some lines of code.
//我们不需要在“impl MyWaker…”中的“MyWaker”项目上实现此功能，而是使用此模式，因为它为我们节省了一些代码行。
fn waker_into_waker(s: *const MyWaker) -> Waker {
    let raw_waker = RawWaker::new(s as *const (), &VTABLE);
    unsafe { Waker::from_raw(raw_waker) }
}

impl Task {
    fn new(reactor: Arc<Mutex<Box<Reactor>>>, data: u64, id: usize) -> Self {
        Task { id, reactor, data }
    }
}

impl Future for Task {
    type Output = usize;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut r = self.reactor.lock().unwrap();

        // First we check if the task is marked as ready
        //首先我们检查任务是否标记为就绪
        if r.is_ready(self.id) {
            *r.tasks.get_mut(&self.id).unwrap() = TaskState::Finished;
            Poll::Ready(self.id)
        
        // If it isn't we check the Reactors map over id's we have registered
        // and see if it's there
        //如果不是的话，我们查一下我们登记的反应堆地图，看看它是否在那里
        } else if r.tasks.contains_key(&self.id) {
            // This is important. The docs says that on multiple calls to poll,
            // only the Waker from the Context passed to the most recent call
            // should be scheduled to receive a wakeup. That's why we insert
            // this waker into the map (which will return the old one which will
            // get dropped) before we return `Pending`.
            //这很重要。文档说，在多个轮询调用中，只有上下文中传递给最近一个调用的唤醒器才应该被安排接收唤醒。
            //这就是为什么我们在返回“Pending”之前将这个唤醒器插入到映射中（它将返回旧的将被丢弃的）。
            r.tasks.insert(self.id, TaskState::NotReady(cx.waker().clone()));
            Poll::Pending
        } else {
            // If it's not ready, and not in the map it's a new task so we
            // register that with the Reactor.
            //如果它还没准备好，而且在map上也没有，这是一个新的任务，所以我们把它注册到反应堆上。
            r.register(self.data, cx.waker().clone(), self.id);
            Poll::Pending
        }
    }
}


// =============================== REACTOR ===================================

// The different states a task can have in this Reactor
//任务在这个反应器中的不同状态
enum TaskState {
    Ready,
    NotReady(Waker),
    Finished,
}

// This is a "fake" reactor. It does no real I/O, but that also makes our
// code possible to run in the book and in the playground
//这是一个“假”反应堆。它没有真正的I/O，但这也使得我们的代码可以在书中和操场上运行
struct Reactor {

    // we need some way of registering a Task with the reactor. Normally this
    // would be an "interest" in an I/O event
    //我们需要一些方法在反应堆上注册一个任务。通常这是对I/O事件的“兴趣”
    dispatcher: Sender<Event>,
    handle: Option<JoinHandle<()>>,

    // This is a list of tasks
    //这是一个任务列表
    tasks: HashMap<usize, TaskState>,
}

// This represents the Events we can send to our reactor thread. In this
// example it's only a Timeout or a Close event.
//这表示我们可以发送到reactor线程的事件。在本例中，它只是一个超时或关闭事件。
#[derive(Debug)]
enum Event {
    Close,
    Timeout(u64, usize),
}

impl Reactor {

    // We choose to return an atomic reference counted, mutex protected, heap
    // allocated `Reactor`. Just to make it easy to explain... No, the reason
    // we do this is:
    //
    // 1. We know that only thread-safe reactors will be created.
    // 2. By heap allocating it we can obtain a reference to a stable address
    // that's not dependent on the stack frame of the function that called `new`
    //我们选择返回一个原子引用计数、互斥保护、堆分配的“Reactor”。只是为了解释一下。。。不，我们这样做的原因是：
    //1我们知道只有线程安全的反应堆会被创造出来。
    //2通过堆分配它，我们可以获得对一个稳定地址的引用，该地址不依赖于名为“new”的函数的堆栈帧`
    fn new() -> Arc<Mutex<Box<Self>>> {
        let (tx, rx) = channel::<Event>();
        let reactor = Arc::new(Mutex::new(Box::new(Reactor {
            dispatcher: tx,
            handle: None,
            tasks: HashMap::new(),
        })));
        
        // Notice that we'll need to use `weak` reference here. If we don't,
        // our `Reactor` will not get `dropped` when our main thread is finished
        // since we're holding internal references to it.
        //注意，这里需要使用'weak'引用。如果我们不这样做，我们的‘Reactor’将不会被'dropped'当我们的主线程完成，因为我们持有内部的引用它。

        // Since we're collecting all `JoinHandles` from the threads we spawn
        // and make sure to join them we know that `Reactor` will be alive
        // longer than any reference held by the threads we spawn here.
        //因为我们从我们生成的线程收集所有的“JoinHandles”，并确保将它们连接起来，我们知道“Reactor”的生存时间将比我们在这里生成的线程所拥有的任何引用都长。
        let reactor_clone = Arc::downgrade(&reactor);

        // This will be our Reactor-thread. The Reactor-thread will in our case
        // just spawn new threads which will serve as timers for us.
        //这将是我们的反应器线程。在我们的例子中，Reactor线程将生成新的线程，这些线程将作为我们的计时器。
        let handle = thread::spawn(move || {
            let mut handles = vec![];

            // This simulates some I/O resource
            //这将模拟一些I/O资源
            for event in rx {
                println!("REACTOR: {:?}", event);
                let reactor = reactor_clone.clone();
                match event {
                    Event::Close => break,
                    Event::Timeout(duration, id) => {

                        // We spawn a new thread that will serve as a timer
                        // and will call `wake` on the correct `Waker` once
                        // it's done.
                        //我们生成一个新线程，它将作为计时器，并在完成后调用正确的“Waker”。
                        let event_handle = thread::spawn(move || {
                            thread::sleep(Duration::from_secs(duration));
                            let reactor = reactor.upgrade().unwrap();
                            reactor.lock().map(|mut r| r.wake(id)).unwrap();
                        });
                        handles.push(event_handle);
                    }
                }
            }

            // This is important for us since we need to know that these
            // threads don't live longer than our Reactor-thread. Our
            // Reactor-thread will be joined when `Reactor` gets dropped.
            //这对我们很重要，因为我们需要知道这些线程的寿命不会比Reactor线程长。我们的Reactor线程将在“Reactor”被丢弃时连接。
            handles.into_iter().for_each(|handle| handle.join().unwrap());
        });
        reactor.lock().map(|mut r| r.handle = Some(handle)).unwrap();
        reactor
    }

    // The wake function will call wake on the waker for the task with the
    // corresponding id.
    //wake函数将为具有相应id的任务调用wake on waker。
    fn wake(&mut self, id: usize) {
        self.tasks.get_mut(&id).map(|state| {

            // No matter what state the task was in we can safely set it
            // to ready at this point. This lets us get ownership over the
            // the data that was there before we replaced it.
            //无论任务处于什么状态，我们都可以安全地将其设置为就绪。这样我们就可以获得替换之前的数据的所有权。
            match mem::replace(state, TaskState::Ready) {
                TaskState::NotReady(waker) => waker.wake(),
                TaskState::Finished => panic!("Called 'wake' twice on task: {}", id),
                _ => unreachable!()
            }
        }).unwrap();
    }

    // Register a new task with the reactor. In this particular example
    // we panic if a task with the same id get's registered twice 
    //向reactor注册一个新任务。在这个特定的例子中，如果一个具有相同id的任务被注册了两次，我们就会惊慌失措
    fn register(&mut self, duration: u64, waker: Waker, id: usize) {
        if self.tasks.insert(id, TaskState::NotReady(waker)).is_some() {
            panic!("Tried to insert a task with id: '{}', twice!", id);
        }
        self.dispatcher.send(Event::Timeout(duration, id)).unwrap();
    }

    // We send a close event to the reactor so it closes down our reactor-thread
    //我们向reactor发送一个close事件，它关闭reactor线程
    fn close(&mut self) {
        self.dispatcher.send(Event::Close).unwrap();
    }

    // We simply checks if a task with this id is in the state `TaskState::Ready`
    //我们只需检查具有此id的任务是否处于“TaskState:：Ready”状态`
    fn is_ready(&self, id: usize) -> bool {
        self.tasks.get(&id).map(|state| match state {
            TaskState::Ready => true,
            _ => false,
        }).unwrap_or(false)
    }
}

impl Drop for Reactor {
    fn drop(&mut self) {
        self.handle.take().map(|h| h.join().unwrap()).unwrap();
    }
}