/*
    一个简单的异步工作程序，并且只有一个协程将异步任务按顺序执行。
*/
use async_channel::{bounded, Receiver, Sender};

pub async fn run(){
    example().await;
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
    let (work_sender, work_receiver) = bounded(100);
    let (result_sender, result_receiver) = bounded(100);

    // 生成子线程用于执行另一个并行组件
    smol::spawn(dispatch(work_receiver, result_sender)).detach();
    println!("111111111");
    work_sender.send(WorkMsg::Work(1)).await;
    work_sender.send(WorkMsg::Work(2)).await;
    work_sender.send(WorkMsg::Work(3)).await;
    work_sender.send(WorkMsg::Work(4)).await;
    work_sender.send(WorkMsg::Work(5)).await;
    work_sender.send(WorkMsg::Exit).await;
    println!("222222222222");

    // worker执行计数
    let mut counter = 0;

    loop{
        match result_receiver.recv().await {
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
    print!("finish!");
}

async fn dispatch(receiver: Receiver<WorkMsg>, sender: Sender<ResultMsg>){
    loop {
        // 接收并处理消息，直到收到 exit 消息
        match receiver.recv().await {
            Ok(WorkMsg::Work(num)) => {
                // 执行一些工作，并且发送消息给 Result 队列
                println!("收到任务消息 start");
                sender.send(ResultMsg::Result(num)).await;
                println!("收到任务消息 end");
            },
            Ok(WorkMsg::Exit) => {
                // 发送 exit 确认消息
                println!("收到Exit消息 start");
                sender.send(ResultMsg::Exited).await;
                println!("收到Exit消息 end");
                break;
            },
            _ => {panic!("Error receiving a WorkMsg.")},
        }
    }
}