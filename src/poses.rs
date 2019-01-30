use std::fs:: OpenOptions;
use std::io::BufReader;
use serde_json::Deserializer;
use serde::{Deserialize, Serialize};
use crate::joints::Joint2DType;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PoseRecord {
    pub name: String,
    pub data: Vec<Joint2D>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Joint2D(pub Joint2DType, pub [f32; 2]);

impl Joint2D {
    pub fn new(jt: Joint2DType, pos: [f32; 2]) -> Self {
        Joint2D(jt, pos)
    }
}

pub fn read_poses() -> Vec<PoseRecord> {
    let mut path = std::env::current_dir().expect("Couldn't find current directory");
    path.push("poses.json");
    let file = OpenOptions::new().read(true).open(path).expect("Failed to open file for writing poses");
    let reader = BufReader::new(file);
    Deserializer::from_reader(reader)
        .into_iter::<PoseRecord>()
        .map(Result::unwrap)
        .collect()
}

/*
fn make_pose_data() {
    for pose_record in poses::read_poses().into_iter() {
        let PoseRecord {
            name,
            data,
        } = pose_record;
        if let Some(name) = Pose::from_name(&name) {
            poses.insert(name,
                         data.into_iter().map(|j2d| (joints::j2d_to_j(j2d.0), Vec2::xy(j2d.1[0], j2d.1[1]))).collect());
        }
    }
}
*/
