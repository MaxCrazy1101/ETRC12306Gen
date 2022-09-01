# ETRC12306Gen

## 备注
 - 部分API参考[cParsing12306](https://github.com/denglihong2007/cParsing_12306)
 - 
 - 

## 安装

前往 [release](https://github.com/Diaosi1111/ETRC12306GEN/releases) 获取自动编译版本, 支持 **Linux**, **Windows**, 以及 **macOS**.


## 使用方法 (Windows)

获取帮助
```
.\etrc12306gen.exe -h
```

第一次启动会生成一个cookie.txt文件，请打开12306官网把cookie复制到这里面

### Train
英文逗号分割要查询的车次
```
.\etrc12306gen.exe -d 20220902 train t114,z88
```

### Route
根据 qETRC 的导出运行图(第一条线路)查询所有车次
```
.\etrc12306gen.exe -d 20220902 route ./example.pyetgr
```

## 许可

Copyright (c) 2022 Alex Newton.

`ETRC12306Gen` is made available under the terms of either the MIT License or the Apache License 2.0, at your option.

See the [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) files for license details.
