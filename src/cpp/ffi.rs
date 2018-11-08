/* automatically generated by rust-bindgen */

pub type vec3 = [f32; 3usize];
pub type mat3 = [f32; 9usize];
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ray {
    pub start: vec3,
    pub direction: vec3,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct sphere {
    pub center: vec3,
    pub radius: f32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct tri {
    pub p: [vec3; 3usize],
}
extern "C" {
    #[link_name = "\u{1}?center@tri@@QEBA?AV?$vec@$02@@XZ"]
    pub fn tri_center(this: *const tri) -> vec3;
}
extern "C" {
    #[link_name = "\u{1}?unit_normal@tri@@QEBA?AV?$vec@$02@@XZ"]
    pub fn tri_unit_normal(this: *const tri) -> vec3;
}
extern "C" {
    #[link_name = "\u{1}??0tri@@QEAA@XZ"]
    pub fn tri_tri(this: *mut tri);
}
extern "C" {
    #[link_name = "\u{1}??0tri@@QEAA@V?$initializer_list@V?$vec@$02@@@std@@@Z"]
    pub fn tri_tri1(this: *mut tri, arg1: u8);
}
impl tri {
    #[inline]
    pub unsafe fn center(&self) -> vec3 {
        tri_center(self)
    }
    #[inline]
    pub unsafe fn unit_normal(&self) -> vec3 {
        tri_unit_normal(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        tri_tri(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new1(arg1: u8) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        tri_tri1(&mut __bindgen_tmp, arg1);
        __bindgen_tmp
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct obb {
    pub center: vec3,
    pub half_width: vec3,
    pub orientation: mat3,
}
extern "C" {
    #[link_name = "\u{1}??0obb@@QEAA@XZ"]
    pub fn obb_obb(this: *mut obb);
}
impl obb {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        obb_obb(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct aabb {
    pub min_x: f32,
    pub min_y: f32,
    pub min_z: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub max_z: f32,
}
extern "C" {
    #[link_name = "\u{1}?center@aabb@@QEBA?AV?$vec@$02@@XZ"]
    pub fn aabb_center(this: *const aabb) -> vec3;
}
extern "C" {
    #[link_name = "\u{1}??0aabb@@QEAA@XZ"]
    pub fn aabb_aabb(this: *mut aabb);
}
extern "C" {
    #[link_name = "\u{1}??0aabb@@QEAA@AEBUtri@@@Z"]
    pub fn aabb_aabb1(this: *mut aabb, arg1: *const tri);
}
extern "C" {
    #[link_name = "\u{1}??0aabb@@QEAA@AEBUobb@@@Z"]
    pub fn aabb_aabb2(this: *mut aabb, arg1: *const obb);
}
extern "C" {
    #[link_name = "\u{1}??0aabb@@QEAA@AEBUsphere@@@Z"]
    pub fn aabb_aabb3(this: *mut aabb, s: *const sphere);
}
extern "C" {
    #[link_name = "\u{1}??0aabb@@QEAA@AEBU0@0@Z"]
    pub fn aabb_aabb4(this: *mut aabb, a: *const aabb, b: *const aabb);
}
impl aabb {
    #[inline]
    pub unsafe fn center(&self) -> vec3 {
        aabb_center(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        aabb_aabb(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new1(arg1: *const tri) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        aabb_aabb1(&mut __bindgen_tmp, arg1);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new2(arg1: *const obb) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        aabb_aabb2(&mut __bindgen_tmp, arg1);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new3(s: *const sphere) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        aabb_aabb3(&mut __bindgen_tmp, s);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new4(a: *const aabb, b: *const aabb) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        aabb_aabb4(&mut __bindgen_tmp, a, b);
        __bindgen_tmp
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bvh {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bvh_bvh_node {
    pub box_: aabb,
    pub code: u64,
}
#[repr(C)]
pub struct Pitch {
    pub empty: aabb,
    pub mesh: [u64; 26usize],
    pub hits: [u64; 3usize],
    pub triangles: [u64; 3usize],
}
extern "C" {
    #[link_name = "\u{1}?in_contact_with@Pitch@@QEAA_NAEBUobb@@@Z"]
    pub fn Pitch_in_contact_with(this: *mut Pitch, o: *const obb) -> bool;
}
extern "C" {
    #[link_name = "\u{1}?in_contact_with@Pitch@@QEAA_NAEBUsphere@@@Z"]
    pub fn Pitch_in_contact_with1(this: *mut Pitch, s: *const sphere) -> bool;
}
extern "C" {
    #[link_name = "\u{1}?last_contact_info@Pitch@@QEAA?AUray@@XZ"]
    pub fn Pitch_last_contact_info(this: *mut Pitch) -> ray;
}
extern "C" {
    #[link_name = "\u{1}?raycast_any@Pitch@@QEAA?AUray@@AEBU2@@Z"]
    pub fn Pitch_raycast_any(this: *mut Pitch, arg1: *const ray) -> ray;
}
extern "C" {
    #[link_name = "\u{1}??0Pitch@@QEAA@XZ"]
    pub fn Pitch_Pitch(this: *mut Pitch);
}
impl Default for Pitch {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Pitch {
    #[inline]
    pub unsafe fn in_contact_with(&mut self, o: *const obb) -> bool {
        Pitch_in_contact_with(self, o)
    }
    #[inline]
    pub unsafe fn in_contact_with1(&mut self, s: *const sphere) -> bool {
        Pitch_in_contact_with1(self, s)
    }
    #[inline]
    pub unsafe fn last_contact_info(&mut self) -> ray {
        Pitch_last_contact_info(self)
    }
    #[inline]
    pub unsafe fn raycast_any(&mut self, arg1: *const ray) -> ray {
        Pitch_raycast_any(self, arg1)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Pitch_Pitch(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Ball {
    pub x: vec3,
    pub v: vec3,
    pub w: vec3,
    pub t: f32,
    pub last_bounce: ray,
    pub radius: f32,
}
extern "C" {
    #[link_name = "\u{1}?p@Ball@@2VPitch@@A"]
    pub static mut Ball_p: Pitch;
}
extern "C" {
    #[link_name = "\u{1}?collider@Ball@@QEAA?AUsphere@@XZ"]
    pub fn Ball_collider(this: *mut Ball) -> sphere;
}
extern "C" {
    #[link_name = "\u{1}?wall_nearby@Ball@@QEAA?AUray@@M@Z"]
    pub fn Ball_wall_nearby(this: *mut Ball, R: f32) -> ray;
}
extern "C" {
    #[link_name = "\u{1}?step@Ball@@QEAAXM@Z"]
    pub fn Ball_step(this: *mut Ball, dt: f32);
}
extern "C" {
    #[link_name = "\u{1}??0Ball@@QEAA@XZ"]
    pub fn Ball_Ball(this: *mut Ball);
}
impl Ball {
    #[inline]
    pub unsafe fn collider(&mut self) -> sphere {
        Ball_collider(self)
    }
    #[inline]
    pub unsafe fn wall_nearby(&mut self, R: f32) -> ray {
        Ball_wall_nearby(self, R)
    }
    #[inline]
    pub unsafe fn step(&mut self, dt: f32) {
        Ball_step(self, dt)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Ball_Ball(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Input {
    pub steer: f32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub throttle: f32,
    pub jump: bool,
    pub boost: bool,
    pub slide: bool,
    pub handbrake: bool,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Car {
    pub x: vec3,
    pub v: vec3,
    pub w: vec3,
    pub o: mat3,
    pub supersonic: bool,
    pub jumped: bool,
    pub double_jumped: bool,
    pub on_ground: bool,
    pub boost: ::std::os::raw::c_int,
    pub can_dodge: bool,
    pub dodge_timer: f32,
    pub time: f32,
    pub hitbox_widths: vec3,
    pub hitbox_offset: vec3,
    pub team: ::std::os::raw::c_int,
    pub id: ::std::os::raw::c_int,
    pub last: Input,
}
extern "C" {
    #[link_name = "\u{1}?env@Car@@0VPitch@@A"]
    pub static mut Car_env: Pitch;
}
extern "C" {
    #[link_name = "\u{1}?step@Car@@QEAAXUInput@@M@Z"]
    pub fn Car_step(this: *mut Car, in_: Input, dt: f32);
}
extern "C" {
    #[link_name = "\u{1}?pitch_surface_normal@Car@@QEAA?AV?$vec@$02@@XZ"]
    pub fn Car_pitch_surface_normal(this: *mut Car) -> vec3;
}
extern "C" {
    #[link_name = "\u{1}?hitbox@Car@@QEAA?AUobb@@XZ"]
    pub fn Car_hitbox(this: *mut Car) -> obb;
}
extern "C" {
    #[link_name = "\u{1}?extrapolate@Car@@QEAAXM@Z"]
    pub fn Car_extrapolate(this: *mut Car, arg1: f32);
}
extern "C" {
    #[link_name = "\u{1}?forward@Car@@QEAA?AV?$vec@$02@@XZ"]
    pub fn Car_forward(this: *mut Car) -> vec3;
}
extern "C" {
    #[link_name = "\u{1}?left@Car@@QEAA?AV?$vec@$02@@XZ"]
    pub fn Car_left(this: *mut Car) -> vec3;
}
extern "C" {
    #[link_name = "\u{1}?up@Car@@QEAA?AV?$vec@$02@@XZ"]
    pub fn Car_up(this: *mut Car) -> vec3;
}
extern "C" {
    #[link_name = "\u{1}??0Car@@QEAA@XZ"]
    pub fn Car_Car(this: *mut Car);
}
impl Car {
    #[inline]
    pub unsafe fn step(&mut self, in_: Input, dt: f32) {
        Car_step(self, in_, dt)
    }
    #[inline]
    pub unsafe fn pitch_surface_normal(&mut self) -> vec3 {
        Car_pitch_surface_normal(self)
    }
    #[inline]
    pub unsafe fn hitbox(&mut self) -> obb {
        Car_hitbox(self)
    }
    #[inline]
    pub unsafe fn extrapolate(&mut self, arg1: f32) {
        Car_extrapolate(self, arg1)
    }
    #[inline]
    pub unsafe fn forward(&mut self) -> vec3 {
        Car_forward(self)
    }
    #[inline]
    pub unsafe fn left(&mut self) -> vec3 {
        Car_left(self)
    }
    #[inline]
    pub unsafe fn up(&mut self) -> vec3 {
        Car_up(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Car_Car(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}?max_curvature@@YAMM@Z"]
    pub fn max_curvature(speed: f32) -> f32;
}
extern "C" {
    #[link_name = "\u{1}?max_speed@@YAMM@Z"]
    pub fn max_speed(curvature: f32) -> f32;
}