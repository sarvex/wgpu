struct GlobalConst {
    a: u32,
    b: vec3<u32>,
    c: i32,
}

struct AlignedWrapper {
    value: i32,
}

struct Bar {
    _matrix: mat4x3<f32>,
    matrix_array: array<mat2x2<f32>, 2>,
    atom: atomic<i32>,
    atom_arr: array<atomic<i32>, 10>,
    arr: array<vec2<u32>, 2>,
    data: array<AlignedWrapper>,
}

struct Baz {
    m: mat3x2<f32>,
}

struct MatCx2InArray {
    am: array<mat4x2<f32>, 2>,
}

struct AssignToMember {
    x: u32,
}

struct S {
    m: i32,
}

struct Inner {
    delicious: i32,
}

struct Outer {
    om_nom_nom: Inner,
    thing: u32,
}

var<private> msl_padding_global_const: GlobalConst = GlobalConst(0u, vec3<u32>(0u, 0u, 0u), 0i);
@group(0) @binding(0) 
var<storage, read_write> bar: Bar;
@group(0) @binding(1) 
var<uniform> baz: Baz;
@group(0) @binding(2) 
var<storage, read_write> qux: vec2<i32>;
@group(0) @binding(3) 
var<uniform> nested_mat_cx2_: MatCx2InArray;

fn test_matrix_within_struct_accesses() {
    var idx: i32 = 1i;
    var t: Baz = Baz(mat3x2<f32>(vec2(1f), vec2(2f), vec2(3f)));

    let _e3 = idx;
    idx = (_e3 - 1i);
    let l0_ = baz.m;
    let l1_ = baz.m[0];
    let _e14 = idx;
    let l2_ = baz.m[_e14];
    let l3_ = baz.m[0][1];
    let _e25 = idx;
    let l4_ = baz.m[0][_e25];
    let _e30 = idx;
    let l5_ = baz.m[_e30][1];
    let _e36 = idx;
    let _e38 = idx;
    let l6_ = baz.m[_e36][_e38];
    let _e51 = idx;
    idx = (_e51 + 1i);
    t.m = mat3x2<f32>(vec2(6f), vec2(5f), vec2(4f));
    t.m[0] = vec2(9f);
    let _e66 = idx;
    t.m[_e66] = vec2(90f);
    t.m[0][1] = 10f;
    let _e76 = idx;
    t.m[0][_e76] = 20f;
    let _e80 = idx;
    t.m[_e80][1] = 30f;
    let _e85 = idx;
    let _e87 = idx;
    t.m[_e85][_e87] = 40f;
    return;
}

fn test_matrix_within_array_within_struct_accesses() {
    var idx_1: i32 = 1i;
    var t_1: MatCx2InArray = MatCx2InArray(array<mat4x2<f32>, 2>());

    let _e3 = idx_1;
    idx_1 = (_e3 - 1i);
    let l0_1 = nested_mat_cx2_.am;
    let l1_1 = nested_mat_cx2_.am[0];
    let l2_1 = nested_mat_cx2_.am[0][0];
    let _e20 = idx_1;
    let l3_1 = nested_mat_cx2_.am[0][_e20];
    let l4_1 = nested_mat_cx2_.am[0][0][1];
    let _e33 = idx_1;
    let l5_1 = nested_mat_cx2_.am[0][0][_e33];
    let _e39 = idx_1;
    let l6_1 = nested_mat_cx2_.am[0][_e39][1];
    let _e46 = idx_1;
    let _e48 = idx_1;
    let l7_ = nested_mat_cx2_.am[0][_e46][_e48];
    let _e55 = idx_1;
    idx_1 = (_e55 + 1i);
    t_1.am = array<mat4x2<f32>, 2>();
    t_1.am[0] = mat4x2<f32>(vec2(8f), vec2(7f), vec2(6f), vec2(5f));
    t_1.am[0][0] = vec2(9f);
    let _e77 = idx_1;
    t_1.am[0][_e77] = vec2(90f);
    t_1.am[0][0][1] = 10f;
    let _e89 = idx_1;
    t_1.am[0][0][_e89] = 20f;
    let _e94 = idx_1;
    t_1.am[0][_e94][1] = 30f;
    let _e100 = idx_1;
    let _e102 = idx_1;
    t_1.am[0][_e100][_e102] = 40f;
    return;
}

fn read_from_private(foo_1: ptr<function, f32>) -> f32 {
    let _e1 = (*foo_1);
    return _e1;
}

fn test_arr_as_arg(a: array<array<f32, 10>, 5>) -> f32 {
    return a[4][9];
}

fn assign_through_ptr_fn(p: ptr<function, u32>) {
    (*p) = 42u;
    return;
}

fn assign_array_through_ptr_fn(foo_2: ptr<function, array<vec4<f32>, 2>>) {
    (*foo_2) = array<vec4<f32>, 2>(vec4(1f), vec4(2f));
    return;
}

fn assign_through_ptr() {
    var val: u32 = 33u;
    var arr: array<vec4<f32>, 2> = array<vec4<f32>, 2>(vec4(6f), vec4(7f));

    assign_through_ptr_fn((&val));
    assign_array_through_ptr_fn((&arr));
    return;
}

fn fetch_arg_ptr_member(p_1: ptr<function, AssignToMember>) -> u32 {
    let _e2 = (*p_1).x;
    return _e2;
}

fn assign_to_arg_ptr_member(p_2: ptr<function, AssignToMember>) {
    (*p_2).x = 10u;
    return;
}

fn fetch_arg_ptr_array_element(p_3: ptr<function, array<u32, 4>>) -> u32 {
    let _e2 = (*p_3)[1];
    return _e2;
}

fn assign_to_arg_ptr_array_element(p_4: ptr<function, array<u32, 4>>) {
    (*p_4)[1] = 10u;
    return;
}

fn assign_to_ptr_components() {
    var s1_: AssignToMember;
    var a1_: array<u32, 4>;

    assign_to_arg_ptr_member((&s1_));
    let _e1 = fetch_arg_ptr_member((&s1_));
    assign_to_arg_ptr_array_element((&a1_));
    let _e3 = fetch_arg_ptr_array_element((&a1_));
    return;
}

fn index_ptr(value: bool) -> bool {
    var a_1: array<bool, 1>;

    a_1 = array<bool, 1>(value);
    let _e4 = a_1[0];
    return _e4;
}

fn member_ptr() -> i32 {
    var s: S = S(42i);

    let _e4 = s.m;
    return _e4;
}

fn let_members_of_members() -> i32 {
    let inner_1 = Outer().om_nom_nom;
    let delishus_1 = inner_1.delicious;
    if (Outer().thing != u32(delishus_1)) {
    }
    return Outer().om_nom_nom.delicious;
}

fn var_members_of_members() -> i32 {
    var thing: Outer = Outer();
    var inner: Inner;
    var delishus: i32;

    let _e3 = thing.om_nom_nom;
    inner = _e3;
    let _e6 = inner.delicious;
    delishus = _e6;
    let _e9 = thing.thing;
    let _e10 = delishus;
    if (_e9 != u32(_e10)) {
    }
    let _e15 = thing.om_nom_nom.delicious;
    return _e15;
}

@vertex 
fn foo_vert(@builtin(vertex_index) vi: u32) -> @builtin(position) vec4<f32> {
    var foo: f32 = 0f;
    var c2_: array<i32, 5>;

    let baz_1 = foo;
    foo = 1f;
    let phony = msl_padding_global_const;
    test_matrix_within_struct_accesses();
    test_matrix_within_array_within_struct_accesses();
    let _matrix = bar._matrix;
    let arr_1 = bar.arr;
    let b = bar._matrix[3u][0];
    let a_2 = bar.data[(arrayLength((&bar.data)) - 2u)].value;
    let c = qux;
    let data_pointer = (&bar.data[0].value);
    let _e35 = read_from_private((&foo));
    c2_ = array<i32, 5>(a_2, i32(b), 3i, 4i, 5i);
    c2_[(vi + 1u)] = 42i;
    let value_1 = c2_[vi];
    let _e49 = test_arr_as_arg(array<array<f32, 10>, 5>());
    return vec4<f32>((_matrix * vec4<f32>(vec4(value_1))), 2f);
}

@fragment 
fn foo_frag() -> @location(0) vec4<f32> {
    bar._matrix[1][2] = 1f;
    bar._matrix = mat4x3<f32>(vec3(0f), vec3(1f), vec3(2f), vec3(3f));
    bar.arr = array<vec2<u32>, 2>(vec2(0u), vec2(1u));
    bar.data[1].value = 1i;
    qux = vec2<i32>();
    return vec4(0f);
}

@compute @workgroup_size(1, 1, 1) 
fn foo_compute() {
    assign_through_ptr();
    assign_to_ptr_components();
    let _e1 = index_ptr(true);
    let _e2 = member_ptr();
    let _e3 = let_members_of_members();
    let _e4 = var_members_of_members();
    return;
}
