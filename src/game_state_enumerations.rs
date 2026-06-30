#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CardLocation{
    P1Hand,
    P2Hand,
    P1Deck,
    P2Deck,
    P1Bench,
    P2Bench,
    P1Active,
    P2Active,
    P1Discard,
    P2Discard,
    P1Prizes,
    P2Prizes,
    Stadium,
    P1LostZone,
    P2LostZone,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CardVisibleState{
    Hidden,
    P1Known,
    P2Known,
    BothKnown,
}


