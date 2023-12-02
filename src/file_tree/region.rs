use serde::{Serialize, Deserialize};

use crate::error::*;

pub trait Identifiable {
    fn get_id(&self) -> usize;
    fn set_id(&mut self, id: usize);
}

#[derive(Serialize, Deserialize, Clone, Debug)]
enum SlotData<T> {
    Data(T),
    NextFree(Option<usize>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Region<T> {
    slots: Vec<SlotData<T>>,
    first_free_slot: Option<usize>,
    last_free_slot: Option<usize>,
}

impl<T> Region<T> where T : Identifiable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            slots: Vec::with_capacity(capacity),
            ..Self::default()
        }
    }

    pub fn alloc_data(&mut self, mut data: T) -> Result<usize> {
        if let Some(free) = self.pop_first_free_slot() {
            data.set_id(free);
            self.slots[free] = SlotData::Data(data);
            Ok(free)
        } else {
            let id = self.slots.len();
            data.set_id(id);
            self.slots.push(SlotData::Data(data));
            Ok(id)
        }
    }

    pub fn free_slot(&mut self, index: usize) -> Result<()> {
        let slot = self.slots.get(index);
        if let Some(SlotData::Data(_)) = slot {
            self.slots[index] = SlotData::NextFree(self.first_free_slot.take());
            self.first_free_slot = Some(index);

            if self.last_free_slot.is_none() {
                self.last_free_slot = Some(index);
            }

            Ok(())
        } else if let Some(SlotData::NextFree(_)) = slot {
            Err(Error::new(ErrorKind::FreeEmptySlot))
        } else {
            Err(Error::new(ErrorKind::FreeInvalidSlot))
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if let Some(SlotData::Data(data)) = self.slots.get(index) {
            Some(data)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if let Some(SlotData::Data(data)) = self.slots.get_mut(index) {
            Some(data)
        } else {
            None
        }
    }

    fn pop_first_free_slot(&mut self) -> Option<usize> {
        let first_free = self.first_free_slot.take();

        if let Some(index) = first_free {
            if let SlotData::NextFree(next_free) = self.slots[index] {
                self.first_free_slot = next_free;
            }

            if self.first_free_slot.is_none() {
                self.last_free_slot = None;
            }
        } 

        first_free
    }
}

impl<T> Default for Region<T> {
    fn default() -> Self {
        Self {
            slots: Vec::new(),
            first_free_slot: None,
            last_free_slot: None,
        }
    }
}
