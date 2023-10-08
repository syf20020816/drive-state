# 视频切分脚本
import logging
import cv2
import os

# 1.加载videos
# 2.使用opencv对视频进行每20帧切割为一张图片


# error code
video_open_error = 101


# 加载视频-------------------------------------------------------------------------
base_dir = "E:\\Rust\\try\\auto_drive_all\\datas\\"
# 目标视频路径
target_video_dir = base_dir + "videos"
# 将视频切割为图片后的存储路径
target_imgs_dir = base_dir + "imgs"


# cv读取视频进行切割-----------------------------------------------------------------
# 遍历目录下所有的视频文件
video_list = os.listdir(target_video_dir)
for video in video_list:
    video_path = target_video_dir + "\\" + video
    # cv进行读取
    target_video = cv2.VideoCapture(video_path)
    # 判断是否能够打开视频
    if not target_video.isOpened():
        logging.error("video can not be opened!")
        exit(video_open_error)

    # 视频帧
    frame_num = 0
    target_frame_num = 20

    # 开始截取图片
    while True:
        # 读取一帧
        ret, frame = target_video.read()
        # 读取失败说明视频结束直接退出循环
        if not ret:
            break
        if frame_num % target_frame_num == 0:
            # 构造完整存储父路径
            cut_img_dir = target_imgs_dir + "\\" + video.split('.avi')[0]
            # 创建存储路径
            if not os.path.exists(cut_img_dir):
                os.mkdir(cut_img_dir)
            # 构造截图图片名称
            cut_img_name = str(frame_num) + ".png"
            complete_path = os.path.join(cut_img_dir, cut_img_name)
            # store
            cv2.imwrite(complete_path, frame)

        frame_num += 1

    target_video.release()