#[crabtime::function]
fn generate_wrapper_object(
    pattern!(
        $(
            #[markers(
                $(
                    $marker:ty
                ),+
            )]
        )?
        $(

            #[unsafe_markers(
                $(
                    $unsafe_marker:ty
                ),+
            )]
        )?
        $name:ty: $_type:ty {
            $(
                $generic:ident: $constraints:ty;
            )+
        }
    ): _,
) {
    use core::fmt::Write;

    const NAME: &str = stringify!($name);
    const TYPE: &str = stringify!($_type);
    const GENERICS: &[&str] = expand!(&[$(stringify!($generic)),*]);
    const CONSTRAINTS: &[&str] = expand!(&[$(stringify!($constraints)),*]);
    const UNSAFE_MARKERS: &[&[&str]] = expand!(&[$(&[$(stringify!($unsafe_marker)),*]),*]);
    const MARKERS: &[&[&str]] = expand!(&[$(&[$(stringify!($marker)),*]),*]);

    let mut clauses = String::with_capacity(96);
    let mut deserialize_clauses = String::with_capacity(96);
    let mut serialize_clauses = String::with_capacity(96);
    let mut clone_clauses = String::with_capacity(96);
    let mut generics = String::with_capacity(96);

    for (generic, constraints) in GENERICS.iter().zip(CONSTRAINTS.iter()) {
        let _ = writeln!(&mut generics, "{generic},");
        let _ = writeln!(&mut clauses, "{generic}: {constraints},");
        let _ = writeln!(
            &mut deserialize_clauses,
            "{generic}: {constraints} + serde::de::DeserializeOwned,"
        );
        let _ = writeln!(&mut serialize_clauses, "{generic}: {constraints} + serde::Serialize,");
        let _ = writeln!(&mut clone_clauses, "{generic}: {constraints} + Clone,");
    }

    crabtime::output! {
        #[derive(Debug)]
        pub struct {{NAME}} <{{generics}}>
        where {{clauses}}
        {
            inner: {{TYPE}},
        }

        impl <{{generics}}> {{NAME}} <{{generics}}>
        where {{clauses}}
        {
            pub fn new(inner: {{TYPE}}) -> Self {
                Self { inner }
            }
        }

        impl <{{generics}}> core::ops::Deref for {{NAME}} <{{generics}}>
        where {{clauses}}
        {
            type Target = {{TYPE}};

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl <{{generics}}> core::ops::DerefMut for {{NAME}} <{{generics}}>
        where {{clauses}}
        {

            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }
    }

    UNSAFE_MARKERS
        .iter()
        .copied()
        .flatten()
        .map(|marker| ("unsafe ", marker))
        .chain(MARKERS.iter().copied().flatten().map(|marker| ("", marker)))
        .for_each(|(prefix, marker)| {
            crabtime::output! {
                {{prefix}} impl <{{generics}}> {{marker}} for {{NAME}} <{{generics}}>
                where {{clauses}}
                {}
            }
        });

    crabtime::output! {
        impl<'de, {{generics}}> serde::Deserialize<'de> for {{NAME}} <{{generics}}>
        where {{deserialize_clauses}}
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let inner = serde::Deserialize::deserialize(deserializer)?;

                Ok(Self { inner })
            }
        }
    }

    crabtime::output! {
        impl<{{generics}}> serde::Serialize for {{NAME}} <{{generics}}>
        where {{serialize_clauses}}
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.inner.serialize(serializer)
            }
        }
    }

    crabtime::output! {
        impl<{{generics}}> Clone for {{NAME}} <{{generics}}>
        where {{clone_clauses}}
        {
            fn clone(&self) -> Self {
                Self {
                    inner: self.inner.clone(),
                }
            }
        }
    }
}

#[crabtime::function]
fn generate_object(
    pattern!(
        $(#[markers($($marker:ty),+)])?
        $(#[unsafe_markers($($unsafe_marker:ty),+)])?
        $name:ty {
            $(
                $(#[
                    $flag:ident $(($arg:tt))?
                ])*

                $generic:ident: $constraint:ty : $($attribute:ident),+ : $value:ty ;
            )+
        }
    ): _,
) {
    // -------------------------------
    // Inner Type Extraction Utility
    // -------------------------------
    /// Extracts inner type from wrapper types like Option<T>
    fn extract_inner_type(text: &str) -> String {
        text.find('<')
            .and_then(|start| text.rfind('>').map(|end| text[start + 1..end].to_string()))
            .unwrap_or_default()
    }

    // -------------------------------
    // Input Parsing & Preparation
    // -------------------------------
    use core::fmt::Write;

    const STRUCT_NAME: &str = stringify!($name);
    const GENERIC_TYPES: &[&str] = expand!(&[$(stringify!($generic)),*]);
    const VALUE_TYPES: &[&str] = expand!(&[$(stringify!($value)),*]);
    const TRAIT_MARKERS: &[&[&str]] = expand!(&[$(&[$(stringify!($marker)),*]),*]);
    const UNSAFE_TRAIT_MARKERS: &[&[&str]] = expand!(&[$(&[$(stringify!($unsafe_marker)),*]),*]);
    const ATTRIBUTE_NAMES: &[&[&str]] = expand!(&[$(&[$(stringify!($attribute)),+]),*]);
    const GENERIC_CONSTRAINTS: &[&str] = expand!(&[$(stringify!($constraint)),+]);
    const FLAGS: &[&[&str]] = expand!(&[$(&[$(stringify!($flag)),*]),*]);
    const FLAGS_ARGUMENTS: &[&[&[&str]]] = expand!(
        &[
            $(
                &[
                    $(
                        &[
                            $(stringify!($arg)),*
                        ]
                    ),*
                ]
            ),*
        ]
    );

    let mut flags_map = std::collections::HashMap::new();

    for (index, flags) in FLAGS.iter().enumerate() {
        for flags_index in 0..flags.len() {
            let flag = flags[flags_index];
            let arguments = FLAGS_ARGUMENTS[index][flags_index];
            flags_map.insert((index, flag), arguments);
        }
    }

    // -------------------------------
    // String Buffer Initialization
    // -------------------------------

    let mut generic_types_string = String::with_capacity(256);
    let mut struct_fields_string = String::with_capacity(256);
    let mut constraint_definitions_string = String::with_capacity(256);

    let mut extracted_inner_value_types = Vec::with_capacity(VALUE_TYPES.len());
    let mut constraint_definitions_list = Vec::with_capacity(GENERIC_CONSTRAINTS.len());
    let mut escaped_attribute_names = vec![vec![]; ATTRIBUTE_NAMES.len()];
    let mut contains_present_constraint = vec![false; GENERIC_CONSTRAINTS.len()];
    let mut is_value_optional_list = vec![false; VALUE_TYPES.len()];

    // -------------------------------
    // Generic Type Processing
    // -------------------------------
    for (index, generic_type) in GENERIC_TYPES.iter().enumerate() {
        let attributes = ATTRIBUTE_NAMES[index];
        let main_attribute = attributes[0];
        let value_type = VALUE_TYPES[index];
        let generic_constrains = GENERIC_CONSTRAINTS[index];

        writeln!(&mut generic_types_string, "{generic_type},");
        writeln!(&mut constraint_definitions_string, "{generic_type}: {generic_constrains},");
        writeln!(&mut struct_fields_string, "{main_attribute}: {value_type},");

        constraint_definitions_list.push(format!("{generic_type}: {generic_constrains}"));

        contains_present_constraint[index] = generic_constrains.contains("Present");

        for attribute in attributes {
            let escaped_attribute_name = attribute.replace("r#", "");
            escaped_attribute_names[index].push(escaped_attribute_name);
        }

        if value_type.contains("Option") {
            is_value_optional_list[index] = true;
            extracted_inner_value_types.push(extract_inner_type(value_type).leak() as &str);
        } else {
            extracted_inner_value_types.push(value_type);
        }
    }

    // -------------------------------
    // Methods
    // -------------------------------

    let mut methods_definitions_string = String::with_capacity(256);
    for (index, attribute_names) in ATTRIBUTE_NAMES.iter().copied().enumerate() {
        let main_attribute_name = attribute_names[0];
        let value_type = extracted_inner_value_types[index];

        // is a T not Option<T>
        if !is_value_optional_list[index] {
            for attribute_name in attribute_names.iter() {
                methods_definitions_string.push_str(&crabtime::quote! {
                    #[inline]
                    pub fn {{attribute_name}}(&self) -> &{{value_type}} {
                        &self.{{main_attribute_name}}
                    }
                });
            }
        // is a Option<T> with Present constraint
        } else if contains_present_constraint[index] {
            for attribute_name in attribute_names.iter() {
                methods_definitions_string.push_str(&crabtime::quote! {
                    #[inline]
                    pub fn {{attribute_name}}(&self) -> &{{value_type}} {
                        unsafe {
                            self.{{main_attribute_name}}.as_ref().unwrap_unchecked()
                        }
                    }
                })
            }
        // is a Option<T>
        } else {
            for escaped_attribute_name in &escaped_attribute_names[index] {
                methods_definitions_string.push_str(&crabtime::quote! {
                    #[inline]
                    pub fn get_{{escaped_attribute_name}}(&self) -> Option<&{{value_type}}> {
                        self.{{main_attribute_name}}.as_ref()
                    }
                });
            }
        }
    }

    // -------------------------------
    // Struct Definition Generation
    // -------------------------------

    crabtime::output! {
        #[derive(Debug)]
        pub struct {{STRUCT_NAME}}<{{generic_types_string}}>
        where {{constraint_definitions_string}}
        {
            {{struct_fields_string}}
        }

        impl<{{generic_types_string}}> {{STRUCT_NAME}}<{{generic_types_string}}>
        where {{constraint_definitions_string}}
        {
            {{methods_definitions_string}}
        }
    }

    // -------------------------------
    // Trait Markers
    // -------------------------------

    UNSAFE_TRAIT_MARKERS
        .iter()
        .copied()
        .flatten()
        .map(|marker| ("unsafe ", marker))
        .chain(
            TRAIT_MARKERS
                .iter()
                .copied()
                .flatten()
                .map(|marker| ("", marker)),
        )
        .for_each(|(prefix, marker)| {
            crabtime::output! {
                {{prefix}} impl <{{generic_types_string}}> {{marker}} for {{STRUCT_NAME}} <{{generic_types_string}}>
                where {{constraint_definitions_string}}
                {}
            }
        });

    // -------------------------------
    // Present acessors
    // -------------------------------

    for (index, attributes_names) in ATTRIBUTE_NAMES.iter().copied().enumerate() {
        // is a T or have Present constraint
        if !is_value_optional_list[index] || contains_present_constraint[index] {
            continue;
        }

        let main_attribute_name = attributes_names[0];
        let value_type = extracted_inner_value_types[index];

        let mut present_constraints_string = String::with_capacity(96);
        for (list_index, constraint) in constraint_definitions_list.iter().enumerate() {
            if list_index == index {
                writeln!(
                    &mut present_constraints_string,
                    "{constraint} + crate::present::Present,"
                );
            } else {
                writeln!(&mut present_constraints_string, "{constraint},");
            }
        }

        let mut accessors = String::with_capacity(96);

        for attribute_name in attributes_names.iter() {
            accessors.push_str(&crabtime::quote! {
                #[inline]
                pub fn {{attribute_name}}(&self) -> &{{value_type}} {
                    unsafe {
                        self.{{main_attribute_name}}.as_ref().unwrap_unchecked()
                    }
                }
            });
        }

        crabtime::output! {
            impl<{{generic_types_string}}> {{STRUCT_NAME}}<{{generic_types_string}}>
            where {{present_constraints_string}}
            {
                {{accessors}}
            }
        }
    }

    // -------------------------------
    // Clone Implementation
    // -------------------------------
    let mut clonable_constraints_string = String::with_capacity(256);
    let mut clone_field_assignments = String::with_capacity(256);

    for (index, attribute_names) in ATTRIBUTE_NAMES.iter().enumerate() {
        let constraint_definition = &constraint_definitions_list[index];
        writeln!(&mut clonable_constraints_string, "{constraint_definition} + Clone,");

        let main_attribute_name = attribute_names[0];
        writeln!(
            clone_field_assignments,
            "{main_attribute_name}: self.{main_attribute_name}.clone(),"
        );
    }

    crabtime::output! {
        impl<{{generic_types_string}}> Clone for {{STRUCT_NAME}}<{{generic_types_string}}>
        where {{clonable_constraints_string}}
        {
            fn clone(&self) -> Self {
                Self {
                    {{clone_field_assignments}}
                }
            }
        }
    }

    // -------------------------------
    // Deserialize Implementation
    // -------------------------------

    let mut deserializable_constraints_string = String::with_capacity(256);
    let mut deserialize_bounds = String::with_capacity(256);
    let mut helper_struct_fields = String::with_capacity(256);
    let mut deserialization_assignments = String::with_capacity(256);

    deserialize_bounds.push('"');

    for (index, generic_type) in GENERIC_TYPES.iter().enumerate() {
        writeln!(&mut deserialize_bounds, "{generic_type}: serde::de::DeserializeOwned,");

        writeln!(
            &mut deserializable_constraints_string,
            "{} + serde::de::DeserializeOwned,",
            &constraint_definitions_list[index]
        );

        if is_value_optional_list[index] {
            writeln!(&mut helper_struct_fields, "#[serde(default)]");
        }

        if let Some(flatten) = flags_map.get(&(index, "flatten")) {
            writeln!(&mut helper_struct_fields, "#[serde(flatten)]");
        }

        if let Some(rename_arguments) = flags_map.get(&(index, "rename")) {
            writeln!(&mut helper_struct_fields, "#[serde(rename = \"{}\")]", rename_arguments[0]);
        }

        let main_attribute_name = ATTRIBUTE_NAMES[index][0];
        let attribute_value_type = VALUE_TYPES[index];
        writeln!(&mut helper_struct_fields, "{main_attribute_name}: {attribute_value_type},");
        writeln!(
            &mut deserialization_assignments,
            "{main_attribute_name}: helper.{main_attribute_name},"
        );
    }

    deserialize_bounds.push('"');

    crabtime::output! {
        impl<'de, {{generic_types_string}}> serde::Deserialize<'de> for {{STRUCT_NAME}}<{{generic_types_string}}>
        where {{deserializable_constraints_string}}
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                use serde::de::DeserializeOwned;
                use serde::Deserialize;

                #[derive(Deserialize)]
                #[serde(bound = {{deserialize_bounds}})]
                struct Helper<{{generic_types_string}}>{
                    {{helper_struct_fields}}
                }

                let helper = Helper::<{{generic_types_string}}>::deserialize(deserializer)?;

                Ok(Self {
                    {{deserialization_assignments}}
                })
            }
        }

    }

    // -------------------------------
    // Serialize Implementation
    // -------------------------------

    let mut serializable_constraints_string = String::with_capacity(256);
    let mut helper_struct_fields = String::with_capacity(256);
    let mut serialize_assignments = String::with_capacity(256);
    let mut serialize_bounds = String::with_capacity(256);

    serialize_bounds.push('"');

    for (index, generic_type) in GENERIC_TYPES.iter().enumerate() {
        writeln!(
            &mut serializable_constraints_string,
            "{} + serde::Serialize,",
            &constraint_definitions_list[index]
        );
        writeln!(&mut serialize_bounds, "{generic_type}: serde::Serialize,");

        let main_attribute_name = ATTRIBUTE_NAMES[index][0];
        let attribute_value = VALUE_TYPES[index];
        let mut attribute_name = main_attribute_name;

        if let Some(rename_arguments) = flags_map.get(&(index, "rename")) {
            let original_name = rename_arguments[0];

            if original_name == "self" {
                writeln!(&mut helper_struct_fields, "#[serde(rename = \"{original_name}\")]");
            } else if original_name.starts_with("r#") {
                let original_name = original_name.replacen("r#", "", 1);
                writeln!(&mut helper_struct_fields, "#[serde(rename = \"{original_name}\")]");
            } else {
                attribute_name = original_name;
            }
        }

        writeln!(&mut helper_struct_fields, "{attribute_name}: &'de {attribute_value},");
        writeln!(&mut serialize_assignments, "{attribute_name}: &self.{main_attribute_name},");
    }

    serialize_bounds.push('"');

    crabtime::output! {
        impl<{{generic_types_string}}> serde::Serialize for {{STRUCT_NAME}}<{{generic_types_string}}>
        where {{serializable_constraints_string}}
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use serde::Serialize;

                #[derive(Serialize)]
                #[serde(bound = {{serialize_bounds}})]
                struct Helper<'de, {{generic_types_string}}> {
                    {{helper_struct_fields}}
                }

                let helper = Helper {
                    {{serialize_assignments}}
                };

                helper.serialize(serializer)
            }
        }
    }
}

#[allow(clippy::single_component_path_imports)]
pub(super) use generate_object;
#[allow(clippy::single_component_path_imports)]
pub(super) use generate_wrapper_object;
