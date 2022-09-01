import requests



source=requests.get("https://kyfw.12306.cn/otn/resources/js/framework/station_name.js?station_version=1.9238").text

with open("./src/consts.rs","w",encoding="utf-8") as f:
    f.write("""use std::collections::HashMap;

lazy_static::lazy_static! {
pub static ref STATION_CODE: HashMap<&'static str, &'static str> =[
""")
    result= source.split("@")
    for station in result:
        print(station)
        if len(station)==0:
            continue
        lst=station.split("|")
        lst.pop()
        lst.pop()
        lst.pop()
        caller=lst.pop()
        name=lst.pop()

        f.write(f"        (\"{name}\" , \"{caller}\"),\n")

    
    f.write("""].iter().cloned().collect();
}""")