#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, String, Symbol, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct Donor {
    pub name: String,
    pub blood_group: String,
    pub donation_count: u64,
    pub reward_points: u64,
}

#[contracttype]
pub enum DonorBook {
    DonorById(u64),
}

const DONOR_COUNT: Symbol = symbol_short!("DONOR_C");

#[contract]
pub struct BloodDonationContract;

#[contractimpl]
impl BloodDonationContract {
    // Register a new donor
    pub fn register_donor(env: Env, name: String, blood_group: String) -> u64 {
        let mut donor_count: u64 = env.storage().instance().get(&DONOR_COUNT).unwrap_or(0);
        donor_count += 1;

        let donor = Donor {
            name,
            blood_group,
            donation_count: 0,
            reward_points: 0,
        };

        env.storage().instance().set(&DonorBook::DonorById(donor_count), &donor);
        env.storage().instance().set(&DONOR_COUNT, &donor_count);
        donor_count
    }

    // Add a donation entry and reward points
    pub fn donate_blood(env: Env, donor_id: u64) {
        let mut donor = env
            .storage()
            .instance()
            .get(&DonorBook::DonorById(donor_id))
            .expect("Donor not found");

        donor.donation_count += 1;
        donor.reward_points += 100;

        env.storage().instance().set(&DonorBook::DonorById(donor_id), &donor);
    }

    // View donor details
    pub fn view_donor(env: Env, donor_id: u64) -> Donor {
        env.storage()
            .instance()
            .get(&DonorBook::DonorById(donor_id))
            .unwrap_or(Donor {
                name: String::from_str(&env, "Not Found"),
                blood_group: String::from_str(&env, "Unknown"),
                donation_count: 0,
                reward_points: 0,
            })
    }
}
