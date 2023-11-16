# Todo (implementation steps)

- implement the image creation/display
  - Create a Canvas in Flutter, and allocate a pixel buffer
  - In Rust, get a `Vec` from the pixel buffer and apply domain coloring to it
  - In Flutter, set `pixels` to this and call `ui.decodeImageFromPixels` on `pixels.buffer.asUint8List()`
  - In Flutter, call `Canvas.drawImage` on the result `Image` object 