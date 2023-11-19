/// Combine chars sequences

pub struct Combination {}

impl Combination {
    pub fn combine<T>(elements_list: Vec<(Vec<T>, usize, usize)>) -> Vec<Vec<T>>
    where
        T: Clone + Copy,
    {
        let mut combinations_list = Vec::new();
        for (elements, min_length, max_length) in elements_list {
            let mut combinations = Vec::new();
            Self::combine_elements(
                min_length,
                max_length,
                elements,
                Vec::new(),
                &mut combinations,
            );
            combinations_list.push(combinations);
        }
        Self::concat_combinations(combinations_list)
    }

    fn combine_elements<T>(
        min_length: usize,
        max_length: usize,
        elements: Vec<T>,
        combination: Vec<T>,
        combinations: &mut Vec<Vec<T>>,
    ) where
        T: Clone + Copy,
    {
        if combination.len() >= min_length {
            if combination.len() >= max_length {
                combinations.push(combination);
                return;
            }
            combinations.push(combination.clone());
        }
        if elements.is_empty() {
            return;
        }

        for i in 0..elements.len() {
            let mut combination = combination.clone();
            combination.push(elements[i]);
            let mut elements: Vec<T> = elements.clone();
            elements.remove(i);
            Self::combine_elements(min_length, max_length, elements, combination, combinations);
        }
    }

    fn concat_combinations<T>(combinations_list: Vec<Vec<Vec<T>>>) -> Vec<Vec<T>>
    where
        T: Clone + Copy,
    {
        let mut result = vec![Vec::new()];
        for combinations in combinations_list {
            let mut concat = Vec::new();
            for combination in combinations {
                for c in result.iter() {
                    concat.push([&c[..], &combination[..]].concat());
                }
            }
            result = concat;
        }

        result
    }
}