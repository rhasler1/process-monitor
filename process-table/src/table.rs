// Testing for this module is in tests/integration_test.rs
use super::{Process, ProcessPid, ProcessStats, ProcessTableRow, RowSort, AST, Error};

use std::collections::{
    hash_map::{
        HashMap,
        Entry::{Occupied, Vacant}
    },
    hash_set::HashSet
};

use std::time::Duration;


pub struct ProcessTable {
    /// sampled processes
    processes:      Vec<Process>,
    /// statistics derived from sampled processes
    process_stats:  HashMap<ProcessPid, ProcessStats>,
    /// Expected sample interval
    sample_interval: Duration,
}

impl ProcessTable {
    pub fn new(
        processes:      Vec<Process>,
        sample_interval: Duration,
        ) -> Result<Self, Error> {
        // Build new table
        let mut table = Self {
            processes,
            process_stats: HashMap::new(),
            sample_interval
        };

        // Update stats
        table.update_process_stats()?;

        Ok(table)
    }

    /// Update processes & stats
    pub fn update(&mut self, processes: Vec<Process>) -> Result<(), Error> {
        self.processes = processes;
        self.update_process_stats()
    }

    /// Updates Process Statistics with current Process values
    fn update_process_stats(&mut self) -> Result<(), Error> {
        let mut seen = HashSet::new();

        for process in &self.processes {
            // Track current pids
            seen.insert(process.pid().to_owned());
            // Get copy of pid to access process stats map

            match self.process_stats.entry(process.pid().to_owned()) {
                Occupied(mut entry) => {
                    let process_stats = entry.get_mut();

                    process_stats.push(
                        process.cpu_total().as_f32(),
                        process.cpu_average().as_f32(),
                        process.mem().as_u64()
                    );
                }
                Vacant(entry) => {
                    // Create new entry with current value
                    let mut process_stats = ProcessStats::new(self.sample_interval)?;
                    
                    process_stats.push(
                        process.cpu_total().as_f32(),
                        process.cpu_average().as_f32(),
                        process.mem().as_u64());
                    
                    entry.insert_entry(process_stats);
                }
            }
        }

        // Remove old entries
        self.process_stats.retain(|pid, _| seen.contains(pid));

        Ok(())
    }


    pub fn visible_rows(&self, sort: &RowSort, ast: &Option<AST>) -> impl Iterator<Item = ProcessTableRow<'_>> {
        self.rows_sorted(sort)
            .filter(|row| {
                match ast {
                    Some(ast) => ast.matches(row),
                    None => true
                }
            })
    }

    fn sort_indices(&self, sort: &RowSort) -> Vec<usize> {
        let mut indices: Vec<usize> = (0..self.processes.len()).collect();

        indices.sort_by(|&i, &j| {
            let row_i = ProcessTableRow::new(
                self.processes.get(i).unwrap(),
                self.process_stats.get(self.processes.get(i).unwrap().pid()).unwrap());

            let row_j = ProcessTableRow::new(
                self.processes.get(j).unwrap(),
                self.process_stats.get(self.processes.get(j).unwrap().pid()).unwrap());

            row_i.cmp(&row_j, sort)
        });
        
        indices
    }

    fn rows_sorted(&self, sort: &RowSort) -> impl Iterator<Item = ProcessTableRow<'_>> {
        self.sort_indices(sort)
            .into_iter()
            .map(|visual_index| {
                ProcessTableRow::new(
                    self.processes.get(visual_index).unwrap(),
                    self.process_stats.get(self.processes.get(visual_index).unwrap().pid()).unwrap())
            })
    }

    pub fn count_visible_rows(&self, sort: &RowSort, ast: &Option<AST>) -> usize {
        self.visible_rows(sort, ast).count()
    }
}

