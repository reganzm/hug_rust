// 先定义component，这里抽象为DataSource
trait DataSource {
    fn write(&mut self);
    fn read(&self);
}

// 具体的component，这里为FileDataSource
#[derive(Default)]
struct FileDataSource;
impl DataSource for FileDataSource {
    fn write(&mut self) {
        println!("文件数据写入...")
    }
    fn read(&self) {
        println!("文件数据读取...")
    }
}

// 装饰器接口，扩展自DataSource
trait DataSourceDecorator: DataSource {
    fn wrap(data_source: impl DataSource + 'static) -> Self;
}

// 具体的压缩装饰器
struct CompressionDataSource {
    wrapper: Box<dyn DataSource>,
}
impl DataSourceDecorator for CompressionDataSource {
    fn wrap(data_source: impl DataSource + 'static) -> Self {
        Self {
            wrapper: Box::new(data_source),
        }
    }
}
impl DataSource for CompressionDataSource {
    fn write(&mut self) {
        println!("压缩...");
        self.wrapper.write();
    }
    fn read(&self) {
        println!("解压缩...");
        self.wrapper.read();
    }
}

// 加密装饰器
struct EncryptionDataSourceDecorator {
    wrapper: Box<dyn DataSource>,
}

impl DataSource for EncryptionDataSourceDecorator {
    fn read(&self) {
        println!("解密...");
        self.wrapper.read();
    }
    fn write(&mut self) {
        println!("加密...");
        self.wrapper.write();
    }
}
impl DataSourceDecorator for EncryptionDataSourceDecorator {
    fn wrap(data_source: impl DataSource + 'static) -> Self {
        Self {
            wrapper: Box::new(data_source),
        }
    }
}

fn main() {
    let file: FileDataSource = FileDataSource::default();
    let encryption_decorator = EncryptionDataSourceDecorator::wrap(file);
    let mut compression_decorator = CompressionDataSource::wrap(encryption_decorator);
    // 压缩+加密 写入
    compression_decorator.write();
    // 解压缩+解密 读取
    compression_decorator.read();
}
