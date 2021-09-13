// Any tests connecting to a server may fail in the future,
// either due to intermittent internet connection to the IP(s),
// or from the IP space being re-allocated to non-SRCDS endpoints.

#[cfg(not(feature = "async"))]
#[test]
fn test_info() {
    let client = a2s::A2SClient::new().unwrap();

    let result = client.info("play.maxdb.net:27015").unwrap();

    println!("{:?}", result);
}

/// As of 2020-12-07 servers may opt-in to a **S2C_CHALLENGE** response for the
/// **A2S_INFO** request.
///
/// So this test covers specifically cases where we expect an S2C response.
///
/// See Announcement: 
/// https://steamcommunity.com/discussions/forum/14/2974028351344359625/
#[cfg(not(feature = "async"))]
#[test]
fn test_info_challenge() {
    let valve_lax2_l4d2 = "162.254.195.121:27083"; // srcds1157-lax2.115.69
    let client = a2s::A2SClient::new().unwrap();
    let result = client.info(valve_lax2_l4d2).unwrap();
    println!("{:?}", result);
}