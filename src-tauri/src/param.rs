// configクレートを用いて、設定ファイルであるparmater.jsonの生成、編集などを行うPrameterManager構造体を定義
// Paramter自体の定義はParamter構造体に定義

use config::{Config, ConfigError, File, FileFormat};

#[derive(Debug, serde::Deserialize)]
pub(crate) struct Parameter {
    pub token: String,
    pub secret: String,
}

pub(crate) struct ParameterManager {
    config: Config,
}

impl ParameterManager {
    pub fn new() -> Result<ParameterManager, ConfigError> {
        let builder = Config::builder()
            .set_default("default", "1").unwrap()
            .add_source(File::new("parameter.json", FileFormat::Json));
        
        match builder.build() {
            Ok(config) => {
                Ok(ParameterManager {
                    config: config,
                })
            },
            Err(e) => {
                Err(e)
            }
        }
    }

    pub fn get_parameter(&self) -> Parameter {
        Parameter { 
            token: self.config.get("token").unwrap(),
            secret: self.config.get("secret").unwrap(),
        }
    }

    pub fn set_parameter(&mut self, param : Parameter) -> bool {
        let mut builder = Config::builder()
            .set_default("default", "1").unwrap()
            .add_source(File::new("parameter.json", FileFormat::Json))
            .set_override("override", "1").unwrap();

        builder = builder.set_override("token", param.token).unwrap();
        builder = builder.set_override("secret", param.secret).unwrap();

        match builder.build() {
            Ok(config) => {
                println!("config: {:?}", config);
                true
            }
            Err(e) => {
                println!("error: {:?}", e);
                false
            }
        }
    }
}