// Windows: {FOLDERID_RoamingAppData}\{app_name}に設定ファイルは自動保存される

use confy;
use confy::ConfyError;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Parameter {
    pub token: String,
    pub secret: String,
}

pub(crate) fn get_parameter() -> Result<Parameter, ConfyError> {
    let err = confy::load("SwitchBotClient", "SwitchBotClient");
    match err {
        Ok(p) => Ok(p),
        Err(e) => Err(e),
    }
}

pub(crate) fn set_parameter(parameter: Parameter) -> Result<(), ConfyError> {
    let err = confy::store("SwitchBotClient", "SwitchBotClient", parameter);

    match err {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}