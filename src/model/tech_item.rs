use std::marker::PhantomData;

use wasm_bindgen::UnwrapThrowExt;

use crate::utils::builder_validation::{BuilderValidator, No, Yes};

#[derive(Debug, Clone)]
pub struct RelatedItem {
    pub id: usize,
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum TechType {
    ProgrammingLanguage,
    Technology,
}

#[derive(Debug, Clone)]
pub struct TechItem {
    pub id: usize,
    pub name: String,
    pub desc: String,
    pub related_tech: Vec<RelatedItem>,
    pub tech_type: TechType,
}

impl TechItem {
    pub fn builder<'a>() -> TechItemBuilder<'a, No, No, No> {
        TechItemBuilder::new()
    }
}

#[derive(Debug)]
pub struct TechItemBuilder<'a, HasId, HasName, HasType>
where
    HasId: BuilderValidator,
    HasName: BuilderValidator,
    HasType: BuilderValidator,
{
    _has_id: PhantomData<HasId>,
    _has_name: PhantomData<HasName>,
    _has_type: PhantomData<HasType>,
    id: Option<usize>,
    name: Option<String>,
    desc: Option<String>,
    related_tech: Vec<&'a RelatedItem>,
    tech_type: Option<TechType>,
}

impl<'a, HasId, HasName, HasType> TechItemBuilder<'a, HasId, HasName, HasType>
where
    HasId: BuilderValidator + Sized + Send + Sync,
    HasName: BuilderValidator,
    HasType: BuilderValidator,
{
    pub fn with_desc(mut self, desc: String) -> Self {
        self.desc = Some(desc);
        self
    }

    pub fn with_related_tech(mut self, related: &[&'a RelatedItem]) -> Self {
        self.related_tech.extend(related);
        self
    }

    pub fn with_type(mut self, tech_type: TechType) -> TechItemBuilder<'a, HasId, HasName, Yes> {
        self.tech_type = Some(tech_type);
        TechItemBuilder {
            _has_type: PhantomData,
            ..self
        }
    }

    pub fn with_name(mut self, name: String) -> TechItemBuilder<'a, HasId, Yes, HasType> {
        self.name = Some(name);
        TechItemBuilder {
            _has_name: PhantomData,
            ..self
        }
    }

    pub fn with_id(mut self, id: usize) -> TechItemBuilder<'a, Yes, HasName, HasType> {
        self.id = Some(id);
        TechItemBuilder {
            _has_id: PhantomData,
            ..self
        }
    }
}

impl<'a> TechItemBuilder<'a, No, No, No> {
    pub fn new() -> Self {
        TechItemBuilder {
            _has_id: PhantomData,
            _has_name: PhantomData,
            _has_type: PhantomData,
            desc: None,
            id: None,
            name: None,
            related_tech: Vec::new(),
            tech_type: None,
        }
    }
}

impl<'a> TechItemBuilder<'a, Yes, Yes, Yes> {
    pub fn build(self) -> TechItem {
        TechItem {
            id: self.id.unwrap(),
            desc: self.desc.unwrap_or("".into()),
            name: self.name.unwrap(),
            related_tech: self.related_tech.into_iter().cloned().collect(),
            tech_type: self.tech_type.unwrap(),
        }
    }
}

pub fn mock_tech_data() -> Vec<TechItem> {
    let ts_related = RelatedItem {
        id: 1,
        name: "TypeScript".into(),
    };

    let cpp_related = RelatedItem {
        id: 3,
        name: "C++".into(),
    };

    let rs_related = RelatedItem {
        id: 2,
        name: "Rust".into(),
    };

    vec![
        TechItem::builder()
            .with_id(1)
            .with_name("TypeScript".into())
            .with_type(TechType::ProgrammingLanguage)
            .with_related_tech(&[])
            .build(),
        TechItem::builder()
            .with_id(2)
            .with_name("Rust".into())
            .with_type(TechType::ProgrammingLanguage)
            .with_related_tech(&[])
            .build(),
        TechItem::builder()
            .with_id(3)
            .with_name("C++".into())
            .with_type(TechType::ProgrammingLanguage)
            .with_related_tech(&[])
            .build(),
        TechItem::builder()
            .with_id(4)
            .with_name("Angular".into())
            .with_type(TechType::Technology)
            .with_related_tech(&[&ts_related])
            .with_desc("The most reasonable Frontend Framework".into())
            .build(),
        TechItem::builder()
            .with_id(5)
            .with_name("Vulkan".into())
            .with_type(TechType::Technology)
            .with_related_tech(&[&cpp_related, &rs_related])
            .with_desc("The greatest graphics API".into())
            .build(),
    ]
}
