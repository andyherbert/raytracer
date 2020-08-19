use crate::{Material, Triangle, Vert};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct WavefrontObj {
    pub triangles: Vec<Triangle>,
}

#[derive(Debug)]
struct WavefrontObjFileError {}

impl std::fmt::Display for WavefrontObjFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "WavefrontObjFileError")
    }
}

impl std::error::Error for WavefrontObjFileError {
    fn description(&self) -> &str {
        "WavefrontObjFileError"
    }
}

impl WavefrontObj {
    pub fn new(path: &str) -> Result<WavefrontObj, Box<dyn std::error::Error>> {
        let file = BufReader::new(File::open(path)?);
        let mut vectors = vec![];
        let mut normals = vec![];
        let mut faces = vec![];
        for line in file.lines() {
            let text = line?;
            let mut words = text.split_whitespace();
            match words.next() {
                Some("v") => {
                    let x = words.next().ok_or(WavefrontObjFileError {})?.parse::<f64>()?;
                    let y = words.next().ok_or(WavefrontObjFileError {})?.parse::<f64>()?;
                    let z = words.next().ok_or(WavefrontObjFileError {})?.parse::<f64>()?;
                    vectors.push(Vert::new(x, y, z));
                }
                Some("vn") => {
                    let x = words.next().ok_or(WavefrontObjFileError {})?.parse::<f64>()?;
                    let y = words.next().ok_or(WavefrontObjFileError {})?.parse::<f64>()?;
                    let z = words.next().ok_or(WavefrontObjFileError {})?.parse::<f64>()?;
                    normals.push(Vert::new(x, y, z));
                }
                Some("f") => {
                    let mut v1 = words.next().ok_or(WavefrontObjFileError {})?.split('/');
                    let mut v2 = words.next().ok_or(WavefrontObjFileError {})?.split('/');
                    let mut v3 = words.next().ok_or(WavefrontObjFileError {})?.split('/');
                    let p1 = v1.next().ok_or(WavefrontObjFileError {})?.parse::<usize>()?;
                    let p2 = v2.next().ok_or(WavefrontObjFileError {})?.parse::<usize>()?;
                    let p3 = v3.next().ok_or(WavefrontObjFileError {})?.parse::<usize>()?;
                    match v1.next() {
                        None => faces.push((p1 - 1, p2 - 1, p3 - 1, None, None, None)),
                        Some(_) => {
                            v2.next().ok_or(WavefrontObjFileError {})?;
                            v3.next().ok_or(WavefrontObjFileError {})?;
                            match v1.next() {
                                None => faces.push((p1 - 1, p2 - 1, p3 - 1, None, None, None)),
                                Some(string) => {
                                    let n1 = string.parse::<usize>()?;
                                    let n2 = v2.next().ok_or(WavefrontObjFileError {})?.parse::<usize>()?;
                                    let n3 = v3.next().ok_or(WavefrontObjFileError {})?.parse::<usize>()?;
                                    faces.push((p1 - 1, p2 - 1, p3 - 1, Some(n1 - 1), Some(n2 - 1), Some(n3 - 1)));
                                }
                            }
                        }
                    }
                }
                _ => {},
            }
        }
        let mut obj = WavefrontObj {triangles: vec![]};
        for (p1_index, p2_index, p3_index, n1_index, n2_index, n3_index) in faces {
            let p1 = vectors[p1_index].clone();
            let p2 = vectors[p2_index].clone();
            let p3 = vectors[p3_index].clone();
            let n1 = match n1_index {
                Some(index) => Some(normals[index].clone()),
                None => None,
            };
            let n2 = match n2_index {
                Some(index) => Some(normals[index].clone()),
                None => None,
            };
            let n3 = match n3_index {
                Some(index) => Some(normals[index].clone()),
                None => None,
            };
            let normals = if n1.is_none() || n2.is_none() || n3.is_none() {
                None
            } else {
                Some((-n1.expect("normal"), -n2.expect("normal"), -n3.expect("normal")))
            };
            obj.triangles.push(Triangle {
                p1,
                p2,
                p3,
                normals,
                material: Material::default(),
            });
        }
        Ok(obj)
    }
}
