fn main(
    initial_dough: u32,
    card_costs: [u32; 4],
    played_indices: [u32; 4],
    final_dough: pub u32
) {
    let mut current_dough = initial_dough;
    
    for i in 0..4 {
        let cost = card_costs[played_indices[i]];
        assert(current_dough >= cost);
        current_dough -= cost;
    }

    assert(current_dough == final_dough);
}

#[test]
fn test_valid_match_sequence() {
    let card_costs = [3, 2, 4, 3]; // Pep, Mush, Chilli, Pine
    let played_indices = [0, 1, 0, 1]; // Play: Pep, Mush, Pep, Mush (3+2+3+2 = 10)
    main(10, card_costs, played_indices, 0);
}
