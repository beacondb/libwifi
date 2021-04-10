use libwifi::{
    frame_types::{FrameSubType, FrameType},
    variants::Frame,
    Frame,
};

#[test]
fn test_beacon() {
    let payload = [
        // Header
        128, 0, // FrameControl
        0, 0, // Duration id
        255, 255, 255, 255, 255, 255, // First address
        248, 50, 228, 173, 71, 184, // Second address
        248, 50, 228, 173, 71, 184, // Third address
        96, 119, // SequencControl
        // Data start
        151, 161, 39, 206, 165, 0, 0, 0, // timestamp
        100, 0, // interval
        17, 4, // capability
        0, 15, 77, 121, 32, 102, 97, 99, 101, 32, 119, 104, 101, 110, 32, 73, 80, // SSID
        1, 8, 130, 132, 139, 150, 36, 48, 72, 108, // Supported rates
        3, 1, 9, 5, 4, 0, 3, 1, 0, 42, 1, 4, 47, 1, 4, 48, 20, 1, 0, 0, 15, 172, 4, 1, 0, 0, 15,
        172, 4, 1, 0, 0, 15, 172, 2, 12, 0, 50, 4, 12, 18, 24, 96, 45, 26, 189, 25, 23, 255, 255,
        255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 61, 22, 9, 8, 4, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 74, 14, 20, 0, 10, 0, 44, 1, 200, 0, 20, 0,
        5, 0, 25, 0, 127, 8, 1, 0, 0, 0, 0, 0, 0, 64, 221, 49, 0, 80, 242, 4, 16, 74, 0, 1, 16, 16,
        68, 0, 1, 2, 16, 71, 0, 16, 190, 15, 245, 213, 137, 177, 64, 140, 203, 243, 77, 29, 90,
        130, 118, 247, 16, 60, 0, 1, 3, 16, 73, 0, 6, 0, 55, 42, 0, 1, 32, 221, 9, 0, 16, 24, 2, 5,
        0, 28, 0, 0, 221, 24, 0, 80, 242, 2, 1, 1, 132, 0, 3, 164, 0, 0, 39, 164, 0, 0, 66, 67, 94,
        0, 98, 50, 47, 0,
    ];

    let frame = Frame::parse(&payload).expect("Payload should be valid");
    println!("{:?}", frame);
    assert!(matches!(frame.control.frame_type, FrameType::Management));
    assert!(matches!(frame.control.frame_subtype, FrameSubType::Beacon));

    if let Frame::Beacon(beacon) = frame.payload {
        assert_eq!("My face when IP", beacon.station_info.ssid.unwrap());
    }
}

#[test]
fn test_probe_request() {
    let payload = [
        // Header
        64, 0, // FrameControl
        0, 0, // Duration id
        255, 255, 255, 255, 255, 255, // First address
        192, 238, 251, 75, 207, 58, // Second address
        255, 255, 255, 255, 255, 255, // Thrid address
        48, 89, // Sequence Control
        // Data start
        0, 0, 1, 4, 2, 4, 11, 22, 50, 8, 12, 18, 24, 36, 48, 72, 96, 108, 221, 7, 0, 80, 242, 8, 0,
        36, 0, 3, 1, 9, 45, 26, 111, 1, 31, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 191, 12, 178, 97, 128, 51, 254, 255, 134, 1, 254, 255, 134, 1,
    ];

    let frame = Frame::parse(&payload).expect("Payload should be valid");
    println!("{:?}", frame);
    assert!(matches!(frame.control.frame_type, FrameType::Management));
    assert!(matches!(
        frame.control.frame_subtype,
        FrameSubType::ProbeRequest
    ));
}

#[test]
fn test_probe_response() {
    let payload = [
        // Header
        80, 0, // FrameControl
        58, 1, // Duration id
        192, 238, 251, 75, 207, 58, // First address
        248, 50, 228, 173, 71, 184, // Second address
        248, 50, 228, 173, 71, 184, // Third address
        144, 1, // SequenceControl
        129, 106, 187, 25, 166, 0, 0, 0, // Timesetamp
        100, 0, //  beacon interval
        17, 4, // capability info
        0, 15, 77, 121, 32, 102, 97, 99, 101, 32, 119, 104, 101, 110, 32, 73, 80, // SSID
        1, 8, 130, 132, 139, 150, 36, 48, 72, 108, 3, 1, 9, 42, 1, 4, 47, 1, 4, 48, 20, 1, 0, 0,
        15, 172, 4, 1, 0, 0, 15, 172, 4, 1, 0, 0, 15, 172, 2, 12, 0, 50, 4, 12, 18, 24, 96, 45, 26,
        189, 25, 23, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 61,
        22, 9, 8, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 74, 14, 20, 0, 10, 0,
        44, 1, 200, 0, 20, 0, 5, 0, 25, 0, 127, 8, 1, 0, 0, 0, 0, 0, 0, 64, 221, 172, 0, 80, 242,
        4, 16, 74, 0, 1, 16, 16, 68, 0, 1, 2, 16, 59, 0, 1, 3, 16, 71, 0, 16, 190, 15, 245, 213,
        137, 177, 64, 140, 203, 243, 77, 29, 90, 130, 118, 247, 16, 33, 0, 21, 65, 83, 85, 83, 84,
        101, 75, 32, 67, 111, 109, 112, 117, 116, 101, 114, 32, 73, 110, 99, 46, 16, 35, 0, 28, 87,
        105, 45, 70, 105, 32, 80, 114, 111, 116, 101, 99, 116, 101, 100, 32, 83, 101, 116, 117,
        112, 32, 82, 111, 117, 116, 101, 114, 16, 36, 0, 7, 82, 84, 45, 78, 54, 54, 85, 16, 66, 0,
        17, 102, 56, 58, 51, 50, 58, 101, 52, 58, 97, 100, 58, 52, 55, 58, 98, 56, 16, 84, 0, 8, 0,
        6, 0, 80, 242, 4, 0, 1, 16, 17, 0, 7, 82, 84, 45, 78, 54, 54, 85, 16, 8, 0, 2, 32, 8, 16,
        60, 0, 1, 3, 16, 73, 0, 6, 0, 55, 42, 0, 1, 32, 221, 9, 0, 16, 24, 2, 5, 0, 28, 0, 0, 221,
        24, 0, 80, 242, 2, 1, 1, 132, 0, 3, 164, 0, 0, 39, 164, 0, 0, 66, 67, 94, 0, 98, 50, 47, 0,
    ];

    let frame = Frame::parse(&payload).expect("Payload should be valid");
    println!("{:?}", frame);
    assert!(matches!(frame.control.frame_type, FrameType::Management));
    assert!(matches!(
        frame.control.frame_subtype,
        FrameSubType::ProbeResponse
    ));

    if let Frame::ProbeResponse(response) = frame.payload {
        assert_eq!("My face when IP", response.station_info.ssid.unwrap());
    }
}

#[test]
fn test_authentication() {
    let _payload = [
        176, 0, // FrameControl
        58, 1, // Duration id
        248, 50, 228, 173, 71, 184, // First Address
        192, 238, 251, 75, 207, 58, // Second Address
        248, 50, 228, 173, 71, 184, // Third address
        0, 147, 0, 0, 1, 0, 0, 0,
    ];
}

#[test]
fn test_deauthentication() {
    let _payload = [
        192, 0, // FrameControl
        58, 1, // Duration id
        248, 50, 228, 173, 71, 184, // First Address
        192, 238, 251, 75, 207, 58, // Second Address
        248, 50, 228, 173, 71, 184, // Third Address
        224, 146, 3, 0,
    ];
}
