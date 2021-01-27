use crate::testutil::*;

#[quickcheck]
fn all_games_end(r: Run) -> bool {
    let end = r.turns.iter().fold(
        Some(Pass(quarto::new_game())), 
        |game, &turn| game.and_then(|g| play(g, turn))
    );
    
    match end {
        Some(Final(_)) => true,
        _ => false,
    }
}