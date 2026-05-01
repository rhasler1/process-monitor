// TODO [3/24/26]
// Required behavior: If a user entered config is invalid, then use program default
// Component models are to implement Default and From<Config>


/*
 * Example config file entry:
 *
 * [Column]
 * pid
 * name
 * cpu
 * mem mb
 * mem kb
 *
 * I must determine how to build these configurable `models` from the config
 * There must be instructions on the internet on the best way to go about this
 * */

// Going to use TOML format
//
// TODO [3/28/26] Singing off here
use serde::{Serialize, Deserialize};

pub struct ProcessTableColumnConfig {
}

impl ProcessTableConfig {
    fn translate()
}

pub struct ProcessRefreshIntervalConfig {

} 

// Aggregate other `Config's` here
pub struct Config {

}
