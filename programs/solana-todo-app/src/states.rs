use::anchor_lang::prelude::*;
pub const MAX_CONTENT_LEN:usize = 200;
#[account]
pub struct UserProfile {
    pub authority: Pubkey,
    pub todo_count: u8,
    pub last_count:u8
}

#[account]
pub struct TodoAccount {
    pub authority:Pubkey,
    pub idx: u8,
    pub content: String,
    pub marked: bool
}

impl UserProfile{
    pub const LEN:usize = 8 + 32 + 1 + 1;

}
impl TodoAccount {
    pub const LEN: usize = 32 + 1 + 1+ 4+ MAX_CONTENT_LEN; 
}