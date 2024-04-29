use std::error::Error;

pub struct Patch<'a> {
    pub desc: &'static str,
    search: &'a [u8],
    replace: &'a [u8],
}

impl<'a> Patch<'a> {
    pub fn apply(&self, data: &mut [u8]) -> Result<(), Box<dyn Error>> {
        let mut positions = data
            .windows(self.search.len())
            .enumerate()
            .filter_map(|(i, window)| if window == self.search { Some(i) } else { None });

        match (positions.next(), positions.next()) {
            (Some(_), Some(_)) => Err("Duplicate found".into()),
            (Some(y), None) => {
                data[y..y + self.replace.len()].copy_from_slice(self.replace);
                Ok(())
            }
            _ => Err("Not found".into())
        }
    }
}

pub static PATTERNS: &[Patch] = &[
    Patch {
        desc: "Ryzen Master (v.1.5 -> v2.2)",
        search: &[0x44, 0x39, 0x6D, 0xA8, 0x0F, 0x84, 0xF7],
        replace: &[0x44, 0x39, 0x6D, 0xA8, 0x90, 0xe9, 0xf7],
    },
    Patch {
        desc: "Ryzen Master (v2.3 -> 2.6.0)",
        search: &[0x44, 0x39, 0xad, 0xf8, 0, 0, 0, 0x0f, 0x84],
        replace: &[0x44, 0x39, 0xad, 0xf8, 0, 0, 0, 0x90, 0xe9],
    },
    Patch {
        desc: "Ryzen Master (v2.6.2 -> ?)",
        search: &[0x8D, 0x48, 0xFA, 0x83, 0xF9, 0x01, 0x0F, 0x87],
        // mov     edi, eax
        // lea     ecx, [rax-6]
        // cmp     ecx, 1
        // ja      loc_14001BF3A
        replace: &[0x8D, 0x48, 0xFA, 0x83, 0xF9, 0x01, 0x90, 0xe9],
        // mov     edi, eax
        // lea     ecx, [rax-6]
        // cmp     ecx, 1
        // nop
        // jmp     loc_14001BF3A
    },
    Patch {
        desc: "Ryzen Master (? -> v2.13.0 -> ?)",
        search: &[0xc3, 0xcc, 0x40, 0x55, 0x53, 0x56, 0x57, 0x41],
        replace: &[0xc3, 0xcc, 0xb8, 0x00, 0x00, 0x00, 0x00, 0xc3],
        // mov     EAX, 0x0
        // ret
    },
];

pub fn apply_patches(data: &mut [u8]) -> bool {
    let mut found = false;
    for patch in PATTERNS {
        let r = patch.apply(data);
        if r.is_ok() {
            println!("[+] Patched: {}", patch.desc);
            found = true;
        } else if let Err(e) = r {
            eprintln!("[-] {}: {}", e, patch.desc);
        }
    }
    found
}
