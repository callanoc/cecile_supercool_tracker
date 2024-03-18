use cecile_supercool_tracker::examples::BoxGen2;
use cecile_supercool_tracker::trackers::sort::metric::DEFAULT_MINIMAL_SORT_CONFIDENCE;
use cecile_supercool_tracker::trackers::sort::simple_api::Sort;
use cecile_supercool_tracker::trackers::sort::PositionalMetricType::IoU;
use cecile_supercool_tracker::trackers::sort::DEFAULT_SORT_IOU_THRESHOLD;
use cecile_supercool_tracker::trackers::tracker_api::TrackerAPI;
use cecile_supercool_tracker::utils::bbox::BoundingBox;

fn main() {
    let mut tracker = Sort::new(
        1,
        10,
        1,
        IoU(DEFAULT_SORT_IOU_THRESHOLD),
        DEFAULT_MINIMAL_SORT_CONFIDENCE,
        None,
        1.0 / 20.0,
        1.0 / 160.0,
    );

    let pos_drift = 1.0;
    let box_drift = 0.01;
    let mut b1 = BoxGen2::new_monotonous(100.0, 100.0, 10.0, 15.0, pos_drift, box_drift);
    let mut b2 = BoxGen2::new_monotonous(10.0, 10.0, 12.0, 18.0, pos_drift, box_drift);

    for _ in 0..100 {
        let obj1b = b1.next().unwrap();
        let obj2b = b2.next().unwrap();
        let _tracks = tracker.predict(&[(obj1b.into(), None), (obj2b.into(), None)]);
    }

    tracker.skip_epochs(2);

    let tracks = tracker.wasted();
    for t in tracks {
        eprintln!("Track id: {}", t.get_track_id());
        eprintln!(
            "Boxes: {:#?}",
            t.get_attributes()
                .predicted_boxes
                .clone()
                .into_iter()
                .map(|x| BoundingBox::try_from(x).unwrap())
                .collect::<Vec<_>>()
        );
    }
}
