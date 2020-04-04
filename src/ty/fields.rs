// Copyright 2019-2020
//     by  Centrality Investments Ltd.
//     and Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::tm_std::*;

use crate::{
	form::{CompactForm, Form, MetaForm},
	IntoCompact, MetaType, Metadata, Registry,
};
use serde::Serialize;

// Field::named_of_generic<Option<bool>>

// How to identify common generic type - using Path?

// How to recognise generic type

// idea to enforce registering the correct  - must implement GenericTypeX
// perhaps first do non type safe API then add that afterwards

// pub trait Generic1 {
// 	type A;
// }

#[derive(derive_more::From)]
pub enum FieldType<F: Form = MetaForm> {
	Generic(F::String),
	Concrete(F::Type),
}

impl IntoCompact for FieldType {
	type Output = FieldType<CompactForm>;

	fn into_compact(self, registry: &mut Registry) -> Self::Output {
		match self {
			FieldType::Concrete(ref ty) => registry.register_type(ty).into(),
			FieldType::Generic(ref name) => registry.register_string(name).into(),
		}
	}
}

/// A field of a struct like data type.
///
/// Name is optional so it can represent both named and unnamed fields.
///
/// This can be a named field of a struct type or a struct variant.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize)]
#[serde(bound = "F::TypeId: Serialize")]
pub struct Field<F: Form = MetaForm> {
	/// The name of the field. None for unnamed fields.
	#[serde(skip_serializing_if = "Option::is_none")]
	name: Option<F::String>,
	/// The type of the field.
	#[serde(rename = "type")]
	ty: FieldType<F>,
}

impl IntoCompact for Field {
	type Output = Field<CompactForm>;

	fn into_compact(self, registry: &mut Registry) -> Self::Output {
		Field {
			name: self.name.map(|name| registry.register_string(name)),
			ty: self.ty.into_compact(registry),
		}
	}
}

impl Field {
	/// Creates a new field.
	///
	/// Use this constructor if you want to instantiate from a given meta type.
	pub fn new(name: Option<<MetaForm as Form>::String>, ty: MetaType) -> Self {
		Self { name, ty }
	}

	// pub fn concrete(name: Option<<MetaForm as Form>::String>, ty: MetaType) -> Self {
	// 	Self::new(name, FieldType::Concrete(ty))
	// }
	//
	// pub fn parameterized(name: Option<<MetaForm as Form>::String>, ty: MetaType) -> Self {
	// 	Self::new(name, FieldType::Parameter(ty))
	// }

	/// Creates a new named field
	pub fn named(name: <MetaForm as Form>::String, ty: MetaType) -> Self {
		Self::new(Some(name), ty)
	}

	/// Creates a new named field.
	///
	/// Use this constructor if you want to instantiate from a given
	/// compile-time type.
	pub fn named_of<T>(name: <MetaForm as Form>::String) -> Self
	where
		T: Metadata + ?Sized + 'static,
	{
		Self::new(Some(name), MetaType::new::<T>())
	}

	/// Creates a new unnamed field.
	///
	/// Use this constructor if you want to instantiate an unnamed field from a
	/// given meta type.
	pub fn unnamed(meta_type: MetaType) -> Self {
		Self::new(None, meta_type)
	}

	/// Creates a new unnamed field.
	///
	/// Use this constructor if you want to instantiate an unnamed field from a
	/// given compile-time type.
	pub fn unnamed_of<T>() -> Self
	where
		T: Metadata + ?Sized + 'static,
	{
		Self::unnamed(MetaType::new::<T>())
	}
}

/// A fields builder has no fields (e.g. a unit struct)
pub enum NoFields {}
/// A fields builder only allows named fields (e.g. a struct)
pub enum NamedFields {}
/// A fields builder only allows unnamed fields (e.g. a tuple)
pub enum UnnamedFields {}

/// Empty enum for FieldsBuilder constructors
pub enum Fields {}

impl Fields {
	pub fn unit() -> FieldsBuilder<NoFields> {
		FieldsBuilder::default()
	}

	pub fn named() -> FieldsBuilder<NamedFields> {
		FieldsBuilder::default()
	}

	pub fn unnamed() -> FieldsBuilder<UnnamedFields> {
		FieldsBuilder::default()
	}
}

pub struct FieldsBuilder<T> {
	fields: Vec<Field<MetaForm>>,
	marker: PhantomData<fn() -> T>,
}

impl<T> Default for FieldsBuilder<T> {
	fn default() -> Self {
		Self {
			fields: Vec::new(),
			marker: Default::default(),
		}
	}
}

impl<T> FieldsBuilder<T> {
	pub fn done(self) -> Vec<Field<MetaForm>> {
		self.fields
	}
}

impl FieldsBuilder<NamedFields> {
	pub fn field(self, name: &'static str, ty: MetaType) -> Self {
		let mut this = self;
		this.fields.push(Field::named(name, ty));
		this
	}

	pub fn field_of<T>(self, name: &'static str) -> Self
	where
		T: Metadata + ?Sized + 'static,
	{
		let mut this = self;
		this.fields.push(Field::named_of::<T>(name));
		this
	}
}

impl FieldsBuilder<UnnamedFields> {
	pub fn field(self, ty: MetaType) -> Self {
		let mut this = self;
		this.fields.push(Field::unnamed(ty));
		this
	}

	pub fn field_of<T>(self) -> Self
	where
		T: Metadata + ?Sized + 'static,
	{
		let mut this = self;
		this.fields.push(Field::unnamed_of::<T>());
		this
	}
}
