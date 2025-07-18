use std::marker::PhantomData;

use rand::Rng;

pub struct Printer3D<S> {
    _marker: PhantomData<S>
}

/* States */

/// The 3D printer encountered an error and needs resetting
pub enum ErrorState{}
/// The 3D printer is waiting for a job
pub enum IdleState{}
/// The 3D printer is currently printing
pub enum PrintingState{}
/// The 3D printed product is ready
pub enum ProductReadyState {}

/// Check if we're out of filament
fn out_of_filament() -> bool {
    let rand: usize = rand::thread_rng().gen_range(0..100);
    rand > 95
}