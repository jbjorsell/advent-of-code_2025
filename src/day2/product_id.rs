pub struct ProductId {
    pub id: u64,
}
impl ProductId {
    pub fn new(id: u64) -> Self {
        Self { id }
    }

    fn repeated_sequence(&self, only_repeat_twice: bool) -> bool {
        let id_str = self.id.to_string();
        let str_length = id_str.len();

        for repeated_seq_size in 1..=str_length / 2 {
            let indivisible = str_length % repeated_seq_size != 0;
            let is_half_length = str_length / repeated_seq_size == 2;

            if indivisible || (only_repeat_twice && !is_half_length) {
                continue;
            }

            let mut seqs = id_str.as_bytes().chunks(repeated_seq_size);

            let first = seqs
                .next()
                .expect("The first subsequence of the product id does not exist");
            if seqs.all(|s| *s == *first) {
                tracing::debug!(
                    "Product ID {} has repeated sequence of size {}",
                    self.id,
                    repeated_seq_size
                );
                return true;
            }
        }
        false
    }

    pub fn is_valid(&self, only_repeat_twice: bool) -> bool {
        !self.repeated_sequence(only_repeat_twice)
    }
}

pub struct ProductIdSequence {
    pub ids: Vec<ProductId>,
}

impl ProductIdSequence {
    pub fn new(start_id: u64, end_id: u64) -> Self {
        let ids = (start_id..end_id)
            .into_iter()
            .map(|id| ProductId::new(id))
            .collect();

        Self { ids }
    }
}
