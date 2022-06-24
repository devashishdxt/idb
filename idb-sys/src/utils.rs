use web_sys::DomStringList;

pub fn dom_string_list_to_vec(list: &DomStringList) -> Vec<String> {
    let mut vec = vec![];

    for i in 0..list.length() {
        if let Some(s) = list.get(i) {
            vec.push(s);
        }
    }

    vec
}
