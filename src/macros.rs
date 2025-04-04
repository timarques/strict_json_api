#[crabtime::function]
fn generate_wrapper_object(
    pattern!(
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

    if UNSAFE_MARKERS.len() > 0 {
        for marker in UNSAFE_MARKERS[0] {
            crabtime::output! {
                unsafe impl <{{generics}}> {{marker}} for {{NAME}} <{{generics}}>
                where {{clauses}}
                {}
            }
        }
    }

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
        $(
            #[unsafe_markers(
                $(
                    $unsafe_marker:ty
                ),+
            )]
        )?
        $name:ty {
            $(
                $(#[rename($rename:tt)])?
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

    // Extract macro input components
    const NAME: &str = stringify!($name);
    const GENERICS: &[&str] = expand!(&[$(stringify!($generic)),*]);
    const VALUES: &[&str] = expand!(&[$(stringify!($value)),*]);
    const UNSAFE_MARKERS: &[&[&str]] = expand!(&[$(&[$(stringify!($unsafe_marker)),*]),*]);
    const RENAMES: &[&[&str]] = expand!(&[$(&[$(stringify!($rename),),*],)*]);
    const ATTRIBUTES: &[&[&str]] = expand!(&[$(&[$(stringify!($attribute)),+]),*]);
    const CONSTRAINTS: &[&str] = expand!(&[$(stringify!($constraint)),+]);

    // -------------------------------
    // String Buffer Initialization
    // -------------------------------
    let mut generics_string = String::with_capacity(256);
    let mut attributes_string = String::with_capacity(256);
    let mut constraints_string = String::with_capacity(256);
    let mut attributes_meta = Vec::with_capacity(GENERICS.len());

    // -------------------------------
    // Generic Parameter Processing
    // -------------------------------
    for (index, generic) in GENERICS.iter().enumerate() {
        if index > 0 {
            attributes_string.push(',');
            generics_string.push(',');
            constraints_string.push(',');
        }

        generics_string.push_str(generic);

        writeln!(&mut attributes_string, "pub {}: {}", ATTRIBUTES[index][0], VALUES[index]);
        writeln!(&mut constraints_string, "{generic}: {}", CONSTRAINTS[index]);

        attributes_meta.push(if VALUES[index].contains("Option") {
            (true, extract_inner_type(VALUES[index]).leak() as &str)
        } else {
            (false, VALUES[index])
        });
    }

    // -------------------------------
    // Struct Definition Generation
    // -------------------------------
    let constraints_vec: Vec<String> = constraints_string.split(',').map(String::from).collect();

    crabtime::output! {
        #[derive(Debug)]
        pub struct {{NAME}}<{{generics_string}}>
        where {{constraints_string}}
        {
            {{attributes_string}}
        }
    }

    if UNSAFE_MARKERS.len() > 0 {
        for marker in UNSAFE_MARKERS[0] {
            crabtime::output! {
                unsafe impl <{{generics_string}}> {{marker}} for {{NAME}} <{{generics_string}}>
                where {{constraints_string}}
                {}
            }
        }
    }

    // -------------------------------
    // Methods
    // -------------------------------

    let mut normal_methods = String::with_capacity(256);
    for (index, attributes) in ATTRIBUTES.iter().copied().enumerate() {
        let main_attribute = attributes[0];
        let value_type = attributes_meta[index].1;

        if !attributes_meta[index].0 {
            for attribute in attributes.iter() {
                normal_methods.push_str(&crabtime::quote! {
                    #[inline]
                    pub fn {{attribute}}(&self) -> &{{value_type}} {
                        &self.{{main_attribute}}
                    }
                });
                normal_methods.push('\n');
            }

            continue;
        }

        let mut present_constraints = constraints_vec.clone();
        if !present_constraints[index].contains("Present") {
            present_constraints[index].push_str(" + crate::present::Present");

            for attribute in attributes.iter() {
                let attribute = attribute.replace("r#", "");
                normal_methods.push_str(&crabtime::quote! {
                    #[inline]
                    pub fn get_{{attribute}}(&self) -> Option<&{{value_type}}> {
                        self.{{main_attribute}}.as_ref()
                    }
                });
                normal_methods.push('\n');
            }
        }

        let present_constraints_str = present_constraints.join(",");

        let mut accessors = String::with_capacity(96);

        for attribute in attributes.iter() {
            accessors.push_str(&crabtime::quote! {
                #[inline]
                pub fn {{attribute}}(&self) -> &{{value_type}} {
                    unsafe {
                        self.{{main_attribute}}.as_ref().unwrap_unchecked()
                    }
                }
            });
        }

        crabtime::output! {
            impl<{{generics_string}}> {{NAME}}<{{generics_string}}>
            where {{present_constraints_str}}
            {
                {{accessors}}
            }
        }
    }

    crabtime::output! {
        impl<{{generics_string}}> {{NAME}}<{{generics_string}}>
        where {{constraints_string}}
        {
            {{normal_methods}}
        }
    }

    // -------------------------------
    // Clone Implementation
    // -------------------------------
    let mut clonable_constraints = String::with_capacity(256);
    let mut assigns = String::with_capacity(256);

    for (index, attributes) in ATTRIBUTES.iter().enumerate() {
        let constraint = &constraints_vec[index];
        writeln!(&mut clonable_constraints, "{constraint} + Clone,");

        let main_attribute = attributes[0];
        writeln!(assigns, "{main_attribute}: self.{main_attribute}.clone(),");
    }

    crabtime::output! {
        impl<{{generics_string}}> Clone for {{NAME}}<{{generics_string}}>
        where {{clonable_constraints}}
        {
            fn clone(&self) -> Self {
                Self {
                    {{assigns}}
                }
            }
        }
    }

    let mut deserializable = String::with_capacity(256);
    let mut bounds = String::with_capacity(256);
    let mut helper_attributes = String::with_capacity(256);
    let mut assigns = String::with_capacity(256);

    bounds.push('"');

    for (index, generic) in GENERICS.iter().enumerate() {
        let constraint = &constraints_vec[index];
        writeln!(&mut deserializable, "{constraint} + serde::de::DeserializeOwned,");
        writeln!(&mut bounds, "{generic}: serde::de::DeserializeOwned,");

        if attributes_meta[index].0 {
            writeln!(&mut helper_attributes, "#[serde(default)]");
        }

        if RENAMES[index].len() > 0 {
            let rename = RENAMES[index][0];
            writeln!(&mut helper_attributes, "#[serde(rename = \"{rename}\")]");
        }

        let main_attribute = ATTRIBUTES[index][0];
        let attribute_value = VALUES[index];
        writeln!(&mut helper_attributes, "{main_attribute}: {attribute_value},");
        writeln!(&mut assigns, "{main_attribute}: helper.{main_attribute},");
    }

    bounds.push('"');

    crabtime::output! {
        impl<'de, {{generics_string}}> serde::Deserialize<'de> for {{NAME}}<{{generics_string}}>
        where {{deserializable}}
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                use serde::de::DeserializeOwned;
                use serde::Deserialize;

                #[derive(Deserialize)]
                #[serde(bound = {{bounds}})]
                struct Helper<{{generics_string}}>{
                    {{helper_attributes}}
                }

                let helper = Helper::<{{generics_string}}>::deserialize(deserializer)?;

                Ok(Self {
                    {{assigns}}
                })
            }
        }

    }

    let mut serializable = String::with_capacity(256);
    let mut helper_attributes = String::with_capacity(256);
    let mut assigns = String::with_capacity(256);
    let mut bounds = String::with_capacity(256);

    bounds.push('"');

    for (index, generic) in GENERICS.iter().enumerate() {
        let constraint = &constraints_vec[index];
        writeln!(&mut serializable, "{constraint} + serde::Serialize,");
        writeln!(&mut bounds, "{generic}: serde::Serialize,");

        let main_attribute = ATTRIBUTES[index][0];
        let attribute_value = VALUES[index];
        let attribute_name = if RENAMES[index].len() > 0 {
            let original_name = RENAMES[index][0];
            if RENAMES[index][0] == "self" {
                writeln!(&mut helper_attributes, "#[serde(rename = \"{original_name}\")]");
                main_attribute
            } else if RENAMES[index][0].starts_with("r#") {
                let rename = RENAMES[index][0].replacen("r#", "", 1);
                writeln!(&mut helper_attributes, "#[serde(rename = \"{rename}\")]");
                main_attribute
            } else {
                original_name
            }
        } else {
            main_attribute
        };

        writeln!(&mut helper_attributes, "{attribute_name}: &'de {attribute_value},");
        writeln!(&mut assigns, "{attribute_name}: &self.{main_attribute},");
    }

    bounds.push('"');

    crabtime::output! {
        impl<{{generics_string}}> serde::Serialize for {{NAME}}<{{generics_string}}>
        where {{serializable}}
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use serde::Serialize;

                #[derive(Serialize)]
                #[serde(bound = {{bounds}})]
                struct Helper<'de, {{generics_string}}> {
                    {{helper_attributes}}
                }

                let helper = Helper {
                    {{assigns}}
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
