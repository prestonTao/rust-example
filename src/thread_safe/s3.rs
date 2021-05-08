/*
    在s2基础上，要保证所有异步程序执行完才能退出程序。
*/

// // use async_channel::{bounded, Receiver, Sender};
// use crossbeam_channel::{Receiver, Sender, bounded, select, unbounded};
// use async_io::Timer;
// use std::time::Duration;

// pub async fn run(){
//     example().await;
//     Timer::after(Duration::from_secs(1)).await;
// }

// // 此消息用于发送到与「主组件」并行运行的其他组件。
// enum WorkMsg{
//     Work(u8),
//     Exit,
// }

// // 此消息用于从并行运行的其他组件 发送回「主组件」。
// enum ResultMsg{
//      Result(u8),
//      Exited,
// }


// async fn example(){
//     let (work_sender, work_receiver) = bounded(100);
//     let (result_sender, result_receiver) = bounded(100);

//     // 生成子线程用于执行另一个并行组件
//     smol::spawn(dispatch(work_receiver, result_sender)).detach();
//     println!("111111111");
//     work_sender.send(WorkMsg::Work(1));
//     work_sender.send(WorkMsg::Work(2));
//     work_sender.send(WorkMsg::Work(3));
//     work_sender.send(WorkMsg::Work(4));
//     work_sender.send(WorkMsg::Work(5));
//     work_sender.send(WorkMsg::Exit);
//     println!("222222222222");

//     // worker执行计数
//     let mut counter = 0;

//     loop{
//         match result_receiver.recv() {
//             Ok(ResultMsg::Result(num)) => {
//                 println!("接收到返回消息 start");
//                 counter += 1;
//                 println!("接收到返回消息 end");
//             },
//             Ok(ResultMsg::Exited) => {
//                 println!("接收到Exit消息");
//                 break;
//             },
//             _ => panic!("Error receiving a ResultMsg."),
//         }
//     }
//     println!("{}", counter);
//     println!("finish!");
// }

// async fn dispatch(receiver: Receiver<WorkMsg>, sender: Sender<ResultMsg>){
//     loop {
//         // 接收并处理消息，直到收到 exit 消息
//         match receiver.recv() {
//             Ok(WorkMsg::Work(num)) => {
//                 // 执行一些工作，并且发送消息给 Result 队列
//                 println!("收到任务消息 start");
//                 smol::spawn(task(sender.clone(), num)).detach();
//                 println!("收到任务消息 end");
//             },
//             Ok(WorkMsg::Exit) => {
//                 // 发送 exit 确认消息
//                 println!("收到Exit消息 start");
//                 sender.send(ResultMsg::Exited);
//                 println!("收到Exit消息 end");
//                 break;
//             },
//             _ => {panic!("Error receiving a WorkMsg.")},
//         }
//     }
// }


// async fn task(sender: Sender<ResultMsg>, param: u8){
//     println!("开始工作 {}",param);
//     sender.send(ResultMsg::Result(param));
// }


use async_channel::{unbounded, Receiver, Sender};
// use crossbeam_channel::{Receiver, Sender, bounded, select, unbounded};
use async_io::Timer;
use std::time::Duration;
use std::thread;
use futures::future;
use smol;

use crate::libother::tokio::example;

pub async fn run(){

    // let num_threads = num_cpus::get().max(1);
    // for _ in 0..num_threads {
    //     thread::spawn(|| smol::run(future::pending::<()>()));
    // }

    // smol::block_on(example());
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
  
    //异步分发任务的方法
    smol::spawn(DistributeTasks(work_sender)).detach();



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
    println!("finish!");
}

async fn DistributeTasks(work_sender: Sender<WorkMsg>){
    work_sender.send(WorkMsg::Work(1)).await;
    Timer::after(Duration::from_secs(1)).await;
    work_sender.send(WorkMsg::Work(2)).await;
    Timer::after(Duration::from_secs(1)).await;
    work_sender.send(WorkMsg::Work(3)).await;
    Timer::after(Duration::from_secs(1)).await;
    work_sender.send(WorkMsg::Work(4)).await;
    Timer::after(Duration::from_secs(1)).await;
    work_sender.send(WorkMsg::Work(5)).await;
    Timer::after(Duration::from_secs(1)).await;
    work_sender.send(WorkMsg::Exit).await;
}

async fn dispatch(work_receiver: Receiver<WorkMsg>, result_sender: Sender<ResultMsg>){
    
    let (pool_task_sender, pool_task_receiver) = unbounded();
    let (pool_finish_sender, pool_finish_receiver) = unbounded();
    let (pool_stop_sender, pool_stop_receiver) = unbounded();
    let (pool_result_sender, pool_result_receiver) = unbounded();
    smol::spawn(taskPool(pool_result_sender.clone(), pool_task_receiver.clone(), pool_finish_receiver.clone(), pool_stop_receiver.clone())).detach();
    loop {
        // 接收并处理消息，直到收到 exit 消息
        match work_receiver.recv().await {
            Ok(WorkMsg::Work(num)) => {
                // 执行一些工作，并且发送消息给 Result 队列
                println!("收到任务消息 start");
                pool_task_sender.send(()).await;
                smol::spawn(task(result_sender.clone(), num, pool_finish_sender.clone())).detach();
                println!("收到任务消息 end");
            },
            Ok(WorkMsg::Exit) => {
                // 发送 exit 确认消息
                println!("收到Exit消息 start");
                pool_stop_sender.send(()).await;
                println!("收到Exit消息 end");
                break;
            },
            _ => {panic!("Error receiving a WorkMsg.")},
        }
    }
    //等待任务完成
    println!("等待任务完成");
    let _ = pool_result_receiver.recv().await;


    result_sender.send(ResultMsg::Exited).await;
}


async fn task(result_sender: Sender<ResultMsg>, param: u8, pool_finish_sender: Sender<()>){
    println!("开始工作 {}",param);
    result_sender.send(ResultMsg::Result(param)).await;
    pool_finish_sender.send(()).await;
}


async fn taskPool(pool_result_sender: Sender<()>, pool_task_receiver: Receiver<()>, pool_finish_receiver: Receiver<()>, pool_stop_receiver: Receiver<()>){
    println!("execute taskPool function");
    // let mut count = 0;
    // let mut stop = false;
    loop{
        // pool_task_receiver.recv_timeout(Duration::from_secs(1));
        match pool_task_receiver.try_recv() {
            Ok(_) => {
                pool_finish_receiver.recv().await;
            }
            Err(_) => {
                match pool_stop_receiver.try_recv() {
                    Ok(_) => {
                        break;
                    }
                    Err(_) => {
                        Timer::after(Duration::from_micros(10)).await;
                        continue;
                    }
                }
            }
        }

        // select!{
        //     recv(pool_task_receiver) -> msg => {
        //         println!("未完成任务 +1");
        //         count = count + 1;
        //     },
        //     // recv(pool_finish_receiver) -> msg => {
        //     //     println!("未完成任务 -1");
        //     //     count = count - 1;
        //     //     if stop && count == 0{
        //     //         break;
        //     //     }
        //     // },
        //     // recv(pool_stop_receiver) -> msg => {
        //     //     stop = true;
        //     //     if count == 0 {
        //     //         break;
        //     //     }
        //     // }
        //     default(Duration::from_secs(1)) => println!("timed out"),
        // }
    }



    pool_result_sender.send(()).await;
}

