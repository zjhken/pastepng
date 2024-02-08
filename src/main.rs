use std::io::{stdout, Stdout, Write};

use image::{DynamicImage, ImageBuffer, ImageOutputFormat, RgbaImage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut clipboard = arboard::Clipboard::new()?;
	let data = clipboard
		.get_image()
		.expect("No image data found in clipboard");

	let image: RgbaImage = ImageBuffer::from_raw(
		data.width.try_into()?,
		data.height.try_into()?,
		data.bytes.into_owned(),
	)
	.unwrap();
	let image = DynamicImage::ImageRgba8(image);
	let stdout = stdout();
	let mut stdout = NewStdOut(stdout);
	image.write_to(&mut stdout, ImageOutputFormat::Png).unwrap();
	stdout.flush()?;
	Ok(())
}

struct NewStdOut(Stdout);

impl std::io::Seek for NewStdOut {
	fn seek(&mut self, _pos: std::io::SeekFrom) -> std::io::Result<u64> {
		return std::io::Result::Ok(0u64);
	}
}

impl std::io::Write for NewStdOut {
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		return self.0.write(buf);
	}

	fn flush(&mut self) -> std::io::Result<()> {
		return self.0.flush();
	}
}
