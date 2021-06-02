table! {
    admin (account) {
        account -> Varchar,
        password -> Nullable<Varchar>,
    }
}

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
        mc -> Nullable<Varchar>,
        dz -> Nullable<Varchar>,
        lxdh -> Nullable<Varchar>,
    }
}

table! {
    openclass (xq, kh, gh) {
        xq -> Varchar,
        kh -> Bpchar,
        gh -> Bpchar,
        sksj -> Nullable<Varchar>,
        num -> Nullable<Int4>,
    }
}

table! {
    student (xh) {
        xh -> Bpchar,
        xm -> Nullable<Varchar>,
        xb -> Nullable<Bpchar>,
        csrq -> Nullable<Timestamp>,
        jg -> Nullable<Varchar>,
        sjhm -> Nullable<Varchar>,
        yxh -> Bpchar,
    }
}

table! {
    teacher (gh) {
        gh -> Bpchar,
        xm -> Nullable<Varchar>,
        xb -> Nullable<Bpchar>,
        csrq -> Nullable<Timestamp>,
        zc -> Nullable<Varchar>,
        yxh -> Bpchar,
    }
}

table! {
    terms (id) {
        term -> Varchar,
        id -> Int4,
    }
}

table! {
    xuanketable (xh, xq, kh, gh) {
        xh -> Bpchar,
        xq -> Varchar,
        kh -> Bpchar,
        gh -> Bpchar,
        zpcj -> Nullable<Int4>,
        grade -> Nullable<Float4>,
    }
}

joinable!(class -> department (yxh));
joinable!(openclass -> class (kh));
joinable!(openclass -> teacher (gh));
joinable!(student -> department (yxh));
joinable!(teacher -> department (yxh));

allow_tables_to_appear_in_same_query!(
    admin,
    class,
    department,
    openclass,
    student,
    teacher,
    terms,
    xuanketable,
);
