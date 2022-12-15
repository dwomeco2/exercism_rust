use std::collections::HashMap;

/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellId(usize);

struct ComputeCell<'a, T> {
    value: T,
    dependencies: Vec<CellId>,
    compute_fn: Box<dyn Fn(&[T]) -> T + 'a>,
    callback_id_set: Vec<CallbackId>,
}

impl<'a, T> ComputeCell<'a, T> {
    pub fn new<F>(val: T, dependencies: &[CellId], f: F) -> Self
    where
        F: Fn(&[T]) -> T + 'a,
    {
        Self {
            value: val,
            dependencies: dependencies.into(),
            compute_fn: Box::new(f),
            callback_id_set: vec![],
        }
    }

    pub fn add_callback(&mut self, id: CallbackId) {
        self.callback_id_set.push(id);
    }
    // pub fn call<F>(&self, args: &[T]) -> T
    // where
    //     F: Fn(&[T]) -> T + 'a,
    // {
    //     (self.compute_fn)(args)
    // }
}

struct Callback<'a, T> {
    compute_cell_id_set: Vec<ComputeCellId>,
    callback_fn: Box<dyn FnMut(T) + 'a>,
}

impl<'a, T> Callback<'a, T> {
    pub fn new<F>(compute_cell_id: ComputeCellId, callback_fn: F) -> Self
    where
        F: FnMut(T) + 'a,
    {
        Self {
            compute_cell_id_set: vec![compute_cell_id],
            callback_fn: Box::new(callback_fn),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackId(usize);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

pub struct Reactor<'a, T> {
    input_set: HashMap<InputCellId, T>,
    compute_set: HashMap<ComputeCellId, ComputeCell<'a, T>>,
    callback_set: HashMap<CallbackId, Callback<'a, T>>,
    id_counter: usize,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T> Reactor<'a, T>
where
    T: Copy + PartialEq,
{
    pub fn new() -> Self {
        Self {
            input_set: HashMap::new(),
            compute_set: HashMap::new(),
            callback_set: HashMap::new(),
            id_counter: 0usize,
        }
    }

    fn gen_id(&mut self) -> usize {
        self.id_counter += 1;
        self.id_counter
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellId {
        let input_cell = InputCellId(self.gen_id());
        self.input_set.insert(input_cell, initial);
        input_cell
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
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        dependencies
            .iter()
            .find(|&cell| match cell {
                CellId::Input(input) => self.input_set.get(input).is_none(),
                CellId::Compute(compute) => self.compute_set.get(compute).is_none(),
            })
            .map_or(
                {
                    let compute_cell_id = ComputeCellId(self.gen_id());

                    // assumes all depedencies exist for this exercise
                    let args = dependencies
                        .iter()
                        .filter_map(|&d| self.value(d))
                        .collect::<Vec<_>>();
                    let value = (compute_func)(&args);

                    let compute_cell = ComputeCell::new(value, dependencies, compute_func);
                    self.compute_set.insert(compute_cell_id, compute_cell);
                    Ok(compute_cell_id)
                },
                |&id| Err(id),
            )
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        match id {
            CellId::Input(input) => self.input_set.get(&input).copied(),
            CellId::Compute(compute) => self.cal_compute_value(&compute),
        }
    }

    fn cal_compute_value(&self, id: &ComputeCellId) -> Option<T> {
        self.compute_set.get(&id).and_then(|com| {
            let args = com
                .dependencies
                .iter()
                .filter_map(|&d| self.value(d))
                .collect::<Vec<_>>();
            return Some((com.compute_fn)(&args));
        })
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        match self.input_set.get_mut(&id) {
            Some(old_val) => {
                // if *old_val == new_value {
                //     return false;
                // }
                *old_val = new_value;

                // callbacks
                let compute_new_values = {
                    self.compute_set
                        .keys()
                        .map(|k| (k, self.cal_compute_value(k)))
                        .filter_map(|v| match v.1 {
                            Some(val) => Some((*v.0, val)),
                            None => None,
                        })
                        .collect::<Vec<_>>()
                };

                for (key, new_val) in compute_new_values.iter() {
                    let com = self.compute_set.get_mut(key).unwrap();
                    if com.value != *new_val {
                        (*com).value = *new_val;
                        for cb_id in com.callback_id_set.iter() {
                            if let Some(cb) = self.callback_set.get_mut(&cb_id) {
                                (cb.callback_fn)(*new_val)
                            }
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
    pub fn add_callback<F: FnMut(T) + 'a>(
        &mut self,
        id: ComputeCellId,
        callback: F,
    ) -> Option<CallbackId> {
        let callback_id = self.gen_id();
        let compute = self.compute_set.get_mut(&id);
        if compute.is_none() {
            return None;
        }
        let compute_cell = compute.unwrap();
        let callback_id = CallbackId(callback_id);
        let callback = Callback::new(id, callback);
        compute_cell.add_callback(callback_id);
        self.callback_set.insert(callback_id, callback);

        Some(callback_id)
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        let cb = self.callback_set.remove(&callback);
        if cb.is_none() {
            return Err(RemoveCallbackError::NonexistentCallback);
        }
        let cb = cb.unwrap();
        if !cb.compute_cell_id_set.contains(&cell) {
            println!("WTF???????????????????????????");
        }
        if self.compute_set.get(&cell).is_none() {
            return Err(RemoveCallbackError::NonexistentCell);
        }
        Ok(())
    }
}
