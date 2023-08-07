# Mini Slam

Toy implementation of the Visual Simultaneous Localization and Mapping (SLAM) algorithm in Rust with a trick up its sleeve. This implementation of VSLAM utilizes a dense depth model proposed in the paper High Quality Monocular Depth Estimation via Transfer Learning (arXiv:1812.11941) for the mapping component.

### Building

Make sure you have cargo and rust installed.

#### MacOS

To run this make sure you have Xcode 14 installed.
To build this project run:

```bash
$ cargo build
```

To run this project:

```bash
$ cargo run datasets/vids/test_ohio.mp4
```
