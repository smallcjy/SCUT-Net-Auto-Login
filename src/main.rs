use std::{fs::File, io::Read, net::UdpSocket, process::exit, time::Duration};

use reqwest::Response;
use tokio::time::sleep;

#[derive(Clone)]
struct MyUrl {
    user_account: String,
    user_password: String,
    user_ip: String,
}

impl MyUrl {
    pub fn new(parms: Vec<String>) -> Self {
        Self {
            user_account: parms[0].clone(),
            user_password: parms[1].clone(),
            user_ip: parms[2].clone(),
        }
    }

    pub fn to_url(&self) -> String {
        format!(
            "https://s.scut.edu.cn:802/eportal/portal/login?callback=dr1003&login_method=1&user_account={}&user_password={}&wlan_user_ip={}&wlan_user_ipv6=&wlan_user_mac=000000000000&wlan_ac_ip=172.18.50.11&wlan_ac_name=&jsVersion=4.1.3&terminal_type=1&lang=en&v=1575&lang=en",
            self.user_account,
            self.user_password,
            self.user_ip
        )
    }
}

struct Executor {
    interval: Duration,
    repetitions: usize,
}

impl Executor {
    fn new(interval: Duration, repetitions: usize) -> Executor {
        Executor {
            interval,
            repetitions,
        }
    }

    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        for _ in 0..self.repetitions {
            // 获取url
            let mut parms = parse_from_file()?;
            match get_ip() {
                Some(ip) => parms.push(ip),
                None => {
                    println!("获取IP失败！");
                    continue;
                }
            }

            let my_url = MyUrl::new(parms);

            println!("尝试联网！");
            match create_get_request(my_url).await {
                Ok(_) => {
                    //check
                    if check_internet_connection().await? {
                        println!("成功联网！");
                        exit(0);
                    } else {
                        sleep(self.interval).await;
                        continue;
                    }
                }
                Err(_) => {
                    sleep(self.interval).await;
                    continue;
                }
            }
        }
        Ok(())
    }
}

async fn create_get_request(myurl: MyUrl) -> Result<Response, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().no_proxy().build()?;
    let url = myurl.to_url();
    let res = client.get(url).send().await?;
    Ok(res)
}

fn parse_from_file() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut file = File::open("./config.txt")?;
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);

    let mut lines = contents.lines();
    let user_account = lines.next().unwrap_or_default().to_string();
    let user_password = lines.next().unwrap_or_default().to_string();

    let mut res = Vec::new();
    res.push(user_account);
    res.push(user_password);

    return Ok(res);
}

fn get_ip() -> Option<String> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    let local_ip = socket.local_addr().ok()?;
    Some(local_ip.ip().to_string())
}

async fn check_internet_connection() -> Result<bool, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().no_proxy().build()?;
    let res = client.get("https://www.baidu.com").send().await?;
    Ok(res.status().is_success())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exector = Executor::new(Duration::from_secs(1), 5);
    exector.run().await?;
    Ok(())
}
