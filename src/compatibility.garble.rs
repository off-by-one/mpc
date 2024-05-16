enum ProximityPolicy {
    Allow(u8),
    Disallow,
}

enum OrderPolicy {
    Allow,
    Disallow,
}

struct PrivacyPolicy {
    proximity: ProximityPolicy,
    order: OrderPolicy,
}

enum Ranking {
    Rank(i8, PrivacyPolicy),
    Unranked,
}

enum OrderResult {
    GreaterThan,
    LessThan,
    Equal,
    Unknown,
}

enum ProximityResult {
    Close,
    Unknown,
}

struct Result {
    order: OrderResult,
    proximity: ProximityResult,
}

pub fn min(x: u8, y: u8) -> u8 {
    if x < y {
        x
    } else {
        y
    }
}

pub fn main(a: Ranking, b: Ranking) -> Result {
    // Constant, since garble doesn't allow global values
    let UNKNOWN_RESULT = Result {
        order: OrderResult::Unknown,
        proximity: ProximityResult::Unknown,
    };

    // Apply ranking policy, returning unranked results
    match (a, b) {
        (Ranking::Rank(a_score, a_policy), Ranking::Rank(b_score, b_policy)) => {
            // Calculate order result
            let (l, s, order) = if a_score > b_score {
                (a_score, b_score, OrderResult::GreaterThan)
            } else if b_score > a_score {
                (b_score, a_score, OrderResult::LessThan)
            } else {
                (a_score, b_score, OrderResult::Equal)
            };

            // Apply order privacy policies and equality case
            let order = match (order, a_policy.order, b_policy.order) {
                (_, OrderPolicy::Allow, OrderPolicy::Allow) => order,
                (OrderResult::Equal, _, _) => order,
                _ => OrderResult::Unknown,
            };

            // Apply proximity privacy policy
            let proximity = match ((a_policy.proximity, b_policy.proximity)) {
                (ProximityPolicy::Disallow, _) => ProximityResult::Unknown,
                (_, ProximityPolicy::Disallow) => ProximityResult::Unknown,
                // Calculate proximity result
                (ProximityPolicy::Allow(a_proximity), ProximityPolicy::Allow(b_proximity)) => {
                    // `as u8` is valid because l >= s
                    if (l - s) as u8 <= min(a_proximity, b_proximity) {
                        ProximityResult::Close
                    } else {
                        ProximityResult::Unknown
                    }
                }
            };

            // Apply equality case
            let proximity = match order {
                OrderResult::Equal => ProximityResult::Close,
                _ => proximity,
            };

            Result { order, proximity }
        }
        _ => UNKNOWN_RESULT,
    }
}
