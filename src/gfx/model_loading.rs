use crate::math::{Vector4, Vector3};
use super::render_3d::{Model, MaterialData, PolygonData};


pub fn load_obj_file(file: &str) -> Model{
    let mut lines = file.lines();
    
    let mut vertices : Vec<Vector4> = Vec::new();
    let mut vertex_normals: Vec<Vector3> = Vec::new();
    let mut v_texture_coords: Vec<Vector3> = Vec::new();

    let mut triangles: Vec<PolygonData> = Vec::new();
    
    for (num, line) in lines.enumerate() {
        let mut split = line.split_ascii_whitespace();        
        if let Some(first) = split.next() {
            match first {
                "#" => continue,
                "mtllib" => continue,
                "o" => continue,
                "usemtl" => continue,
                "s" => continue,
                "v" => {
                    let er_num = &format!("Insufficient number of elements in field v at line {} (expected 3 or 4)", num);
                    let er_kind = &format!("Wrong element format in field v at line {} (expected float)", num);
                    
                    let new_vertex: Vector4 = [
                        split.next().expect(er_num)
                            .parse().expect(er_kind),
                        split.next().expect(er_num)
                            .parse().expect(er_kind),
                        split.next().expect(er_num)
                            .parse().expect(er_kind),
                        split.next().unwrap_or("1.0")
                            .parse().unwrap()
                    ]; 
                    vertices.push(new_vertex);
                },
                
                "vn" => {
                     let er_num = &format!("Insufficient number of elements in field vn at line {} (expected 3)", num);
                    let er_kind = &format!("Wrong element format in field vn at line {} (expected float)", num);
                    
                    let new_normal: Vector3 = [
                        split.next().expect(er_num)
                            .parse().expect(er_kind),
                        split.next().expect(er_num)
                            .parse().expect(er_kind),
                        split.next().expect(er_num)
                            .parse().expect(er_kind),
                    ];
                    vertex_normals.push(new_normal);
                },
                
                
                "vt" => {
                    
                     let er_num = &format!("Insufficient number of elements in field vt at line {} (expected 2 or 3)", num);
                    let er_kind = &format!("Wrong element format in field vt at line {} (expected float)", num);
                    
                    let new_texture: Vector3 = [
                        split.next().expect(er_num)
                            .parse().expect(er_kind),
                        split.next().expect(er_num)
                            .parse().expect(er_kind),
                        split.next().unwrap_or("0.0")
                            .parse().unwrap()
                    ];
                    
                    v_texture_coords.push(new_texture);
                },

                "f" => {
                    let er_kind = format!("Wrong element format in field vt at line {} (expected integer)", num);
                    
                    let mut vertex = [0, 0, 0];
                    let mut normal = [0, 0, 0];
                    let mut uv_coord = [0, 0, 0];
                    for (i, elem) in split.enumerate() {
                        if i > 2 {
                            panic!("Too many verticies found in face at line {} - only triangles supported!", num)
                        }
                        for (value, kind) in elem.split("/").zip([&mut vertex, &mut uv_coord, &mut normal]) {
                            if value != "" { 
                                let v : usize = value.parse().expect(&er_kind);
                                kind[i] = v -1;
                            }
                        }
                        
                    }
                    

                    triangles.push(
                        PolygonData{vertex, normal, uv_coord}
                    );
                    
                    
                },


                _ => panic!("Unexpected element found at line {}", num)

            }

        }
        
    }

    Model {
        vertices,
        uv_map: v_texture_coords,
        vertex_normals: vertex_normals,
        triangles
    }
    
}
