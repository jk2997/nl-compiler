/*!

  Traits to abstract mapping symbols to gates.

*/

/// A trait to specify how to map primitive instantiation names ([Identifier]s) to the instance [Instantiable] type.
pub trait FromId: Sized {
    /// Maps primitive instantiation names ([Identifier]s) to the instance [Instantiable] type.
    fn from_id(s: &safety_net::circuit::Identifier) -> Result<Self, String>;
}
