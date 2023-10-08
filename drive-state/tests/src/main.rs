//! # do what
//! 1. 使用ncollide重标注图片
//!     1. 获取YOLO标注的数据 -> Rust结构体
//!     2. Rust结构体+YOLO标注图 -> ncollide进行标注
//! 2. 获取重标注数据
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

impl OriginDatas{
    pub fn get_items(&self)->Vec<OriginData>{
        let mut res = vec![];
        //遍历
        for i in 0..self.cls.len() {
            res.push(OriginData::new(self.cls[i].clone(), self.pos[i].clone()))
        }
        res
    }
}

#[derive(Debug)]
struct OriginData{
    cls:u8,
    pos:Vec<f32>
}

impl OriginData{
    pub fn new(cls:u8,pos:Vec<f32>)->Self{
        OriginData{
            cls,
            pos
        }
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
        let f_dir_path = format!("{}{}{}", DATA_DIR, SPLIT_CHAR, f_dir_str);
        let source_files = get_data(&f_dir_path);
        let loop_num = source_files.len() / 2_usize;
        for i in 0..loop_num {
            let origin_counter = ADDING * i;
            let origin_filepath = (format!("{}{}{}.txt", f_dir_path, SPLIT_CHAR, origin_counter), format!("{}{}{}.png", f_dir_path, SPLIT_CHAR, origin_counter));
            //对图片进行重新标注，进行重新画框
            // 读取txt中的数据通过serde解析为结构体
            let source_data_str = read_to_string(PathBuf::from_str(&origin_filepath.0).unwrap().as_path()).unwrap();
            let source_data = OriginDatas::from(source_data_str);
            dbg!(source_data.get_items());
            break;
            //画线，五分之一
        }
    }
}
