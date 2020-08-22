use crate::{Material, Map, Mesh, Triangle, UVMap, Vert};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::{Split, SplitWhitespace};
use std::sync::Arc;


#[derive(Debug, Default)]
pub struct MtlDefinition {
    pub diffusion: Option<Vert>,
    pub map: Option<Arc<Map>>,
}

#[derive(Default)]
pub struct WavefrontObj {
    mtllib: WavefrontMtl,
    triangles: Vec<Triangle>,
}

#[derive(Debug)]
struct VertIndex {x: usize, y: usize, z: usize}

#[derive(Debug)]
pub struct Face {
    v: VertIndex,
    vt: Option<VertIndex>,
    vn: Option<VertIndex>,
    usemtl: Option<String>,
}

#[derive(Debug)]
pub struct VertDefinition {
    v: usize,
    t: Option<usize>,
    n: Option<usize>,
}

pub fn expect_string(words: &mut SplitWhitespace) -> Result<String, FileError> {
    match words.next() {
        Some(str) => Ok(str.to_string()),
        None => Err(FileError {}),
    }
}

fn expect_f64(words: &mut SplitWhitespace) -> Result<f64, FileError> {
    match words.next() {
        Some(str) => {
            match str.parse::<f64>() {
                Ok(value) => Ok(value),
                Err(_)=> Err(FileError {}),
            }
        }
        None => Err(FileError {}),
    }
}

fn expect_xy(words: &mut SplitWhitespace) -> Result<Vert, FileError> {
    let x= expect_f64(words)?;
    let y= expect_f64(words)?;
    let vert = Vert::new(x, y, 0.0);
    Ok(vert)
}

pub fn expect_xyz(words: &mut SplitWhitespace) -> Result<Vert, FileError> {
    let x= expect_f64(words)?;
    let y= expect_f64(words)?;
    let z= expect_f64(words)?;
    let vert = Vert::new(x, y, z);
    Ok(vert)
}

fn expect_usize(split: &mut Split<char>) -> Result<usize, FileError> {
    match split.next() {
        Some(str) => {
            match str.parse::<usize>() {
                Ok(value) => Ok(value),
                Err(_)=> Err(FileError {}),
            }
        }
        None => Err(FileError {}),
    }
}

fn maybe_usize(split: &mut Split<char>) -> Result<Option<usize>, FileError> {
    match split.next() {
        Some(str) if str.is_empty() => Ok(None),
        Some(str) => {
            match str.parse::<usize>() {
                Ok(value) => Ok(Some(value)),
                Err(_)=> return Err(FileError {}),
            }
        }
        None => Ok(None),
    }
}

fn expect_vert_face_definition(definition: &str) -> Result<VertDefinition, FileError> {
    let mut split = definition.split('/');
    let v = expect_usize(&mut split)?;
    let t = maybe_usize(&mut split)?;
    let n = maybe_usize(&mut split)?;
    Ok(VertDefinition {v, t, n})
}

fn expect_face(words: &mut SplitWhitespace, usemtl: Option<String>) -> Result<Face, FileError> {
    let v1 = match words.next() {
        Some(str) => expect_vert_face_definition(str)?,
        None => return Err(FileError {}),
    };
    let v2 = match words.next() {
        Some(str) => expect_vert_face_definition(str)?,
        None => return Err(FileError {}),
    };
    let v3 = match words.next() {
        Some(str) => expect_vert_face_definition(str)?,
        None => return Err(FileError {}),
    };
    let vt = if v1.t.is_some() && v2.t.is_some() && v3.t.is_some()  {
        Some(VertIndex {
            x: v1.t.expect("value"),
            y: v2.t.expect("value"),
            z: v3.t.expect("value"),
        })
    } else if v1.t.is_none() && v2.t.is_none() && v3.t.is_none() {
        None
    } else {
        return Err(FileError {});
    };
    let vn = if v1.n.is_some() && v2.n.is_some() && v3.n.is_some()  {
        Some(VertIndex {
            x: v1.n.expect("value"),
            y: v2.n.expect("value"),
            z: v3.n.expect("value"),
        })
    } else if v1.n.is_none() && v2.n.is_none() && v3.n.is_none() {
        None
    } else {
        return Err(FileError {});
    };
    let face = Face {
        v: VertIndex {
            x: v1.v,
            y: v2.v,
            z: v3.v
        },
        vt,
        vn,
        usemtl,
    };
    Ok(face)
}

fn expect_mtl(words: &mut SplitWhitespace, parent: &Path) -> Result<WavefrontMtl, Box<dyn Error>> {
    let mtl_file_string = expect_string(words)?;
    let joined_path = parent.join(mtl_file_string);
    match joined_path.to_str() {
        Some(joined_path_str) => {
            let mtl = WavefrontMtl::open(joined_path_str)?;
            Ok(mtl)
        },
        None => Err(Box::new(FileError {})),
    }
}

#[derive(Debug)]
pub struct FileError {}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "WavefrontObjFileError")
    }
}

impl std::error::Error for FileError {
    fn description(&self) -> &str {
        "WavefrontObjFileError"
    }
}

impl WavefrontObj {
    pub fn open(path: &str) -> Result<Self, Box<dyn Error>> {
        let file = BufReader::new(File::open(path)?);
        let mut usemtl = None;
        let mut v = Vec::new();
        let mut vt = Vec::new();
        let mut vn = Vec::new();
        let mut f = Vec::new();
        let mut mtllib = WavefrontMtl::default();
        for line in file.lines() {
            let line = line?;
            let mut words = line.split_whitespace();
            match words.next() {
                Some("mtllib") => {
                    mtllib = match Path::new(path).parent() {
                        Some(parent) => expect_mtl(&mut words, parent)?,
                        _ => return Err(Box::new(FileError {})),
                    };
                },
                Some("usemtl") => usemtl = Some(expect_string(&mut words)?),
                Some("v") => v.push(expect_xyz(&mut words)?),
                Some("vt") => vt.push(expect_xy(&mut words)?),
                Some("vn") => vn.push(expect_xy(&mut words)?),
                Some("f") => f.push(expect_face(&mut words, usemtl.clone())?),
                _ => continue,
            }
        }
        let mut obj = WavefrontObj::default();
        for face in f {
            let p1 = v[face.v.x - 1].clone();
            let p2 = v[face.v.y - 1].clone();
            let p3 = v[face.v.z - 1].clone();
            let uv_map = match face.vt {
                Some(t) => {
                    let t1 = vt[t.x - 1].clone();
                    let t2 = vt[t.y - 1].clone();
                    let t3 = vt[t.z - 1].clone();
                    match &face.usemtl {
                        Some(usemtl) => {
                            match mtllib.definitions.get(usemtl) {
                                Some(definition) => {
                                    match &definition.map {
                                        Some(map) => Some(UVMap {t1, t2, t3, map: map.clone()}),
                                        None => None,
                                    }
                                },
                                None => return Err(Box::new(FileError {})),
                            }
                        },
                        None => return Err(Box::new(FileError {})),
                    }
                },
                None => None,
            };
            let normals = match face.vn {
                Some(n) => {
                    let normals = (
                        -vn[n.x - 1].clone(),
                        -vn[n.y - 1].clone(),
                        -vn[n.z - 1].clone(),
                    );
                    Some(normals)
                },
                None => None,
            };
            let mut material = Material::default();
            if let Some(usemtl) = &face.usemtl {
                if let Some(definition) = mtllib.definitions.get(usemtl) {
                    if let Some(diffusion) = &definition.diffusion {
                        material.colour = diffusion.clone();
                    }
                }
            }
            let triangle = Triangle {
                p1,
                p2,
                p3,
                normals,
                material,
                uv_map,
            };
            obj.triangles.push(triangle);
        }
        Ok(obj)
    }
}

#[derive(Debug, Default)]
pub struct WavefrontMtl {
    pub definitions: HashMap<String, MtlDefinition>,
    pub maps: HashMap<String, Arc<Map>>,
}

impl WavefrontMtl {
    pub fn open(path: &str) -> Result<Self, Box<dyn Error>> {
        let file = BufReader::new(File::open(path)?);
        let mut mtl = WavefrontMtl::default();
        let mut name = None;
        let mut definition = MtlDefinition::default();
        for line in file.lines() {
            let line = line?;
            let mut words = line.split_whitespace();
            match words.next() {
                Some("newmtl") => {
                    if let Some(name) = name {
                        mtl.definitions.insert(name, definition);
                        definition = MtlDefinition::default();
                    }
                    name = Some(expect_string(&mut words)?);
                },
                Some("Kd") => definition.diffusion = Some(expect_xyz(&mut words)?),
                Some("map_Kd") => {
                    let name = expect_string(&mut words)?;
                    match mtl.maps.get(&name) {
                        Some(map) => definition.map = Some(map.clone()),
                        None => {
                            match Path::new(path).parent() {
                                Some(parent) => {
                                    match parent.join(name.clone()).to_str() {
                                        Some(joined_path_string) => {
                                            let map = Arc::new(Map::open(joined_path_string)?);
                                            mtl.maps.insert(name.clone(), map.clone());
                                            definition.map = Some(map);
                                        },
                                        None => return Err(Box::new(FileError {})),
                                    };
                                },
                                None => return Err(Box::new(FileError {})),
                            }
                        },
                    }
                },
                _ => continue,
            }
        }
        if let Some(name) = name {
            mtl.definitions.insert(name, definition);
        }
        Ok(mtl)
    }
}

impl From<WavefrontObj> for Mesh {
    fn from(obj: WavefrontObj) -> Self {
        Mesh {
            maps: obj.mtllib.maps,
            triangles: obj.triangles,
            position: Vert::default(),
            rotation: Vert::default(),
            scale: Vert::new(1.0, 1.0, 1.0),
        }
    }
}
