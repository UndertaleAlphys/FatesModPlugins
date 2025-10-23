use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, punctuated::Punctuated, visit_mut::VisitMut, Expr, ExprLit, ItemFn, LitStr,
    Meta, Token,
};

/// snake_case -> CamelCase
fn snake_to_camel(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut upper = true;
    for ch in s.chars() {
        if ch == '_' {
            upper = true;
        } else if upper {
            out.extend(ch.to_uppercase());
            upper = false;
        } else {
            out.push(ch);
        }
    }
    out
}

struct StringReplacer {
    snake: String,
    camel: String,
}

impl VisitMut for StringReplacer {
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        if let Expr::Lit(ExprLit {
            lit: syn::Lit::Str(ref mut s),
            ..
        }) = node
        {
            let v = s.value();
            let replaced = v
                .replace("__SNAKENAME__", &self.snake)
                .replace("__CAMELNAME__", &self.camel);
            if replaced != v {
                *s = LitStr::new(&replaced, s.span());
            }
        }
        syn::visit_mut::visit_expr_mut(self, node);
    }
}

/// #[gen(name1, name2, ...)]
#[proc_macro_attribute]
pub fn gen(attr: TokenStream, item: TokenStream) -> TokenStream {
    use syn::{punctuated::Punctuated, Meta, Token};

    let metas = parse_macro_input!(attr with Punctuated::<Meta, Token![,]>::parse_terminated);

    let template_fn = parse_macro_input!(item as ItemFn);
    let mut names = Vec::<String>::new();
    for m in metas {
        if let Meta::Path(path) = m {
            if let Some(ident) = path.get_ident() {
                names.push(ident.to_string());
            }
        }
    }

    let base_name = template_fn.sig.ident.to_string();
    let mut generated = Vec::new();

    for snake in names {
        let camel = snake_to_camel(&snake);
        let func_name = base_name.replace("__SNAKENAME__", &snake);
        let ident = format_ident!("{}", func_name);

        let mut fn_copy = template_fn.clone();
        fn_copy.sig.ident = ident;

        let mut replacer = StringReplacer {
            snake: snake.clone(),
            camel,
        };
        replacer.visit_block_mut(&mut fn_copy.block);

        generated.push(fn_copy);
    }

    quote! { #( #generated )* }.into()
}
