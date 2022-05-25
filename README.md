# douyin-downloader-rs 

## 简介

Rust语言编写的抖音视频下载器, 支持通过分享链接下载视频丶音频丶封面。

## 声明

本工具仅为视频收藏者提供便捷，因此不支持批量下载视频！

## 下载

- 在release页面下载对应平台的二进制文件。

- 自行编译
 ```
 git clone https://github.com/ClassmateLin/douyin-downloader-rs.git
 cd douyin-downloader-rs && cargo build --release
```

## 使用方法

- 获取帮助
```
❯ ./dydl -h
dydl 0.1.0
Douyin video downloader

USAGE:
    dydl [OPTIONS] --link <LINK>

OPTIONS:
    -a, --all            Download video, cover and music
    -c, --cover          Download cover
    -d, --dir <DIR>      File storage directory, default: ./download [default: download]
    -h, --help           Print help information
    -l, --link <LINK>    Douyin video sharing link
    -m, --music          Download music
    -v, --video          Download music
    -V, --version        Print version information
```

- 下载分享链接`https://v.douyin.com/Fpyn1Xf/`的全部内容到video目录:
```
❯ ./dydl -a --link https://v.douyin.com/Fpyn1Xf/ --dir video
56.27 KB / 56.27 KB [=======================================================================================] 100.00 % 93.56 KB/s 
video/cover_7100478603577593088.jpeg, 下载成功!
966.57 KB / 966.57 KB [====================================================================================] 100.00 % 627.74 KB/s 
video/music_7100478603577593088.m4a, 下载成功!
8.08 MB / 26.87 MB [=========================>----------------------------------------------------------] 30.06 % 647.97 KB/s 30s 
```

- 下载分享链接`https://v.douyin.com/Fpyn1Xf/`的视频到video目录:

```
❯ ./dydl -v --link https://v.douyin.com/Fpyn1Xf/ --dir tmp
5.62 MB / 26.87 MB [=================>------------------------------------------------------------------] 20.93 % 638.96 KB/s 34s 
```
