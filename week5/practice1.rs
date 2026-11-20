fn passing_scores(scores: &[i32], pass_mark: i32) -> Vec<i32> {
    scores.iter().copied()
        .filter(|score| *score >= pass_mark)
        .collect()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::passing_scores;

    #[test]
    fn keeps_scores_above_pass_mark() {
        let scores = vec![30, 70, 90];

        let result = passing_scores(&scores, 50);

        assert_eq!(result, vec![70, 90]);
    }

    #[test]
    fn includes_score_equal_to_pass_mark() {
        // Arrange
        let scores = vec![40, 50, 60];

        // Act
        let result = passing_scores(&scores, 50);
        assert_eq!(result,vec![50,60]);
    }
    #[test]
    fn returns_empty_for_empty_input() {
        // Arrange
        let scores: Vec<i32> = vec![];

        // Act
        let result = passing_scores(&scores, 50);

        assert_eq!(result,vec![]);
    }
    #[test]
    fn preserves_order_duplicates_and_original_input(){
        let scores = vec![80,40,60,80];
        let result = passing_scores(&scores,50);
        assert_eq!(result,vec![80,60,80]);
        assert_eq!(scores, vec![80, 40, 60, 80]);
    }
}