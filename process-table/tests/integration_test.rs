use process_table::{Sort, Lexer, Parser, ProcessEntry as Row, ProcessTable, ProcessTableViewState};

fn test_rows() -> Vec<Row> {
    vec![
        Row::new(1, 1.0, 1, "a".to_string()),
        Row::new(2, 2.0, 2, "b".to_string()),
        Row::new(3, 3.0, 3, "c".to_string()),
        Row::new(4, 4.0, 4, "d".to_string()),
    ]
}

#[test]
fn test_table_and_view() {
    let rows = test_rows();

    let table = ProcessTable::new(rows);

    let mut view_state = ProcessTableViewState::default();

    let visual_row_count = table.count_visible_rows(view_state.filter_ast());
    view_state.update_row_selection(visual_row_count);
    assert_eq!(view_state.visual_row_selection(), Some(0));

    // BVA: Cannot inc past visual row count
    for _ in 0..=visual_row_count {
        view_state.inc_visual_row_selection(visual_row_count);
    }

    assert_eq!(
        view_state.visual_row_selection(),
        Some(visual_row_count - 1)
    );

    // Mock filter
    let filter_string = "pid = 1 | pid = 2 & cpu < 2".to_string();
    view_state.insert_str_filter_str(&filter_string).unwrap();

    view_state
        .update_row_filter()
        .inspect_err(|_| {
            // Consumer can set status message and discard or propagate error
        }).unwrap();

    // Update visual selection after filtering (Valdation/clamp)
    view_state.update_row_selection(table.count_visible_rows(view_state.filter_ast()));
    
    let sort = Sort::PidInc;
    let visible_rows: Vec<_> = table.visible_rows(&sort, view_state.filter_ast()).collect();

    assert_eq!(visible_rows.len(), 1);
    assert_eq!(visible_rows[0], &Row::new(1, 1.0, 1, "a".to_string()));
    assert_eq!(view_state.visual_row_selection(), Some(0));
}

