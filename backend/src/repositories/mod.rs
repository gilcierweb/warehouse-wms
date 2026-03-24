pub mod base;
pub mod base_repository;
pub mod macros;
pub mod container;

pub mod users_repository;
pub mod profiles_repository;
pub mod movements_repository;
pub mod slots_repository;
pub mod alert_configs_repository;

// Re-export key types for convenient access
pub use container::AppContainer;
pub use users_repository::IUserRepository;
pub use profiles_repository::IProfileRepository;
pub use movements_repository::IMovementRepository;
pub use slots_repository::ISlotRepository;
pub use alert_configs_repository::IAlertConfigRepository;

// ── Macro-generated CRUD implementations on BaseRepo ─────────
// These must be here (after `mod macros` is compiled) so the
// `impl_crud!` macro and all schema/model types are in scope.

use crate::impl_crud;
use crate::db::schema::{users, profiles, movements, slots, alert_configs};
use crate::models::user::{User, NewUser};
use crate::models::profile::{Profile, NewProfile};
use crate::models::movement::{Movement, NewMovement};
use crate::models::slot::{Slot, NewSlot};
use crate::models::alert_config::{AlertConfig, NewAlertConfig};

impl_crud!(IUserRepository, User, NewUser, users::table);
impl_crud!(IProfileRepository, Profile, NewProfile, profiles::table);
impl_crud!(IMovementRepository, Movement, NewMovement, movements::table);
impl_crud!(ISlotRepository, Slot, NewSlot, slots::table);
impl_crud!(IAlertConfigRepository, AlertConfig, NewAlertConfig, alert_configs::table);