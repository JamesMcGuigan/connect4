use contracts::requires;
pub type PlayerID = u8;
pub struct Players;

impl Players {
    const MAX_PLAYERS: u8 = 2;

    /// Returns the next player in the sequence
    /// ```
    /// use connectx::games::players::Players;
    /// assert_eq!(Players::next(1), 2);
    /// assert_eq!(Players::next(2), 1);
    /// ```
    #[requires(Self::is_valid(player_id))]
    #[ensures(Self::is_valid(ret))]
    pub fn next(player_id: PlayerID) -> PlayerID {
        (player_id % Self::MAX_PLAYERS) + 1
    }

    /// Returns the next player in the sequence
    /// ```
    /// use connectx::games::players::Players;
    /// assert_eq!(Players::is_valid(0), false);
    /// assert_eq!(Players::is_valid(1), true);
    /// assert_eq!(Players::is_valid(2), true);
    /// assert_eq!(Players::is_valid(3), false);
    /// ```
    pub fn is_valid(player_id: PlayerID) -> bool {
        (1..=Self::MAX_PLAYERS).contains(&player_id)
    }

    /// Returns a vector of all players
    /// ```
    /// use connectx::games::players::Players;
    /// assert_eq!(Players::all(), vec![1, 2]);
    /// ```
    pub fn all() -> Vec<PlayerID> {
        (1..=Self::MAX_PLAYERS).collect()
    }
}
