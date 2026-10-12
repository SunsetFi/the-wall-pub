use crate::{entity::Entity, quantity::QuantityOffset};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntitySlot<E: Entity>(Option<E>);

impl<E: Entity> EntitySlot<E> {
  pub fn new() -> Self {
    Self(None)
  }

  pub fn is_empty(&self) -> bool {
    self.0.is_none()
  }

  pub fn ent(&self) -> Option<&E> {
    self.0.as_ref()
  }

  pub fn ent_mut(&mut self) -> Option<&mut E> {
    self.0.as_mut()
  }

  pub fn set_ent(&mut self, entity: E) {
    self.0 = Some(entity);
  }

  pub fn offset_quantity(&mut self, value: f64) {
    let Some(entity) = &mut self.0 else {
      return;
    };

    match entity.quantity().offset(value) {
      Some(quantity) => entity.set_quantity(quantity),
      None => {
        self.clear();
      }
    }
  }

  pub fn clear(&mut self) -> bool {
    match self.0 {
      Some(_) => {
        self.0 = None;
        true
      }
      None => false,
    }
  }
}

impl<E: Entity> Default for EntitySlot<E> {
  fn default() -> Self {
    Self(None)
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    entity::Entity,
    quantity::{Bounds, Quantity},
  };

  #[derive(Copy, Clone, PartialEq, Eq, Debug)]
  enum TestEntityKind {
    Own,
  }
  struct TestEntityDef {}

  #[derive(Debug)]
  struct TestEntityBounds {}
  impl Bounds for TestEntityBounds {
    const MIN: f64 = 0.1;
    const MAX: f64 = 1.0;
  }
  struct TestEntity {
    quantity: TestEntityQuantity,
  }

  type TestEntityQuantity = Quantity<TestEntityBounds>;

  impl Entity for TestEntity {
    type Kind = TestEntityKind;
    type Def = TestEntityDef;
    type Quantity = TestEntityQuantity;

    fn kind(&self) -> Self::Kind {
      unimplemented!()
    }

    fn def(&self) -> &'static Self::Def {
      unimplemented!()
    }

    fn quantity(&self) -> Self::Quantity {
      self.quantity
    }

    fn set_quantity(&mut self, quantity: Self::Quantity) {
      self.quantity = quantity;
    }
  }

  mod offset_quantity {
    use super::super::EntitySlot;
    use super::{TestEntity, TestEntityQuantity};
    use crate::entity::Entity;

    #[test]
    fn it_does_nothing_when_empty() {
      let mut slot: EntitySlot<TestEntity> = EntitySlot::new();
      slot.offset_quantity(1.0);
      assert!(slot.is_empty());
    }

    #[test]
    fn it_increases_quantity_when_not_empty() {
      let mut slot: EntitySlot<TestEntity> = EntitySlot::new();
      let entity = TestEntity {
        quantity: TestEntityQuantity::new(0.5).unwrap(),
      };
      slot.set_ent(entity);
      slot.offset_quantity(0.3);
      assert_eq!(
        slot.ent().unwrap().quantity(),
        TestEntityQuantity::new(0.8).unwrap()
      );
    }

    #[test]
    fn it_caps_increase_to_max() {
      let mut slot: EntitySlot<TestEntity> = EntitySlot::new();
      let entity = TestEntity {
        quantity: TestEntityQuantity::new(0.9).unwrap(),
      };
      slot.set_ent(entity);
      slot.offset_quantity(0.5);
      assert_eq!(
        slot.ent().unwrap().quantity(),
        TestEntityQuantity::new(1.0).unwrap()
      );
    }

    #[test]
    fn it_decreases_quantity_when_not_empty() {
      let mut slot: EntitySlot<TestEntity> = EntitySlot::new();
      let entity = TestEntity {
        quantity: TestEntityQuantity::new(0.5).unwrap(),
      };
      slot.set_ent(entity);
      slot.offset_quantity(-0.3);
      assert_eq!(
        slot.ent().unwrap().quantity(),
        TestEntityQuantity::new(0.2).unwrap()
      );
    }

    #[test]
    fn it_clears_quantity_when_below_min() {
      let mut slot: EntitySlot<TestEntity> = EntitySlot::new();
      let entity = TestEntity {
        quantity: TestEntityQuantity::new(0.2).unwrap(),
      };
      slot.set_ent(entity);
      slot.offset_quantity(-0.15);
      assert!(slot.is_empty());
    }
  }
}
