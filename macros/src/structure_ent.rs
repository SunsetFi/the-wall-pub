use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::Parser;

pub fn structure_ent_impl(attr: TokenStream, item: TokenStream) -> Result<TokenStream, syn::Error> {
  let mut alignment: Option<syn::Path> = None;
  let mut permeability: Option<syn::Path> =
    Some(syn::parse_quote!(crate::permeability::Permeability::Open));
  let mut material: Option<syn::Path> = None;
  let mut toughness: Option<syn::Expr> = None;
  let mut construction_density: Option<syn::Expr> = None;

  let attr_parser = syn::meta::parser(|meta| {
    if meta.path.is_ident("alignment") {
      alignment = Some(meta.value()?.parse()?);
      Ok(())
    } else if meta.path.is_ident("permeability") {
      permeability = Some(meta.value()?.parse()?);
      Ok(())
    } else if meta.path.is_ident("material") {
      material = Some(meta.value()?.parse()?);
      Ok(())
    } else if meta.path.is_ident("toughness") {
      toughness = Some(meta.value()?.parse()?);
      Ok(())
    } else if meta.path.is_ident("construction_density") {
      construction_density = Some(meta.value()?.parse()?);
      Ok(())
    } else {
      Err(meta.error("Unsupported micro_ent property"))
    }
  });

  attr_parser.parse2(attr)?;

  let ast = syn::parse2::<syn::ItemStruct>(item)?;

  let name = &ast.ident;
  let name_str = name.to_string();
  let kind = syn::Ident::new(name_str.trim_end_matches("Structure"), name.span());
  let construction_density = match construction_density {
    Some(expr) => quote!(::core::option::Option::Some(#expr)),
    None => quote!(::core::option::Option::None),
  };
  let generated = quote! {
    #ast

    impl crate::structure_entity::StructureType for #name {
      const DEF: crate::structure_entity::StructureDef = crate::structure_entity::StructureDef {
        kind: crate::structure_entity::StructureKind::#kind,
        alignment: #alignment,
        permeability: #permeability,
        material: #material,
        toughness: crate::structure_entity::Integrity::new(#toughness).unwrap(),
        construction_density: #construction_density,
      };
    }
  };
  Ok(generated)
}
