# RUST树莓派小车

## 项目三方依赖
1# libc="0.2.108"
> 用于解析读取内核文件`/dev/input/js0` 

2# glob = "0.3.0"
> 用于检测内核文件以判断手柄连接情况

3# tokio = { version = "1", features = ["full"] }
> 异步运行时


4# rppal = "0.13.1"
> 树莓派驱动

