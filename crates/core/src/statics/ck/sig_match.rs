//! Checks whether an environment matches a signature.
//!
//! As noted by the Definition, when matching a signature against an environment, the type functions
//! from the environment that correspond to the bound type names in the signature are used as the
//! range of the type realization, with the type names themselves being the domain.
//!
//! Type names in Millet are symbols (`Sym`s). Symbols are essentially globally unique identifiers -
//! that is, what a symbol semantically "means" (i.e. what it refers to) does not change as a result
//! of definitions coming into or going out of scope. This distinguishes it from e.g. `StrRef`.
//!
//! To implement signature matching, we must substitute the symbols bound by the signature for the
//! corresponding symbols with the same name bound by the environment which we are matching against
//! the signature, in the context of the environment of the signature. (Note that a signature is
//! essentially a 2-tuple of a set of bound type names and and environment. Note also that when we
//! say "the same name", we mean with respect to the `StrRef` inside every `Sym`.)
//!
//! To do this, we construct a `TyRealization` and pass that down to `enrich::ck`, which applies the
//! `TyRealization` when appropriate, that is, right before we are about to check if two types
//! unify.
//!
//! This approach is mildly unfortunate, since enrichment checking is not supposed to be concerned
//! with type realizations. Is this what is called a 'leaky abstraction'? Anyway, look at
//! `enrich.rs` for more commentary.

use crate::loc::Loc;
use crate::statics::ck::enrich;
use crate::statics::ck::util::get_ty_sym;
use crate::statics::ty_rzn::TyRealization;
use crate::statics::types::{Env, Result, Sig, State, TyEnv};

/// Returns `Ok((E, ty_rzn))` iff `sig >= E` (and `ty_rzn` is the witness) and `env >> E`.
pub fn ck(st: &mut State, loc: Loc, env: Env, sig: &Sig) -> Result<(Env, TyRealization)> {
  let mut ty_rzn = TyRealization::default();
  for &bound_ty_sym in sig.ty_names.iter() {
    let ty_name = loc.wrap(bound_ty_sym.name());
    let env_ty_sym = get_ty_sym(&env, ty_name)?;
    let ty_fcn = st.tys.get(&env_ty_sym).ty_fcn.clone();
    ty_rzn.insert_ty_fcn(bound_ty_sym, ty_fcn);
  }
  enrich::ck(loc, &st.tys, &ty_rzn, &env, &sig.env)?;
  let env = Env {
    str_env: env
      .str_env
      .into_iter()
      .filter(|(name, _)| sig.env.str_env.contains_key(name))
      .collect(),
    ty_env: TyEnv {
      inner: env
        .ty_env
        .inner
        .into_iter()
        .filter(|(name, _)| sig.env.ty_env.inner.contains_key(name))
        .collect(),
    },
    val_env: env
      .val_env
      .into_iter()
      .filter(|(name, _)| sig.env.val_env.contains_key(name))
      .collect(),
  };
  Ok((env, ty_rzn))
}
