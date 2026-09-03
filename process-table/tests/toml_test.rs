
use process_table::{ProcessTableState, ProcessTableStateConfig};

#[test]
fn test_default_serialize_deserialize() {
    let table_state = ProcessTableState::default();
    let table_state_config = ProcessTableStateConfig::from(&table_state);

    let serialized = toml::to_string(&table_state_config);

    assert!(serialized.is_ok());

    let table_state_config: ProcessTableStateConfig = toml::from_str(&serialized.unwrap()).unwrap();
    let table_state: ProcessTableState = ProcessTableState::try_from(&table_state_config).unwrap();

    assert!(table_state.row_selection().selection().is_none());
    assert!(table_state.filter_ast().is_none());
}

