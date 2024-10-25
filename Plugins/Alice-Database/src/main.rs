
/*                          MIT License

Copyright (c) 2024 Daniil Ermolaev

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE. */

use std::{ env, fs };
use std::io::{ self, Read, Write };
use std::path::{ PathBuf, Path };
use std::sync::{ Arc, Mutex };

use serde_json::{ json, Value, Result as JsonResult };

use log::{ info, error, trace };
use simplelog::*;

use chrono::Local;

pub mod json_engine;
pub mod engines;
pub mod grpc_server;
pub mod instance;
pub mod utils;
pub mod command_executor;

use json_engine::*;
use engines::*;
use grpc_server::*;
use instance::*;
use utils::*;
use command_executor::*;

/* gRPC 
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root_path: PathBuf = get_root_path();

    let instance_manager = GRPCInstanceManager {
        instance_manager: Arc::new(Mutex::new(InstanceManager::new(&root_path))),
    };
    
    println!("Starting gRPC server...");

    Server::builder()
        .add_service(InstanceServiceServer::new(instance_manager))
        .serve("[::1]:50052".parse()?)
        .await?;

    Ok(())
}
*/

impl IMCommandExecutor for InstanceManager {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root_path: PathBuf = get_root_path();

    let mut im = InstanceManager::new(&root_path);
    match im.execute("CREATE INSTANCE instance_q;") {
        Ok(_) => println!("Done!"),
        Err(e) => println!("Something went wrong."),
    }
    println!("New command");
    im.execute("GET ALL INSTANCES");
    Ok(())
}
