use super::*;

#[derive(Debug, Clone, Copy, FromMeta)]
pub(crate) enum TestRunner {
    Test,
    Theory,
    Fact,
}

impl Default for TestRunner {
    fn default() -> Self {
        Self::Test
    }
}

#[derive(Debug, FromMeta)]
pub(crate) struct TestEntryPointArgs {
    executor: Executor,
    #[darling(default)]
    runner: TestRunner,
}

impl Parse for TestEntryPointArgs {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let attrs_args: Punctuated<NestedMeta, Token![,]> =
            input.parse_terminated(NestedMeta::parse)?;
        let attrs_args: AttributeArgs = attrs_args
            .pairs()
            .map(|item| {
                match item {
                    Pair::Punctuated(meta, _) => meta,
                    Pair::End(meta) => meta,
                }
                .clone()
            })
            .collect();
        Ok(Self::from_list(&attrs_args)
            .expect_or_abort("An error occured while parsing the input."))
    }
}

#[derive(Debug, new)]
pub(crate) struct TestEntryPoint {
    args: TestEntryPointArgs,
    item: ItemFn,
}

impl ToTokens for TestEntryPoint {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let TestEntryPoint { item, args } = self;
        let mut sig = self.item.sig.clone();

        if sig.asyncness.is_none() {
            abort!(sig.fn_token.span(), "Only async functions are supported");
        }

        let vis = &item.vis;
        let attrs = &item.attrs;
        let body = &item.block;
        sig.asyncness = None;
        tokens.extend(match args.runner {
            TestRunner::Theory => {
                quote! {
                    #[theory]
                    #(#attrs)*
                }
            }
            TestRunner::Fact => {
                quote! {
                    #[fact]
                    #(#attrs)*
                }
            }
            TestRunner::Test => {
                quote! {
                    #[test]
                    #(#attrs)*
                }
            }
        });

        let block_on = match args.executor {
            Executor::AsyncStd => {
                quote! {
                    async_std::task::block_on
                }
            }
            Executor::Tokio => {
                quote! {
                    tokio::runtime::Builder::new()
                        .basic_scheduler()
                        .enable_all()
                        .build()
                        .unwrap()
                        .block_on
                }
            }
        };

        tokens.extend(quote! {
            #vis #sig {
                #block_on(async move {
                    #body
                })
            }
        })
    }
}

impl From<TestEntryPoint> for TokenStream {
    fn from(entry: TestEntryPoint) -> Self {
        Self::from(quote! {
            #entry
        })
    }
}
