use rand::Rng;

// Define a trait to generate a random enum variant

pub trait RandomEnum: Copy
where
    Self: 'static,
{
    fn random() -> Self {
        let variants = Self::variants();
        let index = rand::thread_rng().gen_range(0..variants.len());
        variants[index]
    }

    fn variants() -> &'static [Self];
}
