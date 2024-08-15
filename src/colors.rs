use std::collections::VecDeque;

use bevy::prelude::*;

/// An easy way to retrieve player colors
#[derive(Clone, Debug, PartialEq)]
pub struct PlayerColors {
    colors: VecDeque<Color>,
}

impl PlayerColors {
    pub fn new() -> Self {
        Self {
            colors: VecDeque::from(vec![
                Color::srgb(1.0, 0.0, 0.0),
                Color::srgb(0.0, 1.0, 0.0),
                Color::srgb(0.0, 0.0, 1.0),
                Color::srgb(1.0, 1.0, 0.0),
            ]),
        }
    }

    /// The easiest way to get a stable player color via a wrapping index, while not actually
    /// storing a [`PlayerColors`] struct anywhere.
    pub fn index(index: usize) -> Color {
        let player_colors = PlayerColors::new();
        player_colors.colors[index % player_colors.colors.len()]
    }

    /// Get a stable player color via wrapping index.
    pub fn get(&self, index: usize) -> Color {
        self.colors[index % self.colors.len()]
    }

    /// Remove and return a player color from the back of the list
    pub fn pop_back(&mut self) -> Option<Color> {
        self.colors.pop_back()
    }

    /// Remove and return a player color from the front of the list
    pub fn pop_front(&mut self) -> Option<Color> {
        self.colors.pop_front()
    }
}

impl Default for PlayerColors {
    fn default() -> Self {
        PlayerColors::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn index_wraps_correctly() {
        assert_eq!(
            PlayerColors::index(0),
            PlayerColors::index(4) // will break when we change the number of colors
        );
        assert_eq!(
            PlayerColors::index(1),
            PlayerColors::index(5) // will break when we change the number of colors
        );
    }

    #[test]
    fn get_wraps_correctly() {
        let player_colors = PlayerColors::new();
        assert_eq!(
            player_colors.get(0),
            player_colors.get(player_colors.colors.len())
        );
        assert_eq!(
            player_colors.get(1),
            player_colors.get(player_colors.colors.len() + 1)
        );
    }

    #[test]
    fn popping_works() {
        let mut player_colors = PlayerColors::new();
        let first = player_colors.get(0);
        let last = player_colors.get(player_colors.colors.len() - 1);
        assert_eq!(player_colors.pop_front(), Some(first));
        assert_eq!(player_colors.pop_back(), Some(last));
    }
}
