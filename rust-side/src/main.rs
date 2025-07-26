#[repr(C)]
#[derive(Debug)]
pub struct Person {
    name: Str,
    last_name: Str,
}

impl Person {
    pub fn new(name: &str, last_name: &str) -> Person {
        return Person {
            name: Str::new(name),
            last_name: Str::new(last_name),
        };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Str {
    ptr: *const u8,
    len: usize,
}
impl Str {
    pub fn new(string: &str) -> Str {
        return Str {
            ptr: string.as_ptr(),
            len: string.len(),
        };
    }
}

impl ToString for Str {
    fn to_string(&self) -> String {
        let bytes = unsafe { std::slice::from_raw_parts(self.ptr, self.len) };
        let human_readable = std::str::from_utf8(bytes).unwrap().to_owned();
        return human_readable;
    }
}

unsafe extern "C" {
    fn takes_struct(person: Person) -> Str;
}
fn main() {
    let person = Person::new("German", "Interfacio");
    let resp = unsafe { takes_struct(person) }.to_string();
    println!("La respuesta es = {:?}", resp);
}
