//! # do what
//! 1. 使用ncollide重标注图片 ⛔
//!     1. 获取YOLO标注的数据 -> Rust结构体
//!     2. Rust结构体+YOLO标注图 -> ncollide进行标注
//! 2. 状态机加算法处理
//! 3. 包装数据
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/10/8
//! @version:0.0.1
//! @description:
//! ```

use std::path::{Path, PathBuf};
use std::fs::{read_dir, read_to_string, create_dir};
use std::ops::Deref;
use std::str::FromStr;
use serde::{Serialize, Deserialize};


// 设置全局常量
const ADDING: usize = 20;
const DATA_DIR: &str = "E:\\Rust\\try\\auto_drive_all\\datas\\res";
const IMG_DIR: &str = "E:\\Rust\\try\\auto_drive_all\\datas\\imgs";
const RES_DIR: &str = "E:\\Rust\\try\\auto_drive_all\\datas\\reoverlays";
const SPLIT_CHAR: &str = "\\";

#[derive(Debug, Serialize, Deserialize)]
struct OriginDatas {
    cls: Vec<u8>,
    pos: Vec<Vec<f32>>,
}

impl From<String> for OriginDatas {
    fn from(value: String) -> Self {
        let res: OriginDatas = serde_json::from_str(&value).unwrap();
        res
    }
}

impl OriginDatas {
    pub fn get_items(&self) -> Vec<OriginData> {
        let mut res = vec![];
        //遍历
        for i in 0..self.cls.len() {
            res.push(OriginData::new(self.cls[i].clone(), self.pos[i].clone()))
        }
        res
    }
}

#[derive(Debug)]
struct OriginData {
    cls: u8,
    pos: Vec<f32>,
}

impl OriginData {
    pub fn new(cls: u8, pos: Vec<f32>) -> Self {
        OriginData {
            cls,
            pos,
        }
    }
    pub fn cls(&self) -> u8 {
        self.cls
    }
    pub fn pos(&self) -> Vec<f32> {
        self.pos.clone()
    }
}

/// 遍历YOLO标注完的数据目录
/// 获取图片目录集
fn get_data(dir: &str) -> Vec<String> {
    let data_dir = PathBuf::from_str(dir).unwrap();
    let target_dir = read_dir(data_dir.as_path()).expect("can not read");
    let mut dirs = vec![];
    for child_dir in target_dir {
        dirs.push(child_dir.unwrap().file_name().to_str().unwrap().to_string())
    }
    dirs
}

fn main() {

    // 获取原始数据父级目录
    let f_source_dirs = get_data(DATA_DIR);
    // 循环遍历
    for f_dir_str in f_source_dirs {
        //向下级继续遍历
        let f_dir_path = format!("{}{}{}", DATA_DIR, SPLIT_CHAR, &f_dir_str);
        let origin_img_path = format!("{}{}{}", IMG_DIR, SPLIT_CHAR, &f_dir_str);
        let source_files = get_data(&f_dir_path);
        let loop_num = source_files.len() / 2_usize;
        for i in 0..loop_num {
            let origin_counter = ADDING * i;
            let origin_filepath = (format!("{}{}{}.txt", f_dir_path, SPLIT_CHAR, origin_counter), format!("{}{}{}.png", origin_img_path, SPLIT_CHAR, origin_counter));
            // 读取txt中的数据通过serde解析为结构体
            let source_data_str = read_to_string(PathBuf::from_str(&origin_filepath.0).unwrap().as_path()).unwrap();
            let source_data = OriginDatas::from(source_data_str);
            let item_list = source_data.get_items();
            for item in item_list {
                dbg!(item.cls());
                dbg!(item.pos());
                let pos = item.pos();
                let h = pos[0] - pos[2];
                let w = pos[1] - pos[3];
                //


            }


        }
    }
}
