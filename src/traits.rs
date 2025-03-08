use trait_set::trait_set;

trait_set! {
  pub trait Parallelism = Send + Sync + 'static;
}
