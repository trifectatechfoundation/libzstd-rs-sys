use core::sync::atomic::AtomicI32;
use std::{ffi::CStr, sync::atomic::Ordering};

use libc::{memcpy, size_t, strlen};

static kWords: [&CStr; 255] = [
    c"lorem",
    c"ipsum",
    c"dolor",
    c"sit",
    c"amet",
    c"consectetur",
    c"adipiscing",
    c"elit",
    c"sed",
    c"do",
    c"eiusmod",
    c"tempor",
    c"incididunt",
    c"ut",
    c"labore",
    c"et",
    c"dolore",
    c"magna",
    c"aliqua",
    c"dis",
    c"lectus",
    c"vestibulum",
    c"mattis",
    c"ullamcorper",
    c"velit",
    c"commodo",
    c"a",
    c"lacus",
    c"arcu",
    c"magnis",
    c"parturient",
    c"montes",
    c"nascetur",
    c"ridiculus",
    c"mus",
    c"mauris",
    c"nulla",
    c"malesuada",
    c"pellentesque",
    c"eget",
    c"gravida",
    c"in",
    c"dictum",
    c"non",
    c"erat",
    c"nam",
    c"voluptat",
    c"maecenas",
    c"blandit",
    c"aliquam",
    c"etiam",
    c"enim",
    c"lobortis",
    c"scelerisque",
    c"fermentum",
    c"dui",
    c"faucibus",
    c"ornare",
    c"at",
    c"elementum",
    c"eu",
    c"facilisis",
    c"odio",
    c"morbi",
    c"quis",
    c"eros",
    c"donec",
    c"ac",
    c"orci",
    c"purus",
    c"turpis",
    c"cursus",
    c"leo",
    c"vel",
    c"porta",
    c"consequat",
    c"interdum",
    c"varius",
    c"vulputate",
    c"aliquet",
    c"pharetra",
    c"nunc",
    c"auctor",
    c"urna",
    c"id",
    c"metus",
    c"viverra",
    c"nibh",
    c"cras",
    c"mi",
    c"unde",
    c"omnis",
    c"iste",
    c"natus",
    c"error",
    c"perspiciatis",
    c"voluptatem",
    c"accusantium",
    c"doloremque",
    c"laudantium",
    c"totam",
    c"rem",
    c"aperiam",
    c"eaque",
    c"ipsa",
    c"quae",
    c"ab",
    c"illo",
    c"inventore",
    c"veritatis",
    c"quasi",
    c"architecto",
    c"beatae",
    c"vitae",
    c"dicta",
    c"sunt",
    c"explicabo",
    c"nemo",
    c"ipsam",
    c"quia",
    c"voluptas",
    c"aspernatur",
    c"aut",
    c"odit",
    c"fugit",
    c"consequuntur",
    c"magni",
    c"dolores",
    c"eos",
    c"qui",
    c"ratione",
    c"sequi",
    c"nesciunt",
    c"neque",
    c"porro",
    c"quisquam",
    c"est",
    c"dolorem",
    c"adipisci",
    c"numquam",
    c"eius",
    c"modi",
    c"tempora",
    c"incidunt",
    c"magnam",
    c"quaerat",
    c"ad",
    c"minima",
    c"veniam",
    c"nostrum",
    c"ullam",
    c"corporis",
    c"suscipit",
    c"laboriosam",
    c"nisi",
    c"aliquid",
    c"ex",
    c"ea",
    c"commodi",
    c"consequatur",
    c"autem",
    c"eum",
    c"iure",
    c"voluptate",
    c"esse",
    c"quam",
    c"nihil",
    c"molestiae",
    c"illum",
    c"fugiat",
    c"quo",
    c"pariatur",
    c"vero",
    c"accusamus",
    c"iusto",
    c"dignissimos",
    c"ducimus",
    c"blanditiis",
    c"praesentium",
    c"voluptatum",
    c"deleniti",
    c"atque",
    c"corrupti",
    c"quos",
    c"quas",
    c"molestias",
    c"excepturi",
    c"sint",
    c"occaecati",
    c"cupiditate",
    c"provident",
    c"similique",
    c"culpa",
    c"officia",
    c"deserunt",
    c"mollitia",
    c"animi",
    c"laborum",
    c"dolorum",
    c"fuga",
    c"harum",
    c"quidem",
    c"rerum",
    c"facilis",
    c"expedita",
    c"distinctio",
    c"libero",
    c"tempore",
    c"cum",
    c"soluta",
    c"nobis",
    c"eligendi",
    c"optio",
    c"cumque",
    c"impedit",
    c"minus",
    c"quod",
    c"maxime",
    c"placeat",
    c"facere",
    c"possimus",
    c"assumenda",
    c"repellendus",
    c"temporibus",
    c"quibusdam",
    c"officiis",
    c"debitis",
    c"saepe",
    c"eveniet",
    c"voluptates",
    c"repudiandae",
    c"recusandae",
    c"itaque",
    c"earum",
    c"hic",
    c"tenetur",
    c"sapiente",
    c"delectus",
    c"reiciendis",
    c"cillum",
    c"maiores",
    c"alias",
    c"perferendis",
    c"doloribus",
    c"asperiores",
    c"repellat",
    c"minim",
    c"nostrud",
    c"exercitation",
    c"ullamco",
    c"laboris",
    c"aliquip",
    c"duis",
    c"aute",
    c"irure",
];
static kNbWords: usize = kWords.len();
static kWeights: [core::ffi::c_int; 6] = [0, 8, 6, 4, 3, 2];
static kNbWeights: usize = kWeights.len();
const DISTRIB_MAX_SIZE: core::ffi::c_uint = 650;
static g_distrib: [AtomicI32; DISTRIB_MAX_SIZE as usize] = [const { AtomicI32::new(0) }; 650];
static mut g_distribCount: core::ffi::c_uint = 0;
unsafe fn countFreqs(
    words: *const &CStr,
    nbWords: size_t,
    weights: *const core::ffi::c_int,
    nbWeights: size_t,
) {
    let mut total = 0 as core::ffi::c_uint;
    let mut w: size_t = 0;
    w = 0;
    while w < nbWords {
        let mut len = (*words.add(w)).to_bytes().len();
        let mut lmax: core::ffi::c_int = 0;
        if len >= nbWeights {
            len = nbWeights.wrapping_sub(1);
        }
        lmax = *weights.add(len);
        total = total.wrapping_add(lmax as core::ffi::c_uint);
        w = w.wrapping_add(1);
    }
    g_distribCount = total;
    assert!(g_distribCount <= DISTRIB_MAX_SIZE);
}
unsafe fn init_word_distrib(
    words: *const &CStr,
    nbWords: size_t,
    weights: *const core::ffi::c_int,
    nbWeights: size_t,
) {
    let mut w: size_t = 0;
    let mut d = 0 as size_t;
    countFreqs(words, nbWords, weights, nbWeights);
    w = 0;
    while w < nbWords {
        let mut len = (*words.add(w)).to_bytes().len();
        let mut l: core::ffi::c_int = 0;
        let mut lmax: core::ffi::c_int = 0;
        if len >= nbWeights {
            len = nbWeights.wrapping_sub(1);
        }
        lmax = *weights.add(len);
        l = 0;
        while l < lmax {
            let fresh0 = d;
            d = d.wrapping_add(1);
            g_distrib[fresh0].store(w as i32, Ordering::Relaxed);
            l += 1;
        }
        w = w.wrapping_add(1);
    }
}
static mut g_ptr: *mut core::ffi::c_char = core::ptr::null_mut();
static mut g_nbChars: size_t = 0;
static mut g_maxChars: size_t = 10000000;
static mut g_randRoot: core::ffi::c_uint = 0;
unsafe fn LOREM_rand(range: core::ffi::c_uint) -> core::ffi::c_uint {
    static prime1: core::ffi::c_uint = 2654435761;
    static prime2: core::ffi::c_uint = 2246822519;
    let mut rand32 = g_randRoot;
    rand32 = rand32.wrapping_mul(prime1);
    rand32 ^= prime2;
    rand32 = rand32.rotate_left(13);
    g_randRoot = rand32;
    (core::ffi::c_ulonglong::from(rand32).wrapping_mul(core::ffi::c_ulonglong::from(range)) >> 32)
        as core::ffi::c_uint
}
unsafe fn writeLastCharacters() {
    let lastChars = g_maxChars.wrapping_sub(g_nbChars);
    assert!(g_maxChars >= g_nbChars);
    if lastChars == 0 {
        return;
    }
    let fresh1 = g_nbChars;
    g_nbChars = g_nbChars.wrapping_add(1);
    *g_ptr.add(fresh1) = '.' as i32 as core::ffi::c_char;
    if lastChars > 2 {
        core::ptr::write_bytes(g_ptr.add(g_nbChars), b' ', lastChars.wrapping_sub(2));
    }
    if lastChars > 1 {
        *g_ptr.add(g_maxChars.wrapping_sub(1)) = '\n' as i32 as core::ffi::c_char;
    }
    g_nbChars = g_maxChars;
}
unsafe fn generateWord(
    word: *const core::ffi::c_char,
    separator: *const core::ffi::c_char,
    upCase: core::ffi::c_int,
) {
    let len = (strlen(word)).wrapping_add(strlen(separator));
    if g_nbChars.wrapping_add(len) > g_maxChars {
        writeLastCharacters();
        return;
    }
    memcpy(
        g_ptr.add(g_nbChars) as *mut core::ffi::c_void,
        word as *const core::ffi::c_void,
        strlen(word),
    );
    if upCase != 0 {
        static toUp: core::ffi::c_char = ('A' as i32 - 'a' as i32) as core::ffi::c_char;
        *g_ptr.add(g_nbChars) = (core::ffi::c_int::from(*g_ptr.add(g_nbChars))
            + core::ffi::c_int::from(toUp)) as core::ffi::c_char;
    }
    g_nbChars = g_nbChars.wrapping_add(strlen(word));
    memcpy(
        g_ptr.add(g_nbChars) as *mut core::ffi::c_void,
        separator as *const core::ffi::c_void,
        strlen(separator),
    );
    g_nbChars = g_nbChars.wrapping_add(strlen(separator));
}
unsafe fn about(target: core::ffi::c_uint) -> core::ffi::c_int {
    (LOREM_rand(target))
        .wrapping_add(LOREM_rand(target))
        .wrapping_add(1) as core::ffi::c_int
}
unsafe fn generateSentence(nbWords: core::ffi::c_int) {
    let commaPos = about(9);
    let comma2 = commaPos + about(7);
    let qmark = core::ffi::c_int::from(LOREM_rand(11) == 7);
    let endSep = if qmark != 0 {
        b"? \0" as *const u8 as *const core::ffi::c_char
    } else {
        b". \0" as *const u8 as *const core::ffi::c_char
    };
    let mut i: core::ffi::c_int = 0;
    i = 0;
    while i < nbWords {
        let wordID = g_distrib[LOREM_rand(g_distribCount) as usize].load(Ordering::Relaxed);
        let word = kWords[wordID as usize].as_ptr();
        let mut sep = b" \0" as *const u8 as *const core::ffi::c_char;
        if i == commaPos {
            sep = b", \0" as *const u8 as *const core::ffi::c_char;
        }
        if i == comma2 {
            sep = b", \0" as *const u8 as *const core::ffi::c_char;
        }
        if i == nbWords - 1 {
            sep = endSep;
        }
        generateWord(word, sep, core::ffi::c_int::from(i == 0));
        i += 1;
    }
}
unsafe fn generateParagraph(nbSentences: core::ffi::c_int) {
    let mut i: core::ffi::c_int = 0;
    i = 0;
    while i < nbSentences {
        let wordsPerSentence = about(11);
        generateSentence(wordsPerSentence);
        i += 1;
    }
    if g_nbChars < g_maxChars {
        let fresh2 = g_nbChars;
        g_nbChars = g_nbChars.wrapping_add(1);
        *g_ptr.add(fresh2) = '\n' as i32 as core::ffi::c_char;
    }
    if g_nbChars < g_maxChars {
        let fresh3 = g_nbChars;
        g_nbChars = g_nbChars.wrapping_add(1);
        *g_ptr.add(fresh3) = '\n' as i32 as core::ffi::c_char;
    }
}
unsafe fn generateFirstSentence() {
    let mut i = 0;
    while i < 18 {
        let word = kWords[i].as_ptr();
        let mut separator = b" \0" as *const u8 as *const core::ffi::c_char;
        if i == 4 {
            separator = b", \0" as *const u8 as *const core::ffi::c_char;
        }
        if i == 7 {
            separator = b", \0" as *const u8 as *const core::ffi::c_char;
        }
        generateWord(word, separator, core::ffi::c_int::from(i == 0));
        i += 1;
    }
    generateWord(
        kWords[18].as_ptr(),
        b". \0" as *const u8 as *const core::ffi::c_char,
        0,
    );
}
pub unsafe fn LOREM_genBlock(
    buffer: *mut core::ffi::c_void,
    size: size_t,
    seed: core::ffi::c_uint,
    first: core::ffi::c_int,
    fill: core::ffi::c_int,
) -> size_t {
    g_ptr = buffer as *mut core::ffi::c_char;
    assert!(size < core::ffi::c_int::MAX as size_t);
    g_maxChars = size;
    g_nbChars = 0;
    g_randRoot = seed;
    if g_distribCount == 0 {
        init_word_distrib(kWords.as_ptr(), kNbWords, kWeights.as_ptr(), kNbWeights);
    }
    if first != 0 {
        generateFirstSentence();
    }
    while g_nbChars < g_maxChars {
        let sentencePerParagraph = about(7);
        generateParagraph(sentencePerParagraph);
        if fill == 0 {
            break;
        }
    }
    g_ptr = core::ptr::null_mut();
    g_nbChars
}
pub unsafe fn LOREM_genBuffer(
    buffer: *mut core::ffi::c_void,
    size: size_t,
    seed: core::ffi::c_uint,
) {
    LOREM_genBlock(buffer, size, seed, 1, 1);
}
