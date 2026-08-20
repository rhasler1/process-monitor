
use process_table::ProcessTableState;

#[test]
fn test_default_serialize_deserialize() {
    let table_state = ProcessTableState::default();

    let serialized = toml::to_string(&table_state);

    assert!(serialized.is_ok());

    let table_state: ProcessTableState = toml::from_str(&serialized.unwrap()).unwrap();

    assert!(table_state.validate_deserialization().is_ok());

    assert!(table_state.row_selection().selection().is_none());

    assert!(table_state.filter_ast().is_none());
}

#[test]
fn test_modified_state_serialize_deserialize() {
    let mut table_state = ProcessTableState::default();

    // modify filter
    table_state.mut_filter_string().insert_ascii_str("pid = 1").unwrap();

    table_state.update_filter_ast().unwrap();

    assert!(table_state.filter_ast().is_some());

    // modify row selection
    table_state.mut_row_selection().update_selection(10);

    assert!(table_state.row_selection().selection().is_some());

    // serialize
    let serialized = toml::to_string(&table_state);

    assert!(serialized.is_ok());

    let table_state: ProcessTableState = toml::from_str(&serialized.unwrap()).unwrap();

    assert!(table_state.validate_deserialization().is_ok());

    // filter_string is saved
    assert!(table_state.filter_string().as_str() == "pid = 1");

    // row selection is default: None
    assert!(table_state.row_selection().selection().is_none());

    // filter ast is default: None
    assert!(table_state.filter_ast().is_none());
}

