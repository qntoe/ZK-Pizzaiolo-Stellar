fn main(
    accuracies: [u32; 5], 
    placements: [u32; 5], 
    expected_score: pub u32
) {
    let mut current_score: u32 = 0;

    for i in 0..5 {
        // Simple logic: 100 points max per hit, minus the inaccuracy
        // We use safety checks to avoid underflow
        if accuracies[i] < 100 {
            current_score += 100 - accuracies[i];
        }
        
        if placements[i] < 100 {
            current_score += 100 - placements[i];
        }
    }

    assert(current_score == expected_score);
}

#[test]
fn test_perfect_score() {
    let accuracies = [0, 0, 0, 0, 0];
    let placements = [0, 0, 0, 0, 0];
    let expected_score = 1000; // (100*5) + (100*5)
    main(accuracies, placements, expected_score);
}

#[test]
fn test_normal_score() {
    let accuracies = [10, 20, 0, 5, 15]; // Sum: 50. Pts: 500 - 50 = 450
    let placements = [5, 5, 5, 5, 5];    // Sum: 25. Pts: 500 - 25 = 475
    let expected_score = 925; 
    main(accuracies, placements, expected_score);
}
