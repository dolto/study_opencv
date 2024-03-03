# Rust OpenCV 개발 환경 설정

## 목차
1. 소개
2. 과정
3. 주의할점
4. 결론

### 소개
[수상한 인도인의 얼굴인식 튜토리얼](https://www.youtube.com/watch?v=iWficV_pmxY)
- 위의 영상을 보면서 rust환경에서 opencv를 이용한 얼굴인식 프로그램을 만들어보려다, 개발환경 설정이 제대로 되지 않아서 초반의 ```cargo run``` 마저 제대로 되지 않아, 여기저기 알아낸 사실들을 정리하고자 한다.
- OS는 Windwos10이다.

### 과정
- 먼저 opencv를 설치해준다.
- [링크](https://opencv.org/releases/)에서 다음 사진과 같이 클릭하여 설치를 진행해주면 된다. (exe로 되어있는 인스톨러이니 굳이 그 이상의 설치 과정은 해주지 않겠다.)
- ![자료화면](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Opencv_Setting/1.webp)
- 설치한 다음, 환경변수를 설정해줘야하는데, 설치된 경로를 기준으로 환경변수를 설정해주면 된다.
- 필자의 경우 설치경로가 ```E:\opencv\opencv\```이므로 환경변수도 다음과같이 설정해준다.
- ![자료화면](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Opencv_Setting/2.webp)
- 특히 ```OPENCV_LINK_LIBS```의 경우는 ```OPENCV_LINK_PATHS```경로안에 있는 파일을 확인해주면 된다.
- ![자료화면](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Opencv_Setting/3.webp)
- 마지막으로 PATH설정까지 해주면 끝!
- ![자료화면](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Opencv_Setting/2.5.webp)
- 또한 clang과 llvm을 설치해줘야 하는데, 이는 [링크](https://releases.llvm.org/)에서 찾아볼 수 있다. (스크롤을 내리면 찾을 수 있다.)
- ![자료화면](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Opencv_Setting/4.webp)
- 설치가 완료되면 터미널에서 다음과 같은 명령어에서 정상적인 에러가 발생하면 설치를 잘 했다는 뜻이다.
- ![자료화면](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Opencv_Setting/5.webp)
- 이렇게 한 다음 코드(깃허브 참조)를 실행하면 다음과 같이 수행되는 것을 알 수 있다.
-  ![자료화면](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Opencv_Setting/6.webp)

### 주의할점
- 당연한 이야기 이지만 환경변수 설정을 잘 해주어야한다.
- 또한 환경변수 설정할때마다 터미널을 새롭게 켜줘야 하고, 오타가 있는지 없는지 잘 확인해야한다.
- 환경변수가 잘 되었다 싶은데, 등록된 exe파일같은게 실행되는지 확인해보고, 안되면, 경로를 다시 한번 확인해줘야한다.
- 또한 러스트에서는 최소한 bin경로 설정만 되었다면 다음과같은 에러를 발생시키는데
```
...중략
 --- stderr
  === Crate version: Some("0.88.8")
  === Environment configuration:
  ===   OPENCV_PACKAGE_NAME = None
  ===   OPENCV_PKGCONFIG_NAME = None
  ===   OPENCV_CMAKE_NAME = None
  ===   OPENCV_CMAKE_BIN = None
  ===   OPENCV_VCPKG_NAME = None
  ===   OPENCV_LINK_LIBS = None
  ===   OPENCV_LINK_PATHS = None
  ===   OPENCV_INCLUDE_PATHS = None
  ===   OPENCV_DISABLE_PROBES = None
  ===   OPENCV_MSVC_CRT = None
  ===   CMAKE_PREFIX_PATH = None
  ===   OpenCV_DIR = Some("E:\\opencv\\opencv\\build")
  ===   PKG_CONFIG_PATH = None
  ===   VCPKG_ROOT = None
  ===   VCPKGRS_DYNAMIC = None
  ===   VCPKGRS_TRIPLET = None
  ===   OCVRS_DOCS_GENERATE_DIR = None
  ===   DOCS_RS = None
  ... 중략
```
- 이런 에러메세지에서 발견되는 None인 환경변수들중 필수적으로 필요한 환경변수를 찾아보는 것도 방법이다.

### 결론
- 원래 안면인식 관련 코드를 공부해보려고 시작한 간단한 튜토리얼이었으나, 개발환경 세팅을 맞추는 것이 상당히 어려운 일이라는 것을 알게 되었다.
- 정리해놓고 보니 별거 없는 내용인 것 같지만, 이 과정을 거치는데 무려 이틀이란 시간이 걸렸다면 믿을 것인가? 이런 개발환경 세팅 경험이 축적되어서 다음엔 처음보는 개발환경이라도 시간소모를 적게 할 수 있을 거라고 기대한다.

[블로그 링크](https://portfolio-user-kohl.vercel.app/?is_blog=true&langs_slecets=[]&skills_slects=[]&project_id=65e4180ac2961561e22dc64a)
