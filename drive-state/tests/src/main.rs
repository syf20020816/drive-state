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
use std::fs::{read_dir, read_to_string, create_dir, write, remove_dir_all};
use std::ops::Deref;
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use image::{open};
use image::io::Reader;
use drive_state::LowerState;


// 设置全局常量
const ADDING: usize = 20;
const DATA_DIR: &str = "E:\\Rust\\try\\auto_drive_all\\datas\\res";
const IMG_DIR: &str = "E:\\Rust\\try\\auto_drive_all\\datas\\imgs";
const RES_DIR: &str = "E:\\Rust\\try\\auto_drive_all\\datas\\reoverlays";
const SPLIT_CHAR: &str = "\\";

#[derive(Debug, Serialize, Deserialize, Default)]
struct OriginDatas {
    cls: Vec<u8>,
    pos: Vec<Vec<f32>>,
}

impl From<String> for OriginDatas {
    fn from(value: String) -> Self {
        match serde_json::from_str(&value) {
            Ok(value) => value,
            Err(_) => OriginDatas::default()
        }
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
        // let origin_img_path = format!("{}{}{}", IMG_DIR, SPLIT_CHAR, &f_dir_str);
        let source_files = get_data(&f_dir_path);
        let loop_num = source_files.len() / 2_usize;
        //构建重标数据目录
        let res_path_str = format!("{}{}{}.json", RES_DIR, SPLIT_CHAR, &f_dir_str);
        // let res_path_str = format!("{}{}{}", RES_DIR, SPLIT_CHAR, &f_dir_str);
        // let res_path = Path::new(&res_path_str);
        // if res_path.exists() {
        //     let _ = remove_dir_all(res_path);
        // } else {
        //     create_dir(res_path);
        // }
        let mut data_list = Vec::new();
        for i in 0..loop_num {
            let origin_counter = ADDING * i;
            let origin_filepath = (format!("{}{}{}.txt", f_dir_path, SPLIT_CHAR, origin_counter), format!("{}{}{}.png", f_dir_path, SPLIT_CHAR, origin_counter));
            // 读取txt中的数据通过serde解析为结构体
            let source_data_str = read_to_string(PathBuf::from_str(&origin_filepath.0).unwrap().as_path()).unwrap();
            let source_data = OriginDatas::from(source_data_str);
            let item_list = source_data.get_items();
            // 获取图片分辨率
            let target_img = Reader::open(Path::new(origin_filepath.1.as_str())).unwrap().decode().unwrap();
            // 分辨率
            let img_h = target_img.height() as f32;
            let img_w = target_img.width() as f32;
            let mut tmp_list = Vec::new();
            for item in item_list {
                let cls = item.cls();
                let pos = item.pos();
                let h = pos[0] - pos[2];
                let w = pos[1] - pos[3];
                //通过三角可视区域过滤，通过比值进行判断
                // area_x_left = img_w*(img_h - pos[3]) / img_h
                // area_x_right = img_w - area_x_left
                // area_x_range = [ area_x_left , area_x_right ]
                let area_x_left = img_w * (img_h - pos[3]) / img_h;
                let area_x_right = img_w - area_x_left;
                if pos[0] >= area_x_left && pos[0] <= area_x_right {
                    let state = LowerState::new(img_h, img_w, pos, i as u32, cls as u32);
                    let _ = tmp_list.push(state);
                }else{
                    let _ = tmp_list.push(LowerState::default(
                        i as u32, cls as u32
                    ));
                }

            }
            data_list.push(tmp_list);

        }
        let res_file_path = Path::new(&res_path_str);
        // 写入数据
        write(res_file_path, serde_json::to_string(&data_list).unwrap());
    }
}
