use std::fs::File;
use std::io::{BufReader, Read};

use crate::error::Result;
use encoding::all::UTF_8;
use encoding::{DecoderTrap, Encoding};
use log::info;

pub fn extract_text_from_pdf(file_path: &str) -> Result<String> {
    info!("extract_text_from_pdf");
    // 从PDF文件中提取文本的实用函数
    // 返回提取的文本或错误
    // 使用 error.rs 中的 Result 类型
    // 可能的错误：IoError

    // 打开文件并创建读取器
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    // 创建一个字节缓冲区
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    // 尝试解码为 UTF-8
    Ok(UTF_8.decode(&buffer, DecoderTrap::Replace).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_extract_text_from_pdf() {
        let file_path = "/Users/huisheng/Desktop/bbbb.txt";

        match extract_text_from_pdf(file_path) {
            Ok(text) => {
                // 保存提取的文本到文件
                let output_file = "/Users/huisheng/Desktop/aaa.txt";
                let mut file = File::create(output_file).expect("Failed to create output file");
                file.write_all(text.as_bytes())
                    .expect("Failed to write to output file");

                println!("Text extracted and saved to: {}", output_file);
            }
            Err(err) => {
                panic!("Failed to extract text from PDF: {}", err);
            }
        }
    }
}
