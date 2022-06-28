use std::sync::atomic::{AtomicUsize, Ordering};

use serde::{Serialize, Deserialize};

use crate::pronouns::Pronouns;

// generate a new sequential player ID. Normally
// this would be randomized for better security, but it
// doesn't really matter in this case
fn new_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);

    COUNTER.fetch_add(1, Ordering::Relaxed)
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tribute {
    pub name: String,
    pub avatar_url: String,
    pub pronouns: Pronouns,
    #[serde(skip)]
    pub kills: u32,
    #[serde(skip)]
    #[serde(default = "default_true")]
    pub is_alive: bool,
    //#[serde(skip)]
    //#[serde(default = "def_cache_img")]
    //pub avatar: CachedImage,
    #[serde(skip)]
    #[serde(default = "new_id")]
    id: usize
}

impl Tribute {
    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn kill(&mut self) {
        self.is_alive = false;
    }

    pub fn add_kill(&mut self) {
        self.kills += 1;
    }

    /*pub async fn update_avatar(&self) -> Result<Tribute, Error> {
        let mut tribute = self.clone();
        tribute.avatar = CachedImage::new(self.avatar_url.clone());
        
        match tribute.avatar.fetch_value_async() {
            Ok(handle) => {
                tribute.avatar.value = Option::Some(handle);
                return Ok(tribute)
            },
            Err(e) => return Err(e)
        }
    }*/
}