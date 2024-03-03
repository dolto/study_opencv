use opencv::{
    core, highgui, imgproc, objdetect,
    prelude::*,
    types::{self, VectorOfRect},
    videoio, Result,
};

fn main() {
    let mut camera = videoio::VideoCapture::new(0, videoio::CAP_ANY).unwrap();
    let xml_face = "E:/opencv/opencv/build/etc/haarcascades/haarcascade_frontalface_default.xml";
    let xml_eye = "E:/opencv/opencv/build/etc/haarcascades/haarcascade_eye.xml";
    // opencv설치 위치에 있는 xml파일 경로

    let mut face_detector = objdetect::CascadeClassifier::new(xml_face).unwrap();
    let mut eye_detector = objdetect::CascadeClassifier::new(xml_eye).unwrap();

    let mut img = Mat::default();

    loop {
        if camera.read(&mut img).is_ok_and(|b| b) {
            let mut gray = Mat::default();
            if imgproc::cvt_color(&img, &mut gray, imgproc::COLOR_BGR2GRAY, 0).is_ok() {
                let mut faces = types::VectorOfRect::new();
                let mut eyes = types::VectorOfRect::new();
                let _ = face_detector.detect_multi_scale(
                    &gray,
                    &mut faces,
                    1.1,
                    10,
                    objdetect::CASCADE_SCALE_IMAGE,
                    core::Size::new(10, 10),
                    core::Size::new(0, 0),
                );
                let _ = eye_detector.detect_multi_scale(
                    &gray,
                    &mut eyes,
                    1.1,
                    20,
                    objdetect::CASCADE_SCALE_IMAGE,
                    core::Size::new(5, 5),
                    core::Size::new(0, 0),
                );
                // println!("{:?}", faces);
                if faces.len() > 0 {
                    for face in faces.iter() {
                        let _ = imgproc::rectangle(
                            &mut img,
                            face,
                            core::Scalar::new(0f64, 255f64, 0f64, 0f64),
                            2,
                            imgproc::LINE_8,
                            0,
                        );
                    }
                    for eye in eyes.iter() {
                        let _ = imgproc::rectangle(
                            &mut img,
                            eye,
                            core::Scalar::new(1., 1., 0., 0.),
                            2,
                            imgproc::LINE_8,
                            0,
                        );
                    }
                }
                let _ = highgui::imshow("gray", &img);
                let _ = highgui::wait_key(1);
            }
        }
    }
}
