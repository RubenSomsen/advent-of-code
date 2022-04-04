use std::collections::HashMap;

pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let (products, mut ingredients) = parse_file(file_location);
    
    use std::time::Instant;
    let now = Instant::now();

    let a = answer_a(&products, &mut ingredients);
    let b = answer_b(&mut ingredients); // lazy answer, allergens<ingredient>

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {:?}", a);
    println!("ANSWER B: {:?}", b);
}

// Could be faster if non-allergen products were removed
fn answer_b(ingredients: &mut Vec<(usize, Vec<bool>)>) -> Vec<usize> {
    let all_len = ingredients[0].1.len();
    let mut checked_vector = vec![99; all_len];
    // Remove unique allergens from all other ingredients
    let mut i_all = 0;
    while i_all < all_len { // loop until each allergen is found
        for i_i in 0..ingredients.len() { // loop through each ingredient
            let ingredient = &ingredients[i_i];
            let mut allergen_count: u8 = 0;
            let mut allergen = 0;
            for i_a in 0..all_len { // find an allergen with 1 true value
                if checked_vector[i_a] != 99 {  continue }
                let is_allergen = ingredient.1[i_a];
                if is_allergen {
                    allergen_count += 1;
                    allergen = i_a;
                }
            }
            if allergen_count == 1 {
                checked_vector[allergen] = i_i;
                i_all += 1;
                for i_ii in 0..ingredients.len() { // loop through each ingredient
                    if i_ii == i_i { continue };
                    let allergen_vector = &mut ingredients[i_ii].1;
                    allergen_vector[allergen] = false;
                }
            }
        }
    }
    return checked_vector
}

fn answer_a(products: &Vec<(Vec<usize>, Vec<usize>)>, ingredients: &mut Vec<(usize, Vec<bool>)>) -> usize {
    let mut count = 0;
    // Remove all allergens that an ingredient cannot contain
    for i_p in 0..products.len() {
        let product = &products[i_p];
        for i_i in 0..ingredients.len() {
            let ingredient = &mut ingredients[i_i];
            if !product.0.contains(&i_i) {
                for allergen in &product.1 {
                    ingredient.1[*allergen] = false;
                }
            }
        }
    }
    // Count ingredients without allergens (incl. doubles)
    for i in 0..ingredients.len() {
        let ingredient = &ingredients[i];
        let mut has_allergen = false;
        for allergen in &ingredient.1 {
            has_allergen |= allergen;
        }
        if !has_allergen {
            count += 1 + ingredient.0;
        }
    }
    return count
}

                                // products<(ingredients, allergens)>, ingredients<(occurences, has_allergen)>
fn parse_file(file_location: &str) -> (Vec<(Vec<usize>, Vec<usize>)>, Vec<(usize, Vec<bool>)>) {
    use std::fs;
    let file = fs::read_to_string(file_location).expect("Unable to read file");

    let (mut indexmap, mut ing_len, mut all_len): (HashMap<&str, usize>, usize, usize) = (HashMap::new(), 0, 0);
    let mut products = vec![];
    let mut ingredients = vec![];
    for line in file.lines() {
        let mut tuple = (vec![], vec![]);
        let line = &line[0..line.len()-1];
        let mut line_split = line.split(" (contains ");
        let ingredient_line = line_split.next().unwrap().split(" ");
        let allergen_line = line_split.next().unwrap().split(", ");
        for ingredient in ingredient_line { // assign a unique uint to each, and count instances
            let index_check = indexmap.get(ingredient);
            if index_check == None { 
                indexmap.insert(ingredient, ing_len);
                ingredients.push((0, vec![]));
                tuple.0.push(ing_len); 
                ing_len += 1; }
            else { 
                let index = *index_check.unwrap();
                tuple.0.push(index); 
                ingredients[index].0 += 1;
            } 
        }
        for allergen in allergen_line {
            let index_check = indexmap.get_mut(allergen);
            if index_check == None { 
                indexmap.insert(allergen, all_len);
                tuple.1.push(all_len); 
                all_len += 1; }
            else { 
                let index = *index_check.unwrap();
                tuple.1.push(index);
            } 
        }
        products.push(tuple);
    }
    for i in 0..ing_len { ingredients[i].1 = vec![true; all_len]; }
    return (products, ingredients)
}