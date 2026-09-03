// The tests in the file look at the interaction
// between ProcessTable & ProcessTableViewState.

use process_table::{
    Process, ProcessTable, ProcessTableState, RowSort as Sort,
};

use std::time::Duration;

fn test_four_processes() -> Vec<Process> {
    vec![
        Process::new(1, 1.0, 1.0, 1, "a".to_string()),
        Process::new(2, 2.0, 2.0, 2, "b".to_string()),
        Process::new(3, 3.0, 3.0, 3, "c".to_string()),
        Process::new(4, 4.0, 4.0, 4, "d".to_string()),
    ]
}

fn test_2s_duration() -> Duration {
    Duration::from_secs(2)
}

#[test]
fn test_row_sorting_by_pid() {
    let table = ProcessTable::new(test_four_processes(), test_2s_duration()).unwrap();

    let mut view_state = ProcessTableState::default();

    assert_eq!(*view_state.row_sort(), Sort::CpuDec);

    // Set row sort to PidInc
    view_state.row_sort_by(Sort::PidInc);

    assert_eq!(*view_state.row_sort(), Sort::PidInc);

    {
        let mut iter = table
            .visible_rows(
                view_state.row_sort(),
                view_state.filter_ast()
            );
        assert_eq!(
            iter.next().unwrap().process().pid().as_u32(),
            1
        );

        assert_eq!(
            iter.next().unwrap().process().pid().as_u32(),
            2
        );

        assert_eq!(
            iter.next().unwrap().process().pid().as_u32(),
            3
        );

        assert_eq!(
            iter.next().unwrap().process().pid().as_u32(),
            4
        );
    }

    // Set row sort to PidDec
    view_state.row_sort_by(Sort::PidDec);

    assert_eq!(*view_state.row_sort(), Sort::PidDec);
    
    {
        let mut iter = table
            .visible_rows(
                view_state.row_sort(),
                view_state.filter_ast()
            );
        
        assert_eq!(
            iter.next().unwrap().process().pid().as_u32(),
            4
        );

        assert_eq!(
            iter.next().unwrap().process().pid().as_u32(),
            3
        );

        assert_eq!(
            iter.next().unwrap().process().pid().as_u32(),
            2
        );

        assert_eq!(
            iter.next().unwrap().process().pid().as_u32(),
            1
        );
    }
}

#[test]
fn test_table_visible_rows_with_invalid_filter_string() {
    let table = ProcessTable::new(test_four_processes(), test_2s_duration()).unwrap();

    let mut view_state = ProcessTableState::default();

    // Malformed string
    let filter_string = "pi d = 1".to_string();

    // Update view_state with malformed string
    let res = view_state
        .mut_filter_string()
        .insert_ascii_str(&filter_string);

    assert!(res.is_ok());

    // Update view_state filter_ast with malformed string
    let res = view_state.update_filter_ast();

    // Report error
    assert!(res.is_err());

    // AST could not be created
    assert!(view_state.filter_ast().is_none());

    // Default Sort is CpuDec
    let mut iter = table.visible_rows(
        view_state.row_sort(),
        view_state.filter_ast()
    );

    assert_eq!(iter.next().unwrap().process().cpu_total().as_f32(), 4.0);
    assert_eq!(iter.next().unwrap().process().cpu_total().as_f32(), 3.0);
    assert_eq!(iter.next().unwrap().process().cpu_total().as_f32(), 2.0);
    assert_eq!(iter.next().unwrap().process().cpu_total().as_f32(), 1.0);
}

