use pak_db::{builder::PakBuilder, index::PakSearchable, item::PakSerialize};
use serde::{Deserialize, Serialize};

use crate::{common::{identifier::Identifier, meta::HasItemMeta}, error::{FreError, FreException}};

#[derive(Clone, PartialEq, Debug)]
pub enum RulebookValue<T> where T : Serialize + for<'d> Deserialize<'d> + PakSearchable + HasItemMeta {
    Value(T),
    Ref(Identifier)
}

impl <T> RulebookValue<T> where T : Serialize + for<'de> Deserialize<'de> + PakSearchable + HasItemMeta {
    pub fn save(&mut self, builder : &mut PakBuilder) -> FreException {
        if self.is_value() {
            let old = std::mem::replace(self, RulebookValue::Ref(Identifier::default()));
            builder.pak(old.as_value().unwrap())?;
        }
        Ok(())
    }

    /// Returns `true` if the rulebook value is [`Value`].
    ///
    /// [`Value`]: RulebookValue::Value
    #[must_use]
    pub fn is_value(&self) -> bool {
        matches!(self, Self::Value(..))
    }

    /// Returns `true` if the rulebook value is [`Ref`].
    ///
    /// [`Ref`]: RulebookValue::Ref
    #[must_use]
    pub fn is_ref(&self) -> bool {
        matches!(self, Self::Ref(..))
    }

    pub fn as_value(&self) -> Option<&T> {
        if let Self::Value(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl <T> From<T> for RulebookValue<T> where T : Serialize + for<'de> Deserialize<'de> + PakSearchable + HasItemMeta {
    fn from(value: T) -> Self {
        RulebookValue::Value(value)
    }
}

impl <T> TryFrom<Identifier> for RulebookValue<T> where T : Serialize + for<'de> Deserialize<'de> + PakSearchable + HasItemMeta {
    type Error = FreError;

    fn try_from(value: Identifier) -> Result<Self, Self::Error> {
        if !value.is_type::<T>() { return Err(FreError::IdentifierTypeMismatch(value.type_name().to_string(), std::any::type_name::<T>().to_string()))}
        Ok(RulebookValue::Ref(value))
    }
}