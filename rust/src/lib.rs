use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
    program_error::ProgramError,
};

#[derive(Debug, PartialEq)]
pub enum Instruction {
    SignUp,
    UpdateProfile,
    AddFriend,
    AcceptFriendRequest,
}

pub struct User {
    pub name: String,
    pub bio: String,
    pub profile_picture: Vec<u8>,
    pub friends: Vec<Pubkey>,
    pub friend_requests: Vec<Pubkey>,
}

impl User {
    pub fn new(name: String, bio: String, profile_picture: Vec<u8>) -> Self {
        Self {
            name,
            bio,
            profile_picture,
            friends: Vec::new(),
            friend_requests: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum NetworkingError {
    InvalidInstruction,
}

impl From<NetworkingError> for ProgramError {
    fn from(e: NetworkingError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = match instruction_data[0] {
        0 => Instruction::SignUp,
        1 => Instruction::UpdateProfile,
        2 => Instruction::AddFriend,
        3 => Instruction::AcceptFriendRequest,
        _ => return Err(NetworkingError::InvalidInstruction.into()),
    };

    match instruction {
        Instruction::SignUp => sign_up_user(accounts, &instruction_data[1..]),
        Instruction::UpdateProfile => update_profile(accounts, &instruction_data[1..]),
        Instruction::AddFriend => add_friend(accounts, &instruction_data[1..]),
        Instruction::AcceptFriendRequest => accept_friend_request(accounts, &instruction_data[1..]),
    }
}

fn sign_up_user(accounts: &[AccountInfo], registration_data: &[u8]) -> ProgramResult {
    let name_length = registration_data[0] as usize;
    let surname_length = registration_data[1] as usize;
    let age = registration_data[2];
    let email_length = registration_data[3] as usize;
    let password_length = registration_data[4] as usize;

    if name_length + surname_length + email_length + password_length != registration_data.len() - 5 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let name = String::from_utf8_lossy(&registration_data[5..5 + name_length]).to_string();
    let surname = String::from_utf8_lossy(&registration_data[5 + name_length..5 + name_length + surname_length]).to_string();
    let email = String::from_utf8_lossy(&registration_data[5 + name_length + surname_length..5 + name_length + surname_length + email_length]).to_string();
    let password = String::from_utf8_lossy(&registration_data[5 + name_length + surname_length + email_length..]).to_string();

    let user_account_info = &accounts[0];
    let mut user_data = user_account_info.try_borrow_mut_data()?;
    let mut cursor = 0;

    user_data[cursor..cursor + name_length].copy_from_slice(name.as_bytes());
    cursor += name_length;
    user_data[cursor..cursor + surname_length].copy_from_slice(surname.as_bytes());
    cursor += surname_length;
    user_data[cursor] = age;
    cursor += 1;
    user_data[cursor..cursor + email_length].copy_from_slice(email.as_bytes());
    cursor += email_length;
    user_data[cursor..cursor + password_length].copy_from_slice(password.as_bytes());


    Ok(())
}

fn update_profile(accounts: &[AccountInfo]) -> ProgramResult {

    Ok(())
}

fn add_friend(accounts: &[AccountInfo]) -> ProgramResult {

    Ok(())
}

fn accept_friend_request(accounts: &[AccountInfo]) -> ProgramResult {

    Ok(())
}
