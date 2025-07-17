extern "C" {
    fn __assert_fail(
        __assertion: *const std::ffi::c_char,
        __file: *const std::ffi::c_char,
        __line: std::ffi::c_uint,
        __function: *const std::ffi::c_char,
    ) -> !;
    fn memcpy(
        _: *mut std::ffi::c_void,
        _: *const std::ffi::c_void,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
    fn memset(
        _: *mut std::ffi::c_void,
        _: std::ffi::c_int,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
    fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
}
pub type size_t = std::ffi::c_ulong;
pub const NULL: std::ffi::c_int = 0 as std::ffi::c_int;
static mut kWords: [*const std::ffi::c_char; 255] = [
    b"lorem\0" as *const u8 as *const std::ffi::c_char,
    b"ipsum\0" as *const u8 as *const std::ffi::c_char,
    b"dolor\0" as *const u8 as *const std::ffi::c_char,
    b"sit\0" as *const u8 as *const std::ffi::c_char,
    b"amet\0" as *const u8 as *const std::ffi::c_char,
    b"consectetur\0" as *const u8 as *const std::ffi::c_char,
    b"adipiscing\0" as *const u8 as *const std::ffi::c_char,
    b"elit\0" as *const u8 as *const std::ffi::c_char,
    b"sed\0" as *const u8 as *const std::ffi::c_char,
    b"do\0" as *const u8 as *const std::ffi::c_char,
    b"eiusmod\0" as *const u8 as *const std::ffi::c_char,
    b"tempor\0" as *const u8 as *const std::ffi::c_char,
    b"incididunt\0" as *const u8 as *const std::ffi::c_char,
    b"ut\0" as *const u8 as *const std::ffi::c_char,
    b"labore\0" as *const u8 as *const std::ffi::c_char,
    b"et\0" as *const u8 as *const std::ffi::c_char,
    b"dolore\0" as *const u8 as *const std::ffi::c_char,
    b"magna\0" as *const u8 as *const std::ffi::c_char,
    b"aliqua\0" as *const u8 as *const std::ffi::c_char,
    b"dis\0" as *const u8 as *const std::ffi::c_char,
    b"lectus\0" as *const u8 as *const std::ffi::c_char,
    b"vestibulum\0" as *const u8 as *const std::ffi::c_char,
    b"mattis\0" as *const u8 as *const std::ffi::c_char,
    b"ullamcorper\0" as *const u8 as *const std::ffi::c_char,
    b"velit\0" as *const u8 as *const std::ffi::c_char,
    b"commodo\0" as *const u8 as *const std::ffi::c_char,
    b"a\0" as *const u8 as *const std::ffi::c_char,
    b"lacus\0" as *const u8 as *const std::ffi::c_char,
    b"arcu\0" as *const u8 as *const std::ffi::c_char,
    b"magnis\0" as *const u8 as *const std::ffi::c_char,
    b"parturient\0" as *const u8 as *const std::ffi::c_char,
    b"montes\0" as *const u8 as *const std::ffi::c_char,
    b"nascetur\0" as *const u8 as *const std::ffi::c_char,
    b"ridiculus\0" as *const u8 as *const std::ffi::c_char,
    b"mus\0" as *const u8 as *const std::ffi::c_char,
    b"mauris\0" as *const u8 as *const std::ffi::c_char,
    b"nulla\0" as *const u8 as *const std::ffi::c_char,
    b"malesuada\0" as *const u8 as *const std::ffi::c_char,
    b"pellentesque\0" as *const u8 as *const std::ffi::c_char,
    b"eget\0" as *const u8 as *const std::ffi::c_char,
    b"gravida\0" as *const u8 as *const std::ffi::c_char,
    b"in\0" as *const u8 as *const std::ffi::c_char,
    b"dictum\0" as *const u8 as *const std::ffi::c_char,
    b"non\0" as *const u8 as *const std::ffi::c_char,
    b"erat\0" as *const u8 as *const std::ffi::c_char,
    b"nam\0" as *const u8 as *const std::ffi::c_char,
    b"voluptat\0" as *const u8 as *const std::ffi::c_char,
    b"maecenas\0" as *const u8 as *const std::ffi::c_char,
    b"blandit\0" as *const u8 as *const std::ffi::c_char,
    b"aliquam\0" as *const u8 as *const std::ffi::c_char,
    b"etiam\0" as *const u8 as *const std::ffi::c_char,
    b"enim\0" as *const u8 as *const std::ffi::c_char,
    b"lobortis\0" as *const u8 as *const std::ffi::c_char,
    b"scelerisque\0" as *const u8 as *const std::ffi::c_char,
    b"fermentum\0" as *const u8 as *const std::ffi::c_char,
    b"dui\0" as *const u8 as *const std::ffi::c_char,
    b"faucibus\0" as *const u8 as *const std::ffi::c_char,
    b"ornare\0" as *const u8 as *const std::ffi::c_char,
    b"at\0" as *const u8 as *const std::ffi::c_char,
    b"elementum\0" as *const u8 as *const std::ffi::c_char,
    b"eu\0" as *const u8 as *const std::ffi::c_char,
    b"facilisis\0" as *const u8 as *const std::ffi::c_char,
    b"odio\0" as *const u8 as *const std::ffi::c_char,
    b"morbi\0" as *const u8 as *const std::ffi::c_char,
    b"quis\0" as *const u8 as *const std::ffi::c_char,
    b"eros\0" as *const u8 as *const std::ffi::c_char,
    b"donec\0" as *const u8 as *const std::ffi::c_char,
    b"ac\0" as *const u8 as *const std::ffi::c_char,
    b"orci\0" as *const u8 as *const std::ffi::c_char,
    b"purus\0" as *const u8 as *const std::ffi::c_char,
    b"turpis\0" as *const u8 as *const std::ffi::c_char,
    b"cursus\0" as *const u8 as *const std::ffi::c_char,
    b"leo\0" as *const u8 as *const std::ffi::c_char,
    b"vel\0" as *const u8 as *const std::ffi::c_char,
    b"porta\0" as *const u8 as *const std::ffi::c_char,
    b"consequat\0" as *const u8 as *const std::ffi::c_char,
    b"interdum\0" as *const u8 as *const std::ffi::c_char,
    b"varius\0" as *const u8 as *const std::ffi::c_char,
    b"vulputate\0" as *const u8 as *const std::ffi::c_char,
    b"aliquet\0" as *const u8 as *const std::ffi::c_char,
    b"pharetra\0" as *const u8 as *const std::ffi::c_char,
    b"nunc\0" as *const u8 as *const std::ffi::c_char,
    b"auctor\0" as *const u8 as *const std::ffi::c_char,
    b"urna\0" as *const u8 as *const std::ffi::c_char,
    b"id\0" as *const u8 as *const std::ffi::c_char,
    b"metus\0" as *const u8 as *const std::ffi::c_char,
    b"viverra\0" as *const u8 as *const std::ffi::c_char,
    b"nibh\0" as *const u8 as *const std::ffi::c_char,
    b"cras\0" as *const u8 as *const std::ffi::c_char,
    b"mi\0" as *const u8 as *const std::ffi::c_char,
    b"unde\0" as *const u8 as *const std::ffi::c_char,
    b"omnis\0" as *const u8 as *const std::ffi::c_char,
    b"iste\0" as *const u8 as *const std::ffi::c_char,
    b"natus\0" as *const u8 as *const std::ffi::c_char,
    b"error\0" as *const u8 as *const std::ffi::c_char,
    b"perspiciatis\0" as *const u8 as *const std::ffi::c_char,
    b"voluptatem\0" as *const u8 as *const std::ffi::c_char,
    b"accusantium\0" as *const u8 as *const std::ffi::c_char,
    b"doloremque\0" as *const u8 as *const std::ffi::c_char,
    b"laudantium\0" as *const u8 as *const std::ffi::c_char,
    b"totam\0" as *const u8 as *const std::ffi::c_char,
    b"rem\0" as *const u8 as *const std::ffi::c_char,
    b"aperiam\0" as *const u8 as *const std::ffi::c_char,
    b"eaque\0" as *const u8 as *const std::ffi::c_char,
    b"ipsa\0" as *const u8 as *const std::ffi::c_char,
    b"quae\0" as *const u8 as *const std::ffi::c_char,
    b"ab\0" as *const u8 as *const std::ffi::c_char,
    b"illo\0" as *const u8 as *const std::ffi::c_char,
    b"inventore\0" as *const u8 as *const std::ffi::c_char,
    b"veritatis\0" as *const u8 as *const std::ffi::c_char,
    b"quasi\0" as *const u8 as *const std::ffi::c_char,
    b"architecto\0" as *const u8 as *const std::ffi::c_char,
    b"beatae\0" as *const u8 as *const std::ffi::c_char,
    b"vitae\0" as *const u8 as *const std::ffi::c_char,
    b"dicta\0" as *const u8 as *const std::ffi::c_char,
    b"sunt\0" as *const u8 as *const std::ffi::c_char,
    b"explicabo\0" as *const u8 as *const std::ffi::c_char,
    b"nemo\0" as *const u8 as *const std::ffi::c_char,
    b"ipsam\0" as *const u8 as *const std::ffi::c_char,
    b"quia\0" as *const u8 as *const std::ffi::c_char,
    b"voluptas\0" as *const u8 as *const std::ffi::c_char,
    b"aspernatur\0" as *const u8 as *const std::ffi::c_char,
    b"aut\0" as *const u8 as *const std::ffi::c_char,
    b"odit\0" as *const u8 as *const std::ffi::c_char,
    b"fugit\0" as *const u8 as *const std::ffi::c_char,
    b"consequuntur\0" as *const u8 as *const std::ffi::c_char,
    b"magni\0" as *const u8 as *const std::ffi::c_char,
    b"dolores\0" as *const u8 as *const std::ffi::c_char,
    b"eos\0" as *const u8 as *const std::ffi::c_char,
    b"qui\0" as *const u8 as *const std::ffi::c_char,
    b"ratione\0" as *const u8 as *const std::ffi::c_char,
    b"sequi\0" as *const u8 as *const std::ffi::c_char,
    b"nesciunt\0" as *const u8 as *const std::ffi::c_char,
    b"neque\0" as *const u8 as *const std::ffi::c_char,
    b"porro\0" as *const u8 as *const std::ffi::c_char,
    b"quisquam\0" as *const u8 as *const std::ffi::c_char,
    b"est\0" as *const u8 as *const std::ffi::c_char,
    b"dolorem\0" as *const u8 as *const std::ffi::c_char,
    b"adipisci\0" as *const u8 as *const std::ffi::c_char,
    b"numquam\0" as *const u8 as *const std::ffi::c_char,
    b"eius\0" as *const u8 as *const std::ffi::c_char,
    b"modi\0" as *const u8 as *const std::ffi::c_char,
    b"tempora\0" as *const u8 as *const std::ffi::c_char,
    b"incidunt\0" as *const u8 as *const std::ffi::c_char,
    b"magnam\0" as *const u8 as *const std::ffi::c_char,
    b"quaerat\0" as *const u8 as *const std::ffi::c_char,
    b"ad\0" as *const u8 as *const std::ffi::c_char,
    b"minima\0" as *const u8 as *const std::ffi::c_char,
    b"veniam\0" as *const u8 as *const std::ffi::c_char,
    b"nostrum\0" as *const u8 as *const std::ffi::c_char,
    b"ullam\0" as *const u8 as *const std::ffi::c_char,
    b"corporis\0" as *const u8 as *const std::ffi::c_char,
    b"suscipit\0" as *const u8 as *const std::ffi::c_char,
    b"laboriosam\0" as *const u8 as *const std::ffi::c_char,
    b"nisi\0" as *const u8 as *const std::ffi::c_char,
    b"aliquid\0" as *const u8 as *const std::ffi::c_char,
    b"ex\0" as *const u8 as *const std::ffi::c_char,
    b"ea\0" as *const u8 as *const std::ffi::c_char,
    b"commodi\0" as *const u8 as *const std::ffi::c_char,
    b"consequatur\0" as *const u8 as *const std::ffi::c_char,
    b"autem\0" as *const u8 as *const std::ffi::c_char,
    b"eum\0" as *const u8 as *const std::ffi::c_char,
    b"iure\0" as *const u8 as *const std::ffi::c_char,
    b"voluptate\0" as *const u8 as *const std::ffi::c_char,
    b"esse\0" as *const u8 as *const std::ffi::c_char,
    b"quam\0" as *const u8 as *const std::ffi::c_char,
    b"nihil\0" as *const u8 as *const std::ffi::c_char,
    b"molestiae\0" as *const u8 as *const std::ffi::c_char,
    b"illum\0" as *const u8 as *const std::ffi::c_char,
    b"fugiat\0" as *const u8 as *const std::ffi::c_char,
    b"quo\0" as *const u8 as *const std::ffi::c_char,
    b"pariatur\0" as *const u8 as *const std::ffi::c_char,
    b"vero\0" as *const u8 as *const std::ffi::c_char,
    b"accusamus\0" as *const u8 as *const std::ffi::c_char,
    b"iusto\0" as *const u8 as *const std::ffi::c_char,
    b"dignissimos\0" as *const u8 as *const std::ffi::c_char,
    b"ducimus\0" as *const u8 as *const std::ffi::c_char,
    b"blanditiis\0" as *const u8 as *const std::ffi::c_char,
    b"praesentium\0" as *const u8 as *const std::ffi::c_char,
    b"voluptatum\0" as *const u8 as *const std::ffi::c_char,
    b"deleniti\0" as *const u8 as *const std::ffi::c_char,
    b"atque\0" as *const u8 as *const std::ffi::c_char,
    b"corrupti\0" as *const u8 as *const std::ffi::c_char,
    b"quos\0" as *const u8 as *const std::ffi::c_char,
    b"quas\0" as *const u8 as *const std::ffi::c_char,
    b"molestias\0" as *const u8 as *const std::ffi::c_char,
    b"excepturi\0" as *const u8 as *const std::ffi::c_char,
    b"sint\0" as *const u8 as *const std::ffi::c_char,
    b"occaecati\0" as *const u8 as *const std::ffi::c_char,
    b"cupiditate\0" as *const u8 as *const std::ffi::c_char,
    b"provident\0" as *const u8 as *const std::ffi::c_char,
    b"similique\0" as *const u8 as *const std::ffi::c_char,
    b"culpa\0" as *const u8 as *const std::ffi::c_char,
    b"officia\0" as *const u8 as *const std::ffi::c_char,
    b"deserunt\0" as *const u8 as *const std::ffi::c_char,
    b"mollitia\0" as *const u8 as *const std::ffi::c_char,
    b"animi\0" as *const u8 as *const std::ffi::c_char,
    b"laborum\0" as *const u8 as *const std::ffi::c_char,
    b"dolorum\0" as *const u8 as *const std::ffi::c_char,
    b"fuga\0" as *const u8 as *const std::ffi::c_char,
    b"harum\0" as *const u8 as *const std::ffi::c_char,
    b"quidem\0" as *const u8 as *const std::ffi::c_char,
    b"rerum\0" as *const u8 as *const std::ffi::c_char,
    b"facilis\0" as *const u8 as *const std::ffi::c_char,
    b"expedita\0" as *const u8 as *const std::ffi::c_char,
    b"distinctio\0" as *const u8 as *const std::ffi::c_char,
    b"libero\0" as *const u8 as *const std::ffi::c_char,
    b"tempore\0" as *const u8 as *const std::ffi::c_char,
    b"cum\0" as *const u8 as *const std::ffi::c_char,
    b"soluta\0" as *const u8 as *const std::ffi::c_char,
    b"nobis\0" as *const u8 as *const std::ffi::c_char,
    b"eligendi\0" as *const u8 as *const std::ffi::c_char,
    b"optio\0" as *const u8 as *const std::ffi::c_char,
    b"cumque\0" as *const u8 as *const std::ffi::c_char,
    b"impedit\0" as *const u8 as *const std::ffi::c_char,
    b"minus\0" as *const u8 as *const std::ffi::c_char,
    b"quod\0" as *const u8 as *const std::ffi::c_char,
    b"maxime\0" as *const u8 as *const std::ffi::c_char,
    b"placeat\0" as *const u8 as *const std::ffi::c_char,
    b"facere\0" as *const u8 as *const std::ffi::c_char,
    b"possimus\0" as *const u8 as *const std::ffi::c_char,
    b"assumenda\0" as *const u8 as *const std::ffi::c_char,
    b"repellendus\0" as *const u8 as *const std::ffi::c_char,
    b"temporibus\0" as *const u8 as *const std::ffi::c_char,
    b"quibusdam\0" as *const u8 as *const std::ffi::c_char,
    b"officiis\0" as *const u8 as *const std::ffi::c_char,
    b"debitis\0" as *const u8 as *const std::ffi::c_char,
    b"saepe\0" as *const u8 as *const std::ffi::c_char,
    b"eveniet\0" as *const u8 as *const std::ffi::c_char,
    b"voluptates\0" as *const u8 as *const std::ffi::c_char,
    b"repudiandae\0" as *const u8 as *const std::ffi::c_char,
    b"recusandae\0" as *const u8 as *const std::ffi::c_char,
    b"itaque\0" as *const u8 as *const std::ffi::c_char,
    b"earum\0" as *const u8 as *const std::ffi::c_char,
    b"hic\0" as *const u8 as *const std::ffi::c_char,
    b"tenetur\0" as *const u8 as *const std::ffi::c_char,
    b"sapiente\0" as *const u8 as *const std::ffi::c_char,
    b"delectus\0" as *const u8 as *const std::ffi::c_char,
    b"reiciendis\0" as *const u8 as *const std::ffi::c_char,
    b"cillum\0" as *const u8 as *const std::ffi::c_char,
    b"maiores\0" as *const u8 as *const std::ffi::c_char,
    b"alias\0" as *const u8 as *const std::ffi::c_char,
    b"perferendis\0" as *const u8 as *const std::ffi::c_char,
    b"doloribus\0" as *const u8 as *const std::ffi::c_char,
    b"asperiores\0" as *const u8 as *const std::ffi::c_char,
    b"repellat\0" as *const u8 as *const std::ffi::c_char,
    b"minim\0" as *const u8 as *const std::ffi::c_char,
    b"nostrud\0" as *const u8 as *const std::ffi::c_char,
    b"exercitation\0" as *const u8 as *const std::ffi::c_char,
    b"ullamco\0" as *const u8 as *const std::ffi::c_char,
    b"laboris\0" as *const u8 as *const std::ffi::c_char,
    b"aliquip\0" as *const u8 as *const std::ffi::c_char,
    b"duis\0" as *const u8 as *const std::ffi::c_char,
    b"aute\0" as *const u8 as *const std::ffi::c_char,
    b"irure\0" as *const u8 as *const std::ffi::c_char,
];
static mut kNbWords: std::ffi::c_uint = 0;
static mut kWeights: [std::ffi::c_int; 6] = [
    0 as std::ffi::c_int,
    8 as std::ffi::c_int,
    6 as std::ffi::c_int,
    4 as std::ffi::c_int,
    3 as std::ffi::c_int,
    2 as std::ffi::c_int,
];
static mut kNbWeights: size_t = 0;
static mut g_distrib: [std::ffi::c_int; 650] = [0 as std::ffi::c_int; 650];
static mut g_distribCount: std::ffi::c_uint = 0 as std::ffi::c_int as std::ffi::c_uint;
unsafe extern "C" fn countFreqs(
    mut words: *mut *const std::ffi::c_char,
    mut nbWords: size_t,
    mut weights: *const std::ffi::c_int,
    mut nbWeights: size_t,
) {
    let mut total = 0 as std::ffi::c_int as std::ffi::c_uint;
    let mut w: size_t = 0;
    w = 0 as std::ffi::c_int as size_t;
    while w < nbWords {
        let mut len = strlen(*words.offset(w as isize));
        let mut lmax: std::ffi::c_int = 0;
        if len >= nbWeights {
            len = nbWeights.wrapping_sub(1 as std::ffi::c_int as size_t);
        }
        lmax = *weights.offset(len as isize);
        total = total.wrapping_add(lmax as std::ffi::c_uint);
        w = w.wrapping_add(1);
        w;
    }
    g_distribCount = total;
    if g_distribCount <= 650 as std::ffi::c_int as std::ffi::c_uint {
    } else {
        __assert_fail(
            b"g_distribCount <= DISTRIB_SIZE_MAX\0" as *const u8 as *const std::ffi::c_char,
            b"lorem.c\0" as *const u8 as *const std::ffi::c_char,
            122 as std::ffi::c_int as std::ffi::c_uint,
            (*::core::mem::transmute::<&[u8; 60], &[std::ffi::c_char; 60]>(
                b"void countFreqs(const char **, size_t, const int *, size_t)\0",
            ))
            .as_ptr(),
        );
    }
    'c_1439: {
        if g_distribCount <= 650 as std::ffi::c_int as std::ffi::c_uint {
        } else {
            __assert_fail(
                b"g_distribCount <= DISTRIB_SIZE_MAX\0" as *const u8 as *const std::ffi::c_char,
                b"lorem.c\0" as *const u8 as *const std::ffi::c_char,
                122 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<&[u8; 60], &[std::ffi::c_char; 60]>(
                    b"void countFreqs(const char **, size_t, const int *, size_t)\0",
                ))
                .as_ptr(),
            );
        }
    };
}
unsafe extern "C" fn init_word_distrib(
    mut words: *mut *const std::ffi::c_char,
    mut nbWords: size_t,
    mut weights: *const std::ffi::c_int,
    mut nbWeights: size_t,
) {
    let mut w: size_t = 0;
    let mut d = 0 as std::ffi::c_int as size_t;
    countFreqs(words, nbWords, weights, nbWeights);
    w = 0 as std::ffi::c_int as size_t;
    while w < nbWords {
        let mut len = strlen(*words.offset(w as isize));
        let mut l: std::ffi::c_int = 0;
        let mut lmax: std::ffi::c_int = 0;
        if len >= nbWeights {
            len = nbWeights.wrapping_sub(1 as std::ffi::c_int as size_t);
        }
        lmax = *weights.offset(len as isize);
        l = 0 as std::ffi::c_int;
        while l < lmax {
            let fresh0 = d;
            d = d.wrapping_add(1);
            *g_distrib.as_mut_ptr().offset(fresh0 as isize) = w as std::ffi::c_int;
            l += 1;
            l;
        }
        w = w.wrapping_add(1);
        w;
    }
}
static mut g_ptr: *mut std::ffi::c_char = NULL as *mut std::ffi::c_char;
static mut g_nbChars: size_t = 0 as std::ffi::c_int as size_t;
static mut g_maxChars: size_t = 10000000 as std::ffi::c_int as size_t;
static mut g_randRoot: std::ffi::c_uint = 0 as std::ffi::c_int as std::ffi::c_uint;
unsafe extern "C" fn LOREM_rand(mut range: std::ffi::c_uint) -> std::ffi::c_uint {
    static mut prime1: std::ffi::c_uint = 2654435761 as std::ffi::c_uint;
    static mut prime2: std::ffi::c_uint = 2246822519 as std::ffi::c_uint;
    let mut rand32 = g_randRoot;
    rand32 = rand32.wrapping_mul(prime1);
    rand32 ^= prime2;
    rand32 =
        rand32 << 13 as std::ffi::c_int | rand32 >> (32 as std::ffi::c_int - 13 as std::ffi::c_int);
    g_randRoot = rand32;
    ((rand32 as std::ffi::c_ulonglong).wrapping_mul(range as std::ffi::c_ulonglong)
        >> 32 as std::ffi::c_int) as std::ffi::c_uint
}
unsafe extern "C" fn writeLastCharacters() {
    let mut lastChars = g_maxChars.wrapping_sub(g_nbChars);
    if g_maxChars >= g_nbChars {
    } else {
        __assert_fail(
            b"g_maxChars >= g_nbChars\0" as *const u8 as *const std::ffi::c_char,
            b"lorem.c\0" as *const u8 as *const std::ffi::c_char,
            168 as std::ffi::c_int as std::ffi::c_uint,
            (*::core::mem::transmute::<&[u8; 31], &[std::ffi::c_char; 31]>(
                b"void writeLastCharacters(void)\0",
            ))
            .as_ptr(),
        );
    }
    'c_1048: {
        if g_maxChars >= g_nbChars {
        } else {
            __assert_fail(
                b"g_maxChars >= g_nbChars\0" as *const u8 as *const std::ffi::c_char,
                b"lorem.c\0" as *const u8 as *const std::ffi::c_char,
                168 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<&[u8; 31], &[std::ffi::c_char; 31]>(
                    b"void writeLastCharacters(void)\0",
                ))
                .as_ptr(),
            );
        }
    };
    if lastChars == 0 as std::ffi::c_int as size_t {
        return;
    }
    let fresh1 = g_nbChars;
    g_nbChars = g_nbChars.wrapping_add(1);
    *g_ptr.offset(fresh1 as isize) = '.' as i32 as std::ffi::c_char;
    if lastChars > 2 as std::ffi::c_int as size_t {
        memset(
            g_ptr.offset(g_nbChars as isize) as *mut std::ffi::c_void,
            ' ' as i32,
            lastChars.wrapping_sub(2 as std::ffi::c_int as size_t),
        );
    }
    if lastChars > 1 as std::ffi::c_int as size_t {
        *g_ptr.offset(g_maxChars.wrapping_sub(1 as std::ffi::c_int as size_t) as isize) =
            '\n' as i32 as std::ffi::c_char;
    }
    g_nbChars = g_maxChars;
}
unsafe extern "C" fn generateWord(
    mut word: *const std::ffi::c_char,
    mut separator: *const std::ffi::c_char,
    mut upCase: std::ffi::c_int,
) {
    let len = (strlen(word)).wrapping_add(strlen(separator));
    if g_nbChars.wrapping_add(len) > g_maxChars {
        writeLastCharacters();
        return;
    }
    memcpy(
        g_ptr.offset(g_nbChars as isize) as *mut std::ffi::c_void,
        word as *const std::ffi::c_void,
        strlen(word),
    );
    if upCase != 0 {
        static mut toUp: std::ffi::c_char = ('A' as i32 - 'a' as i32) as std::ffi::c_char;
        *g_ptr.offset(g_nbChars as isize) = (*g_ptr.offset(g_nbChars as isize) as std::ffi::c_int
            + toUp as std::ffi::c_int)
            as std::ffi::c_char;
    }
    g_nbChars = (g_nbChars as std::ffi::c_ulong).wrapping_add(strlen(word)) as size_t as size_t;
    memcpy(
        g_ptr.offset(g_nbChars as isize) as *mut std::ffi::c_void,
        separator as *const std::ffi::c_void,
        strlen(separator),
    );
    g_nbChars =
        (g_nbChars as std::ffi::c_ulong).wrapping_add(strlen(separator)) as size_t as size_t;
}
unsafe extern "C" fn about(mut target: std::ffi::c_uint) -> std::ffi::c_int {
    (LOREM_rand(target))
        .wrapping_add(LOREM_rand(target))
        .wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint) as std::ffi::c_int
}
unsafe extern "C" fn generateSentence(mut nbWords: std::ffi::c_int) {
    let mut commaPos = about(9 as std::ffi::c_int as std::ffi::c_uint);
    let mut comma2 = commaPos + about(7 as std::ffi::c_int as std::ffi::c_uint);
    let mut qmark = (LOREM_rand(11 as std::ffi::c_int as std::ffi::c_uint)
        == 7 as std::ffi::c_int as std::ffi::c_uint) as std::ffi::c_int;
    let mut endSep = if qmark != 0 {
        b"? \0" as *const u8 as *const std::ffi::c_char
    } else {
        b". \0" as *const u8 as *const std::ffi::c_char
    };
    let mut i: std::ffi::c_int = 0;
    i = 0 as std::ffi::c_int;
    while i < nbWords {
        let wordID = *g_distrib
            .as_mut_ptr()
            .offset(LOREM_rand(g_distribCount) as isize);
        let word = *kWords.as_mut_ptr().offset(wordID as isize);
        let mut sep = b" \0" as *const u8 as *const std::ffi::c_char;
        if i == commaPos {
            sep = b", \0" as *const u8 as *const std::ffi::c_char;
        }
        if i == comma2 {
            sep = b", \0" as *const u8 as *const std::ffi::c_char;
        }
        if i == nbWords - 1 as std::ffi::c_int {
            sep = endSep;
        }
        generateWord(word, sep, (i == 0 as std::ffi::c_int) as std::ffi::c_int);
        i += 1;
        i;
    }
}
unsafe extern "C" fn generateParagraph(mut nbSentences: std::ffi::c_int) {
    let mut i: std::ffi::c_int = 0;
    i = 0 as std::ffi::c_int;
    while i < nbSentences {
        let mut wordsPerSentence = about(11 as std::ffi::c_int as std::ffi::c_uint);
        generateSentence(wordsPerSentence);
        i += 1;
        i;
    }
    if g_nbChars < g_maxChars {
        let fresh2 = g_nbChars;
        g_nbChars = g_nbChars.wrapping_add(1);
        *g_ptr.offset(fresh2 as isize) = '\n' as i32 as std::ffi::c_char;
    }
    if g_nbChars < g_maxChars {
        let fresh3 = g_nbChars;
        g_nbChars = g_nbChars.wrapping_add(1);
        *g_ptr.offset(fresh3 as isize) = '\n' as i32 as std::ffi::c_char;
    }
}
unsafe extern "C" fn generateFirstSentence() {
    let mut i: std::ffi::c_int = 0;
    i = 0 as std::ffi::c_int;
    while i < 18 as std::ffi::c_int {
        let mut word = *kWords.as_mut_ptr().offset(i as isize);
        let mut separator = b" \0" as *const u8 as *const std::ffi::c_char;
        if i == 4 as std::ffi::c_int {
            separator = b", \0" as *const u8 as *const std::ffi::c_char;
        }
        if i == 7 as std::ffi::c_int {
            separator = b", \0" as *const u8 as *const std::ffi::c_char;
        }
        generateWord(
            word,
            separator,
            (i == 0 as std::ffi::c_int) as std::ffi::c_int,
        );
        i += 1;
        i;
    }
    generateWord(
        *kWords.as_mut_ptr().offset(18 as std::ffi::c_int as isize),
        b". \0" as *const u8 as *const std::ffi::c_char,
        0 as std::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn LOREM_genBlock(
    mut buffer: *mut std::ffi::c_void,
    mut size: size_t,
    mut seed: std::ffi::c_uint,
    mut first: std::ffi::c_int,
    mut fill: std::ffi::c_int,
) -> size_t {
    g_ptr = buffer as *mut std::ffi::c_char;
    if size < 2147483647 as std::ffi::c_int as size_t {
    } else {
        __assert_fail(
            b"size < INT_MAX\0" as *const u8 as *const std::ffi::c_char,
            b"lorem.c\0" as *const u8 as *const std::ffi::c_char,
            261 as std::ffi::c_int as std::ffi::c_uint,
            (*::core::mem::transmute::<&[u8; 62], &[std::ffi::c_char; 62]>(
                b"size_t LOREM_genBlock(void *, size_t, unsigned int, int, int)\0",
            ))
            .as_ptr(),
        );
    }
    'c_1558: {
        if size < 2147483647 as std::ffi::c_int as size_t {
        } else {
            __assert_fail(
                b"size < INT_MAX\0" as *const u8 as *const std::ffi::c_char,
                b"lorem.c\0" as *const u8 as *const std::ffi::c_char,
                261 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<&[u8; 62], &[std::ffi::c_char; 62]>(
                    b"size_t LOREM_genBlock(void *, size_t, unsigned int, int, int)\0",
                ))
                .as_ptr(),
            );
        }
    };
    g_maxChars = size;
    g_nbChars = 0 as std::ffi::c_int as size_t;
    g_randRoot = seed;
    if g_distribCount == 0 as std::ffi::c_int as std::ffi::c_uint {
        init_word_distrib(
            kWords.as_mut_ptr(),
            kNbWords as size_t,
            kWeights.as_ptr(),
            kNbWeights,
        );
    }
    if first != 0 {
        generateFirstSentence();
    }
    while g_nbChars < g_maxChars {
        let mut sentencePerParagraph = about(7 as std::ffi::c_int as std::ffi::c_uint);
        generateParagraph(sentencePerParagraph);
        if fill == 0 {
            break;
        }
    }
    g_ptr = NULL as *mut std::ffi::c_char;
    g_nbChars
}
#[no_mangle]
pub unsafe extern "C" fn LOREM_genBuffer(
    mut buffer: *mut std::ffi::c_void,
    mut size: size_t,
    mut seed: std::ffi::c_uint,
) {
    LOREM_genBlock(
        buffer,
        size,
        seed,
        1 as std::ffi::c_int,
        1 as std::ffi::c_int,
    );
}
unsafe extern "C" fn run_static_initializers() {
    kNbWords = (::core::mem::size_of::<[*const std::ffi::c_char; 255]>() as std::ffi::c_ulong)
        .wrapping_div(::core::mem::size_of::<*const std::ffi::c_char>() as std::ffi::c_ulong)
        as std::ffi::c_uint;
    kNbWeights = (::core::mem::size_of::<[std::ffi::c_int; 6]>() as std::ffi::c_ulong)
        .wrapping_div(::core::mem::size_of::<std::ffi::c_int>() as std::ffi::c_ulong);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
