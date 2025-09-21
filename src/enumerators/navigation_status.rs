#[allow(dead_code)]
enum NavigationStatus {
    UnderwayUsingEngine,
    AtAnchor,
    NotUnderCommand,
    RestrictedManoeuvrability,
    ConstrainedByDraught,
    Moored,
    Aground,
    EngagedInFishing,
    UnderwaySailing,
    ReservedForHSC,
    ReservedForWIG,
    TowingAstern,
    PushingAheadOrTowingAlongSide,
    Reserved,
    AISSARTIsActive,
    Undefined
}

#[allow(dead_code)]
fn get_navigation_status(code: u8) -> NavigationStatus {
    match code {
        0 => NavigationStatus::UnderwayUsingEngine,
        1 => NavigationStatus::AtAnchor,
        2 => NavigationStatus::NotUnderCommand,
        3 => NavigationStatus::RestrictedManoeuvrability,
        4 => NavigationStatus::ConstrainedByDraught,
        5 => NavigationStatus::Moored,
        6 => NavigationStatus::Aground,
        7 => NavigationStatus::EngagedInFishing,
        8 => NavigationStatus::UnderwaySailing,
        9 => NavigationStatus::ReservedForHSC,
        10 => NavigationStatus::ReservedForWIG,
        11 => NavigationStatus::TowingAstern,
        12 => NavigationStatus::PushingAheadOrTowingAlongSide,
        13 => NavigationStatus::Reserved,
        14 => NavigationStatus::AISSARTIsActive,
        _ => NavigationStatus::Undefined,
    }
}