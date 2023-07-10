use higher::{Apply, Bind, Functor, Pure};

#[derive(Debug, PartialEq, Eq)]
pub struct OptionT<M> {
    pub value: M,
}

impl<M> From<M> for OptionT<M> {
    fn from(v: M) -> Self {
        Self { value: v }
    }
}

impl<'a, A, M> Functor<'a, A> for OptionT<M>
where
    M: Functor<'a, Option<A>>,
{
    type Target<B> = OptionT<<M as Functor<'a, Option<A>>>::Target<Option<B>>>;

    fn fmap<B, F>(self, f: F) -> Self::Target<B>
    where
        F: Fn(A) -> B + 'a,
    {
        OptionT {
            value: self.value.fmap(move |v| match v {
                None => None,
                Some(vv) => Some(f(vv)),
            }),
        }
    }
}

impl<A, M> Pure<Option<A>> for OptionT<M>
where
    M: Pure<Option<A>>,
{
    fn pure(value: Option<A>) -> Self {
        OptionT {
            value: M::pure(value),
        }
    }
}

impl<'a, A, M> Apply<'a, A> for OptionT<M>
where
    A: 'a,
    M: Apply<'a, Option<A>>,
{
    type Target<B> = OptionT<<M as Apply<'a, Option<A>>>::Target<Option<B>>> where B: 'a;

    fn apply<B>(
        self,
        f: <Self as Apply<'a, A>>::Target<higher::apply::ApplyFn<'a, A, B>>,
    ) -> <Self as Apply<'a, A>>::Target<B>
    where
        B: 'a,
    {
        todo!();
    }
}

impl<'a, A, M> Bind<'a, A> for OptionT<M>
where
    M: Pure<Option<A>> + Bind<'a, Option<A>>,
{
    type Target<B> = OptionT<M::Target<Option<B>>> 
        where M: Bind<'a, Option<A>>;

    fn bind<B, F>(self, f: F) -> Self::Target<B>
    where
        F: Fn(A) -> Self::Target<B> + 'a,
    {
        let new_value: M::Target<Option<B>> = self.value.bind(move |oa| {
            match oa {
                Some(a) => f(a).value,

                /*
                i believe that this is the right expression, however i cannot figure out
                how i can constrain `M::Target<B>: Pure<B>` in this code.
                • the definition of the `Bind` interface has specific contraints that
                  cannot be modified.
                • `<B>` is only defined for this function, so it it can't be constrained
                  for the impl. `<B>` is not there… but I think that's where I want it
                  to be.

                -- no function or associated item named `pure` found for associated
                   type `<M as Bind<'a, Option<A>>>::Target<B>` in the current scope
                */
                //None => M::Target::<B>::pure(None),
                None => todo!(),
            }
        });
        OptionT { value: new_value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn upper(s: String) -> String {
        s.to_uppercase()
    }

    #[test]
    fn test_option_option_fmap() {
        let oo = Some(Some(String::from("a")));
        let ot1 = OptionT::from(oo);
        let ot2 = ot1.fmap(upper);
        assert_eq!(ot2.value, Some(Some(String::from("A"))))
    }

    #[test]
    fn test_option_option_pure() {
        let o = Some("a");
        let p = OptionT::<Option<_>>::pure(o);
        assert_eq!(p.value, Some(Some("a")))
    }

    #[test]
    fn test_result_option_pure() {
        type Error = String;
        let o = Some("a");
        let p = OptionT::<Result<_, Error>>::pure(o);
        assert_eq!(p.value, Ok(Some("a")))
    }

    #[test]
    fn option_option_apply() {
        let ot = OptionT::from(Some(Some("x")));
    }

    #[test]
    fn test_option_option_bind() {
        let ot1 = OptionT::from(Some(Some(String::from("a"))));
        let ot2 = ot1.bind(|o| OptionT::pure(Some(upper(o))));
        assert_eq!(ot2.value, Some(Some("A".to_string())))
    }
}
