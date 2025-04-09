#[crabtime::function]
fn generate_markers(
    pattern!(
        $(
            $name:ident:
            $first_constraint:ty $(,$constraint:ty)*
            {
                $(
                    $(#[$flag:ident])?
                    $_type:ty;
                )*
            }
        )+
    ): _,
) {
    const NAMES: &[&str] = expand!(&[$(stringify!($name)),+]);
    const FIRST_CONSTRAINTS: &[&str] = expand!(&[$(stringify!($first_constraint)),+]);
    const CONSTRAINTS: &[&[&str]] = expand!(&[$(&[$(stringify!($constraint)),*]),*]);
    const TYPES: &[&[&str]] = expand!(&[$(&[$(stringify!($_type)),*]),*]);
    const FLAGS: &[&[&[&str]]] = expand!(&[$(&[$(&[$(stringify!($flag)),*]),*]),*]);

    use core::fmt::Write;

    for (index, name) in NAMES.iter().copied().enumerate() {
        let mut constraints = String::with_capacity(32);
        constraints.push_str(FIRST_CONSTRAINTS[index]);

        for constraint in CONSTRAINTS[index] {
            constraints.push_str(" + ");
            constraints.push_str(constraint);
        }

        crabtime::output! {
            pub trait {{name}}: {{constraints}} {}
        }

        for (type_index, _type) in TYPES[index].iter().enumerate() {
            match FLAGS[index][type_index].get(0).copied() {
                Some("wrap") => {
                    crabtime::output! {
                        impl <T> {{name}} for {{_type}}<T> where T: {{name}} {}
                    }
                }
                Some("dyn") => {
                    crabtime::output! {
                        impl <T> {{name}} for T where T: {{_type}} {}
                    }
                }
                None => {
                    crabtime::output! {
                        impl {{name}} for {{_type}} {}
                    }
                }
                _ => {
                    panic!("bad flag")
                }
            }
        }
    }
}

#[crabtime::function]
fn generate_alias(
    pattern!(
        $(
            #[mark(
                $(
                    $marker:ty
                ),+
            )]
        )?
        $(

            #[unsafe_mark(
                $(
                    $unsafe_marker:ty
                ),+
            )]
        )?
        $name:ty: $_type:ty $({
            $(
                $generic:ident: $constraints:ty;
            )+
        })?
    ): _,
) {
    use core::fmt::Write;

    const NAME: &str = stringify!($name);
    const TYPE: &str = stringify!($_type);
    const GENERICS: &[&[&str]] = expand!(&[$(&[$(stringify!($generic)),*]),*]);
    const CONSTRAINTS: &[&[&str]] = expand!(&[$(&[$(stringify!($constraints)),*]),*]);
    const UNSAFE_MARKERS: &[&[&str]] = expand!(&[$(&[$(stringify!($unsafe_marker)),*]),*]);
    const MARKERS: &[&[&str]] = expand!(&[$(&[$(stringify!($marker)),*]),*]);

    let mut clauses = String::with_capacity(96);
    let mut deserialize_clauses = String::with_capacity(96);
    let mut serialize_clauses = String::with_capacity(96);
    let mut clone_clauses = String::with_capacity(96);
    let mut generics = String::with_capacity(96);

    for (generic, constraints) in GENERICS
        .iter()
        .copied()
        .flatten()
        .zip(CONSTRAINTS.iter().copied().flatten())
    {
        let _ = writeln!(&mut generics, "{generic},");
        let _ = writeln!(&mut clauses, "{generic}: {constraints},");
    }

    crabtime::output! {
        pub type {{NAME}}<{{generics}}> = {{TYPE}};
    }

    UNSAFE_MARKERS
        .iter()
        .copied()
        .flatten()
        .map(|marker| ("unsafe ", marker))
        .chain(MARKERS.iter().copied().flatten().map(|marker| ("", marker)))
        .for_each(|(prefix, marker)| {
            crabtime::output! {
                {{prefix}} impl <{{generics}}> {{marker}} for {{TYPE}}
                where {{clauses}}
                {}
            }
        });
}

#[crabtime::function]
fn generate_object(
    pattern!(
        $(#[mark($($marker:ty),+)])?
        $(#[unsafe_mark($($unsafe_marker:ty),+)])?
        $name:ty
        {
            $(
                $(#[
                    $flag:ident $(($flag_argument:tt))?
                ])*
                $first_attribute_name:ident $(,$attribute_name:ident)*:
                $value_type:ty
                $(:$constraint:ty)?;
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
            .unwrap_or(text.to_string())
    }

    // -------------------------------
    // Input Parsing & Preparation
    // -------------------------------
    use core::fmt::Write;

    const STRUCT_NAME: &str = stringify!($name);
    const CONSTRAINTS: &[&[&str]] = expand!(&[$(&[$(stringify!($constraint)),*]),*]);
    const VALUE_TYPES: &[&str] = expand!(&[$(stringify!($value_type)),*]);
    const TRAIT_MARKERS: &[&[&str]] = expand!(&[$(&[$(stringify!($marker)),*]),*]);
    const UNSAFE_TRAIT_MARKERS: &[&[&str]] = expand!(&[$(&[$(stringify!($unsafe_marker)),*]),*]);
    const FIRST_ATTRIBUTE_NAMES: &[&str] = expand!(&[$(stringify!($first_attribute_name)),*]);
    const ATTRIBUTE_NAMES: &[&[&str]] = expand!(&[$(&[$(stringify!($attribute_name)),*]),*]);
    const FLAGS: &[&[&str]] = expand!(&[$(&[$(stringify!($flag)),*]),*]);
    const FLAGS_ARGUMENTS: &[&[&[&str]]] =
        expand!(&[$(&[$(&[$(stringify!($flag_argument)),*]),*]),*]);

    // // ----------------------------
    // // Flags maps
    // // ----------------------------

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
    let mut constructor_assignments = String::with_capacity(64);

    let mut attribute_names = vec![vec![]; FIRST_ATTRIBUTE_NAMES.len()];
    let mut escaped_attribute_names = vec![vec![]; ATTRIBUTE_NAMES.len()];
    let mut extracted_inner_value_types = vec![String::new(); VALUE_TYPES.len()];
    let mut is_value_optional_list = vec![false; VALUE_TYPES.len()];

    let mut constraint_definitions_list = vec![None; CONSTRAINTS.len()];
    let mut contains_present_constraint = vec![false; CONSTRAINTS.len()];

    // -------------------------------
    // Generic Type Processing
    // -------------------------------
    //
    for (index, &main_attribute) in FIRST_ATTRIBUTE_NAMES.iter().enumerate() {
        let value_type = VALUE_TYPES[index];

        writeln!(&mut struct_fields_string, "{main_attribute}: {value_type},").unwrap();

        constructor_assignments.push_str(main_attribute);
        constructor_assignments.push(',');

        for attribute_name in
            core::iter::once(main_attribute).chain(ATTRIBUTE_NAMES[index].iter().copied())
        {
            escaped_attribute_names[index].push(attribute_name.replace("r#", ""));
            attribute_names[index].push(attribute_name);
        }

        extracted_inner_value_types[index] = extract_inner_type(value_type);
        is_value_optional_list[index] = value_type.contains("Option");

        if let Some(constraint) = CONSTRAINTS[index].get(0) {
            let generic = &extracted_inner_value_types[index];
            let generic_constraint = format!("{generic}: {constraint}");

            writeln!(&mut generic_types_string, "{generic},").unwrap();

            constraint_definitions_string.push_str(&generic_constraint);
            constraint_definitions_string.push(',');
            constraint_definitions_string.push('\n');

            constraint_definitions_list[index] = Some(generic_constraint);
            contains_present_constraint[index] = constraint.contains("IsPresent");
        }
    }

    // -------------------------------
    // Methods
    // -------------------------------

    let mut methods_definitions_string = String::with_capacity(256);
    for (index, attribute_names) in attribute_names.iter().enumerate() {
        let main_attribute_name = &attribute_names[0];

        // is a T not Option<T>
        if !is_value_optional_list[index] {
            let value_type = VALUE_TYPES[index];
            for attribute_name in attribute_names.iter() {
                methods_definitions_string.push_str(&crabtime::quote! {
                    #[inline]
                    pub const fn {{attribute_name}}(&self) -> &{{value_type}} {
                        &self.{{main_attribute_name}}
                    }
                });
            }
        // is a Option<T> with IsPresent constraint
        } else if contains_present_constraint[index] {
            let value_type = &extracted_inner_value_types[index];
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
            let value_type = &extracted_inner_value_types[index];
            for escaped_attribute_name in &escaped_attribute_names[index] {
                methods_definitions_string.push_str(&crabtime::quote! {
                    #[inline]
                    pub const fn get_{{escaped_attribute_name}}(&self) -> Option<&{{value_type}}> {
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
            pub const fn new({{struct_fields_string}}) -> Self {
                Self {
                    {{constructor_assignments}}
                }
            }

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
    // IsPresent acessors
    // -------------------------------

    for (index, attributes_names) in attribute_names.iter().enumerate() {
        // is a T or doesn't have constraint or have IsPresent constraint
        if !is_value_optional_list[index]
            || constraint_definitions_list[index].is_none()
            || contains_present_constraint[index]
        {
            continue;
        }

        let mut present_constraints_string = String::with_capacity(96);
        for (list_index, constraint_definition) in constraint_definitions_list.iter().enumerate() {
            if let Some(constraint_definition) = &constraint_definition {
                if list_index == index {
                    writeln!(
                        &mut present_constraints_string,
                        "{constraint_definition} + crate::present::IsPresent,"
                    )
                    .unwrap();
                } else {
                    writeln!(&mut present_constraints_string, "{constraint_definition},").unwrap();
                }
            }
        }

        let main_attribute_name = &attributes_names[0];
        let value_type = &extracted_inner_value_types[index];

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

    for (index, attribute_names) in attribute_names.iter().enumerate() {
        if let Some(constraint_definition) = &constraint_definitions_list[index] {
            writeln!(&mut clonable_constraints_string, "{constraint_definition} + Clone,").unwrap();
        }

        let main_attribute_name = &attribute_names[0];
        let _ = writeln!(
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

    for (index, constraint_definition) in constraint_definitions_list.iter().enumerate() {
        if let Some(constraint_definition) = &constraint_definition {
            let generic_type = &extracted_inner_value_types[index];
            writeln!(&mut deserialize_bounds, "{generic_type}: serde::de::DeserializeOwned,");
            writeln!(
                &mut deserializable_constraints_string,
                "{constraint_definition} + serde::de::DeserializeOwned,"
            );
        }

        if is_value_optional_list[index] {
            writeln!(&mut helper_struct_fields, "#[serde(default)]");
        }

        if flags_map.get(&(index, "flatten")).is_some() {
            writeln!(&mut helper_struct_fields, "#[serde(flatten)]");
        }

        if let Some(rename_arguments) = flags_map.get(&(index, "rename")) {
            let rename = rename_arguments[0].replace("r#", "");
            writeln!(&mut helper_struct_fields, "#[serde(rename = \"{rename}\")]");
        }

        let main_attribute_name = FIRST_ATTRIBUTE_NAMES[index];
        let attribute_value_type = VALUE_TYPES[index];
        writeln!(&mut helper_struct_fields, "{main_attribute_name}: {attribute_value_type},")
            .unwrap();

        writeln!(
            &mut deserialization_assignments,
            "{main_attribute_name}: helper.{main_attribute_name},"
        )
        .unwrap();
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

    for (index, constraint_definition) in constraint_definitions_list.iter().enumerate() {
        if let Some(constraint_definition) = &constraint_definition {
            let generic_type = &extracted_inner_value_types[index];
            writeln!(
                &mut serializable_constraints_string,
                "{constraint_definition} + serde::Serialize,",
            );
            writeln!(&mut serialize_bounds, "{generic_type}: serde::Serialize,");
        }

        let main_attribute_name = FIRST_ATTRIBUTE_NAMES[index];
        let attribute_value = VALUE_TYPES[index];
        let mut attribute_name = main_attribute_name;

        if flags_map.get(&(index, "flatten")).is_some() {
            writeln!(&mut helper_struct_fields, "#[serde(flatten)]");
        }

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
pub(super) use generate_alias;
#[allow(clippy::single_component_path_imports)]
pub(super) use generate_markers;
#[allow(clippy::single_component_path_imports)]
pub(super) use generate_object;
