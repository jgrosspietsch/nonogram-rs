use hashbrown::HashMap;

const MIN_DIMENSION: usize = 3;
const MAX_DIMENSION: usize = 20;

#[derive(Eq, PartialEq, Hash)]
struct CombinationKey {
    spaces: usize,
    parts: usize,
}

pub struct PossibilitiesTable {
    map: HashMap<CombinationKey, usize>,
    min_dimension: usize,
    max_dimension: usize,
}

impl PossibilitiesTable {
    pub fn num_possible(&self, size: usize, clue: &[usize]) -> Option<usize> {
        self.map.get(&CombinationKey {
            spaces: size - (clue.iter().sum::<usize>() + clue.len() - 1usize),
            parts: clue.len() + 1,
        }).cloned()
    }
}

lazy_static! {
    pub static ref SEGMENT_POSSIBILITIES: PossibilitiesTable  = {
        let mut map : HashMap<CombinationKey, usize> = HashMap::new();

        

        PossibilitiesTable {
            map,
            min_dimension: MIN_DIMENSION,
            max_dimension: MAX_DIMENSION,
        }
    };
}
// https://math.stackexchange.com/questions/1462099/number-of-possible-combinations-of-x-numbers-that-sum-to-y
