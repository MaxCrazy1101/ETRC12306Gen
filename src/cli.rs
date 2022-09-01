use std::time;

use clap::{Args, Parser, Subcommand};
use log::debug;
use reqwest::{blocking::Client, Proxy};

use crate::parse::{parse_line, parse_train};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// 爬取日期，默认为第二天，输入格式如20220829
    #[clap(short, long, group = "input")]
    pub date: Option<String>,

    /// 使用http代理 输入格式 ip:port
    #[clap(short, long, group = "input",value_parser=check_proxy)]
    pub proxy: Option<Proxy>,

    /// (未实现)规则文件（UTF-8编码，格式为A B 1 0，意为将A站改为B站且把到达时间推迟一分钟设为通过状态）
    #[clap(short, long, group = "input")]
    pub rule: Option<String>,

    #[clap(subcommand)]
    pub command: Commands,
}
impl Cli {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        pretty_env_logger::init();
        let mut builder = Client::builder()
            .cookie_store(true)
            .timeout(time::Duration::from_secs(7));
        if let Some(proxy) = &self.proxy {
            debug!("proxy:{:?}", proxy);
            builder = builder.proxy(proxy.clone());
        }
        let http_client = builder.build().expect("创建http客户端失败");
        let date = match &self.date {
            Some(d) => d.clone(),
            None => "20220902".to_string(),
        };
        match &self.command {
            Commands::Train(train) => train.run(date, http_client),
            Commands::Route(route) => route.run(date, http_client),
        }
    }
}
#[derive(Subcommand)]
pub enum Commands {
    /// 使用车次信息生成
    Train(TrainArgs),
    /// 使用线路信息生成
    Route(RouteArges),
}

#[derive(Args)]
pub struct TrainArgs {
    /// 车次，每个车次用英文逗号隔开，如: T114,Z86
    #[clap(value_parser=construct_trains)]
    pub trains: (Vec<String>, Vec<String>),
}

impl TrainArgs {
    fn run(&self, date: String, http: Client) -> Result<(), Box<dyn std::error::Error>> {
        parse_train(date, self.trains.0.clone(), http)
    }
}

#[derive(Args)]
pub struct RouteArges {
    /// pyergr文件位置
    #[clap(parse(from_os_str))]
    pub route_path: std::path::PathBuf,
}
impl RouteArges {
    fn run(&self, date: String, http: Client) -> Result<(), Box<dyn std::error::Error>> {
        parse_line(&self.route_path, date, http)
    }
}

#[inline]
fn check_proxy(s: &str) -> Result<Proxy, reqwest::Error> {
    Proxy::all(format!("http://{}", s))
}

/// 解析命令行传递的待查询列车号
fn construct_trains(s: &str) -> Result<(Vec<String>, Vec<String>), String> {
    let mut get_query = vec![];
    let mut post_query = vec![];

    for i in s.split(',') {
        let train = i.trim().to_string().to_uppercase();
        if i.is_empty() {
            continue;
        }

        if (train.len() == 5 && train.parse::<u32>().is_ok())
            || &train[0..1] == "X"
            || train.contains('/')
        {
            post_query.push(train)
        } else {
            get_query.push(train);
        }
    }
    Ok((get_query, post_query))
}
