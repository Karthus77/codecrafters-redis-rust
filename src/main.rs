use std::ptr::null;

use tokio::net::{TcpListener, TcpStream};
use resp::Value;
use anyhow::Result;

mod resp;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    loop {
        let stream = listener.accept().await;
        match stream {
            Ok((stream, _ )) => {
                println!("accept new connection");
                tokio::spawn(async move {
                    handle_conn(stream).await
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

async fn handle_conn(stream: TcpStream ) {
    let mut handler = resp::RespHandler::new(stream);
    let mut storage: std::collections::HashMap<String,String> = std::collections::HashMap::new();
    println!("Starting read loop");
    loop {
        let value = handler.read_value().await.unwrap();
        println!("Got value {:?}", value);
        let response = if let Some(v) = value{
            let (commmand,args ) = extract_command(v).unwrap();
        match commmand.as_str() {
            "ping" => Value::SimpleSrting("PONG".to_string()),
            "echo" => args.first().unwrap().clone(),
            "set"  => set(&mut storage, unpack_bulk_str(args[0].clone()).unwrap(), unpack_bulk_str(args[1].clone()).unwrap()),
            "get"  => get(&mut storage, unpack_bulk_str(args[0].clone()).unwrap()),
            c=> panic!("Cannnot handle command {}", c),
            }
        } else {
            break;
        };
        
        println!("Sending value {:?}", response);
        handler.write_value(response).await.unwrap();
    }
}

fn set(storage: &mut std::collections::HashMap<String,String>,key: String, value: String) -> Value{
    storage.insert(key, value);
    Value::SimpleSrting("OK".to_string())
}

fn get(storage: &mut std::collections::HashMap<String,String>,key: String) -> Value{
    match storage.get(&key) {
        Some(v) => Value::BulkString((v.to_string())),
        None => Value::Null,
    }
}

//返回命令及后续内容
fn extract_command(value: Value) -> Result<(String, Vec<Value>)> {
    match value {
        Value::Array(a) => {
            Ok((
                unpack_bulk_str(a.first().unwrap().clone())?,//解包element
                a.into_iter().skip(1).collect(),
            ))
        },
        _ => Err(anyhow::anyhow!("Unexpected command format")),
    }
}

// 解包bulk_str 
fn unpack_bulk_str(value:Value) -> Result<String> {
    match value {
        Value::BulkString(s) => Ok(s),
        _  => Err(anyhow::anyhow!("Unexpected command format"))
    }
}
