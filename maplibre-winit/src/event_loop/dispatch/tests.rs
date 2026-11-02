use super::NextFrame;

#[test]
fn an_idle_map_asks_for_no_frame() {
    assert!(!NextFrame::default().wanted());
}

#[test]
fn each_reason_alone_asks_for_a_frame() {
    let reasons = [
        NextFrame {
            map: true,
            ..NextFrame::default()
        },
        NextFrame {
            arrivals: true,
            ..NextFrame::default()
        },
        NextFrame {
            input: true,
            ..NextFrame::default()
        },
        NextFrame {
            counting_frames: true,
            ..NextFrame::default()
        },
    ];
    for reason in reasons {
        assert!(reason.wanted(), "{reason:?}");
    }
}
