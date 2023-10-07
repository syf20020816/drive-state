# Automatic driving accident detection(自动驾驶事故检测)

自动驾驶事故检测

## 目录

- datas：事故数据以及处理数据目录
- drive-state：自动驾驶事故状态机（Rust）
- Image_overlays：自动驾驶事故图片标注处理（Python）

## 技术总览

| 项目           | 技术                              |
| -------------- | --------------------------------- |
| drive-state    | Rust+state_machine+ncollide       |
| Image_overlays | Python+OpenCV+Ultralytics(YOLOv8) |

## 处理路线

![image-20231008002443421](E:\Rust\try\auto_drive_all\README\imgs\image-20231008002443421.png)