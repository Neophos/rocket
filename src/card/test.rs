#[cfg(test)]
#[allow(non_snake_case)]

#[test]
fn shouldWinIfAttackingWithHigherPower() {
    use super::*;

    let atk = Card {
        power: 2
    };
    let def = Card {
        power: 1
    };

    assert_eq!(atk, atk.battle(&def));
}