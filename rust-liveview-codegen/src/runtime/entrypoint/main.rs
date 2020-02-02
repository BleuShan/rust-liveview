use super::*;

#[derive(Debug)]
pub(crate) struct MainEntryPointArgs {
    executor: Executor,
}

impl Parse for MainEntryPointArgs {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let attrs_args: Punctuated<NestedMeta, Token![,]> =
            input.parse_terminated(NestedMeta::parse)?;
        let maybe_executor: Option<Executor> = Executor::from_iter(box attrs_args.iter());

        if let Some(executor) = maybe_executor {
            Ok(Self { executor })
        } else {
            Err(Error::new(
                Span::call_site(),
                "Missing executor configuration.",
            ))
        }
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
        let block_on = args.executor;
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
