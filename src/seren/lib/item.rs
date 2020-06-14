use serde::{Deserialize, Serialize};

pub type Item = u8;

pub struct ItemStack(Item, u64);

#[derive(Serialize, Deserialize)]
pub struct SaveItemStack(Item, u64);
