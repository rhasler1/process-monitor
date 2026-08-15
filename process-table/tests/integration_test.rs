// The tests in the file look at the interaction
// between ProcessTable & ProcessTableViewState.

use process_table::{
    ProcessEntry as Row, ProcessTable, ProcessTableState, Sort,
};

fn test_four_rows() -> Vec<Row> {
    vec![
        Row::new(1, 1.0, 1, "a".to_string()),
        Row::new(2, 2.0, 2, "b".to_string()),
        Row::new(3, 3.0, 3, "c".to_string()),
        Row::new(4, 4.0, 4, "d".to_string()),
    ]
}

#[test]
fn test_filtering_clamps_selection() {
    // Using nonempty rows
    let table = ProcessTable::new(test_four_rows());

    let mut view_state = ProcessTableState::default();

    // Selection is initialized to None.
    assert!(view_state.visual_row_selection().is_none());

    // Update Row selection
    view_state.update_row_selection(
        table.count_visible_rows(
            view_state.row_sort(),
            view_state.filter_ast())
    );

    // Selection is set to Some(0)
    assert_eq!(view_state.visual_row_selection(), Some(0));

    // Set visible row count variable
    let visible_row_count = table.count_visible_rows(
        view_state.row_sort(),
        view_state.filter_ast()
    );

    // Move selection to last row
    for _ in 0..visible_row_count {
        view_state.inc_visual_row_selection(visible_row_count);
    }

    // Seletion is at last row
    assert_eq!(
        view_state.visual_row_selection(),
        Some(visible_row_count - 1)
    );

    // Apply filter that leaves only 1 row
    view_state
        .filter_string_insert_str("pid = 1")
        .unwrap();

    view_state.update_filter_ast().unwrap();

    // Get visible row count
    let visible_row_count = table.count_visible_rows(
        view_state.row_sort(),
        view_state.filter_ast()
    );

    assert_eq!(visible_row_count, 1);

    // Update row selection
    view_state.update_row_selection(visible_row_count);

    // Selection is clamped
    assert_eq!(view_state.visual_row_selection(), Some(0));

    // Get selected row
    let row = table
        .get_row(
            view_state.row_sort(),
            view_state.filter_ast(),
            view_state.visual_row_selection()
        ).unwrap();

    assert_eq!(
        row,
        &Row::new(1, 1.0, 1, "a".to_string()),
    );
}

#[test]
fn test_row_scroll_tracks_selection() {
    let table = ProcessTable::new(test_four_rows());

    let mut view_state = ProcessTableState::default();

    let terminal_height = 2;

    assert!(view_state.visual_row_selection().is_none());

    let start = view_state.row_scroll_calc_start(terminal_height);

    assert_eq!(start, 0);

    // Move selection to Some(0)
    view_state
        .inc_visual_row_selection(
            table.count_visible_rows(
                view_state.row_sort(),
                view_state.filter_ast()
            )
        );
    assert_eq!(view_state.visual_row_selection(), Some(0));
    // Selection still in window
    let start = view_state.row_scroll_calc_start(terminal_height);
    assert_eq!(start, 0);

    // Move selection to Some(1)
    view_state
        .inc_visual_row_selection(
            table.count_visible_rows(
                view_state.row_sort(),
                view_state.filter_ast()
            )
        );
    assert_eq!(view_state.visual_row_selection(), Some(1));
    // Selection still in window
    let start = view_state.row_scroll_calc_start(terminal_height);
    assert_eq!(start, 0);

    // Move selection to Some(2)
    view_state
        .inc_visual_row_selection(
            table.count_visible_rows(
                view_state.row_sort(),
                view_state.filter_ast()
            )
        );
    assert_eq!(view_state.visual_row_selection(), Some(2));
    // Selection is no longer in window
    let start = view_state.row_scroll_calc_start(terminal_height);
    assert_eq!(start, 1);

    // Move selection to Some(3)
    view_state
        .inc_visual_row_selection(
            table.count_visible_rows(
                view_state.row_sort(),
                view_state.filter_ast()
            )
        );
    assert_eq!(view_state.visual_row_selection(), Some(3));
    // Selection is no longer in window
    let start = view_state.row_scroll_calc_start(terminal_height);
    assert_eq!(start, 2);
}

#[test]
fn test_row_sorting_by_pid() {
    let table = ProcessTable::new(test_four_rows());

    let mut view_state = ProcessTableState::default();

    assert_eq!(*view_state.row_sort(), Sort::CpuDec);

    // Set row sort to PidInc
    view_state.row_sort_by_pid_inc();

    assert_eq!(*view_state.row_sort(), Sort::PidInc);

    {
        let mut iter = table
            .visible_rows(
                view_state.row_sort(),
                view_state.filter_ast()
            );
        assert_eq!(
            iter.next().unwrap().pid().as_u32(),
            1
        );

        assert_eq!(
            iter.next().unwrap().pid().as_u32(),
            2
        );

        assert_eq!(
            iter.next().unwrap().pid().as_u32(),
            3
        );

        assert_eq!(
            iter.next().unwrap().pid().as_u32(),
            4
        );
    }

    // Set row sort to PidDec
    view_state.row_sort_by_pid_dec();

    assert_eq!(*view_state.row_sort(), Sort::PidDec);
    
    {
        let mut iter = table
            .visible_rows(
                view_state.row_sort(),
                view_state.filter_ast()
            );

        assert_eq!(
            iter.next().unwrap().pid().as_u32(),
            4
        );

        assert_eq!(
            iter.next().unwrap().pid().as_u32(),
            3
        );

        assert_eq!(
            iter.next().unwrap().pid().as_u32(),
            2
        );

        assert_eq!(
            iter.next().unwrap().pid().as_u32(),
            1
        );
    }
}

#[test]
fn test_table_visible_rows_with_invalid_filter_string() {
    let table = ProcessTable::new(test_four_rows());

    let mut view_state = ProcessTableState::default();

    // Malformed string
    let filter_string = "pi d = 1".to_string();

    // Update view_state with malformed string
    let res = view_state.filter_string_insert_str(&filter_string);

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

    assert_eq!(iter.next().unwrap().cpu().as_f32(), 4.0);
    assert_eq!(iter.next().unwrap().cpu().as_f32(), 3.0);
    assert_eq!(iter.next().unwrap().cpu().as_f32(), 2.0);
    assert_eq!(iter.next().unwrap().cpu().as_f32(), 1.0);
}

