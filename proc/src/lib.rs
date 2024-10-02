use std::sync::Mutex;

use proc_macro::TokenStream;
use proc_macro2::{Punct, TokenTree};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_str,
    punctuated::Punctuated,
    Expr, Ident, ItemFn, LitStr, Path, Token,
};

extern crate proc_macro;

struct Instruction {
    fn_path: String,
    op_code: String,
}

static INSTRUCTIONS: Mutex<Vec<Instruction>> = Mutex::new(Vec::new());

impl Parse for Instruction {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let op_code = Ident::parse(input)?;
        input.step(
            |cursor| match cursor.token_tree().ok_or(cursor.error("Expected `,`"))? {
                (TokenTree::Punct(punct), next) => match punct.to_string().as_str() {
                    "," => return Ok(((), next)),
                    unexpected => {
                        return Err(cursor.error(format!("Expected `,` found `{unexpected}`")))
                    }
                },
                (unexpected, ..) => Err(cursor.error(format!("Expected `,` found `{unexpected}`"))),
            },
        )?;

        let fn_path: LitStr = Parse::parse(input)?;
        let fn_path = fn_path.parse::<Path>()?;

        Ok(Instruction {
            fn_path: quote!(#fn_path).to_string(),
            op_code: op_code.to_string(),
        })
    }
}

#[proc_macro_attribute]
pub fn instruction(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    INSTRUCTIONS
        .lock()
        .unwrap()
        .push(parse_macro_input!(args as Instruction));
    quote!(#input_fn).into()
}

struct VariableArgs {
    identifiers: Punctuated<Ident, Token![,]>,
}

impl Parse for VariableArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let identifiers = input.parse_terminated(Ident::parse, Token![,])?;

        Ok(VariableArgs { identifiers })
    }
}

#[proc_macro]
pub fn collect_instruction(args: TokenStream) -> TokenStream {
    let instruction = INSTRUCTIONS.lock().unwrap();
    let input = parse_macro_input!(args as VariableArgs);
    let op_code_var = input.identifiers.get(0).expect("Invalid argument");
    let register_var = input.identifiers.get(1).expect("Invalid argument");
    let memory_var = input.identifiers.get(2).expect("Invalid argument");
    let argument_var = input.identifiers.get(3).expect("Invalid argument");
    let instruction_length_var = input.identifiers.get(4).expect("Invalid argument");
    let ret_stack_var = input.identifiers.get(5).expect("Invalid argument");
    let section_manager_var = input.identifiers.get(6).expect("Invalid argument");
    let executor_state_var = input.identifiers.get(7).expect("Invalid argument");
    let decode_logic = instruction
        .iter()
        .map(|instruction| {
            let opcode_ts = syn::Ident::new(&instruction.op_code, proc_macro2::Span::call_site());
            let instruction_ts: Path =
                parse_str(&instruction.fn_path).expect("Failed to parse path");
            quote! {
                common::constants::#opcode_ts => {
                    return Ok(Self {
                        instruction_executor: #instruction_ts,
                        instruction_argument: InstructionArgument {
                            register: #register_var,
                            memory: #memory_var,
                            argument: #argument_var,
                            instruction_length: #instruction_length_var,
                            ret_stack: #ret_stack_var,
                            section_manager: #section_manager_var,
                            executor_state: #executor_state_var
                        },
                        opcode: #op_code_var,
                    })
                },
            }
        })
        .collect::<Vec<_>>();
    let output = quote! {
        match #op_code_var {
            #(#decode_logic)*
            iop_code => return Err(DecoderError::InvalidOpCode(iop_code)),
        }
    };

    return output.into();
}
