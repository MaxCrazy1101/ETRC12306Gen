use clap::Parser;
use etrc12306gen::cli::Cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    cli.run()?;
    // let train_number = &cli.trains.0[0];
    // let mut resp: String = String::new();
    // for bllx in vec!["01", "02", "03", "05"] {
    //     resp = http_client
    //         .post("https://ec.95306.cn/api/bl/queryProduct/queryProductQuery")
    //         .header("Content-Type", "application/json")
    //         .json(&json!({
    //             "pageNum":1,"pageSize":10,"bllx":bllx,"cprq":format!("{}-{}-{}",&date[..4],&date[4..6],&date[6..]),
    //             "zcztmism":"","zczItem":"","zcz":"","fashi":"","gbcc" :train_number,"xcztmism":"","xczItem":"",
    //             "xcz":"","daoshi":"","fjhz":"","fj":"","fjItem":"","dj":"","djdm":"","djItem":"","djhz":""
    //         }))
    //         .send().unwrap().text().unwrap();
    //     if !resp.contains("\"list\":[]") {
    //         break;
    //     }
    // }
    // println!("{}", resp);

    Ok(())
}
