use std::io::{Error, Read, Write};//引入IO口中的读、写、错误方法
use std::net::{TcpListener, TcpStream};//引入TCP监听和TCP数据流格式
use std::thread;//引用线程包
use std::time;//引入时间包

fn handle_client(mut stream: TcpStream) -> Result<(), Error>{
    let mut buf = [0;512];//生成一个缓存数组
    println!("服务器在等待客户端发送消息");
    for _ in 0..1000{ //循环执行1000次
       
        let byte_read = stream.read(&mut buf)?; //从数据流中读取数据存到buff
        if byte_read ==  0 { //如果数据流为0返回Ok
            return Ok(());
        }

        stream.write(&buf[..byte_read])?; //将缓存的数组写回数据流并且发送
        thread::sleep(time::Duration::from_secs(1 as u64));//休眠1s
    }

    Ok(())//返回ok
    
}

fn main()  -> std::io::Result<()>{
    let listener = TcpListener::bind("0.0.0.0:4444")?;//监听任意地址的4444端口,bind绑定端口
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();//基于线程管理生成一个线程数组

    for stream in listener.incoming(){//listener的incoming方法可以返回一个产生流序列的迭代器
        let stream = stream.expect("failed!");//获取从客户端收到的数据
        let handle = thread::spawn(move||{
            handle_client(stream)//对数据流进行处理
            .unwrap_or_else(|Error| eprintln!("{:?}", Error));
        });

        thread_vec.push(handle); //将处理结果放在线程数组上
    }
    
    for handle in thread_vec{ //依次执行处理结果，并加入到工作线程
        handle.join().unwrap();
    }

    Ok(())
}
