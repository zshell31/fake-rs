use crate::{Dummy, Fake, Faker};
use rand::Rng;
use uuid::Uuid;

impl Dummy<Faker> for Uuid {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        Uuid::new_v4()
    }
}
