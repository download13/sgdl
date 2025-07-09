#[macro_export]
macro_rules! generate_update_type {
    // Match a struct definition
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $($field_vis:vis $field_name:ident: $field_type:ty),*
            $(,)?
        }
    ) => {
        // Create the original struct
        $(#[$meta])*
				#[derive($($original_derive),*)]
        $vis struct $name {
            $($field_vis $field_name: $field_type),*
        }

        // Create the optional version with "Update" suffix
        $(#[$meta])*
				#[derive($($update_derive),*)]
        $vis struct ${concat($name, Update)} {
            $($field_vis $field_name: Option<$field_type>),*
        }
    };
}
