#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, contracterror,
    symbol_short, Address, Env, BytesN, panic_with_error,
};

#[derive(Clone)]
#[contracttype]
pub struct Attendance {
    pub student: Address,
    pub date: BytesN<32>,
    pub present: bool,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Record(Address, BytesN<32>), // student + date
}

// ✅ Proper Soroban error handling
#[contracterror]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    AlreadyMarked = 1,
    NotFound = 2,
}

#[contract]
pub struct AttendPayContract;

#[contractimpl]
impl AttendPayContract {

    // 📌 Mark attendance
    pub fn mark_attendance(env: Env, student: Address, date: BytesN<32>) {
        let key = DataKey::Record(student.clone(), date.clone());

        // Prevent duplicate attendance
        if env.storage().instance().has(&key) {
            panic_with_error!(&env, Error::AlreadyMarked);
        }

        let record = Attendance {
            student: student.clone(),
            date: date.clone(),
            present: true,
        };

        env.storage().instance().set(&key, &record);

        // Emit event
        env.events().publish(
            (symbol_short!("ATTEND"),),
            (student, date),
        );
    }

    // 💰 Reward student (event-based simulation)
    pub fn reward_student(env: Env, student: Address) {
        env.events().publish(
            (symbol_short!("REWARD"),),
            student,
        );
    }

    // ✅ Check attendance
    pub fn check_attendance(env: Env, student: Address, date: BytesN<32>) -> bool {
        let key = DataKey::Record(student, date);

        env.storage().instance().has(&key)
    }
}