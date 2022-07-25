use syn::Field;

macro_rules! path_contains {
    ($ident:ident, $segment:tt) => {
        match &$ident.ty {
            syn::Type::Path(path) => {
                path.path.segments.iter().next().unwrap().ident.to_string().to_lowercase().contains($segment)
            }
            _ => false,
        }
    }
}

pub(crate) fn is_uuid(field: &Field) -> bool {
    path_contains!(field, "uuid")
}

pub(crate) fn is_datetime(field: &Field) -> bool {
    path_contains!(field, "datetime")
}

pub(crate) fn is_events_vec(field: &Field) -> bool {
    true

    // match &field.ty {
    //     syn::Type::Path(path) => {
    //         path.path.segments.iter().next().unwrap().ident.to_string().to_lowercase().contains("vec")
    //     }
    //     _ => false,
    // }
}