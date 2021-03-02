table! {
    class (kh) {
        kh -> Bpchar,
        km -> Nullable<Varchar>,
        xf -> Nullable<Int4>,
        xs -> Nullable<Int4>,
        yxh -> Nullable<Bpchar>,
    }
}

table! {
    department (yxh) {
        yxh -> Bpchar,
        mc -> Varchar,
        dz -> Varchar,
        lxdh -> Varchar,
    }
}

table! {
    openclass (kh, gh, xq) {
        xq -> Varchar,
        kh -> Bpchar,
        gh -> Bpchar,
        sksj -> Nullable<Varchar>,
    }
}

table! {
    student (xh) {
        xh -> Bpchar,
        xm -> Varchar,
        xb -> Bpchar,
        csrq -> Nullable<Date>,
        jg -> Nullable<Varchar>,
        sjhm -> Varchar,
        yxh -> Bpchar,
    }
}

table! {
    teacher (gh) {
        gh -> Bpchar,
        xm -> Varchar,
        xb -> Bpchar,
        csrq -> Bpchar,
        zc -> Nullable<Varchar>,
        yxh -> Bpchar,
    }
}

table! {
    xuanketable (xh, xq, kh, gh) {
        xh -> Bpchar,
        xq -> Varchar,
        kh -> Bpchar,
        gh -> Bpchar,
        cj -> Nullable<Int4>,
    }
}

joinable!(class -> department (yxh));
joinable!(openclass -> class (kh));
joinable!(openclass -> teacher (gh));
joinable!(student -> department (yxh));
joinable!(teacher -> department (yxh));

allow_tables_to_appear_in_same_query!(
    class,
    department,
    openclass,
    student,
    teacher,
    xuanketable,
);
