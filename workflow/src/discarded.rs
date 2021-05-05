//////////// main.rs
// fn construct_viewport_matrix(factor: f32) -> Matrix4<f32> {
//
//     let x = (WIDTH  as f32 - WIDTH  as f32 * factor)/2.0;
//     let y = (HEIGHT as f32 - HEIGHT as f32 * factor)/2.0;
//     let w = WIDTH  as f32 * factor;
//     let h = HEIGHT as f32 * factor;
//
//     let mut matrix = Matrix4::<f32>::identity();
//     matrix[(0,3)] = x+w/2.0;
//     matrix[(1,3)] = y+h/2.0;
//     matrix[(2,3)] = DEPTH as f32/2.0;
//
//     matrix[(0,0)] = w/2.0;
//     matrix[(1,1)] = h/2.0;
//     matrix[(2,2)] = DEPTH as f32/2.0;
//     matrix
// }
//
// fn construct_projection_matrix(camera: Vector3<f32>, origin: Vector3<f32>) -> Matrix4<f32> {
//     let mut matrix = Matrix4::<f32>::identity();
//     matrix[(3,2)] = -1.0/(camera-origin).norm();
//     matrix
// }
//
// fn construct_modelview_matrix(camera: Vector3<f32>, up: Vector3<f32>, origin: Vector3<f32>) -> Matrix4<f32> {
//     let z: Vector3<f32> = (camera-origin).normalize();
//     let x: Vector3<f32> = up.cross(&z).normalize();
//     let y: Vector3<f32> = z.cross(&x).normalize();
//     let mut matrix: Matrix4<f32> = Matrix4::<f32>::identity();
//     for i in 0..3 as usize {
//         matrix[(0, i)] = x[i];
//         matrix[(1, i)] = y[i];
//         matrix[(2, i)] = z[i];
//         matrix[(i, 3)] = -origin[i];
//     }
//     matrix
// }
//////////// main.rs
/////////// 扫描线光栅化法
// let mut triangle = primitives::Triangle::new();
// triangle.set_vertices_i32(v34f_to_v33i(&screen_coords));
// triangle.set_tex_coords(tex_coords);
// triangle.set_normals(normals);
// rasterize(&triangle, &mut d, &model, intensities);
//fn rasterize(t: &primitives::Triangle, d: &mut delusion::Delusion,
//              m: &Objcracker, mut intensities: [f32;3]) {
//
//     if t.ai().y == t.bi().y && t.ai().y == t.ci().y { return; }
//
//     let mut tex_a = t.tex_a(); let mut tex_b = t.tex_b(); let mut tex_c = t.tex_c();
//     let mut a = t.ai(); let mut b = t.bi(); let mut c = t.ci();
//     if a.y > b.y { mem::swap(&mut a, &mut b); mem::swap(&mut tex_a, &mut tex_b); intensities.swap(0, 1); }
//     if a.y > c.y { mem::swap(&mut a, &mut c); mem::swap(&mut tex_a, &mut tex_c); intensities.swap(0, 2); }
//     if b.y > c.y { mem::swap(&mut b, &mut c); mem::swap(&mut tex_b, &mut tex_c); intensities.swap(1, 2); }
//
//     let h1: i32 = c.y - a.y;
//     let h2: i32 = b.y - a.y;
//     let h3: i32 = c.y - b.y;
//
//     for i in 0..h1 as usize {
//         let second_half = i as i32 > h2 || b.y == a.y;
//
//         let segment_height = match second_half {
//             true  => h3,
//             false => h2
//         };
//
//         let alpha: f32 = i as f32 / h1 as f32;
//         let beta:  f32 = (i as f32 - match second_half {
//             true => h2 as f32,
//             false => 0.0
//         }) / segment_height as f32;
//
//         let mut _a: Vector3<i32> = a + vec3f_to_vec3i(&(vec3i_to_vec3f(&(c-a))*alpha));
//         let mut _b: Vector3<i32> = match second_half {
//             true  => b + vec3f_to_vec3i(&(vec3i_to_vec3f(&(c-b))*beta)),
//             false => a + vec3f_to_vec3i(&(vec3i_to_vec3f(&(b-a))*beta))
//         };
//
//         let mut uva = tex_a + vec2f_to_vec2i(&(vec2i_to_vec2f(&(tex_c-tex_a))*alpha));
//         let mut uvb = match second_half {
//             true  => tex_b + vec2f_to_vec2i(&(vec2i_to_vec2f(&(tex_c-tex_b))*beta)),
//             false => tex_a + vec2f_to_vec2i(&(vec2i_to_vec2f(&(tex_b-tex_a))*beta))
//         };
//
//         let mut ita = intensities[0] + (intensities[2]-intensities[0])*alpha;
//         let mut itb = match second_half {
//             true  => intensities[1] + (intensities[2]-intensities[1])*beta,
//             false => intensities[0] + (intensities[1]-intensities[0])*beta
//         };
//
//         if _a.x > _b.x {
//             mem::swap(&mut _a,&mut _b);
//             mem::swap(&mut uva,&mut uvb);
//             mem::swap(&mut ita, &mut itb);
//         }
//
//         for j in _a.x as usize..(_b.x+1) as usize {
//             let phi: f32 = match _b.x ==_a.x {
//                 true  => 1.0,
//                 false => (j as i32 -_a.x) as f32 / (_b.x -_a.x) as f32
//             };
//
//             let _p  : Vector3<i32> = _a + vec3f_to_vec3i(&(vec3i_to_vec3f(&(_b-_a))*phi));
//             let _uvp: Vector2<i32> = uva + vec2f_to_vec2i(&(vec2i_to_vec2f(&(uvb-uva))*phi));
//             let mut _itp: f32      = ita + (itb-ita)*phi;
//
//             let _itp: f32 = match _itp > 1.0 {
//                 true  => 1.0,
//                 false => match _itp < 0.0 {
//                     true  => 0.0,
//                     false => _itp
//                 }
//             };
//
//             if _p.x >= WIDTH as i32 || _p.y >= HEIGHT as i32 || _p.x < 0 || _p.y < 0 {
//                 continue;
//             }
//
//             if d.get_depth(_p.x as usize, _p.y as usize) < _p.z {
//                 d.set_depth(_p.x as usize, _p.y as usize,_p.z);
//                 let tmp: Vector2<u32> = Vector2::new(_uvp.x as u32,_uvp.y as u32);
//                 let color = m.diffuse(&tmp) * _itp;
//                 d.set_color(_p.x as usize, _p.y as usize,
//                             color);
//             }
//         }
//     }
// }
//////////// main.rs
/////////// 中点Bresenham画直线
// fn bresemham_midpoint(d: &mut delusion::Render, t0: Vector2<i32>,
//                       t1: Vector2<i32>, color: Vector3<f32>) {
//     let mut x0 = t0.x;
//     let mut y0 = t0.y;
//     let mut x1 = t1.x;
//     let mut y1 = t1.y;
//     // 确保x为最大位移方向
//     let mut steep: bool = false;
//     if (x0-x1).abs()<(y0-y1).abs() {
//         mem::swap(&mut x0, &mut y0);
//         mem::swap(&mut x1, &mut y1);
//         steep = true;
//     }
//     // 确保计算方向为正向
//     if x0 > x1 {
//         mem::swap(&mut x0, &mut x1);
//         mem::swap(&mut y0, &mut y1);
//     }
//
//     let dx = (x1 - x0).abs();
//     let dy = (y1 - y0).abs();
//     let mut delta = dx - 2 * dx;
//
//     let d_step_up = 2 * (dx - dy);
//     let d_step_down = -2 * dy;
//
//     let x = x0;
//     let mut y = y0;
//
//     for i in x..x1 {
//         match steep {
//             true => d.set_color(y as usize, i as usize, color),
//             false => d.set_color(i as usize, y as usize, color),
//         }
//         if delta < 0 {
//             y += match y0<y1 {
//                 true => 1,
//                 false => -1,
//             };
//             delta = delta + d_step_up;
//         } else {
//             delta += d_step_down;
//         }
//     }
// }
//////////// main.rs
// let modelview = construct_modelview_matrix(camera, UP, ORIGIN);
// let transform = viewport*projection*modelview;
//
// d.clear_frame_buff(CLEAR_COLOR);
// d.clear_depth_buff();
//
// for i in 0..model.nfaces() {
// let face = model.face(i);
//
// // let mut world_coords:  Vector3<Vector3<f32>> = Default::default();
// // let mut screen_coords: Vector3<Vector4<f32>> = Default::default();
// // let mut tex_coords:    Vector3<Vector2<i32>> = Default::default();
// // let mut normals:       Vector3<Vector3<f32>> = Default::default();
// // let mut intensities: [f32;3] = [0.0, 0.0, 0.0];
// //
// // for j in 0..3 as usize {
// //     world_coords[j]  = model.vert(face[j]);
// //     screen_coords[j] = &transform*vec3f_to_vec4f(&world_coords[j], 1.0);
// //     screen_coords[j] = screen_coords[j] / screen_coords[j].w;
// //     tex_coords[j]    = model.calc_uv(i,j);
// //     normals[j]       = model.calc_normal(i,j);
// //     intensities[j]   = normals[j].dot(&LIGHT.normalize());
// // }
//
// // let mut tri = primitives::Triangle::new();
// // tri.set_vertices(v34f_to_v33f(&screen_coords));
// // tri.set_tex_coords(tex_coords);
// // tri.set_normals(normals);
// // triangle(&tri, &mut d, &model, intensities);
// }
/////////// main.rs Flat Shading
// let mut face_normal: Vector3<f32> = (world_coords[2]-world_coords[0])
//     .cross(&(world_coords[1]-world_coords[0]));
// face_normal = normalize_vec3f(face_normal);
//
// let intensity: f32 = face_normal.dot(&LIGHT);
//
// if intensity > 0.0 {
//     let mut screen_coords: Vector3<Vector4<f32>> = Default::default();
//     let mut tex_coords:    Vector3<Vector2<i32>> = Default::default();
//     let mut normals:       Vector3<Vector3<f32>> = Default::default();
//     for j in 0..3 as usize{
//         screen_coords[j] = &transform*vec3f_to_vec4f(&world_coords[j], 1.0);
//         screen_coords[j] = screen_coords[j] / screen_coords[j].w;
//
//         tex_coords[j] = model.calc_uv(i,j);
//         normals[j] = model.calc_normal(i,j);
//     }
//
//     let mut triangle = delusion::Triangle::new();
//
//     triangle.set_vertices(v34f_to_v33i(screen_coords));
//     triangle.set_tex_coords(tex_coords);
//     triangle.set_normals(normals);
//     rasterize(&triangle, &mut d, &model, intensity);
// }

//////////// objcracker.rs
// 打印obj内容
// println!("*****Delusion::Debug*******");
// for vert in &self.verts {
//     println!("v {} {} {}", vert.x, vert.y, vert.z);
// }
// for uv in &self.uvs {
//     println!("vt {} {}", uv.x, uv.y);
// }
// for normal in &self.normals {
//     println!("vn {} {} {}", normal.x, normal.y, normal.z);
// }
// for face in &self.faces {
//     print!("f ");
//     for tuple in face {
//         print!("{}/{}/{} ",tuple.x, tuple.y, tuple.z);
//     }
//     println!();
// }
// println!("***************************\n");
// else if hit == 4.0 {
//     let weights = barycentric(&(pts[0]/pts[0][3]),&(pts[1]/pts[1][3]),
//                               &(pts[2]/pts[2][3]),x as f32, y as f32);
//     let z: f32 = pts[0][2]*weights.x + pts[1][2]*weights.y + pts[2][2]*weights.z;
//     let w: f32 = pts[0][3]*weights.x + pts[1][3]*weights.y + pts[2][3]*weights.z;
//     let dep: f32 = (z/w+0.5).min(255.0).max(0.0);
//     if self.get_depth(x,y)<=dep {
//         self.set_depth(x,y,dep);
//         self.set_color(x,y,&shader.fragment(&weights,&model));
//     }
//     continue;
// }
