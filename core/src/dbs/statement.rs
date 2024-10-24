use crate::sql::cond::Cond;
use crate::sql::data::Data;
use crate::sql::fetch::Fetchs;
use crate::sql::field::Fields;
use crate::sql::group::Groups;
use crate::sql::idiom::Idioms;
use crate::sql::limit::Limit;
use crate::sql::order::Ordering;
use crate::sql::output::Output;
use crate::sql::split::Splits;
use crate::sql::start::Start;
use crate::sql::statements::access::AccessStatement;
use crate::sql::statements::create::CreateStatement;
use crate::sql::statements::delete::DeleteStatement;
use crate::sql::statements::insert::InsertStatement;
use crate::sql::statements::live::LiveStatement;
use crate::sql::statements::relate::RelateStatement;
use crate::sql::statements::select::SelectStatement;
use crate::sql::statements::show::ShowStatement;
use crate::sql::statements::update::UpdateStatement;
use crate::sql::statements::upsert::UpsertStatement;
use crate::sql::Explain;
use std::fmt;

#[derive(Clone, Debug)]
pub(crate) enum Statement<'a> {
	Live(&'a LiveStatement),
	Show(&'a ShowStatement),
	Select(&'a SelectStatement),
	Create(&'a CreateStatement),
	Upsert(&'a UpsertStatement),
	Update(&'a UpdateStatement),
	Relate(&'a RelateStatement),
	Delete(&'a DeleteStatement),
	Insert(&'a InsertStatement),
	// TODO(gguillemas): Document once bearer access is no longer experimental.
	#[doc(hidden)]
	Access(&'a AccessStatement),
}

impl<'a> From<&'a LiveStatement> for Statement<'a> {
	fn from(v: &'a LiveStatement) -> Self {
		Statement::Live(v)
	}
}

impl<'a> From<&'a ShowStatement> for Statement<'a> {
	fn from(v: &'a ShowStatement) -> Self {
		Statement::Show(v)
	}
}

impl<'a> From<&'a SelectStatement> for Statement<'a> {
	fn from(v: &'a SelectStatement) -> Self {
		Statement::Select(v)
	}
}

impl<'a> From<&'a CreateStatement> for Statement<'a> {
	fn from(v: &'a CreateStatement) -> Self {
		Statement::Create(v)
	}
}

impl<'a> From<&'a UpsertStatement> for Statement<'a> {
	fn from(v: &'a UpsertStatement) -> Self {
		Statement::Upsert(v)
	}
}

impl<'a> From<&'a UpdateStatement> for Statement<'a> {
	fn from(v: &'a UpdateStatement) -> Self {
		Statement::Update(v)
	}
}

impl<'a> From<&'a RelateStatement> for Statement<'a> {
	fn from(v: &'a RelateStatement) -> Self {
		Statement::Relate(v)
	}
}

impl<'a> From<&'a DeleteStatement> for Statement<'a> {
	fn from(v: &'a DeleteStatement) -> Self {
		Statement::Delete(v)
	}
}

impl<'a> From<&'a InsertStatement> for Statement<'a> {
	fn from(v: &'a InsertStatement) -> Self {
		Statement::Insert(v)
	}
}

impl<'a> From<&'a AccessStatement> for Statement<'a> {
	fn from(v: &'a AccessStatement) -> Self {
		Statement::Access(v)
	}
}

impl<'a> fmt::Display for Statement<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Statement::Live(v) => write!(f, "{v}"),
			Statement::Show(v) => write!(f, "{v}"),
			Statement::Select(v) => write!(f, "{v}"),
			Statement::Create(v) => write!(f, "{v}"),
			Statement::Upsert(v) => write!(f, "{v}"),
			Statement::Update(v) => write!(f, "{v}"),
			Statement::Relate(v) => write!(f, "{v}"),
			Statement::Delete(v) => write!(f, "{v}"),
			Statement::Insert(v) => write!(f, "{v}"),
			Statement::Access(v) => write!(f, "{v}"),
		}
	}
}

impl<'a> Statement<'a> {
	/// Check if this is a SELECT statement
	pub fn is_select(&self) -> bool {
		matches!(self, Statement::Select(_))
	}
	/// Check if this is a CREATE statement
	pub fn is_create(&self) -> bool {
		matches!(self, Statement::Create(_))
	}
	/// Check if this is a DELETE statement
	pub fn is_delete(&self) -> bool {
		matches!(self, Statement::Delete(_))
	}
	/// Returns whether retrieval can be deferred
	pub fn is_deferable(&self) -> bool {
		matches!(self, Statement::Create(_) | Statement::Upsert(_))
	}
	/// Returns whether this requires savepoints
	pub fn is_retryable(&self) -> bool {
		matches!(self, Statement::Insert(_) if self.data().is_some())
	}
	/// Returns whether the IGNORE clause is et
	pub fn is_ignore(&self) -> bool {
		matches!(self, Statement::Insert(i) if i.ignore)
	}
	/// Returns any query fields if specified
	pub fn expr(&self) -> Option<&Fields> {
		match self {
			Statement::Select(v) => Some(&v.expr),
			Statement::Live(v) => Some(&v.expr),
			_ => None,
		}
	}
	/// Returns any OMIT clause if specified
	pub fn omit(&self) -> Option<&Idioms> {
		match self {
			Statement::Select(v) => v.omit.as_ref(),
			_ => None,
		}
	}
	/// Returns any SET, CONTENT, or MERGE clause if specified
	pub fn data(&self) -> Option<&Data> {
		match self {
			Statement::Create(v) => v.data.as_ref(),
			Statement::Upsert(v) => v.data.as_ref(),
			Statement::Update(v) => v.data.as_ref(),
			Statement::Relate(v) => v.data.as_ref(),
			Statement::Insert(v) => v.update.as_ref(),
			_ => None,
		}
	}
	/// Returns any WHERE clause if specified
	pub fn conds(&self) -> Option<&Cond> {
		match self {
			Statement::Live(v) => v.cond.as_ref(),
			Statement::Select(v) => v.cond.as_ref(),
			Statement::Upsert(v) => v.cond.as_ref(),
			Statement::Update(v) => v.cond.as_ref(),
			Statement::Delete(v) => v.cond.as_ref(),
			_ => None,
		}
	}
	/// Returns any SPLIT clause if specified
	pub fn split(&self) -> Option<&Splits> {
		match self {
			Statement::Select(v) => v.split.as_ref(),
			_ => None,
		}
	}
	/// Returns any GROUP clause if specified
	pub fn group(&self) -> Option<&Groups> {
		match self {
			Statement::Select(v) => v.group.as_ref(),
			_ => None,
		}
	}
	/// Returns any ORDER clause if specified
	pub fn order(&self) -> Option<&Ordering> {
		match self {
			Statement::Select(v) => v.order.as_ref(),
			_ => None,
		}
	}
	/// Returns any FETCH clause if specified
	pub fn fetch(&self) -> Option<&Fetchs> {
		match self {
			Statement::Select(v) => v.fetch.as_ref(),
			_ => None,
		}
	}
	/// Returns any START clause if specified
	pub fn start(&self) -> Option<&Start> {
		match self {
			Statement::Select(v) => v.start.as_ref(),
			_ => None,
		}
	}
	/// Returns any LIMIT clause if specified
	pub fn limit(&self) -> Option<&Limit> {
		match self {
			Statement::Select(v) => v.limit.as_ref(),
			_ => None,
		}
	}
	/// Returns any RETURN clause if specified
	pub fn output(&self) -> Option<&Output> {
		match self {
			Statement::Create(v) => v.output.as_ref(),
			Statement::Upsert(v) => v.output.as_ref(),
			Statement::Update(v) => v.output.as_ref(),
			Statement::Relate(v) => v.output.as_ref(),
			Statement::Delete(v) => v.output.as_ref(),
			Statement::Insert(v) => v.output.as_ref(),
			_ => None,
		}
	}
	/// Returns any PARALLEL clause if specified
	#[cfg(not(target_arch = "wasm32"))]
	pub fn parallel(&self) -> bool {
		match self {
			Statement::Select(v) => v.parallel,
			Statement::Create(v) => v.parallel,
			Statement::Upsert(v) => v.parallel,
			Statement::Update(v) => v.parallel,
			Statement::Relate(v) => v.parallel,
			Statement::Delete(v) => v.parallel,
			Statement::Insert(v) => v.parallel,
			_ => false,
		}
	}
	/// Returns any TEMPFILES clause if specified
	#[cfg(storage)]
	pub fn tempfiles(&self) -> bool {
		match self {
			Statement::Select(v) => v.tempfiles,
			_ => false,
		}
	}
	/// Returns any EXPLAIN clause if specified
	pub fn explain(&self) -> Option<&Explain> {
		match self {
			Statement::Select(v) => v.explain.as_ref(),
			_ => None,
		}
	}
}
