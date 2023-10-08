# 标注脚本
import os
from ultralytics import YOLO
from PIL import Image

# 3.加载所有图片
# 4.使用YOLO进行标注
# 5.得到标注结果（标注图和标注坐标信息）

# 加载pt-------------------------------------------------------------------------
model = YOLO('yolov8n.pt')

# 加载处理后所有图片---------------------------------------------------------------
base_dir = "E:\\Rust\\try\\auto_drive_all\\datas\\"
target_img_dir = base_dir + "imgs"
# 将图片进行标注和坐标处理后的存储路径
target_res_dir = base_dir + "res"
# 标注数据路径
img_path_list = os.listdir(target_img_dir)
# 使用YOLO进行标注----------------------------------------------------------------
for img_path in img_path_list:
    # 标注图片路径
    overlays_img_dir = target_img_dir + "\\" + img_path
    # 遍历标注图片路径进行截图标注
    overlays_img_list = os.listdir(overlays_img_dir)
    for img in overlays_img_list:
        # 构建存储目录
        target_res_img_dir = target_res_dir + "\\" + img_path
        if not os.path.exists(target_res_img_dir):
            os.mkdir(target_res_img_dir)

        tmp_img_path = overlays_img_dir + "\\" + img

        result = model(tmp_img_path)

        for r in result:
            im_array = r.plot()  # plot a BGR numpy array of predictions
            im = Image.fromarray(im_array[..., ::-1])  # RGB PIL image
            # 保存标注图片
            im.save(target_res_img_dir + "\\" + img)
            file = open(target_res_img_dir + "\\" + img.split('.png')[0] + ".txt", 'w')
            file.write('{')
            file.write('"cls":' + str(r.boxes.cls).replace('tensor(', '').replace(')', '').replace('.',''))
            file.write(',')
            file.write('"pos":' + str(r.boxes.xyxy).replace('tensor(', '').replace(')', ''))
            file.write('}')
            file.close()
