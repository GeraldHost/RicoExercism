// TODO: add callback
// TODO: add compute cells as dependencies
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputCellID(usize);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComputeCellID(usize);

pub struct ComputeCell<'a, T> {
    pub id: ComputeCellID,
    pub dependencies: Vec<CellID>,
    pub compute_func: Box<dyn Fn(&[T]) -> T + 'a>,
}

impl<'a, T: Copy + PartialEq> ComputeCell<'a, T> {
    pub fn new(
        id: ComputeCellID,
        dependencies: Vec<CellID>,
        compute_func: Box<dyn Fn(&[T]) -> T + 'a>,
    ) -> Self {
        ComputeCell {
            id,
            dependencies,
            compute_func,
        }
    }
    pub fn value(&self, reactor: &Reactor<'a, T>) -> T {
        let compute_func = &self.compute_func;
        let deps: Vec<T> = self
            .dependencies
            .iter()
            .filter_map(move |id| reactor.value(*id))
            .collect();
        compute_func(&deps)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CallbackID();

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

pub struct Reactor<'a, T> {
    cells: Vec<T>,
    compute_cells: Vec<ComputeCell<'a, T>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor {
            cells: vec![],
            compute_cells: vec![],
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellID {
        self.cells.push(initial);
        InputCellID(self.cells.len() - 1)
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'a>(
        &mut self,
        dependencies: &[CellID],
        compute_func: F,
    ) -> Result<ComputeCellID, CellID> {
        let args: Result<Vec<T>, CellID> = dependencies
            .iter()
            .map(|id| self.value(*id).ok_or(*id))
            .collect();

        match args {
            Ok(a) => {
                let compute_cell_id = ComputeCellID(self.cells.len());
                let compute_cell = ComputeCell::new(
                    compute_cell_id,
                    dependencies.to_vec(),
                    Box::new(compute_func),
                );
                let value = compute_cell.value(self);
                self.cells.push(value);
                self.compute_cells.push(compute_cell);
                Ok(compute_cell_id)
            }
            Err(id) => Err(id),
        }
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    pub fn value(&self, id: CellID) -> Option<T> {
        match id {
            CellID::Input(InputCellID(cell_id)) | CellID::Compute(ComputeCellID(cell_id)) => {
                self.cells.get(cell_id).cloned()
            }
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    pub fn set_value(&mut self, id: InputCellID, new_value: T) -> bool {
        let InputCellID(cell_id) = id;
        match self.cells.get_mut(cell_id) {
            Some(value) => {
                *value = new_value;
                // some how I need to recurrsively check all inputs and inputs
                // of inputs until I have calculated the compute values for
                // all inputs
                for compute_cell in self.compute_cells.iter() {
                    if compute_cell.dependencies.contains(&CellID::Input(id)) {
                        let ComputeCellID(cell_id) = compute_cell.id;
                        let new_value = compute_cell.value(self);
                        if let Some(value) = self.cells.get_mut(cell_id) {
                            *value = new_value;
                        }
                    }
                }
                true
            }
            None => false,
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) -> ()>(
        &mut self,
        _id: ComputeCellID,
        _callback: F,
    ) -> Option<CallbackID> {
        unimplemented!()
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellID,
        callback: CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        unimplemented!(
            "Remove the callback identified by the CallbackID {:?} from the cell {:?}",
            callback,
            cell,
        )
    }
}
