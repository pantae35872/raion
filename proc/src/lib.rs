use std::sync::Mutex;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Expr, Ident, ItemFn, Token,
};

extern crate proc_macro;

struct Instruction {
    fn_name: String,
    op_code: String,
}

static INSTRUCTIONS: Mutex<Vec<Instruction>> = Mutex::new(Vec::new());

#[proc_macro_attribute]
pub fn instruction(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let opcode_expr = parse_macro_input!(args as Expr);
    let opcode = match opcode_expr {
        Expr::Lit(expr_lit) => {
            if let syn::Lit::Int(lit_int) = expr_lit.lit {
                lit_int
                    .base10_parse::<u16>()
                    .expect("Expected a u16 literal for the opcode")
                    .to_string() // Convert literal to a string for storage
            } else {
                panic!("Expected a u16 literal for the opcode");
            }
        }
        _ => quote!(#opcode_expr).to_string(), // Store the constant name or expression as a string
    };
    let fn_name = input_fn.sig.ident.to_string();
    INSTRUCTIONS.lock().unwrap().push(Instruction {
        fn_name,
        op_code: opcode,
    });
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
            let instruction_ts =
                syn::Ident::new(&instruction.fn_name, proc_macro2::Span::call_site());
            quote! {
                common::constants::#opcode_ts => {
                    return Ok(Self {
                        instruction_executor: #instruction_ts::#instruction_ts,
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
