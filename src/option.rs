use higher::*;

#[derive(Debug, PartialEq, Eq)]
pub struct OptionT<M> {
    pub value: M,
}

impl<M> From<M> for OptionT<M> {
    fn from(v: M) -> Self {
        Self { value: v }
    }
}

impl<'a, A: 'a, M> Functor<'a, A> for OptionT<M>
where
    A: 'a,
    M: Functor<'a, Option<A>>,
{
    type Target<B: 'a> = OptionT<M::Target<Option<B>>>;

    fn fmap<B: 'a, F>(self, f: F) -> Self::Target<B>
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
    fn apply<B: 'a>(self, f: Self::Target<higher::apply::ApplyFn<'a, A, B>>) -> Self::Target<B> {
        todo!()
    }
}

impl<'a, A: 'a, M> Bind<'a, A> for OptionT<M>
where
    M: Monad<'a, Option<A>>,
{
    fn bind<B, F: 'a>(self, f: F) -> Self::Target<B>
    where
        F: Fn(A) -> Self::Target<B>,
    {
        let nv = self.value.bind(move |ov| {
            match ov {
                Some(v) => f(v).value,
                None => {
                    // let b: Option<B> = None;
                    // let pb: M::Target<Option<B>> = M::Target::<Option<B>>::pure(b);
                    // pb
                    todo!()
                }
            }
        });

        OptionT { value: nv }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(PartialEq, Eq, Debug)]
    enum TestErr {}

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
    #[test]

    fn test_option_option_bind_empty() {
        let ot1 = OptionT::from(Some(None));
        let ot2 = ot1.bind(|o| OptionT::pure(Some(upper(o))));
        assert_eq!(ot2.value, Some(None))
    }

    #[test]
    fn result_of() {
        let ro1: Result<Option<String>, TestErr> = Ok(Some("a".to_string()));
        let ro2 = OptionT::from(ro1).fmap(upper).value;
        assert_eq!(ro2, Ok(Some("A".to_string())))
    }
}
