
/*
    在s2基础上，要保证所有异步程序执行完才能退出程序。
*/

// use async_channel::{bounded, Receiver, Sender};
use async_io::Timer;
use std::time::Duration;
use crossbeam_channel::{select, unbounded, Sender, Receiver};

pub async fn run(){
    example().await;
    // Timer::after(Duration::from_secs(1)).await;
}

// 此消息用于发送到与「主组件」并行运行的其他组件。
enum WorkMsg{
    Work(u8),
    Exit,
}

// 此消息用于从并行运行的其他组件 发送回「主组件」。
enum ResultMsg{
     Result(u8),
     Exited,
}


async fn example(){
    let (work_sender, work_receiver) = unbounded();
    let (result_sender, result_receiver) = unbounded();


    // 生成子线程用于执行另一个并行组件
    smol::spawn(dispatch(work_receiver, result_sender)).detach();
    println!("111111111");
    work_sender.send(WorkMsg::Work(1));
    work_sender.send(WorkMsg::Work(2));
    work_sender.send(WorkMsg::Work(3));
    work_sender.send(WorkMsg::Work(4));
    work_sender.send(WorkMsg::Work(5));
    work_sender.send(WorkMsg::Exit);
    println!("222222222222");

    //
    // let (finish_sender, finish_receiver) = unbounded();

    // worker执行计数
    let mut counter = 0;

    loop{
        match result_receiver.recv() {
            Ok(ResultMsg::Result(num)) => {
                println!("接收到返回消息 start");
                counter += 1;
                println!("接收到返回消息 end");
            },
            Ok(ResultMsg::Exited) => {
                println!("接收到Exit消息");
                break;
            },
            _ => panic!("Error receiving a ResultMsg."),
        }
    }



    println!("{}", counter);
    println!("finish!");
}

async fn finish(receiver: Receiver<WorkMsg>, sender: Sender<ResultMsg>){

}
async fn dispatch(receiver: Receiver<WorkMsg>, sender: Sender<ResultMsg>){
    // 添加一个新的Channel，Worker使用它来通知“并行”组件已经完成了一个工作单元
    let (pool_result_sender, pool_result_receiver) = unbounded();
    let mut ongoing_work = 0;
    let mut exiting = false;
    loop {
        select!{
            recv(receiver) -> msg => {
                println!("接收到任务消息");
                // 接收并处理消息，直到收到 exit 消息
                match msg {
                    Ok(WorkMsg::Work(num)) => {
                        println!("是工作任务");
                        // 注意，这里正在池上启动一个新的工作单元。
                        ongoing_work += 1;
                        // 执行一些工作，并且发送消息给 Result 队列
                        println!("收到任务消息 start");
                        smol::spawn(task(receiver.clone(), sender.clone(), num, pool_result_sender.clone())).detach();
                        println!("收到任务消息 end");
                    },
                    Ok(WorkMsg::Exit) => {
                        println!("是Exit任务");
                        // N注意，这里接收请求并退出
                        exiting = true;
                        // 如果没有正在进行的工作则立即退出
                        if ongoing_work == 0 {
                            let _ = sender.send(ResultMsg::Exited);
                            break;
                        }
                    },
                    _ => {panic!("Error receiving a WorkMsg.")},
                }
            },
            recv(pool_result_receiver) -> msg => {
                if ongoing_work == 0 {
                    panic!("Received an unexpected pool result.");
                }

                // 注意，一个工作单元已经被完成
                ongoing_work -=1;

                // 如果没有正在进行的工作，并且接收到了退出请求，那么就退出
                if ongoing_work == 0 && exiting {
                    let _ = sender.send(ResultMsg::Exited);
                    break;
                }
            },
            // default(Duration::from_secs(1)) => println!("timed out"),
        }
        
    }
}


async fn task(receiver: Receiver<WorkMsg>, sender: Sender<ResultMsg>, param: u8, pool_sender: Sender<()>){
    println!("开始工作 {}",param);
    // 1. 发送结果给「主组件」
    sender.send(ResultMsg::Result(param));
    // 2. 让并行组件知道这里完成了一个工作单元
    let _ = pool_sender.send(());
}