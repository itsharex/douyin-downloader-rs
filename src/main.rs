use std::{sync::Arc, fs, time::Duration, thread};
use anyhow::Result;
use futures::future::join_all;
use pbr::{ProgressBar, Units};

use clap::Parser;
use std::path::Path;
use regex::Regex;


use dydl::downloader::Downloader;


/// Douyin video downloader.
#[derive(Parser, Debug)]
#[clap(version, author, about)]
struct Args {
    /// Douyin video sharing link.
    #[clap(short, long, default_value_t=String::from("https://v.douyin.com/23FsM5g/"))]
    link: String,

    /// Download video, cover and music.
    #[clap(short, long, default_value_t=true)]
    all: bool,

    /// Download music.
    #[clap(short, long)]
    music: bool,

    /// Download cover.
    #[clap(short, long)]
    cover: bool,
    
    /// Download music.
    #[clap(short, long)]
    video: bool,
    
    /// File storage directory, default: ./download
    #[clap(short, long, default_value_t=String::from("download"))]
    dir: String,
}


struct Link {
    aweme_id: String,
    video_url: String,
    cover_url: String,
    music_url: String,
}

pub enum DownloadType {
    Music,
    Cover,
    Video,
}







#[derive(Clone)]
pub struct DouyinDownloader {
    inner: Arc<Downloader>
}

impl DouyinDownloader {

    pub async fn new(aweme_id:String, url: String, mode: DownloadType, dirname: String) -> Result<Arc<Self>> {
        if !Path::new(&dirname).exists(){
            fs::create_dir(&dirname)?;
        }
    
        let filepath: String = match mode {
            DownloadType::Cover => format!("{}/cover_{}.jpeg", dirname, aweme_id),
            DownloadType::Music => format!("{}/music_{}.m4a", dirname, aweme_id),
            DownloadType::Video => format!("{}/video_{}.mp4", dirname, aweme_id),
        };
        let inner = Downloader::new(url, filepath, None).await?;
        Ok(Arc::new(Self { inner }))
    }

    pub async fn download(self: Arc<Self>) -> Result<bool> {
        self.inner.clone().download().await
    }

    pub async fn filesize(self: Arc<Self>) -> u64 {
        self.inner.clone().total_size()
    }

    pub async fn downloaded_size(self: Arc<Self>) -> u64 {
        self.inner.clone().downloaded_size().await
    }
}





async fn download(aweme_id:String, link: String, mode: DownloadType, dirname: String) -> Result<(), Box<dyn std::error::Error>> {
    
    let downloader = DouyinDownloader::new(aweme_id, link, mode, dirname).await?;
    let downloader_pb = downloader.clone();
    let filesize = downloader.clone().filesize().await;
    let mut pb = ProgressBar::new(filesize);
    pb.set_units(Units::Bytes);

    tokio::spawn(async move {
        let _ = downloader.download().await;
    });
    
    let mut process_size = 0;
    loop {
        let downloaded_size = downloader_pb.clone().downloaded_size().await;
        if downloaded_size >= filesize {
            pb.finish();
            break;
        }
        pb.add(downloaded_size - process_size);
        process_size = downloaded_size;
        thread::sleep(Duration::from_millis(200));
    }
    Ok(())
}
 
// #[derive(Debug)]
// pub struct VideoInfo {
//     pub video_id: String,   // 视频ID
//     pub video_title: String, // 视频标题
//     pub video_url: String,  // 视频链接
//     pub video_cover_url: String, // 视频封面URL
//     pub video_music_url: String, // 视频音频URL
//     pub author_uid: String,  // 作者user id
//     pub author_name: String,  // 作者昵称
//     pub author_avatar: String, // 作者头像
// }

/// 解析视频,音频,封面URL
async fn parse(link: String) -> Result<Link, Box<dyn std::error::Error>> {
    
    let real_url = reqwest::get(link).await?.url().to_string();

    let regex = Regex::new(r"/(?P<aweme_id>\d+)\?")?;
    let aweme_id = match regex.captures(&real_url) {
        Some(cap) => {
            cap.name("aweme_id").unwrap().as_str()
        },
        None => ""
    }.to_string();
    
    let api_url = format!("https://www.iesdouyin.com/web/api/v2/aweme/iteminfo/?item_ids={}", aweme_id);
    let res_data = reqwest::get(api_url).await?.text().await?;
    let json_data = json::parse(&res_data).unwrap();
    let data = json_data["item_list"][0].clone();

    // let video_id = data["aweme_id"].to_string();
    // let video_title = data["share_info"]["share_title"].to_string();

    let video_url = data["video"]["play_addr"]["url_list"][0].to_string()
        .replace("playwm", "play")
        .replace("ratio=720p", "ratio=1080p");
    let cover_url:String = data["video"]["origin_cover"]["url_list"][0].to_string();
    let music_url = data["music"]["play_url"]["url_list"][0].to_string();
    // let author_uid = data["author"]["uid"].to_string();
    // let author_name = data["author"]["nickname"].to_string();
    // let author_avatar = data["author"]["avatar_thumb"].to_string();
    // let res = VideoInfo {
    //     video_id: video_id,
    //     video_title: video_title,
    //     video_url: video_url.clone(),
    //     video_cover_url: cover_url.clone(),
    //     video_music_url: music_url.clone(),
    //     author_uid: author_uid,
    //     author_name: author_name,
    //     author_avatar: author_avatar,
    // };
    Ok(Link {aweme_id, video_url, cover_url, music_url})
    
}

/// 功能入口
#[tokio::main]
async fn main() -> Result<()>{
    
    let mut  args = Args::parse();

    if args.all { // download all
        args.music = true;
        args.cover = true;
        args.video = true;
    }

    let mut handler_list = vec![];
    if let Ok(link) = parse(args.link).await {
        if args.cover {
            let cover_dl = download(link.aweme_id.clone(), link.cover_url, DownloadType::Cover, args.dir.clone());
            handler_list.push(cover_dl);
        }
        if args.music {
            let music_dl = download(link.aweme_id.clone(), link.music_url, DownloadType::Music, args.dir.clone());
            handler_list.push(music_dl);
        }
        if args.video {
            let video_dl = download(link.aweme_id.clone(), link.video_url, DownloadType::Video, args.dir.clone());
            handler_list.push(video_dl);
        }
    }else {
        println!("下载失败, 无法解析地址...");
    }
    join_all(handler_list).await;
    Ok(())
}