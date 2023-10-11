# Automatic driving accident detection State(自动驾驶事故检测状态)

根据行车记录仪检测车辆有效碰撞区域是否有碰撞物，然后标注提取信息，得到一张按照时间轴顺序延展的状态变化的有向状态图

状态判断效果很大程度依赖于目标检测算法

## 目录

- datas：事故数据以及处理数据目录
- drive-state：自动驾驶事故状态机（Rust）
- Image_overlays：自动驾驶事故图片标注处理（Python）
- drive_analysis ： 状态图数据转状态图（TS）

## 技术总览

| 项目           | 技术                              |
| -------------- | --------------------------------- |
| drive-state    | Rust+state_machine+ncollide       |
| Image_overlays | Python+OpenCV+Ultralytics(YOLOv8) |
| drive_analysis | TS+ECharts+Tauri+Rust             |

## 处理路线

![image-20231011160101202](E:\Rust\try\auto_drive_all\README\imgs\image-20231011160101202.png)

### 效果

![image-20231011155501916](E:\Rust\try\auto_drive_all\README\imgs\image-20231011155501916.png)