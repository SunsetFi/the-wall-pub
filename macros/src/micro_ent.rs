use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::Parser;

pub fn micro_ent_impl(attr: TokenStream, item: TokenStream) -> Result<TokenStream, syn::Error> {
  let mut alignment: Option<syn::Path> = None;
  let mut permeability: Option<syn::Path> =
    Some(syn::parse_quote!(crate::permeability::Permeability::Open));
  let mut material: Option<syn::Path> = None;
  let mut cohesion: Option<syn::Expr> = None;
  let mut spread_threshold: Option<syn::Expr> = None;
  let mut empty_conductance: Option<syn::Expr> = None;
  let mut peer_conductance: Option<syn::Expr> = None;
  let mut combat_power: Option<syn::Expr> = Some(syn::parse_str("1.0").unwrap());
  let mut combat_power_vs: Option<syn::Expr> = Some(syn::parse_str("&[]").unwrap());
  let mut combat_loss_saturation: Option<syn::Expr> = None;

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
    } else if meta.path.is_ident("cohesion") {
      cohesion = Some(meta.value()?.parse()?);
      Ok(())
    } else if meta.path.is_ident("spread_threshold") {
      spread_threshold = Some(meta.value()?.parse()?);
      Ok(())
    } else if meta.path.is_ident("empty_conductance") {
      empty_conductance = Some(meta.value()?.parse()?);
      Ok(())
    } else if meta.path.is_ident("peer_conductance") {
      peer_conductance = Some(meta.value()?.parse()?);
      Ok(())
    } else if meta.path.is_ident("combat_power") {
      combat_power = Some(meta.value()?.parse()?);
      Ok(())
    } else if meta.path.is_ident("combat_power_vs") {
      combat_power_vs = Some(meta.value()?.parse()?);
      Ok(())
    } else if meta.path.is_ident("combat_loss_saturation") {
      combat_loss_saturation = Some(meta.value()?.parse()?);
      Ok(())
    } else {
      Err(meta.error("Unsupported micro_ent property"))
    }
  });

  attr_parser.parse2(attr)?;

  let ast = syn::parse2::<syn::ItemStruct>(item)?;

  let name = &ast.ident;
  let name_str = name.to_string();
  let kind = syn::Ident::new(name_str.trim_end_matches("Micro"), name.span());
  let generated = quote! {
    #ast

    impl crate::micro_entity::MicroType for #name {
      const DEF: crate::micro_entity::MicroDef = crate::micro_entity::MicroDef {
        kind: crate::micro_entity::MicroKind::#kind,
        alignment: #alignment,
        permeability: #permeability,
        material: #material,
        cohesion: #cohesion,
        spread_threshold: #spread_threshold,
        empty_conductance: #empty_conductance,
        peer_conductance: #peer_conductance,
        combat_power: #combat_power,
        combat_power_vs: #combat_power_vs,
        combat_loss_saturation: #combat_loss_saturation,
      };
    }
  };
  Ok(generated)
}
