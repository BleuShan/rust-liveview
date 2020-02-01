use super::*;

#[derive(Debug, FromMeta)]
pub(crate) struct MainEntryPointArgs {
    executor: Executor,
}

impl Parse for MainEntryPointArgs {
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
pub(crate) struct MainEntryPoint {
    args: MainEntryPointArgs,
    item: ItemFn,
}

impl ToTokens for MainEntryPoint {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let MainEntryPoint { item, args } = self;
        let mut sig = item.sig.clone();

        if sig.asyncness.is_none() {
            abort!(
                sig.fn_token.span(), "Only async functions are supported";
                help = "Consider writing the signature as:\n\nasync {}", sig.to_token_stream();
            );
        }

        let vis = &item.vis;
        let attrs = &item.attrs;
        let body = &item.block;
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
        sig.asyncness = None;
        tokens.extend(quote! {
            #(#attrs)*
            #vis #sig {
                #block_on(async move {
                    #body
                })
            }
        })
    }
}

impl From<MainEntryPoint> for TokenStream {
    fn from(entry: MainEntryPoint) -> Self {
        Self::from(quote! {
            #entry
        })
    }
}
